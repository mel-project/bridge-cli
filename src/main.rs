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
use async_compat::CompatExt;
use clap::Parser;
use colored::Colorize;
use ethers::{
    abi::{ParamType},
    prelude::SignerMiddleware,
    providers::{Http, Provider, Middleware},
    signers::{LocalWallet, Signer},
    types::{BlockId, BlockNumber, Bytes, H160, H256, TransactionReceipt, U64, U256},
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
    Address,
    BlockHeight,
    CoinDataHeight,
    CoinID,
    Header,
    NetID,
    Transaction,
    TxHash,
    STAKE_EPOCH,
};
use tmelcrypt::HashVal;

use cli::*;
use structs::*;

const _COV_PATH: &str = "bridge-covenants/bridge.melo";
const BRIDGE_ADDRESS: &str = "5d2dfe6651b2ba9ff032b85195665b17b608baff";
const COINS_SLOT: H256 = H256([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 254]);
const CONTRACT_DEPLOYMENT_HEIGHT: BlockNumber = BlockNumber::Number(U64([8062514]));

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
            .compat()
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

async fn get_coin(tx_hash: TxHash) -> Result<CoinDataHeight> {
    smol::block_on( async move {
        let snapshot = THE_CLIENT.snapshot().await?;

        let coin = snapshot.get_coin(CoinID::new(tx_hash, 0))
            .await?
            .expect("Coin with provided tx hash does not exist");

        Ok(coin)
    })
}

