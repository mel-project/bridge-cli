use std::{path::Path, str::FromStr};

use anyhow::Result;
use argh::FromArgs;
use ethers::{
    types::{
        Address as EthersAddress,
        // Bytes,
        // H160,
        // U256
    },
    prelude::k256::SecretKey,
    signers::LocalWallet
};
use melorun::LoadFileError;
use mil::compiler::{BinCode, Compile};
use serde::{Deserialize, Serialize};
use serde_big_array::big_array;
use serde_yaml;
use themelio_stf::melvm::Covenant;
use themelio_structs::{
    Address as ThemelioAddress,
    // CoinID,
    // CoinData,
    // Denom,
    // Header,
    // Transaction,
};
use tmelcrypt::Ed25519SK;

big_array! { BigArray; }

#[derive(FromArgs, PartialEq, Debug)]
#[argh(description = "top-level cli argument")]
struct Args {
    #[argh(subcommand)]
    subcommand: Subcommand,

    #[argh(switch, description = "indicates that transactions will be dry runs only")]
    dry_run: bool,

    #[argh(option, description = "url of Ethereum RPC provider")]
    ethereum_rpc: String,

    #[argh(option, description = "themelio secret key")]
    themelio_secret: Ed25519SK,

    #[argh(option, description = "ethereum secret key")]
    ethereum_secret: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommand {
    FreezeAndMint(FreezeAndMintArgs),
    BurnAndThaw(BurnAndThawArgs),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "freeze_and_mint", description = "arguments for coin freezing and token minting transactions (i.e. bridging from Themelio to Ethereum)")]
struct FreezeAndMintArgs {
    #[argh(option, short = 'v', description = "value of the transaction")]
    value: String,

    #[argh(option, short = 'd', description = "denom of the transaction")]
    denom: String,

    #[argh(option, short = 'e', description = "ethereum address of recipient")]
    ethereum_recipient: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "burn_and_thaw", description = "arguments for token burning and coin thawing transactions (i.e. bridging back from Ethereum to Themelio)")]
struct BurnAndThawArgs {
    #[argh(option, short = 'v', description = "value of the transaction")]
    value: String,

    #[argh(option, short = 'd', description = "denom of the transaction")]
    denom: String,

    #[argh(option, short = 't', description = "themelio address of recipient")]
    themelio_recipient: String,
}

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

const COV_PATH: &str = "bridge-covenants/bridge.melo";
const CONFIG_PATH: &str = "config.yaml";

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
    let args: Args = argh::from_env();
    let subcommand = args.subcommand;
    let dry_run = args.dry_run;

    match subcommand {
        Subcommand::FreezeAndMint(sub_args) => println!("{:?}", sub_args),
        Subcommand::BurnAndThaw(sub_args) => println!("{:?}", sub_args),
    }

    let cov = compile_cov()?;

    println!("{:?}", cov);

    let config = Config::try_from(args)
        .expect("Unable to create config from cmd args");
    // let network = config.network;
    // let addr = config.network_addr;
    // let db_name = format!("{network:?}-wallets.db").to_ascii_lowercase();

    // if output_config {
    //     println!(
    //         "{}",
    //         serde_yaml::to_string(&config)
    //             .expect("Critical Failure: Unable to serialize `Config`")
    //     );
    // }

    if dry_run {
        return Ok(());
    }

    Ok(())
}
