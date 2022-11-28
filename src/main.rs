mod cli;
mod structs;

use std::{
    convert::TryFrom,
    io::{BufReader, Read, Stdin, Write},
    ops::Range,
    path::Path,
    sync::{Mutex, Arc},
};

use anyhow::Result;
use bindings::themelio_bridge::ThemelioBridge;
use clap::Parser;
use colored::Colorize;
use ethers::{
    prelude::SignerMiddleware,
    providers::{Http, Provider, Middleware},
    signers::{LocalWallet, Signer},
    types::{BlockNumber, H160, H256, Filter, FilterBlockOption, U64, U256, ValueOrArray, Bytes},
    utils::hex::FromHex,
};
use melnet2::{Backhaul, wire::tcp::TcpBackhaul};
use melorun::LoadFileError;
use mil::compiler::{BinCode, Compile};
use once_cell::sync::Lazy;
use stdcode::StdcodeSerializeExt;
use tabwriter::TabWriter;
use themelio_nodeprot::{NodeRpcClient, ValClient};
use themelio_stf::melvm::Covenant;
use themelio_structs::{
    BlockHeight,
    Header,
    NetID,
    Transaction,
    TxHash, STAKE_EPOCH,
};

use cli::*;
use structs::*;

const COV_PATH: &str = "bridge-covenants/bridge.melo";
const BRIDGE_ADDRESS: &str = "56E618FB75B9344eFBcD63ef138F90277b1C1593";
const HEADER_VERIFIED_TOPIC: &str = "8cee0a7da402e70d36d0d5cba99d9b5f4b6490c10ff25c61043cce84c3f1ac01";
const CONTRACT_DEPLOYMENT_HEIGHT: BlockNumber = BlockNumber::Number(U64([0x753927]));

static STDIN_BUFFER: Lazy<Mutex<BufReader<Stdin>>> =
    Lazy::new(|| Mutex::new(BufReader::new(std::io::stdin())));

static CLI_ARGS: Lazy<Cli> = Lazy::new(Cli::parse);

