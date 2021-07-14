use crate::partition_events::*;

use offchain_utils::offchain_core::ethers;
use offchain_utils::offchain_core::types::Block;

use async_trait::async_trait;
use ethers::core::types::{
    Address, BlockId, Bytes, Filter, Log, TransactionRequest, H256, U64,
};
use ethers::providers::{FromErr, Middleware};
use snafu::ResultExt;
use snafu::Snafu;
use std::convert::TryInto;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum AccessMiddlewareError<M: Middleware + 'static> {
    #[snafu(display("Ethers provider error: {}", source))]
    EthersProviderError { source: M::Error },

    #[snafu(display("Requested log unavailable"))]
    LogUnavailable {},

    #[snafu(display("Requested block incomplete"))]
    BlockIncomplete {},

    #[snafu(display("Partition error: {:?}", sources))]
    PartitionError {
        sources: Vec<AccessMiddlewareError<M>>,
    },
}
pub type Result<T, M> = std::result::Result<T, AccessMiddlewareError<M>>;

///
/// Trait Definitions
///

#[async_trait]
pub trait DelegateAccess: FoldAccess + SyncAccess {
    async fn get_block<T: Into<BlockId> + Send + Sync>(
        &self,
        block: T,
    ) -> Result<Block, Self::InnerMiddleware>;

    async fn get_current_block_number(
        &self,
    ) -> Result<U64, Self::InnerMiddleware>;
}

#[async_trait]
pub trait SyncAccess: FoldAccess {
    type SyncAccessMiddleware: Middleware<
        Inner = Self::InnerMiddleware,
        Error = AccessMiddlewareError<Self::InnerMiddleware>,
    >;

    /// Sets the default value for every query to a specific block number. More
    /// specifically, a query for events will happen from some initial block to
    /// `block_number`, and a call will happen at `block_number`. Setting the
    /// block on a call derived from `Contract<Self::Middleware>` overrides
    /// this binding. However, setting an event query will do nothing: this
    /// binding will override the set block.
    async fn build_sync_contract<T: Into<Address> + Send + Sync, C>(
        &self,
        address: T,
        block_number: U64,
        init: impl FnOnce(T, Arc<Self::SyncAccessMiddleware>) -> C
            + Send
            + Sync
            + 'static,
    ) -> C;
}

#[async_trait]
pub trait FoldAccess: std::fmt::Debug {
    type InnerMiddleware: Middleware + 'static;

    type FoldAccessMiddleware: Middleware<
        Inner = Self::InnerMiddleware,
        Error = AccessMiddlewareError<Self::InnerMiddleware>,
    >;

    /// Binds every query to a specific block hash. More specifically, query for
    /// events and calls will be done at `block_hash` by default. Setting the
    /// block on a query derived from `Contract<Self::Middleware>` does nothing:
    /// they will be overriden.
    async fn build_fold_contract<T: Into<Address> + Send + Sync, C>(
        &self,
        address: T,
        block_hash: H256,
        init: impl FnOnce(T, Arc<Self::FoldAccessMiddleware>) -> C
            + Send
            + Sync
            + 'static,
    ) -> C;
}

///
/// Concrete implementation of DelegateAccess
///

#[derive(Debug)]
pub struct Access<M: Middleware> {
    middleware: RwLock<Arc<M>>,
    genesis_block: U64,

    // If the Ethereum node has a limit on the number of events returned by the
    // method `eth_getLogs` (such as Infura, with a 10k events limit and <10s
    // query limit), `query_limit_error_codes` contains the error codes of when
    // the request fails. In case of a match, Access will attempt a partiotion.
    // In case of Infura, the error code is `-32005`. An empty array means a
    // partition will never be attempted.
    query_limit_error_codes: Vec<i32>,

    // When attempting a partition, the number of concurrent fetches is bounded
    // by `concurrent_events_fetch + 1`. We recommend something like `16`.
    concurrent_events_fetch: usize,
}

