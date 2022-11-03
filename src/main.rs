use std::path::Path;

use anyhow::Result;
use argh::FromArgs;
use melorun::LoadFileError;
use mil::compiler::{BinCode, Compile};
use themelio_stf::melvm::Covenant;
// use themelio_structs::{
//     CoinID,
//     CoinData,
//     Denom,
//     Header,
//     Transaction,
// };

#[derive(FromArgs, PartialEq, Debug)]
#[argh(description = "top-level cli argument")]
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
#[argh(subcommand, name = "burn_and_thaw", description = "arguments for token burning and coin thawing transactions (i.e. bridging back from Ethereum to Themelio)")]
struct BurnAndThawArgs{
    #[argh(option, short = 'v', description = "value of the transaction")]
    value: String,

    #[argh(option, short = 'd', description = "denom of the transaction")]
    denom: String,

    #[argh(option, short = 't', description = "themelio address of recipient")]
    themelio_recipient: String,
}

const COV_PATH: &str = "bridge-covenants/bridge.melo";

fn compile() -> Result<Covenant> {
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

#[tokio::main]
async fn main() -> Result<()> {
    let top_level: TopLevel = argh::from_env();

    match top_level.subcommand {
        Subcommand::FreezeAndMint(args) => println!("{:?}", args),
        Subcommand::BurnAndThaw(args) => println!("{:?}", args),
    }

    let cov = compile()?;

    println!("{:?}", cov);

    Ok(())
}