static THE_CLIENT: Lazy<ValClient> = Lazy::new(|| {
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

static ETH_CLIENT: Lazy<Arc<SignerMiddleware<Provider<Http>, LocalWallet>>> = Lazy::new(|| {
    smol::block_on(async {
        let config = Config::try_from(CLI_ARGS.to_owned()).unwrap();

        let provider = Provider::<Http>::try_from(config.ethereum_rpc.clone()).unwrap();

        let chain_id = provider
            .get_chainid()
            .await
            .unwrap()
            .as_u64();

        let wallet: LocalWallet = config.ethereum_secret.parse().unwrap();
        let wallet = wallet.with_chain_id(chain_id);

        let client = SignerMiddleware::new(provider, wallet);
        let client = Arc::new(client);

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

async fn get_tx(tx_hash: TxHash) -> Result<Transaction> {
    smol::block_on( async move {
        let snapshot = THE_CLIENT.snapshot().await?;

        let tx = snapshot.get_transaction(tx_hash).await?
            .expect("Transaction with provided hash does not exist");

        Ok(tx)
    })
}

async fn get_header(block_height: BlockHeight) -> Result<Header> {
    smol::block_on(async move {
        let snapshot = THE_CLIENT.snapshot().await?;
        let mut header = snapshot.current_header();

        if header.height != block_height {
            header = snapshot
                .get_history(block_height)
                .await?
                .expect("Error retrieving header");
        }

        Ok(header)
    })
}

async fn get_stakes(block_height: BlockHeight) -> Result<Vec<u8>> {
    Ok(vec!())
}

async fn get_signatures(block_height: BlockHeight) -> Result<Vec<[u8; 32]>> {
    Ok(vec!())
}

async fn get_historical_headers(epochs: Range<u64>) -> Result<Vec<Header>> {
    smol::block_on(async move {
        let headers = futures::future::join_all(
            epochs
                .map(|epoch| async move {
                    let header = get_header(BlockHeight((epoch + 1) * STAKE_EPOCH - 1))
                        .await
                        .expect("Error retreiving historical headers");

                    header
                })
        ).await;

        Ok(headers)
    })
}

async fn get_historical_stakes(epochs: Range<u64>) -> Result<Vec<Vec<u8>>> {
    smol::block_on(async move {
        let stakes_vec = futures::future::join_all(
            epochs
                .map(|epoch| async move {
                    let stakes = get_stakes(BlockHeight((epoch + 1) * STAKE_EPOCH - 1))
                        .await
                        .expect("Error retreiving historical stakes");

                    stakes
                })
        ).await;

        Ok(stakes_vec)
    })
}

async fn get_historical_signatures(epochs: Range<u64>) -> Result<Vec<Vec<[u8; 32]>>> {
    smol::block_on(async {
        let signatures = futures::future::join_all(
            epochs
                .map(|epoch| async move {
                    let signature = get_signatures(BlockHeight((epoch + 1) * STAKE_EPOCH - 1))
                        .await
                        .expect("Error retreiving historical signatures");

                    signature
                })
        ).await;

        Ok(signatures)
    })
}

async fn fetch_mintargs(freeze_data: FreezeData) -> Result<MintArgs> {
    smol::block_on( async move {
        let freeze_height = freeze_data.block_height;
        let freeze_epoch = freeze_height.epoch();
        let freeze_header = get_header(freeze_height).await?;
        let freeze_tx = get_tx(freeze_data.tx_hash).await?;
        let freeze_stakes = get_stakes(freeze_height).await?;

        let filter = Filter{
            block_option: FilterBlockOption::Range {
                from_block: Some(CONTRACT_DEPLOYMENT_HEIGHT),
                to_block: Some(BlockNumber::Latest)
            },
            address: Some(ValueOrArray::Value(H160(<[u8; 20]>::from_hex(BRIDGE_ADDRESS)?))),
            topics: [
                Some(ValueOrArray::Value(Some(H256(<[u8; 32]>::from_hex(HEADER_VERIFIED_TOPIC)?)))),
                None,
                None,
                None,
            ],
        };

        let logs = ETH_CLIENT
            .get_logs(&filter)
            .await?;

        let verifier_height = BlockHeight(
            logs[0]
                .block_number
                .expect("Error retrieving latest verified header")
                .0[0]
        );
        let highest_verified_epoch = verifier_height.epoch();

        let history_range: Range<u64>;
        let mut historical_headers: Vec<Header> = vec!();
        let mut historical_stakes: Vec<Vec<u8>> = vec!();
        let mut historical_signatures: Vec<Vec<[u8; 32]>> = vec!();

        // if highest verified epoch is the same as freeze epoch then no historical structs needed
        if freeze_epoch <= highest_verified_epoch ||
            freeze_epoch == (verifier_height + 1.into()).epoch() {
        } else {
            // if highest verified height is the last block of its epoch, verify the next epoch's penultimate block
            if (verifier_height + 1.into()).epoch() != highest_verified_epoch {
                history_range = highest_verified_epoch + 1..freeze_epoch;
            } else {
                history_range = highest_verified_epoch..freeze_epoch;
            }

            historical_headers = get_historical_headers(history_range.clone()).await?;
            historical_stakes = get_historical_stakes(history_range.clone()).await?;
            historical_signatures = get_historical_signatures(history_range).await?;
        }

        Ok(MintArgs{
            header_args: todo!(),
            historical_header_args: todo!(),
        })
    })
}

async fn mint_tokens(mint_args: MintArgs) -> Result<()> {
    smol::block_on(async {
        let bridge_contract = ThemelioBridge::new(<[u8; 20]>::from_hex(BRIDGE_ADDRESS)?, ETH_CLIENT.clone());

        // submit historical stakes and headers
        let header_args = mint_args.header_args;
        let historical_header_args = mint_args.historical_header_args;

        let hist_header_receipts = futures::future::join_all(
            historical_header_args
                .into_iter()
                .map(|historical_args| async {
                    (
                        bridge_contract.verify_stakes(Bytes(historical_args.clone().stakes.into())),
                        bridge_contract.verify_header(
                            U256::from(historical_args.verifier_height.0),
                            Bytes(historical_args.header.stdcode().into()),
                            Bytes(historical_args.stakes.into()),
                            historical_args.signatures
                        )
                    )
                })
        ).await;

        // submit freeze stakes, header, and tx
        let verifier_height = U256::from(header_args.verifier_height.0);
        let freeze_header = Bytes(header_args.header.stdcode().into());
        let freeze_stakes = Bytes(header_args.stakes.into());
        let freeze_signatures = header_args.signatures;

        let freeze_stakes_receipt = bridge_contract.verify_stakes(freeze_stakes);

        Ok(())
    })
}

fn main() -> Result<()> {
    smol::block_on(async move {
        let twriter = TabWriter::new(std::io::stderr());

        let subcommand = CLI_ARGS.subcommand.clone();
        let dry_run = CLI_ARGS.dry_run;

        match subcommand {
            Subcommand::MintTokens(freeze_data) => {
                let mint_args: MintArgs = fetch_mintargs(freeze_data).await?;
                println!("Mintargs: {:#?}", mint_args);

                let mint_receipt = mint_tokens(mint_args).await?;
                println!("Tokens minted successfully:\n{:#?}", mint_receipt);
            }

            Subcommand::BurnTokens(burn_args) => {
                // let burn_data = burn_tokens(burn_args).await?;
                // let thaw_args = fetch_thawargs(burn_data).await?;
                // let thaw_tx = ...

                println!("Tokens burned successfully:\n{:?}", burn_args);
                // println!("Here is the tx you need for thawing: {}\nMore info at https://github.com/themeliolabs/bridge-cli", thaw_tx);
            }
        }

        Ok(())
    })
}