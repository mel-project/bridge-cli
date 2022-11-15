mod cli;

use std::io::{BufReader, Read, Stdin, Write};
use std::sync::Mutex;
use std::path::Path;
use std::convert::TryFrom;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use melorun::LoadFileError;
use mil::compiler::{BinCode, Compile};
use once_cell::sync::Lazy;
use smol;
use stdcode::StdcodeSerializeExt;
use tabwriter::TabWriter;
use themelio_stf::melvm::Covenant;
use themelio_structs::{
    CoinData,
    CoinID,
    CoinValue,
    NetID,
    Transaction,
    TxHash,
    TxKind,
};

use cli::{*};
use tmelcrypt::HashVal;

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
        anyhow::bail!("Canceled");
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
    //wallet: WalletClient,
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

    //let tx_hash = wallet.send_tx(tx).await?;
    let tx_hash = TxHash(HashVal::random());

    //write_txhash(&mut twriter, wallet.name(), tx_hash)?;
    write_txhash(&mut twriter, "todo", tx_hash)?;

    Ok(())
}

async fn freeze(
    wallet: String,
    mut twriter: impl Write,
    freeze_args: &FreezeAndMintArgs,
    dry_run: &bool,
) -> Result<()> {
    let inputs: Vec<CoinID> = vec!();
    let cov = compile_cov()?;
    let output = CoinData{
        covhash: cov.hash(),
        value: freeze_args.value,
        denom: freeze_args.denom,
        additional_data: freeze_args.ethereum_recipient.0.into(),
    };
    let fee = CoinValue(300); // later actually calculate fee

    let tx = Transaction::new(TxKind::Normal)
        .with_inputs(inputs)
        .add_output(output)
        .with_fee(fee);

    if *dry_run {
        println!("Wallet: {}\nTransaction: {:#?}", wallet, tx);
        println!("{}", hex::encode(tx.stdcode()));
    } else {
        send_tx(&mut twriter, tx.clone()).await?;
        println!("{}", serde_json::to_string_pretty(&tx)?);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let twriter = TabWriter::new(std::io::stderr());

    let args = Cli::parse();
    let subcommand = &args.subcommand;
    let dry_run = &args.dry_run;

    let config = Config::try_from(args.clone())
        .expect("Unable to create config from CLI args");

    let network_id = if config.testnet {
        NetID::Testnet
    } else {
        NetID::Mainnet
    };

    let wallet_id = format!("{}{:?}", config.wallet_name, network_id);

    match subcommand {
        Subcommand::FreezeAndMint(args) => {
            println!("{:#?}", args);
            println!("{:#?}", config);

            freeze(wallet_id, twriter, args, dry_run).await?;
        }

        Subcommand::BurnAndThaw(sub_args) => println!("{:?}", sub_args),
    }

    Ok(())
}