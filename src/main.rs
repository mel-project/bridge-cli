mod cli;
mod structs;

use std::{
    convert::TryFrom,
    io::{BufReader, Read, Stdin, Write},
    path::Path,
    sync::{Mutex, Arc},
};

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use ethers::{
    prelude::SignerMiddleware,
    providers::{Http, Provider, Middleware},
    signers::{LocalWallet, Signer},
    types::{H160, H256, Filter, ValueOrArray},
    utils::hex::FromHex,
};
use melnet2::{Backhaul, wire::tcp::TcpBackhaul};
use melorun::LoadFileError;
use mil::compiler::{BinCode, Compile};
use once_cell::sync::Lazy;
use tabwriter::TabWriter;
use themelio_nodeprot::{NodeRpcClient, ValClient};
use themelio_stf::melvm::Covenant;
use themelio_structs::{
    BlockHeight,
    Header,
    NetID,
};

use cli::*;
use structs::*;

const COV_PATH: &str = "bridge-covenants/bridge.melo";
const BRIDGE_ADDRESS: &str = "56E618FB75B9344eFBcD63ef138F90277b1C1593";
const HEADER_VERIFIED_TOPIC: &str = "8cee0a7da402e70d36d0d5cba99d9b5f4b6490c10ff25c61043cce84c3f1ac01";

static STDIN_BUFFER: Lazy<Mutex<BufReader<Stdin>>> =
    Lazy::new(|| Mutex::new(BufReader::new(std::io::stdin())));

static CLI_ARGS: Lazy<Cli> = Lazy::new(Cli::parse);
// pub static CONFIG: Lazy<Config> = Lazy::new(Config::try_from(CLI_ARGS));

static CLIENT: Lazy<ValClient> = Lazy::new(|| {
    smol::block_on(async move {
        let backhaul = TcpBackhaul::new();
        let config = Config::try_from(CLI_ARGS.to_owned()).unwrap();
        let testnet = config.testnet;

        let (network, mut themelio_rpc) = if testnet {
            (NetID::Testnet, themelio_bootstrap::bootstrap_routes(NetID::Testnet)[0].to_string())
        } else {
            (NetID::Mainnet, themelio_bootstrap::bootstrap_routes(NetID::Mainnet)[0].to_string())
        };

        if let Some(rpc_url) = config.themelio_rpc {
            themelio_rpc = rpc_url;
        }

        let client = ValClient::new(
            network,
            NodeRpcClient(
                backhaul
                    .connect(themelio_rpc.into())
                    .await
                    .unwrap()
            ),
        );

        if testnet {
            client.trust(themelio_bootstrap::checkpoint_height(NetID::Testnet).unwrap());
        } else {
            client.trust(themelio_bootstrap::checkpoint_height(NetID::Mainnet).unwrap());
        }
        client
    })
});

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

async fn proceed_prompt() -> Result<()> {
    eprintln!("Proceed? [y/N] ");

    let letter = smol::unblock(move || {
        let mut letter = [0u8; 1];

        match STDIN_BUFFER.lock().as_deref_mut() {
            Ok(stdin) => {
                while letter[0].is_ascii_whitespace() || letter[0] == 0 {
                    stdin.read_exact(&mut letter)?;
                }
                Ok(letter)
            }

            Err(_) => return Err(anyhow::anyhow!("unknown buffer unlock problem")),
        }
    })
    .await?;

    if letter[0].to_ascii_lowercase() != b'y' {
        anyhow::bail!("Cancelled");
    }

    Ok(())
}

fn write_txhash(out: &mut impl Write, wallet_name: &str, txhash: TxHash) -> anyhow::Result<()> {
    writeln!(out, "Transaction hash:\t{}", txhash.to_string().bold())?;
    writeln!(
        out,
        "(wait for confirmation with {})",
        format!(
            "melwallet-cli wait-confirmation -w {} {}",
            wallet_name, txhash
        )
        .bright_blue(),
    )?;

    Ok(())
}

// async fn send_freeze_tx(
//     mut twriter: impl Write,
//     wallet_id: String,
//     tx: Transaction,
// ) -> Result<BlockHeight> {
//     let output = &tx.outputs[0];

