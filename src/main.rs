
use argh::FromArgs;
use themelio_structs::{
    CoinID,
    CoinData,
    Denom,
    Header,
    Transaction,
};

#[derive(FromArgs, PartialEq, Debug)]
struct TopLevel {
    #[argh(subcommand)]
    sequence_type: TxSequenceType,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum TxSequenceType {
    FreezeAndMint(FreezeAndMintArgs),
    BurnAndThaw(BurnAndThawArgs),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "freeze_and_mint_args", description = "arguments for coin freezing and token minting transactions (i.e. bridging from Themelio to Ethereum)")]
struct FreezeAndMintArgs {
    #[argh(option)]
    tx_args: TransactionArgs,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "burn_and_thaw_args", description = "arguments for token burning and coin thawing transaction (i.e. bridging back from Ethereum to Themelio)")]
struct BurnAndThawArgs{
    #[argh(option)]
    tx_args: TransactionArgs,
}

#[derive(FromArgs)]
#[argh(description = "arguments for a transaction: value, denom, and Ethereum recipient")]
struct TransactionArgs {
    #[argh(option, short = 'v', description = "value of the transaction")]
    value: String,

    #[argh(option, short = 'd', description = "denom of the transaction")]
    denom: String,

    #[argh(option, short = 'r', description = "ethereum address of recipient")]
    eth_recipient: String,
}

fn main() {
    let args: TopLevel = argh::from_env();

    match args {
        
    }
    // let value: u128 = args
    //     .value
    //     .parse()
    //     .expect("Please include coin value.");

    // let denom: Denom = args
    //     .denom
    //     .parse()
    //     .expect("Please include coin denom.");

    // let eth_recipient_str: &str = args
    //     .eth_recipient
    //     .strip_prefix("0x")
    //     .expect("Address must start with '0x'.");
    // let eth_recipient: [u8; 20] = hex::decode(eth_recipient_str)
    //     .expect("Error decoding Ethereum recipient address.")
    //     .try_into()
    //     .expect("Recipient address must be 20 bytes long.");

    println!("value: {}\ndenom: {}\nrecipient: {}", value, denom, hex::encode(eth_recipient));
}
