use themelio_structs::{
    BlockHeight,
    Header,
    Transaction,
};

#[derive(Debug)]
pub struct MintArgs {
    pub freeze_height: BlockHeight,
    pub freeze_header: Header,
    pub freeze_tx: Transaction,
    pub freeze_stakes: Vec<u8>,
    pub verifier_height: BlockHeight,
    pub historical_headers: Vec<Header>,
    pub historical_stakes: Vec<Vec<u8>>,
}