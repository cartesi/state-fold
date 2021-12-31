use crate::delegate_access::{FoldMiddleware, SyncMiddleware};
use crate::error::*;
use crate::types::*;
use crate::Foldable;

use super::global_archive::GlobalArchive;

use ethers::core::types::{BlockId, BlockNumber, U64};
use ethers::providers::Middleware;
use offchain_utils::offchain_core::ethers;
use offchain_utils::offchain_core::types::Block;

use snafu::{ensure, ResultExt};
use std::convert::TryInto;
use std::sync::Arc;

pub struct StateFoldEnvironment<M: Middleware, UD> {
    inner_middleware: Arc<M>,
    genesis_block: U64,

    // If the Ethereum node has a limit on the number of events returned by the
    // method `eth_getLogs` (such as Infura, with a 10k events limit and <10s
    // query limit), `query_limit_error_codes` contains the error codes of when
    // the request fails. In case of a match, Access will attempt a partition.
    // In case of Infura, the error code is `-32005`. An empty array means a
    // partition will never be attempted.
    query_limit_error_codes: Vec<i32>,

    // When attempting a partition, the number of concurrent fetches is bounded
    // by `concurrent_events_fetch + 1`. We recommend something like `16`.
    concurrent_events_fetch: usize,

    global_archive: GlobalArchive,

    user_data: UD,
}

impl<M: Middleware + 'static, UD> StateFoldEnvironment<M, UD> {
    pub fn new(
        inner_middleware: Arc<M>,
        safety_margin: usize,
        genesis_block: U64,
        query_limit_error_codes: Vec<i32>,
        concurrent_events_fetch: usize,
        user_data: UD,
    ) -> Self {
        let global_archive = GlobalArchive::new(safety_margin);

        Self {
            inner_middleware,
            genesis_block,
            query_limit_error_codes,
            concurrent_events_fetch,
            global_archive,
            user_data,
        }
    }

    pub fn user_data(&self) -> &UD {
        &self.user_data
    }

    pub fn inner_middleware(&self) -> Arc<M> {
        self.inner_middleware.clone()
    }

    pub async fn get_state_for_block<
        F: Foldable<UserData = UD> + Send + Sync + 'static,
    >(
        &self,
        initial_state: &F::InitialState,
        fold_block: QueryBlock,
    ) -> Result<BlockState<F>, FoldableError<M, F>> {
        let archive = self.global_archive.get_archive::<F>().await;
        let train = archive.get_train(initial_state).await;

        // First check if block exists in archive, returning it if so. This is
        // an optimization and can be removed. The following code will be able
        // to get the requested block regardless. By doing this, we won't need
        // to instantiate an unnecessary provider and we avoid running on mutex
        // locks. This match also reduces unecessary `get_block` queries.
        let block = match fold_block {
            QueryBlock::Latest => {
                let b = self.get_block(BlockNumber::Latest).await?;

                // Check if exists in archive.
                if let Some(block_state) = train.get_block_state(&b.hash).await
                {
                    return Ok(block_state);
                }

                b
            }

            QueryBlock::BlockHash(h) => {
                // Check if exists in archive.
                if let Some(block_state) = train.get_block_state(&h).await {
                    return Ok(block_state);
                }

                self.get_block(h).await?
            }

            QueryBlock::BlockNumber(n) => {
                let b = self.get_block(n).await?;

                // Check if exists in archive.
                if let Some(block_state) = train.get_block_state(&b.hash).await
                {
                    return Ok(block_state);
                }

                b
            }

            QueryBlock::BlockDepth(depth) => {
                let b = {
                    let current = self.get_current_block_number().await?;
                    ensure!(
                        current > depth.into(),
                        QueryDepthTooHigh {
                            depth,
                            current_block: current.as_usize()
                        }
                    );
                    self.get_block(current - depth).await?
                };

                // Check if exists in archive.
                if let Some(block_state) = train.get_block_state(&b.hash).await
                {
                    return Ok(block_state);
                }

                b
            }

            QueryBlock::Block(b) => {
                // Check if exists in archive.
                if let Some(block_state) = train.get_block_state(&b.hash).await
                {
                    return Ok(block_state);
                }

                b
            }
        };

        // If it's not on archive, do the actual work. This method has an
        // internal lock, which makes concurrent calls mutually exclusive, to
        // avoid replicated work.
        train.fetch_block_state(self, &block).await
    }
}

///
/// Internals

impl<M: Middleware + 'static, UD> StateFoldEnvironment<M, UD> {
    pub(crate) fn sync_access(&self, block: &Block) -> Arc<SyncMiddleware<M>> {
        let middleware = SyncMiddleware::new(
            Arc::clone(&self.inner_middleware),
            self.genesis_block,
            block.number,
            self.query_limit_error_codes.clone(),
            self.concurrent_events_fetch,
        );

        Arc::new(middleware)
    }

    pub(crate) fn fold_access(&self, block: &Block) -> Arc<FoldMiddleware<M>> {
        let middleware =
            FoldMiddleware::new(Arc::clone(&self.inner_middleware), block.hash);

        Arc::new(middleware)
    }

    pub(crate) async fn get_block<
        F: Foldable + Send + Sync + 'static,
        T: Into<BlockId> + Send + Sync,
    >(
        &self,
        block: T,
    ) -> Result<Block, FoldableError<M, F>> {
        self.inner_middleware
            .get_block(block)
            .await
            .context(MiddlewareError)?
            .ok_or(snafu::NoneError)
            .context(BlockUnavailable)?
            .try_into()
            .map_err(|_| snafu::NoneError)
            .context(BlockIncomplete)
    }

    pub(crate) async fn get_current_block_number<
        F: Foldable + Send + Sync + 'static,
    >(
        &self,
    ) -> Result<U64, FoldableError<M, F>> {
        self.inner_middleware
            .get_block_number()
            .await
            .context(MiddlewareError)
    }
}
