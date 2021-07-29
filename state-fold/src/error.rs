use crate::delegate_access;
use offchain_utils::offchain_core::ethers;

use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum SyncError<A: delegate_access::SyncAccess + 'static> {
    #[snafu(display("Sync contract error \"{}\": {}", err, source))]
    SyncContractError {
        err: String,
        source: ethers::contract::ContractError<A::SyncAccessMiddleware>,
    },

    #[snafu(display("Sync access error: {}", source))]
    SyncAccessError {
        source: delegate_access::AccessMiddlewareError<A::InnerMiddleware>,
    },

    #[snafu(display("Sync delegate Error: {}", err))]
    SyncDelegateError { err: String },

    #[snafu(display("Sync delegate Error: {}", source))]
    SyncFoldError { source: FoldError<A> },
}
pub type SyncResult<T, A> = std::result::Result<T, SyncError<A>>;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum FoldError<A: delegate_access::FoldAccess + 'static> {
    #[snafu(display("Fold contract error \"{}\": {}", err, source))]
    FoldContractError {
        err: String,
        source: ethers::contract::ContractError<A::FoldAccessMiddleware>,
    },

    #[snafu(display("Fold access error: {}", source))]
    FoldAccessError {
        source: delegate_access::AccessMiddlewareError<A::InnerMiddleware>,
    },

    #[snafu(display("Fold delegate Error: {}", err))]
    FoldDelegateError { err: String },
}
pub type FoldResult<T, A> = std::result::Result<T, FoldError<A>>;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum Error<A: delegate_access::DelegateAccess + 'static> {
    #[snafu(display("Delegate sync error: {}", err))]
    DelegateSyncError { err: SyncError<A> },

    #[snafu(display("Delegate fold error: {}", err))]
    DelegateFoldError { err: FoldError<A> },

    #[snafu(display("Access error: {}", source))]
    AccessError {
        source: delegate_access::AccessMiddlewareError<A::InnerMiddleware>,
    },

    #[snafu(display("Requested block unavailable"))]
    BlockUnavailable {},
}
pub type Result<T, A> = std::result::Result<T, Error<A>>;

impl<A> From<SyncError<A>> for Error<A>
where
    A: delegate_access::DelegateAccess,
{
    fn from(err: SyncError<A>) -> Self {
        DelegateSyncError { err }.build()
    }
}

impl<A> From<FoldError<A>> for Error<A>
where
    A: delegate_access::DelegateAccess,
{
    fn from(err: FoldError<A>) -> Self {
        DelegateFoldError { err }.build()
    }
}

impl<A> From<delegate_access::AccessMiddlewareError<A::InnerMiddleware>>
    for Error<A>
where
    A: delegate_access::DelegateAccess,
{
    fn from(
        source: delegate_access::AccessMiddlewareError<A::InnerMiddleware>,
    ) -> Self {
        Error::AccessError { source }
    }
}
