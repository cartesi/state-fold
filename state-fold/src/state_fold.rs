use crate::archive::Archive;
use crate::delegate_access;
use crate::error::*;
use crate::types;

use ethers::types::BlockNumber;
use offchain_utils::offchain_core::ethers;
use std::sync::Arc;

/// StateFold is a programmable object responsible for representing the moving
/// state of the blockchain. For every given initial state, StateFold is able
/// to build the state of the blockchain at every given block. StateFold is
/// resilient to reorganizations and to transport disconnections.
///
/// The programmability of this actor is achieved through a delegate object,
/// which must implement three methods: `sync`, `fold` and `convert`. Sync is
/// responsible for building a state up to a given block number. It will be
/// called to build the first state. Fold is responsible for building the next
/// state given the previous one. Convert converts the state built by the
/// delegate into an object that is exposed to the outside world.
///
/// NOTE: For StateFold to work, the blocks requested must currently be on the
/// main path of the blockchain and/or be fresh (younger than the safety
/// margin). That is, old uncle blocks are unavailable and will return an error.
pub struct StateFold<Delegate, DelegateAccess>
where
    Delegate: types::StateFoldDelegate,
    DelegateAccess: delegate_access::DelegateAccess,
{
    archive: Archive<Delegate::InitialState, Delegate::Accumulator>,
    delegate: Delegate,
    delegate_access: Arc<DelegateAccess>,
}

impl<D, DA> StateFold<D, DA>
where
    D: types::StateFoldDelegate,
    DA: delegate_access::DelegateAccess + Send + Sync + 'static,
{
    pub fn new(
        delegate: D,
        delegate_access: Arc<DA>,
        safety_margin: usize,
    ) -> Self {
        StateFold {
            delegate,
            delegate_access,
            archive: Archive::new(safety_margin),
        }
    }

    pub async fn get_state_for_block(
        &self,
        initial_state: &D::InitialState,
        block_hash: Option<ethers::types::H256>,
    ) -> Result<D::State, DA> {
        let train = self.archive.get_train(initial_state).await;

        // Use block hash from latest block if the hash is not specified
        let block_hash = block_hash.unwrap_or(
            self.delegate_access
                .get_block(BlockNumber::Latest)
                .await?
                .hash,
        );

        // First check if block exists in archive, returning it if so. This is
        // an optimization and can be removed. The following code will be able
        // to get the requested block regardless. By doing this, we won't need
        // to instantiate an unnecessary provider and we avoid running on mutex
        // locks.
        if let Some(block) = train.get_block(&self.delegate, block_hash).await {
            return Ok(block);
        }

        // If it's not on archive, do the actual work. This method has an
        // internal lock, which makes concurrent calls mutually exclusive, to
        // avoid replicated work.
        train
            .fetch_block(
                &self.delegate,
                block_hash,
                self.delegate_access.as_ref(),
            )
            .await
    }
}
