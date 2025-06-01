use alloc::string::String;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid pool address")]
    InvalidPoolAddress,
    #[error("Invalid tick range")]
    InvalidTickRange,
    #[error("Ticks out of range")]
    TicksOutOfRange,
    #[error("Contract call failed: {0}")]
    ContractCall(String),
}