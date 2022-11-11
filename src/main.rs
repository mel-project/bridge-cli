mod cli;

use std::process::Stdio;
use std::time::Duration;
use std::{path::Path, process::Command};
use std::convert::TryFrom;

use anyhow::{Context, Result};
use clap::Parser;
use melorun::LoadFileError;
use melwallet_client::DaemonClient;
use mil::compiler::{BinCode, Compile};
use prodash::{
    tree::Options,
    render,
    Tree,
};
use serde_big_array::big_array;
use smol;
use tap::Tap;
use themelio_stf::melvm::Covenant;
use themelio_structs::NetID;

use cli::{*};

big_array! { BigArray; }

const COV_PATH: &str = "bridge-covenants/bridge.melo";

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

async fn freeze(config: &Config) -> Result<bool> {
    Ok(true)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let subcommand = &args.subcommand;
    let dry_run = &args.dry_run;

    match subcommand {
        Subcommand::FreezeAndMint(sub_args) => {
            // let wallet = wargs.wallet().await?;
            // let desired_outputs = to.iter().map(|v| v.0.clone()).collect::<Vec<_>>();
            // let tx = wallet
            //     .prepare_transaction(
            //         TxKind::Normal,
            //         force_spend,
            //         desired_outputs,
            //         add_covenant
            //             .into_iter()
            //             .map(|s| Ok(Covenant(hex::decode(&s)?)))
            //             .collect::<anyhow::Result<Vec<_>>>()?,
            //         vec![],
            //         vec![],
            //         fee_ballast,
            //     )
            //     .await?;
            // if dry_run {
            //     println!("{}", hex::encode(tx.stdcode()));
            //     (hex::encode(tx.stdcode()), wargs.common)
            // } else {
            //     send_tx(&mut twriter, wallet, tx.clone()).await?;
            //     (serde_json::to_string_pretty(&tx)?, wargs.common)
            // }
            println!("{:?}", sub_args)
        }
        Subcommand::BurnAndThaw(sub_args) => println!("{:?}", sub_args),
    }

    let cov = compile_cov()?;

    println!("{:?}", cov);

    let config = Config::try_from(args.clone())
        .expect("Unable to create config from cmd args");

    println!("{:?}", config);

    if *dry_run {
        return Ok(());
    }

    let dash_root = Tree::new();
    // let dash_options = Options::default();

    env_logger::init();

    // either start a daemon, or use the provided one
    let mut _running_daemon = None;
    let daemon_addr = if let Some(addr) = config.daemon_addr {
        addr
    } else {
        // start a daemon naw
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

    Ok(())
}