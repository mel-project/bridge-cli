use themelio_structs::{
    BlockHeight,
    Header,
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
pub struct MintArgs {
    pub header_args: HeaderVerificationArgs,
    pub historical_header_args: Vec<HeaderVerificationArgs>,
}