//     writeln!(twriter, "{}", "FREEZING COIN".bold())?;
//     writeln!(twriter, "{}", "Bridge Address\tValue\tDenomination\tAdditional data".italic())?;

//     writeln!(
//         twriter,
//         "{}\t{}\t{}\t{:?}",
//         output.covhash.to_string().bright_blue(),
//         output.value,
//         output.denom,
//         hex::encode(&output.additional_data)
//     )?;

//     writeln!(twriter, "{}\t{} MEL", " (network fees)".yellow(), tx.fee)?;

//     twriter.flush()?;

//     proceed_prompt().await?;

//     //let tx_hash = wallet.send_tx(tx).await?;
//     let tx_hash = TxHash(HashVal::random());
//     let snapshot = CLIENT
//         .snapshot()
//         .await?;
//     let freeze_height = BlockHeight(0);

//     write_txhash(&mut twriter, &wallet_id, tx_hash)?;

//     Ok(freeze_height)
// }

async fn fetch_mintargs(freeze_data: FreezeData) -> Result<MintArgs> {
    let config = Config::try_from(CLI_ARGS.to_owned())?;

    let eth_provider = Provider::<Http>::try_from(config.ethereum_rpc)?;
    let chain_id = eth_provider.get_chainid().await?;
    let eth_wallet: LocalWallet = config.ethereum_secret.parse()?;
    let eth_wallet = eth_wallet.with_chain_id(chain_id.as_u64());
    let eth_client = Arc::new(SignerMiddleware::new(eth_provider, eth_wallet));

    //let current_height = eth_client.get_block_number().await?;
    let filter = Filter{
        block_option: ethers::types::FilterBlockOption::Range { from_block: None, to_block: None },
        address: Some(ValueOrArray::Value(H160(<[u8; 20]>::from_hex(BRIDGE_ADDRESS)?))),
        topics: [
            Some(ValueOrArray::Value(Some(H256(<[u8; 32]>::from_hex(HEADER_VERIFIED_TOPIC)?)))),
            None,
            None,
            None,
        ],
    };
    let logs = eth_client.get_logs(&filter).await?;

    println!("{:?}", logs);

    // let latest_verified_header = 

    let freeze_hash = freeze_data.tx_hash;
    let freeze_height = freeze_data.block_height;

    Ok(MintArgs{
        freeze_height: todo!(),
        freeze_header: todo!(),
        freeze_tx: todo!(),
        freeze_stakes: todo!(),
        historical_headers: todo!(),
    })
}

async fn get_header(block_height: BlockHeight) -> Result<Header> {
    let snapshot = CLIENT
        .snapshot()
        .await?;
    let mut header = snapshot.current_header();

    if header.height != block_height {
        header = if let Ok(fetched_header) = snapshot.get_history(block_height).await {
            fetched_header.unwrap()
        } else {
            header
        }
    }

    Ok(header)
}

// async fn get_stakes() -> Result<()> {
//     let stakes = CLIENT.get_trusted_stakers();

//     Ok(())
// }

// async fn mint() -> Result<()> {
//     Ok(())
// }

fn main() -> Result<()> {
    smol::block_on(async move {
        let twriter = TabWriter::new(std::io::stderr());

        let subcommand = CLI_ARGS.subcommand.clone();
        // let dry_run = CLI_ARGS.dry_run;

        let config = Config::try_from(CLI_ARGS.clone())
            .expect("Unable to create config from CLI args");

        // let network_id = if config.testnet {
        //     NetID::Testnet
        // } else {
        //     NetID::Mainnet
        // };

        match subcommand {
            Subcommand::MintTokens(freeze_data) => {
                let mint_args: MintArgs = fetch_mintargs(freeze_data)
                    .await
                    .expect("Error processing freeze data");

                println!("Tokens minted successfully:\n{:#?}", mint_args);
            }

            Subcommand::BurnTokens(burn_args) => {
                // let burn_data = burn_tokens(burn_args).await?;
                // let thaw_args = fetch_thawargs(burn_data).await?;
                // let thaw_tx = ...

                println!("Tokens burned successfully:\n{:?}", burn_args);
                // println!("Here is the tx you need for thawing: {}\nMore info at https://github.com/themeliolabs/bridge-cli", thaw_tx);
            }
        }
    });

    Ok(())
}