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
    pub freeze_stakes: String,
    pub historical_headers: Vec<Header>,
}