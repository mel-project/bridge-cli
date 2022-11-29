use themelio_structs::{
    BlockHeight,
    Header,
    Transaction,
};

#[derive(Clone, Debug)]
pub struct HeaderVerificationArgs {
    pub header_height: BlockHeight,
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