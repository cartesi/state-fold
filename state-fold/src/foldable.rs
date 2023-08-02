use crate::error::*;
use crate::{FoldMiddleware, StateFoldEnvironment, SyncMiddleware};

use eth_state_fold_types::Block;
use eth_state_fold_types::{BlockState, QueryBlock};

use eth_state_fold_types::ethers;
use ethers::providers::Middleware;

use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Foldable: Send + Sync + std::fmt::Debug + Sized {
    type InitialState: Clone + PartialEq + Eq + std::hash::Hash + Send + Sync;
    type Error: std::error::Error;
    type UserData: Send + Sync;

    async fn sync<M: Middleware + 'static>(
        initial_state: &Self::InitialState,
        block: &Block,
        env: &StateFoldEnvironment<M, Self::UserData>,
        access: Arc<SyncMiddleware<M>>,
    ) -> std::result::Result<Self, Self::Error>;

    async fn fold<M: Middleware + 'static>(
        previous_state: &Self,
        block: &Block,
        env: &StateFoldEnvironment<M, Self::UserData>,
        access: Arc<FoldMiddleware<M>>,
    ) -> std::result::Result<Self, Self::Error>;

    async fn get_state_for_block<M: Middleware + 'static, T: Into<QueryBlock> + Send + Sync>(
        initial_state: &Self::InitialState,
        fold_block: T,
        env: &StateFoldEnvironment<M, Self::UserData>,
    ) -> std::result::Result<BlockState<Self>, FoldableError<M, Self>> {
        env.get_state_for_block(initial_state, fold_block.into())
            .await
    }
}
