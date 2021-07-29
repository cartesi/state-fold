use crate::delegate_access::DelegateAccess;
use crate::error::*;
use crate::types::*;

use offchain_utils::offchain_core;

use offchain_core::ethers::core::types::{H256, U64};
use offchain_core::types::Block;

use snafu::ResultExt;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

pub struct Archive<InitialState, Accumulator>
where
    InitialState: Clone + PartialEq + Eq + std::hash::Hash,
    Accumulator: Clone,
{
    safety_margin: usize,
    trains:
        RwLock<HashMap<InitialState, Arc<Train<InitialState, Accumulator>>>>,
}

impl<IS, A> Archive<IS, A>
where
    IS: Clone + PartialEq + Eq + std::hash::Hash,
    A: Clone,
{
    pub fn new(safety_margin: usize) -> Self {
        Archive {
            safety_margin,
            trains: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get_train(&self, initial_state: &IS) -> Arc<Train<IS, A>> {
        if let Some(train) = self.trains.read().await.get(&initial_state) {
            return Arc::clone(train);
        }

        let train =
            Arc::new(Train::new(initial_state.clone(), self.safety_margin));

        self.trains
            .write()
            .await
            .insert(initial_state.clone(), Arc::clone(&train));

        train
    }
}

pub struct Train<InitialState, State>
where
    State: Clone,
{
    initial_state: InitialState,
    safety_margin: usize,
    state_tree: RwLock<HashMap<H256, BlockState<State>>>,
    earliest_block: RwLock<U64>,
    fetch_mutex: Mutex<()>,
}

impl<IS, S> Train<IS, S>
where
    S: Clone,
{
    pub fn new(initial_state: IS, safety_margin: usize) -> Self {
        Train {
            initial_state,
            safety_margin,
            state_tree: RwLock::new(HashMap::new()),
            earliest_block: RwLock::new(U64::max_value()),
            fetch_mutex: Mutex::new(()),
        }
    }

    pub async fn get_block<D>(
        &self,
        delegate: &D,
        block_hash: H256,
    ) -> Option<D::State>
    where
        D: StateFoldDelegate<InitialState = IS, Accumulator = S>,
    {
        self.state_tree
            .read()
            .await
            .get(&block_hash)
            .clone()
            .map(|x| delegate.convert(x))
    }

    pub async fn fetch_block<D, DA: DelegateAccess + Send + Sync + 'static>(
        &self,
        delegate: &D,
        block_hash: H256,
        access: &DA,
    ) -> Result<D::State, DA>
    where
        D: StateFoldDelegate<InitialState = IS, Accumulator = S>,
    {
        // We assume this function will be called close to the latest block
        // and on the "main" chain, instead of on old blocks on "uncle chains".
        // As such we make multiple concurrent calls to fetch mutually
        // exclusive. This way, under our assumption, we save some bandwidth by
        // only fetching once, reusing the fetch for the other concurrent calls.
        // Note that `get_block` will still work normally, and won't be affected
        // by the mutual exclusion.
        let _guard = self.fetch_mutex.lock().await;

        if let Some(state) = self.get_block(delegate, block_hash).await {
            return Ok(state);
        }

        let block = access.get_block(block_hash).await?;
        let block_state = self.fold_to_leaf(delegate, &block, access).await?;
        let state = delegate.convert(&block_state);
        Ok(state)
    }
}

/// Internals
impl<IS, S> Train<IS, S>
where
    S: Clone,
{
    async fn fold_to_leaf<D, DA: DelegateAccess + Send + Sync + 'static>(
        &self,
        delegate: &D,
        leaf_block: &Block,
        access: &DA,
    ) -> Result<BlockState<S>, DA>
    where
        D: StateFoldDelegate<InitialState = IS, Accumulator = S>,
    {
        // Build stack of blocks to be processed. We are, in essence, searching
        // for an ancestral block that exists in the archive, saving in a stack
        // the "lineage" of blocks from leaf to this ancestral block.
        let mut stack = vec![];
        let mut ancestor_block = leaf_block.clone();
        let mut has_synced = false;
        loop {
            // Check if the ancestral block we're searching for doesn't exist.
            // In other words, if we cross the `earliest_block` threshold, we
            // know for sure there's no ancestral block in the archive.
            // This check guarantees the loop will not run forever.
            if ancestor_block.number < *self.earliest_block.read().await {
                if has_synced {
                    // If we've been here before, there's the requested block
                    // is an old uncle block. As such, it is unavailable. This
                    // should never happen unless the user is actively trying
                    // to sabotage this module. Without this check, this loop
                    // may run forever.
                    return BlockUnavailable {}.fail();
                } else {
                    has_synced = true;
                }

                // If we've crossed the `earliest_block` threshold, we must
                // build an ancestral block by syncing. We call our
                // `sync_to_margin` helper function, which will get us the
                // block `leaf - safety_margin` inside the archive.
                let sync_block = self
                    .sync_to_margin(delegate, leaf_block, access)
                    .await?
                    .block;

                // The function `sync_to_margin` has added an accumualtor to
                // the train, using at least `safety_margin` from the current
                // block. The actual margin used is the distance from the
                // `leaf_block` to the sync block. That means if this function
                // was called with an old enough block, it will sync to the
                // `leaf_block`, and we won't need to fold.

                // Here there are two scenarios. First is the case where the
                // stack is deeper than the margin, and the `sync_block` is a
                // successor of the current `ancestor_block`. Second is the
                // case where the stack is smaller than margin, and the
                // `sync_block` is a predecessor of the current
                // `ancestor_block`.

                // In the first case, we reduce the number of elements in the
                // stack to the margin and break.

                // The second case, we will continue on this loop until the
                // stack fills up to margin, which will happen when
                // `ancestor_block` hits the `sync_block`.

                // Note that if the margin is zero, it means the block built by
                // sync is the leaf itself. We clear the array and are done, as
                // the block we're searching was added to the train by sync. If
                // it isn't, we have to fold on the number of blocks equals to
                // the margin. In the first scenario, these blocks have already
                // been pushed to the stack and we reuse them by truncating, and
                // on the second scenario we continue on this loop until the
                // stack has all the blocks we have to fold on.

                if ancestor_block.number <= sync_block.number {
                    // ancestor_block = sync_block;
                    let margin = leaf_block.number - sync_block.number;
                    stack.truncate(margin.as_usize());
                    break;
                }
            }

            // Check if we've reached a block that we've processed before.
            if self
                .state_tree
                .read()
                .await
                .contains_key(&ancestor_block.hash)
            {
                break;
            } else {
                // If we haven't, add it to the stack.
                stack.push(ancestor_block.clone());
            }

            // Update control vars
            ancestor_block = access
                .get_block(ancestor_block.parent_hash)
                .await
                .context(AccessError)?;
        }

        // Process each block whose hash was pushed into the stack, in LIFO
        // order.
        for block in stack.into_iter().rev() {
            // Compute new state. We can guarantee the previous state is in the
            // archive, either because it is the ancestral block we found in
            // the previous step, or because we inserted it in the previous
            // iteration of this loop.
            let new_state = {
                let state_tree = self.state_tree.read().await;
                let previous_state = state_tree
                    .get(&block.parent_hash)
                    .ok_or(snafu::NoneError)
                    .context(BlockUnavailable {})?;

                let new_state = delegate
                    .fold(&previous_state.state, &block, access)
                    .await?;

                BlockState {
                    block: block.clone(),
                    state: new_state,
                }
            };

            // Add new state to state_tree.
            self.state_tree.write().await.insert(block.hash, new_state);
        }

        Ok(self
            .state_tree
            .read()
            .await
            .get(&leaf_block.hash)
            .ok_or(snafu::NoneError)
            .context(BlockUnavailable {})?
            .clone())
    }

    async fn sync_to_margin<D, DA: DelegateAccess + Send + Sync + 'static>(
        &self,
        delegate: &D,
        leaf_block: &Block,
        access: &DA,
    ) -> Result<BlockState<S>, DA>
    where
        D: StateFoldDelegate<InitialState = IS, Accumulator = S>,
    {
        // Calculate sync block. If `leaf_block` in within the `safety_margin`,
        // then use `leaf_block`. Otherwise, use current block minus
        // `safety_margin`.
        let sync_block = {
            let current = access
                .get_current_block_number()
                .await
                .context(AccessError)?;
            assert!(
                current > self.safety_margin.into(),
                "Safety margin greater than blocks in blockchain"
            );
            let minimum_sync_block = current - self.safety_margin;

            if leaf_block.number <= minimum_sync_block {
                leaf_block.clone()
            } else {
                // NOTE: There's the assumption that we are asking for a block
                // on the main path. Otherwise, `get_block_with_number` will
                // yield an unexpected result.
                access
                    .get_block(minimum_sync_block)
                    .await
                    .context(AccessError)?
            }
        };

        // Now create the state with user defined `sync`.
        let sync_state = {
            let state = delegate
                .sync(&self.initial_state, &sync_block, access)
                .await?;

            BlockState {
                block: sync_block.clone(),
                state: state,
            }
        };

        // Insert it into the archive.
        self.state_tree
            .write()
            .await
            .insert(sync_block.hash, sync_state.clone());

        // Finally, update the earliest block with the minimum value between
        // itself and sync block number. Note that it has the initial value of
        // max_uint64. As such, when syncing for the first time, it will always
        // be updated with the sync block number. If a deep reorg happens and
        // this function is called again, we guarantee the earliest block is
        // set with the actual earliest block inside the archive.
        let new_earliest_block = {
            let earliest_block = self.earliest_block.read().await;
            std::cmp::min(*earliest_block, sync_block.number)
        };
        let mut earliest_block = self.earliest_block.write().await;
        *earliest_block = new_earliest_block;

        Ok(sync_state)
    }
}
