use crate::delegate_access;
use crate::error::*;

use offchain_utils::offchain_core::types::Block;

use async_trait::async_trait;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockState<State> {
    pub block: Block,
    pub state: State,
}

#[async_trait]
pub trait StateFoldDelegate {
    type InitialState: Clone + PartialEq + Eq + std::hash::Hash;
    type Accumulator: Clone;
    type State;

    async fn sync<A: delegate_access::SyncAccess + Send + Sync>(
        &self,
        initial_state: &Self::InitialState,
        block: &Block,
        access: &A,
    ) -> SyncResult<Self::Accumulator, A>;

    async fn fold<A: delegate_access::FoldAccess + Send + Sync>(
        &self,
        previous_state: &Self::Accumulator,
        block: &Block,
        access: &A,
    ) -> FoldResult<Self::Accumulator, A>;

    fn convert(
        &self,
        accumulator: &BlockState<Self::Accumulator>,
    ) -> Self::State;
}
