use std::{
    path::PathBuf,
    convert::TryFrom,
};

use anyhow::Result;
use clap::{ArgGroup, Args, Parser};
use ethers::types::Address as EthersAddress;
use melorun::LoadFileError;
use serde::{Deserialize, Serialize};
use serde_yaml;
use themelio_structs::{
    Address as ThemelioAddress,
    BlockHeight,
    CoinValue,
    Denom,
    TxHash,
};

#[derive(Clone, Deserialize, Debug, Parser)]
#[command(version, about, long_about)]
#[clap(group(
    ArgGroup::new("config_file")
        .args(&["config_path"])
        .args(&["testnet", "ethereum_rpc", "ethereum_secret", "themelio_rpc", "themelio_url"])
        .multiple(false)
))]
pub struct Cli {
    #[clap(long)]
    pub config_path: Option<PathBuf>,

    #[clap(flatten)]
    pub config: Option<Config>,

    #[command(subcommand)]
    pub subcommand: Subcommand,

    #[clap(long)]
    pub dry_run: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, clap::Subcommand)]
pub enum Subcommand {
    FreezeAndMint(FreezeData),
    BurnAndThaw(BurnAndThawArgs),
}

#[derive(Args, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FreezeData {
    #[clap(long)]
    pub value: CoinValue,

    #[clap(long)]
    pub denom: Denom,

    #[clap(long)]
    pub ethereum_recipient: EthersAddress,

    #[clap(long)]
    pub tx_hash: TxHash,

    #[clap(long)]
    pub block_height: BlockHeight,
}

#[derive(Args, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BurnAndThawArgs {
    #[clap(long)]
    pub value: CoinValue,

    #[clap(long)]
    pub denom: Denom,

    #[clap(long)]
    pub themelio_recipient: ThemelioAddress,
}

#[derive(Args, Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    #[clap(long)]
    pub testnet: bool,

    #[clap(long)]
    pub ethereum_rpc: String,

    #[clap(long)]
    pub ethereum_secret: String,

    #[clap(long)]
    pub themelio_rpc: Option<String>,

    #[clap(long)]
    pub themelio_url: String,
}

impl Config {
    fn new(
        testnet: bool,
        ethereum_rpc: String,
        ethereum_secret: String,
        themelio_rpc: Option<String>,
        themelio_url: String,
    ) -> Config {
        Config {
            testnet,
            ethereum_rpc,
            ethereum_secret,
            themelio_rpc,
            themelio_url,
        }
    }
}

impl TryFrom<Cli> for Config {
    type Error = anyhow::Error;

    fn try_from(args: Cli) -> Result<Self, Self::Error> {
        if let Some(config_path) = args.config_path {
            let config_str = std::fs::read_to_string(config_path)
                .map_err(LoadFileError::IoError)?;
            let config: Config = serde_yaml::from_str(&config_str)?;

            Ok(config)
        } else {
            let config = args.config
                .expect("Either config path or config args must be included as CLI args");

            Ok(Config::new(
                config.testnet,
                config.ethereum_rpc,
                config.ethereum_secret,
                config.themelio_rpc,
                config.themelio_url
            ))
        }
    }
}