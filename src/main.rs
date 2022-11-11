mod cli;

use std::io::{BufReader, Read, Stdin, Write};
use std::process::Command;
use std::time::Duration;
use std::sync::Mutex;
use std::path::Path;
use std::process::Stdio;
use std::convert::TryFrom;

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use melorun::LoadFileError;
use melwallet_client::{DaemonClient, WalletClient};
use mil::compiler::{BinCode, Compile};
use once_cell::sync::Lazy;
use prodash::Tree;
use serde_big_array::big_array;
use smol;
use stdcode::StdcodeSerializeExt;
use tabwriter::TabWriter;
use tap::Tap;
use themelio_stf::melvm::Covenant;
use themelio_structs::{
    CoinData,
    CoinID,
    NetID,
    Transaction,
    TxHash,
    TxKind,
};

use cli::{*};

big_array! { BigArray; }

const COV_PATH: &str = "bridge-covenants/bridge.melo";
static STDIN_BUFFER: Lazy<Mutex<BufReader<Stdin>>> =
    Lazy::new(|| Mutex::new(BufReader::new(std::io::stdin())));

fn compile_cov() -> Result<Covenant> {
    let cov_path = Path::new(COV_PATH);

    let melo_str = std::fs::read_to_string(cov_path)
        .map_err(LoadFileError::IoError)?;

    let (s, _) = melodeon::compile(&melo_str, cov_path)
        .map_err(LoadFileError::MeloError)?;

    let parsed = mil::parser::parse_no_optimize(&s)
        .expect("BUG: mil compilation failed");

    let melvm_ops = parsed
        .compile_onto(BinCode::default())
        .0;

    let covenant = Covenant::from_ops(&melvm_ops)?;

    Ok(covenant)
}

async fn proceed_prompt() -> anyhow::Result<()> {
    eprintln!("Proceed? [y/N] ");

    let letter = smol::unblock(move || {
        let mut letter = [0u8; 1];

        match STDIN_BUFFER.lock().as_deref_mut() {
            Ok(stdin) => {
                while letter[0].is_ascii_whitespace() || letter[0] == 0 {
                    stdin.read_exact(&mut letter)?;
                }
                Ok(letter)
            }

            Err(_) => return Err(anyhow::anyhow!("unknown buffer unlock problem")),
        }
    })
    .await?;

    if letter[0].to_ascii_lowercase() != b'y' {
        anyhow::bail!("canceled");
    }

    Ok(())
}

fn write_txhash(out: &mut impl Write, wallet_name: &str, txhash: TxHash) -> anyhow::Result<()> {
    writeln!(out, "Transaction hash:\t{}", txhash.to_string().bold())?;
    writeln!(
        out,
        "(wait for confirmation with {})",
        format!(
            "melwallet-cli wait-confirmation -w {} {}",
            wallet_name, txhash
        )
        .bright_blue(),
    )?;
    Ok(())
}

async fn send_tx(
    mut twriter: impl Write,
    wallet: WalletClient,
    tx: Transaction,
) -> Result<()> {
    writeln!(twriter, "{}", "TRANSACTION RECIPIENTS".bold())?;
    writeln!(twriter, "{}", "Address\tAmount\tAdditional data".italic())?;

    for output in tx.outputs.iter() {
        writeln!(
            twriter,
            "{}\t{} {}\t{:?}",
            output.covhash.to_string().bright_blue(),
            output.value,
            output.denom,
            hex::encode(&output.additional_data)
        )?;
    }

    writeln!(twriter, "{}\t{} MEL", " (network fees)".yellow(), tx.fee)?;

    twriter.flush()?;

    proceed_prompt().await?;

    let txhash = wallet.send_tx(tx).await?;

    write_txhash(&mut twriter, wallet.name(), txhash)?;

    Ok(())
}

async fn freeze(wallet: &WalletClient, twriter: impl Write, args: &Cli, config: &Config) -> Result<bool> {
    let force_spend: Vec<CoinID> = vec!();
    let desired_outputs: Vec<CoinData> = vec!();
    let covenants: Vec<Covenant> = vec!(compile_cov()?);
    let fee_ballast: usize = 300;

    let tx = wallet
        .prepare_transaction(
            TxKind::Normal,
            force_spend,
            desired_outputs,
            covenants,
            vec![],
            vec![],
            fee_ballast,
        )
        .await?;

    if args.dry_run {
        println!("{}", hex::encode(tx.stdcode()));
    } else {
        send_tx(&mut twriter, *wallet, tx.clone()).await?;
        println!("{}", serde_json::to_string_pretty(&tx)?);
    }

    Ok(true)
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut twriter = TabWriter::new(std::io::stderr());

    let args = Cli::parse();
    let subcommand = &args.subcommand;
    let dry_run = &args.dry_run;

    let config = Config::try_from(args.clone())
        .expect("Unable to create config from cmd args");

    let dash_root = Tree::new();
    // let dash_options = Options::default();

    env_logger::init();

    // either use provided daemon or spawn a new one
    let mut _running_daemon = None;
    let daemon_addr = if let Some(addr) = config.daemon_addr {
        addr
    } else {
        // spawn daemon
        let port = fastrand::usize(5000..15000);
        let daemon = Command::new("melwalletd")
            .arg("--listen")
            .arg(format!("127.0.0.1:{}", port))
            .arg("--wallet-dir")
            .arg(dirs::config_dir().unwrap().tap_mut(|p| p.push("bridge-cli")))
            .stderr(Stdio::null())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
            .unwrap();

        smol::Timer::after(Duration::from_secs(1)).await;

        _running_daemon = Some(daemon);

        format!("127.0.0.1:{}", port).parse().unwrap()
    };

    scopeguard::defer!({
        if let Some(mut d) = _running_daemon {
            let _ = d.kill();
        }
    });

    let daemon = DaemonClient::new(daemon_addr);
    let network_id = if config.testnet {
        NetID::Testnet
    } else {
        NetID::Mainnet
    };

    let wallet_name = format!("{}{:?}", config.wallet_name, network_id);
    let wallet = match daemon.get_wallet(&wallet_name).await? {
        Some(wallet) => wallet,
        None => {
            let mut evt = dash_root.add_child(format!("creating new wallet {}", wallet_name));
            evt.init(None, None);

            log::info!("Creating new wallet");

            daemon
                .create_wallet(&wallet_name, config.testnet, None, None)
                .await?;

            daemon
                .get_wallet(&wallet_name)
                .await?
                .context("Wallet creation failed")?
        }
    };

    wallet.unlock(None).await?;

    match subcommand {
        Subcommand::FreezeAndMint(sub_args) => {
            println!("{:?}", sub_args);

            freeze(&wallet, twriter, &args, &config);
        }

        Subcommand::BurnAndThaw(sub_args) => println!("{:?}", sub_args),
    }

    Ok(())
}