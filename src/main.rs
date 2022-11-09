mod cli;

use cli::{*};

use std::{path::Path, str::FromStr};
use std::convert::TryFrom;

use anyhow::Result;
use clap::Parser;
use melorun::LoadFileError;
use mil::compiler::{BinCode, Compile};
use serde::{Deserialize, Serialize};
use serde_big_array::big_array;
use themelio_stf::melvm::Covenant;

big_array! { BigArray; }

const COV_PATH: &str = "bridge-covenants/bridge.melo";
//const MELWALLETD_ADDR: SocketAddr = SocketAddr::from_str("127.0.0.1:11773").unwrap();

/// An ecdsa secret key that implements FromStr that converts from hexadecimal
#[derive(Copy, Clone, Serialize, Deserialize)]
struct EcdsaSK(#[serde(with = "BigArray")] pub [u8; 64]);

impl FromStr for EcdsaSK {
    type Err = hex::FromHexError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vv = hex::decode(s)?;
        Ok(EcdsaSK(
            vv.try_into()
                .map_err(|_| hex::FromHexError::InvalidStringLength)?,
        ))
    }
}

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

fn freeze() -> Result<bool> {
    Ok(true)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let subcommand = &args.subcommand;
    let dry_run = &args.dry_run;

    match subcommand {
        Subcommand::FreezeAndMint(sub_args) => println!("{:?}", sub_args),
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

    Ok(())
}