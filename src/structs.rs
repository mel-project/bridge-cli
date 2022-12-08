use ethers::types::{Address, BlockId, H256};
use themelio_structs::{
    BlockHeight,
    CoinData,
    Header,
    Transaction,
    TxHash,
};

#[derive(Clone, Debug)]
pub struct HeaderVerificationArgs {
    pub header: Header,
    pub verifier_height: BlockHeight,
    pub stakes: Vec<u8>,
    pub signatures: Vec<[u8; 32]>,
}

#[derive(Debug)]
pub struct TxVerificationArgs {
    pub transaction: Transaction,
    pub tx_index: u32,
    pub block_height: BlockHeight,
    pub proof: Vec<[u8; 32]>,
}

#[derive(Debug)]
pub struct MintArgs {
    pub historical_header_args: Vec<HeaderVerificationArgs>,
    pub header_args: HeaderVerificationArgs,
    pub tx_args: TxVerificationArgs,
}

#[derive(Clone, Debug)]
pub struct CoinDataHeightHash {
    pub coin_data: CoinData,
    pub block_height: BlockHeight,
    pub tx_hash: TxHash,
}

#[derive(Debug)]
pub struct ThawArgs {
    pub coins_slot: H256,
    pub contract_address: Address,
    pub tx_hash: H256,
    pub coin: CoinData,
    pub block_id: BlockId,
}

#[derive(Debug)]
pub struct MerkleProof {
    pub bytes: Vec<[u8; 32]>,
    pub tx_index: u32,
}