impl<M: Middleware> Access<M> {
    pub fn new(
        middleware: Arc<M>,
        genesis_block: U64,
        query_limit_error_codes: Vec<i32>,
        concurrent_events_fetch: usize,
    ) -> Self {
        Self {
            middleware: RwLock::new(middleware),
            genesis_block,
            query_limit_error_codes,
            concurrent_events_fetch,
        }
    }

    pub async fn reset(&self, middleware: Arc<M>) {
        let mut m = self.middleware.write().await;
        *m = middleware;
    }
}

#[async_trait]
impl<M: Middleware + 'static> DelegateAccess for Access<M> {
    async fn get_block<T: Into<BlockId> + Send + Sync>(
        &self,
        block: T,
    ) -> Result<Block, M> {
        self.middleware
            .read()
            .await
            .get_block(block)
            .await
            .context(EthersProviderError {})?
            .ok_or(snafu::NoneError)
            .context(BlockIncomplete)?
            .try_into()
            .map_err(|_| snafu::NoneError)
            .context(BlockIncomplete)
    }

    async fn get_current_block_number(&self) -> Result<U64, M> {
        self.middleware
            .read()
            .await
            .get_block_number()
            .await
            .context(EthersProviderError {})
    }
}

#[async_trait]
impl<M: Middleware + 'static> SyncAccess for Access<M> {
    type SyncAccessMiddleware = SyncMiddleware<M>;

    async fn build_sync_contract<T: Into<Address> + Send + Sync, C>(
        &self,
        address: T,
        block_number: U64,
        init: impl FnOnce(T, Arc<Self::SyncAccessMiddleware>) -> C
            + Send
            + Sync
            + 'static,
    ) -> C {
        let middleware = Arc::new(SyncMiddleware::new(
            Arc::clone(&*self.middleware.read().await),
            self.genesis_block,
            block_number,
            self.query_limit_error_codes.clone(),
            self.concurrent_events_fetch,
        ));

        init(address, middleware)
    }
}

#[async_trait]
impl<M: Middleware + 'static> FoldAccess for Access<M> {
    type InnerMiddleware = M;
    type FoldAccessMiddleware = FoldMiddleware<M>;

    async fn build_fold_contract<T: Into<Address> + Send + Sync, C>(
        &self,
        address: T,
        block_hash: H256,
        init: impl FnOnce(T, Arc<Self::FoldAccessMiddleware>) -> C
            + Send
            + Sync
            + 'static,
    ) -> C {
        let middleware = Arc::new(FoldMiddleware::new(
            Arc::clone(&*self.middleware.read().await),
            block_hash,
        ));

        init(address, middleware)
    }
}

///
/// Middleware implementation
///

impl<M: Middleware> FromErr<M::Error> for AccessMiddlewareError<M> {
    fn from(source: M::Error) -> AccessMiddlewareError<M> {
        AccessMiddlewareError::EthersProviderError { source }
    }
}

#[derive(Debug)]
pub struct SyncMiddleware<M> {
    inner: Arc<M>,
    genesis: U64,
    block_number: U64,
    query_limit_error_codes: Vec<i32>,
    concurrent_events_fetch: usize,
}

impl<M> SyncMiddleware<M>
where
    M: Middleware,
{
    fn new(
        inner: Arc<M>,
        genesis: U64,
        block_number: U64,
        query_limit_error_codes: Vec<i32>,
        concurrent_events_fetch: usize,
    ) -> Self {
        Self {
            inner,
            genesis,
            block_number,
            query_limit_error_codes,
            concurrent_events_fetch,
        }
    }

    pub fn get_inner(&self) -> Arc<M> {
        Arc::clone(&self.inner)
    }
}

