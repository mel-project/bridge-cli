use std::fmt;

use argh::FromArgs;
use themelio_structs::{
    CoinID,
    CoinData,
    Denom,
    Header,
    Transaction,
};

#[derive(FromArgs, PartialEq, Debug)]
#[argh(description = "top-level cli argument; an enum of subcommands")]
struct TopLevel {
    #[argh(subcommand)]
    subcommand: Subcommand,
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
#[argh(subcommand, name = "burn_and_thaw", description = "arguments for token burning and coin thawing transaction (i.e. bridging back from Ethereum to Themelio)")]
struct BurnAndThawArgs{
    #[argh(option, short = 'v', description = "value of the transaction")]
    value: String,

    #[argh(option, short = 'd', description = "denom of the transaction")]
    denom: String,

    #[argh(option, short = 't', description = "themelio address of recipient")]
    themelio_recipient: String,
}



fn main() {
    let top_level: TopLevel = argh::from_env();

    match top_level.subcommand {
        Subcommand::FreezeAndMint(args) => println!("{:?}", args),
        Subcommand::BurnAndThaw(args) => println!("{:?}", args),
    }
}
