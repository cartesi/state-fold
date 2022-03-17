use crate::Foldable;

use state_fold_types::ethers;

use ethers::providers::{FromErr, Middleware};

use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum FoldableError<M: Middleware + 'static, F: Foldable + 'static> {
    #[snafu(display("Inner error: {}", source))]
    InnerError { source: F::Error },

    #[snafu(display("Middleware error: {}", source))]
    MiddlewareError { source: M::Error },

    #[snafu(display("Requested block unavailable"))]
    BlockUnavailable {},

    #[snafu(display("Requested block incomplete"))]
    BlockIncomplete {},

    #[snafu(display("Requested log unavailable"))]
    LogUnavailable {},

    #[snafu(display("Partition error: {:?}", sources))]
    PartitionError { sources: Vec<M::Error> },

    #[snafu(display(
        "Query depth of ({}) higher than blocks in chain ({})",
        depth,
        current_block
    ))]
    QueryDepthTooHigh { depth: usize, current_block: usize },
}

impl<M: Middleware, F: Foldable> FromErr<M::Error> for FoldableError<M, F> {
    fn from(source: M::Error) -> Self {
        FoldableError::MiddlewareError { source }
    }
}