async fn get_tx(tx_hash: TxHash) -> Result<Transaction> {
    smol::block_on( async move {
        let snapshot = THE_CLIENT.snapshot().await?;

        let tx = snapshot.get_transaction(tx_hash)
            .await?
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
    todo!()
}

async fn get_signatures(_block_height: BlockHeight) -> Result<Vec<[u8; 32]>> {
    todo!()
}

async fn get_proof(_block_height: BlockHeight, _tx_hash: TxHash) -> Result<MerkleProof> {
    todo!()
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

async fn get_mint_args(freeze_data: FreezeData) -> Result<MintArgs> {
    smol::block_on( async move {
        let freeze_height = freeze_data.block_height;
        let freeze_epoch = freeze_height.epoch();
        let freeze_header = get_header(freeze_height).await?;
        let freeze_tx = get_tx(freeze_data.tx_hash).await?;
        let freeze_stakes = get_stakes(freeze_height).await?;
        let freeze_signatures = get_signatures(freeze_height).await?;
        let freeze_proof = get_proof(freeze_height, freeze_tx.hash_nosigs()).await?;

        let bridge_contract = ThemelioBridge::new(<[u8; 20]>::from_hex(BRIDGE_ADDRESS)?, ETH_CLIENT.clone());
        let header_verified_filter = bridge_contract.header_verified_filter();

        let filter = header_verified_filter
            .filter
            .from_block(CONTRACT_DEPLOYMENT_HEIGHT)
            .to_block(BlockNumber::Latest);

        let logs = ETH_CLIENT
            .get_logs(&filter)
            .compat()
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

async fn get_verification_limit() -> Result<u32> {
    smol::block_on(async {
        Ok(100)
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
                            historical_args.verifier_height.0.into(),
                            historical_args.header.stdcode().into(),
                            historical_args.stakes.into(),
                            historical_args.signatures,
                            get_verification_limit().await.expect("Error simulating header verification").into()
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
            .compat()
            .await?
            .await?;

        println!("{:#?}", freeze_stakes_receipt);

        let freeze_header_tx = bridge_contract.verify_header(
            verifier_height,
            freeze_header,
            freeze_stakes,
            freeze_signatures,
            get_verification_limit().await.expect("Error simulating header verification").into()
        );
        let freeze_header_receipt = freeze_header_tx
            .send()
            .compat()
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
            .compat()
            .await?
            .await?
            .expect("Error minting tokens");

        Ok(freeze_tx_receipt)
    })
}

async fn get_frozen_coins() -> Result<Vec<TxHash>> {
    smol::block_on(async {
        let bridge_contract = ThemelioBridge::new(<[u8; 20]>::from_hex(BRIDGE_ADDRESS)?, ETH_CLIENT.clone());

        let mint_filter = bridge_contract
            .tx_verified_filter()
            .filter
            .from_block(CONTRACT_DEPLOYMENT_HEIGHT)
            .to_block(BlockNumber::Latest);
        let mint_logs = ETH_CLIENT
            .get_logs(&mint_filter)
            .compat()
            .await?;

        let coins_minted: Vec<TxHash> = mint_logs
            .into_iter()
            .map(|mint_log| {
                let tx_hash = mint_log.topics[2];

                TxHash(HashVal(tx_hash.0))
            })
            .collect();

        let burn_filter = bridge_contract
            .tokens_burned_filter()
            .filter
            .from_block(CONTRACT_DEPLOYMENT_HEIGHT)
            .to_block(BlockNumber::Latest);
        let burn_logs = ETH_CLIENT
            .get_logs(&burn_filter)
            .compat()
            .await?;

        let coins_burned: Vec<TxHash> = burn_logs
            .into_iter()
            .map(|burn_log| {
                let tokens = ethers::abi::decode(
                    &[ParamType::Array(Box::new(ParamType::FixedBytes(32)))],
                    &burn_log.data
                ).expect("Error decoding minted coins");

                let txhash_vec_vec: Vec<TxHash> = tokens
                    .into_iter()
                    .map(|token_arr| {
                        let hashes: Vec<TxHash> = token_arr
                            .into_array()
                            .expect("Error turning token into array")
                            .into_iter()
                            .map(|token| {
                                let hash = TxHash(HashVal(
                                    token
                                        .into_fixed_bytes()
                                        .expect("Error converting token to byte vector")
                                        .try_into()
                                        .expect("Error converting vector to fixed-size array")
                                ));

                                hash
                            })
                            .collect();

                        hashes
                    })
                    .flatten()
                    .collect();

                txhash_vec_vec
            })
            .flatten()
            .collect();

        let coins_left: Vec<TxHash> = coins_minted
            .into_iter()
            .filter(|coin| {
                !coins_burned.contains(coin)
            })
            .collect();

        Ok(coins_left)
    })
}

async fn choose_coin_to_thaw(mut twriter: impl Write) -> Result<CoinDataHeightHash> {
    smol::block_on(async move {
        let coin_hashes = get_frozen_coins().await?;

        if coin_hashes.is_empty() {
            writeln!(twriter, "{}", "There are currently no frozen Themelio coins that can be bridged back.".blue())?;

            anyhow::bail!("Exiting");
        }

        let coins: Vec<CoinDataHeight> = futures::future::join_all(
            coin_hashes
                .clone()
                .into_iter()
                .map(|coin_hash| async move {
                    let coin = get_coin(coin_hash).await.expect("Error retreiving coin");

                    coin
                })
        ).await;

        writeln!(twriter, "{}", "COINS".bold().yellow())?;
        writeln!(twriter, "{}", "#\tHEIGHT\tTX HASH\tDENOM\tVALUE")?;

        let choice_range = 0..coin_hashes.len();
        for idx in choice_range.clone() {
            writeln!(
                twriter,
                "{}\t{}\t{}\t{}\t{}",
                idx + 1,
                coins[idx].height,
                coin_hashes[idx],
                coins[idx].coin_data.denom,
                coins[idx].coin_data.value
            )?;
        }

        writeln!(twriter, "{}", "Which coin would you like to bridge back to Themelio? ")?;
        let choice = smol::unblock(move || {
            let mut choice = [0u8; 1];

            match STDIN_BUFFER.lock().as_deref_mut() {
                Ok(stdin) => {
                    while choice[0].is_ascii_whitespace() || choice[0] == 0 {
                        stdin.read_exact(&mut choice)?;
                    }
                    Ok(choice)
                }

                Err(_) => return Err(anyhow::anyhow!("Unknown buffer unlock problem")),
            }
        })
        .await?;

        let validated_choice = if choice_range.contains(&(choice[0] as usize)) {
            choice[0]
        } else {
            return Err(anyhow::anyhow!("Invalid option"));
        };

        let chosen_coin_hash = coin_hashes[validated_choice as usize];
        let chosen_coin = &coins[validated_choice as usize];

        twriter.flush()?;

        Ok(CoinDataHeightHash{
            coin_data: chosen_coin.coin_data.clone(),
            block_height: chosen_coin.height,
            tx_hash: chosen_coin_hash
        })
    })
}

async fn burn_tokens(coin: CoinDataHeightHash, themelio_recipient: Address) -> Result<TransactionReceipt> {
    smol::block_on(async {
        let bridge_contract = ThemelioBridge::new(<[u8; 20]>::from_hex(BRIDGE_ADDRESS)?, ETH_CLIENT.clone());

        let account = bridge_contract.client().address();
        let tx_hash = coin.tx_hash.0.0;
        let themelio_recipient = themelio_recipient.0.0;

        let burn_tx = bridge_contract.burn(
            account,
            tx_hash,
            themelio_recipient
        );
        let burn_receipt = burn_tx
            .send()
            .compat()
            .await?
            .await?
            .expect("Error burning tokens");

        Ok(burn_receipt)
    })
}

async fn to_thaw_args(coin: CoinDataHeightHash, burn_receipt: TransactionReceipt) -> Result<ThawArgs> {
    assert!(burn_receipt.status.expect("Error retreiving burn status") == 1.into());

    let tx_hash = burn_receipt.transaction_hash;

    Ok(ThawArgs {
        coins_slot: COINS_SLOT,
        contract_address: H160(<[u8; 20]>::from_hex(BRIDGE_ADDRESS)?),
        tx_hash,
        coin: coin.coin_data,
        block_id: BlockId::Number(BlockNumber::Number(coin.block_height.0.into()))
    })
}

async fn craft_thaw_tx(thaw_args: ThawArgs) -> Result<Transaction> {
    todo!()
}

fn main() -> Result<()> {
    smol::block_on(async move {
        let twriter = TabWriter::new(std::io::stderr());

        let subcommand = CLI_ARGS.subcommand.clone();
        let dry_run = CLI_ARGS.dry_run;

        match subcommand {
            Subcommand::MintTokens(freeze_data) => {
                let mint_args: MintArgs = get_mint_args(freeze_data).await?;
                println!("Mintargs: {:#?}", mint_args);

                let mint_receipt = mint_tokens(mint_args).await?;
                println!("Tokens minted successfully:\n{:#?}", mint_receipt);
            }

            Subcommand::BurnTokens(burn_args) => {
                let coin = choose_coin_to_thaw(twriter).await?;

                let burn_data = burn_tokens(coin.clone(), burn_args.themelio_recipient).await?;
                println!("Tokens burned successfully:\n{:?}", burn_data);

                let thaw_args = to_thaw_args(coin, burn_data).await?;
                let thaw_tx = craft_thaw_tx(thaw_args).await?;

                println!("Here is the tx you need for thawing: {:#?}\nMore info at https://github.com/themeliolabs/bridge-cli", thaw_tx);
            }
        }

        Ok(())
    })
}