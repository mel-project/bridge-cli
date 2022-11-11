use std::{convert::TryFrom, path::PathBuf, net::SocketAddr};

use anyhow::Result;
use clap::{Args, Parser};
use ethers::types::Address as EthersAddress;
use melorun::LoadFileError;
use serde::{Deserialize, Serialize};
use serde_yaml;
use themelio_structs::{
    Address as ThemelioAddress,
    CoinValue,
    Denom,
};

#[derive(Clone, Deserialize, Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Subcommand,

    #[clap(long)]
    pub config_path: Option<PathBuf>,

    #[clap(flatten)]
    pub config: Option<Config>,

    #[clap(long)]
    pub dry_run: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, clap::Subcommand)]
pub enum Subcommand {
    FreezeAndMint(FreezeAndMintArgs),
    BurnAndThaw(BurnAndThawArgs),
}

#[derive(Args, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FreezeAndMintArgs {
    #[clap(long)]
    value: CoinValue,

    #[clap(long)]
    denom: Denom,

    #[clap(long)]
    ethereum_recipient: EthersAddress,
}

#[derive(Args, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BurnAndThawArgs {
    #[clap(long)]
    value: CoinValue,

    #[clap(long)]
    denom: Denom,

    #[clap(long)]
    themelio_recipient: ThemelioAddress,
}

#[derive(Args, Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    #[clap(long)]
    pub testnet: bool,

    #[clap(long)]
    ethereum_rpc: String,

    #[clap(long)]
    ethereum_secret: String,

    /// Wallet API endpoint. For example localhost:11773
    #[clap(long)]
    pub daemon_addr: Option<SocketAddr>,

    #[clap(long)]
    pub wallet_name: String,
}

impl Config {
    fn new(
        testnet: bool,
        ethereum_rpc: String,
        ethereum_secret: String,
        daemon_addr: Option<SocketAddr>,
        wallet_name: String,
    ) -> Config {
        Config {
            testnet,
            ethereum_rpc,
            ethereum_secret,
            daemon_addr,
            wallet_name,
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
                config.daemon_addr,
                config.wallet_name
            ))
        }
    }
}