#[async_trait]
impl<M> Middleware for SyncMiddleware<M>
where
    M: Middleware + 'static,
{
    type Error = AccessMiddlewareError<M>;
    type Provider = M::Provider;
    type Inner = M;

    fn inner(&self) -> &M {
        Arc::as_ref(&self.inner)
    }

    async fn call(
        &self,
        tx: &TransactionRequest,
        block: Option<BlockId>,
    ) -> std::result::Result<Bytes, Self::Error> {
        // If user provides a block, we use it. Otherwise, we use the defualt
        // blocks given during instantiation.
        let block = block.or_else(|| Some(self.block_number.into()));
        self.inner().call(tx, block).await.map_err(FromErr::from)
    }

    async fn get_logs(
        &self,
        filter: &Filter,
    ) -> std::result::Result<Vec<Log>, Self::Error> {
        let partition_events =
            PartitionEvents::new(self.concurrent_events_fetch, self, filter);

        // Unlike call, we always override user provided range. This is a
        // limitation of ethers, because the type that holds the range is
        // private.
        let mut logs = partition_events
            .get_events(self.genesis.as_u64(), self.block_number.as_u64())
            .await
            .map_err(|err_arr| PartitionError { sources: err_arr }.build())?;

        sort_logs(&mut logs)?;
        Ok(logs)
    }
}

#[async_trait]
impl<M> PartitionProvider<Log, Filter> for SyncMiddleware<M>
where
    M: Middleware + 'static,
{
    type ProviderErr = <Self as Middleware>::Error;

    async fn fetch_events_with_range(
        &self,
        data: &Filter,
        from_block: u64,
        to_block: u64,
    ) -> std::result::Result<Vec<Log>, Self::ProviderErr> {
        let filter = data.clone().from_block(from_block).to_block(to_block);

        let logs = self
            .inner()
            .get_logs(&filter)
            .await
            .map_err(FromErr::from)?;

        Ok(logs)
    }

    fn should_retry_with_partition(&self, err: &Self::ProviderErr) -> bool {
        for code in &self.query_limit_error_codes {
            let s = format!("{:?}", err);
            if s.contains(&code.to_string()) {
                return true;
            }
        }

        false
    }
}

#[derive(Debug)]
pub struct FoldMiddleware<M> {
    inner: Arc<M>,
    block_hash: H256,
}

impl<M> FoldMiddleware<M>
where
    M: Middleware,
{
    fn new(inner: Arc<M>, block_hash: H256) -> Self {
        Self { inner, block_hash }
    }
}

#[async_trait]
impl<M> Middleware for FoldMiddleware<M>
where
    M: Middleware + 'static,
{
    type Error = AccessMiddlewareError<M>;
    type Provider = M::Provider;
    type Inner = M;

    fn inner(&self) -> &M {
        Arc::as_ref(&self.inner)
    }

    async fn call(
        &self,
        tx: &TransactionRequest,
        block: Option<BlockId>,
    ) -> std::result::Result<Bytes, Self::Error> {
        // If user provides a block, we use it. Otherwise, we use the defualt
        // block given during instantiation.
        let block = block.or_else(|| Some(self.block_hash.into()));
        self.inner().call(tx, block).await.map_err(FromErr::from)
    }

    async fn get_logs(
        &self,
        filter: &Filter,
    ) -> std::result::Result<Vec<Log>, Self::Error> {
        // Unlike call, we always override user provided range. This is a
        // limitation of ethers, because the type that holds the range is
        // private.
        let filter = filter.clone().at_block_hash(self.block_hash);
        let mut logs = self
            .inner()
            .get_logs(&filter)
            .await
            .map_err(FromErr::from)?;

        sort_logs(&mut logs)?;
        Ok(logs)
    }
}

fn sort_logs<M: Middleware>(logs: &mut Vec<Log>) -> Result<(), M> {
    for log in logs.iter() {
        if !(log.block_number.is_some() && log.log_index.is_some()) {
            return LogUnavailable {}.fail();
        }
    }

    logs.sort_by(|a, b| {
        let c = a.block_number.unwrap().cmp(&b.block_number.unwrap());
        if let std::cmp::Ordering::Equal = c {
            a.log_index.unwrap().cmp(&b.log_index.unwrap())
        } else {
            c
        }
    });

    Ok(())
}
