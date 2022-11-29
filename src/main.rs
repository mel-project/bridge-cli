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
    types::{BlockNumber, Bytes, Filter, FilterBlockOption, H160, H256, TransactionReceipt, U64, U256, ValueOrArray},
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

const _COV_PATH: &str = "bridge-covenants/bridge.melo";
const BRIDGE_ADDRESS: &str = "56E618FB75B9344eFBcD63ef138F90277b1C1593";
const HEADER_VERIFIED_TOPIC: &str = "8cee0a7da402e70d36d0d5cba99d9b5f4b6490c10ff25c61043cce84c3f1ac01";
const CONTRACT_DEPLOYMENT_HEIGHT: BlockNumber = BlockNumber::Number(U64([0x753927]));

static _STDIN_BUFFER: Lazy<Mutex<BufReader<Stdin>>> =
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

fn _compile_cov() -> Result<Covenant> {
    let cov_path = Path::new(_COV_PATH);

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

async fn _proceed_prompt() -> Result<()> {
    eprintln!("Proceed? [y/N] ");

    let letter = smol::unblock(move || {
        let mut letter = [0u8; 1];

        match _STDIN_BUFFER.lock().as_deref_mut() {
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

fn _write_txhash(out: &mut impl Write, wallet_name: &str, txhash: TxHash) -> anyhow::Result<()> {
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

async fn get_stakes(_block_height: BlockHeight) -> Result<Vec<u8>> {
    Ok(vec!())
}

async fn get_signatures(_block_height: BlockHeight) -> Result<Vec<[u8; 32]>> {
    Ok(vec!())
}

async fn get_proof(_block_height: BlockHeight, _tx_hash: TxHash) -> Result<MerkleProof> {
    Ok(MerkleProof {
        bytes: vec!(),
        tx_index: 0
    })
}

async fn get_historical_data(epochs: Range<u64>, base_verifier_height: BlockHeight) -> Result<Vec<HeaderVerificationArgs>> {
    smol::block_on(async move {
        let mut historical_data = futures::future::join_all(
            epochs
                .map(|epoch| async move {
                    let block_height = BlockHeight((epoch + 1) * STAKE_EPOCH - 1);

                    let header = get_header(block_height)
                        .await
                        .expect("Error retreiving historical header");

                    let stakes = get_stakes(block_height)
                        .await
                        .expect("Error retreiving historical stakes");

                    let signatures = get_signatures(block_height)
                        .await
                        .expect("Error retreiving historical signatures");

                    HeaderVerificationArgs {
                        header,
                        verifier_height: block_height - STAKE_EPOCH.into(),
                        stakes,
                        signatures,
                    }
                })
        ).await;

        historical_data[0].verifier_height = base_verifier_height;

        Ok(historical_data)
    })
}

async fn fetch_mintargs(freeze_data: FreezeData) -> Result<MintArgs> {
    smol::block_on( async move {
        let freeze_height = freeze_data.block_height;
        let freeze_epoch = freeze_height.epoch();
        let freeze_header = get_header(freeze_height).await?;
        let freeze_tx = get_tx(freeze_data.tx_hash).await?;
        let freeze_stakes = get_stakes(freeze_height).await?;
        let freeze_signatures = get_signatures(freeze_height).await?;
        let freeze_proof = get_proof(freeze_height, freeze_tx.hash_nosigs()).await?;

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

        let highest_verified_height = BlockHeight(
            logs[0]
                .block_number
                .expect("Error retrieving latest verified header")
                .0[0]
        );
        let highest_verified_epoch = highest_verified_height.epoch();

        let verifier_height: BlockHeight;
        let history_range: Range<u64>;
        let mut historical_data: Vec<HeaderVerificationArgs> = vec!();

        // if highest verified epoch is the same as freeze epoch then no historical structs needed
        if freeze_epoch <= highest_verified_epoch || freeze_epoch == (highest_verified_height + 1.into()).epoch() {
                verifier_height = highest_verified_height;
        } else {
            // if highest verified height is the last block of its epoch, verify the next epoch's penultimate block
            if (highest_verified_height + 1.into()).epoch() != highest_verified_epoch {
                history_range = highest_verified_epoch + 1..freeze_epoch;
            } else {
                history_range = highest_verified_epoch..freeze_epoch;
            }

            historical_data = get_historical_data(history_range.clone(), highest_verified_height).await?;
            verifier_height = historical_data[historical_data.len() - 1].header.height;
        }

        Ok(MintArgs{
            historical_header_args: historical_data,
            header_args: HeaderVerificationArgs {
                header: freeze_header,
                verifier_height,
                stakes: freeze_stakes,
                signatures: freeze_signatures
            },
            tx_args: TxVerificationArgs {
                transaction: freeze_tx,
                tx_index: freeze_proof.tx_index,
                block_height: freeze_height,
                proof: freeze_proof.bytes
            },
        })
    })
}

async fn mint_tokens(mint_args: MintArgs) -> Result<TransactionReceipt> {
    smol::block_on(async {
        let bridge_contract = ThemelioBridge::new(<[u8; 20]>::from_hex(BRIDGE_ADDRESS)?, ETH_CLIENT.clone());

        // submit historical stakes and headers
        let historical_header_args = mint_args.historical_header_args;
        let header_args = mint_args.header_args;
        let tx_args = mint_args.tx_args;

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

        println!("{:#?}", hist_header_receipts);

        // submit freeze stakes, header, and tx
        let verifier_height = U256::from(header_args.verifier_height.0);
        let freeze_header = Bytes(header_args.header.stdcode().into());
        let freeze_stakes = Bytes(header_args.stakes.into());
        let freeze_signatures = header_args.signatures;

        let freeze_stakes_tx = bridge_contract.verify_stakes(freeze_stakes.clone());
        let freeze_stakes_receipt = freeze_stakes_tx
            .send()
            .await?
            .await?;

        println!("{:#?}", freeze_stakes_receipt);

        let freeze_header_tx = bridge_contract.verify_header(
            verifier_height,
            freeze_header,
            freeze_stakes,
            freeze_signatures
        );
        let freeze_header_receipt = freeze_header_tx
            .send()
            .await?
            .await?;

        println!("{:#?}", freeze_header_receipt);

        let transaction = Bytes(tx_args.transaction.stdcode().into());
        let tx_index = U256::from(tx_args.tx_index);
        let block_height = U256::from(tx_args.block_height.0);
        let proof = tx_args.proof;

        let freeze_tx_tx = bridge_contract.verify_tx(
            transaction,
            tx_index,
            block_height,
            proof
        );
        let freeze_tx_receipt = freeze_tx_tx
            .send()
            .await?
            .await?
            .expect("Error minting tokens");

        Ok(freeze_tx_receipt)
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