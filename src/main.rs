
use clap::Parser;
use themelio_structs::{
    CoinID,
    CoinData,
    Denom,
    Header,
    Transaction,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    value: String,

    #[clap(short, long)]
    denom: String,

    #[clap(short, long)]
    eth_recipient: String,
}

fn main() {
    let args = Args::parse();

    let value: u128 = args
        .value
        .parse()
        .expect("Please include coin value.");

    let denom: Denom = args
        .denom
        .parse()
        .expect("Please include coin denom.");

    let eth_recipient_str: &str = args
        .eth_recipient
        .strip_prefix("0x")
        .expect("Address must start with '0x'.");
    let eth_recipient: [u8; 20] = hex::decode(eth_recipient_str)
        .expect("Error decoding Ethereum recipient address.")
        .try_into()
        .expect("Recipient address must be 20 bytes long.");

    println!("value: {}\ndenom: {}\nrecipient: {}", value, denom, hex::encode(eth_recipient));
}
