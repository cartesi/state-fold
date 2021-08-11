use crate::test_struct_contract_lib::ContractCtx;
use crate::types::ExampleContractFoldDelegate;

use super::contracts::test_struct_contract;

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
pub struct StructContractFoldDelegate {
    contract_address: Address,
}

impl ExampleContractFoldDelegate for StructContractFoldDelegate {
    fn new(contract_address: Address) -> Self {
        StructContractFoldDelegate { contract_address }
    }
}

#[async_trait]
impl StateFoldDelegate for StructContractFoldDelegate {
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
                test_struct_contract::TestStructContract::new,
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
        );

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
                test_struct_contract::TestStructContract::new,
            )
            .await;

        // Get all events.
        let events =
            contract.events().query().await.context(FoldContractError {
                err: "Error querying for events",
            })?;

        let state = compute_state(events, previous_state.clone());

        Ok(state)
    }

    fn convert(&self, state: &BlockState<Self::Accumulator>) -> Self::State {
        state.clone()
    }
}

/// Computes the state from the events emission
fn compute_state(
    events: Vec<test_struct_contract::TestStructContractEvents>,
    previous_state: ContractState,
) -> ContractState {
    let ctx =
        events
            .into_iter()
            .fold(previous_state.ctx, |ctx, event| match event {
                test_struct_contract::TestStructContractEvents::ModifiedNameFilter(e) => {
                    ctx.modify_name(e.name)
                }
                test_struct_contract::TestStructContractEvents::ModifiedAgeFilter(e) => {
                    ctx.modify_age(e.age)
                }
            });

    ContractState { ctx }
}
