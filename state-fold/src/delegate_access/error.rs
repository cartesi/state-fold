use ethers::providers::{FromErr, Middleware};
use offchain_utils::offchain_core::ethers;

use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum AccessError<M: Middleware + 'static> {
    #[snafu(display("Ethers provider error: {}", source))]
    EthersProviderError { source: M::Error },

    #[snafu(display("Requested log unavailable"))]
    LogUnavailable {},

    #[snafu(display("Requested block incomplete"))]
    BlockIncomplete {},

    #[snafu(display("Requested block unavailable"))]
    BlockUnavailable {},

    #[snafu(display("Partition error: {:?}", sources))]
    PartitionError { sources: Vec<M::Error> },
}
pub type Result<T, M> = std::result::Result<T, AccessError<M>>;

impl<M: Middleware> FromErr<M::Error> for AccessError<M> {
    fn from(source: M::Error) -> Self {
        AccessError::EthersProviderError { source }
    }
}
