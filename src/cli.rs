use std::{convert::TryFrom, path::PathBuf};

use anyhow::Result;
use clap::{ArgGroup, Args, Parser};
use ethers::{
    types::{
        Address as EthersAddress,
        // Bytes,
        // H160,
        // U256
    },
};
use melorun::LoadFileError;
use serde::{Deserialize, Serialize};
use serde_yaml;
use themelio_structs::{
    Address as ThemelioAddress,
    // CoinID,
    // CoinData,
    CoinValue,
    Denom,
    // Header,
    NetID,
    // Transaction,
};

#[derive(Clone, Deserialize, Debug, Parser)]
#[clap(group(
    ArgGroup::new("options")
        .required(true)
        .args(&["config-path", "config"])),
)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Subcommand,

    #[clap(long)]
    pub config_path: Option<PathBuf>,

    #[clap(long, default_value = "mainnet")]
    pub network: NetID,

    #[clap(long)]
    pub ethereum_rpc: Option<String>,

    #[clap(long)]
    pub ethereum_secret: Option<String>,

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
    value: CoinValue,
    denom: Denom,
    ethereum_recipient: EthersAddress,
}

#[derive(Args, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BurnAndThawArgs {
    value: CoinValue,
    denom: Denom,
    themelio_recipient: ThemelioAddress,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    network: NetID,
    ethereum_rpc: String,
    ethereum_secret: String,
}

impl Config {
    fn new(
        network: NetID,
        ethereum_rpc: String,
        ethereum_secret: String,
    ) -> Config {
        Config {
            network,
            ethereum_rpc,
            ethereum_secret,
        }
    }
}

impl TryFrom<Cli> for Config {
    type Error = anyhow::Error;

    fn try_from(args: Cli) -> Result<Self, Self::Error> {
        match args.config_path {
            Some(config_path) => {
                let config_str = std::fs::read_to_string(config_path)
                    .map_err(LoadFileError::IoError)?;
                let config: Config = serde_yaml::from_str(&config_str)?;

                Ok(config)
            }

            None => {
                Ok(Config::new(
                    args.network,
                    args.ethereum_rpc.expect("ethereum_rpc required if no config path supplied"),
                    args.ethereum_secret.expect("ethereum_secret required if no config path supplied"),
                ))
            }
        }
    }
}