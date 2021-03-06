use crate::test_mapping_contract_lib::ContractCtx;
use crate::types::ExampleContractFoldDelegate;

use super::contracts::test_mapping_contract;

use offchain_core::types::Block;
use state_fold::{
    delegate_access::{FoldAccess, SyncAccess},
    error::*,
    types::{BlockState, StateFoldDelegate},
    utils as fold_utils,
};

use async_trait::async_trait;
use ethers::types::Address;
use snafu::ResultExt;

/// Test Contract state, to be passed to and returned by fold.
#[derive(Clone, Debug)]
pub struct ContractState {
    pub ctx: ContractCtx,
}

/// Test Contract StateFold Delegate, which implements `sync` and `fold`.
#[derive(Default)]
pub struct MappingContractFoldDelegate {
    contract_address: Address,
}

impl ExampleContractFoldDelegate for MappingContractFoldDelegate {
    fn new(contract_address: Address) -> Self {
        MappingContractFoldDelegate { contract_address }
    }
}

#[async_trait]
impl StateFoldDelegate for MappingContractFoldDelegate {
    type InitialState = ();
    type Accumulator = ContractState;
    type State = BlockState<Self::Accumulator>;

    async fn sync<A: SyncAccess + Send + Sync>(
        &self,
        _: &Self::InitialState,
        block: &Block,
        access: &A,
    ) -> SyncResult<Self::Accumulator, A> {
        let contract = access
            .build_sync_contract(
                self.contract_address,
                block.number,
                test_mapping_contract::TestMappingContract::new,
            )
            .await;

        // Get all events.
        let events =
            contract.events().query().await.context(SyncContractError {
                err: "Error querying for events",
            })?;

        let state = compute_state(
            events,
            ContractState {
                ctx: ContractCtx::default(),
            },
        )
        .map_err(|e| {
            SyncDelegateError {
                err: format!("Could not update contract state: {}", e),
            }
            .build()
        })?;

        Ok(state)
    }

    async fn fold<A: FoldAccess + Send + Sync>(
        &self,
        previous_state: &Self::Accumulator,
        block: &Block,
        access: &A,
    ) -> FoldResult<Self::Accumulator, A> {
        // Check if there was (possibly) some log emited on this block.
        let bloom = block.logs_bloom;
        if !fold_utils::contains_address(&bloom, &self.contract_address) {
            return Ok(previous_state.clone());
        }

        let contract = access
            .build_fold_contract(
                self.contract_address,
                block.hash,
                test_mapping_contract::TestMappingContract::new,
            )
            .await;

        // Get all events.
        let events =
            contract.events().query().await.context(FoldContractError {
                err: "Error querying for events",
            })?;

        let state =
            compute_state(events, previous_state.clone()).map_err(|e| {
                FoldDelegateError {
                    err: format!("Could not update contract state: {}", e),
                }
                .build()
            })?;

        Ok(state)
    }

    fn convert(&self, state: &BlockState<Self::Accumulator>) -> Self::State {
        state.clone()
    }
}

/// Computes the state from the events emission
fn compute_state(
    events: Vec<test_mapping_contract::TestMappingContractEvents>,
    previous_state: ContractState,
) -> crate::test_mapping_contract_lib::Result<ContractState> {
    let ctx =
        events
            .into_iter()
            .try_fold(previous_state.ctx, |ctx, event| match event {
                test_mapping_contract::TestMappingContractEvents::RemovedFilter(e) => {
                    ctx.remove(e.key)
                }

                test_mapping_contract::TestMappingContractEvents::ModifiedFilter(e) => {
                    Ok(ctx.modify(e.key, e.value))
                }
            })?;

    Ok(ContractState { ctx })
}
