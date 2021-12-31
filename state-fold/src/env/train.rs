use crate::error::*;
use crate::types::*;
use crate::Foldable;

use super::StateFoldEnvironment;

use offchain_utils::offchain_core::types::Block;

use ethers::core::types::{H256, U64};
use ethers::providers::Middleware;
use offchain_utils::offchain_core::ethers;

use snafu::ResultExt;
use std::collections::HashMap;
use tokio::sync::{Mutex, RwLock};

pub(crate) struct Train<F>
where
    F: Foldable,
{
    initial_state: F::InitialState,
    safety_margin: usize,
    state_tree: RwLock<HashMap<H256, BlockState<F>>>,
    earliest_block: RwLock<U64>,
    fetch_mutex: Mutex<()>,
}

impl<F> Train<F>
where
    F: Foldable + Send + Sync + 'static,
{
    pub fn new(initial_state: F::InitialState, safety_margin: usize) -> Self {
        Self {
            initial_state,
            safety_margin,
            state_tree: RwLock::new(HashMap::new()),
            earliest_block: RwLock::new(U64::max_value()),
            fetch_mutex: Mutex::new(()),
        }
    }

    pub async fn get_block_state(
        &self,
        block_hash: &H256,
    ) -> Option<BlockState<F>> {
        self.state_tree
            .read()
            .await
            .get(block_hash)
            .map(|x| x.clone())
    }

    pub async fn fetch_block_state<M: Middleware + 'static>(
        &self,
        env: &StateFoldEnvironment<M, F::UserData>,
        block: &Block,
    ) -> Result<BlockState<F>, FoldableError<M, F>> {
        // We assume this function will be called close to the latest block
        // and on the "main" chain, instead of on old blocks on "uncle chains".
        // As such we make multiple concurrent calls to fetch mutually
        // exclusive. This way, under our assumption, we save some bandwidth by
        // only fetching once, reusing the fetch for the other concurrent calls.
        // Note that `get_block` will still work normally, and won't be affected
        // by the mutual exclusion.
        let _guard = self.fetch_mutex.lock().await;

        if let Some(state) = self.get_block_state(&block.hash).await {
            return Ok(state);
        }

        let block_state = self.fold_to_leaf(env, &block).await?;
        Ok(block_state)
    }
}

/// Internals
impl<F> Train<F>
where
    F: Foldable + Send + Sync + 'static,
{
    async fn fold_to_leaf<M: Middleware + 'static>(
        &self,
        env: &StateFoldEnvironment<M, F::UserData>,
        leaf_block: &Block,
    ) -> Result<BlockState<F>, FoldableError<M, F>> {
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
                let sync_block =
                    self.sync_to_margin(env, leaf_block).await?.block;

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
            ancestor_block = env.get_block(ancestor_block.parent_hash).await?;
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

                let new_state = F::fold(
                    &previous_state.state,
                    &block,
                    env,
                    env.fold_access(&block),
                )
                .await
                .context(InnerError)?;

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

    async fn sync_to_margin<M: Middleware + 'static>(
        &self,
        env: &StateFoldEnvironment<M, F::UserData>,
        leaf_block: &Block,
    ) -> Result<BlockState<F>, FoldableError<M, F>> {
        // Calculate sync block. If `leaf_block` in within the `safety_margin`,
        // then use `leaf_block`. Otherwise, use current block minus
        // `safety_margin`.
        let sync_block = {
            let current: U64 = env.get_current_block_number().await?;

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
                env.get_block(minimum_sync_block).await?
            }
        };

        // Now create the state with user defined `sync`.
        let sync_state = {
            let state = F::sync(
                &self.initial_state,
                &sync_block,
                env,
                env.sync_access(&sync_block),
            )
            .await
            .context(InnerError)?;

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

#[cfg(test)]
mod tests {
    use super::Train;
    use crate::test_utils::mocks::{IncrementFold, MockMiddleware};
    use crate::StateFoldEnvironment;
    use std::sync::Arc;

    use ethers::core::types::{H256, U64};
    use offchain_utils::offchain_core::ethers;

    const INITIAL_VALUE: u64 = 42;
    const SAFETY_MARGIN: usize = 8;

    async fn instantiate_all() -> (
        Train<IncrementFold>,
        Arc<MockMiddleware>,
        StateFoldEnvironment<MockMiddleware, ()>,
    ) {
        let train = Train::<IncrementFold>::new(INITIAL_VALUE, SAFETY_MARGIN);
        let m = MockMiddleware::new(128).await;
        let env = StateFoldEnvironment::new(
            Arc::clone(&m),
            SAFETY_MARGIN,
            0.into(),
            vec![],
            1,
            (),
        );

        (train, m, env)
    }

    #[tokio::test]
    async fn latest_state_test() {
        let (train, m, env) = instantiate_all().await;

        let latest_block = m.get_latest_block().await.unwrap();

        assert!(train.get_block_state(&latest_block.hash).await.is_none());
        assert_eq!(*train.earliest_block.read().await, U64::max_value());

        let state = train
            .fetch_block_state(&env, &latest_block)
            .await
            .unwrap()
            .state;

        assert_eq!(
            state,
            IncrementFold {
                low_hash: latest_block.hash.to_low_u64_be(),
                n: 128 + INITIAL_VALUE,
                initial_state: INITIAL_VALUE,
            }
        );

        assert_eq!(
            *train.earliest_block.read().await,
            U64::from(128 - SAFETY_MARGIN)
        );
    }

    #[tokio::test]
    async fn straight_blockchain_test() {
        let (train, m, env) = instantiate_all().await;

        for i in 64u64..=128 {
            let block = m.get_block_with_number(i.into()).await.unwrap();
            assert!(train.get_block_state(&block.hash).await.is_none());

            let state =
                train.fetch_block_state(&env, &block).await.unwrap().state;

            assert_eq!(
                state,
                IncrementFold {
                    low_hash: block.hash.to_low_u64_be(),
                    n: i + INITIAL_VALUE,
                    initial_state: INITIAL_VALUE,
                }
            );

            assert_eq!(*train.earliest_block.read().await, U64::from(64));
        }

        for i in 32u64..=50 {
            let block = m.get_block_with_number(i.into()).await.unwrap();
            assert!(train.get_block_state(&block.hash).await.is_none());

            let state =
                train.fetch_block_state(&env, &block).await.unwrap().state;

            assert_eq!(
                state,
                IncrementFold {
                    low_hash: block.hash.to_low_u64_be(),
                    n: i + INITIAL_VALUE,
                    initial_state: INITIAL_VALUE,
                }
            );

            assert_eq!(*train.earliest_block.read().await, U64::from(32));
        }

        for i in 58u64..=63 {
            let block = m.get_block_with_number(i.into()).await.unwrap();
            assert!(train.get_block_state(&block.hash).await.is_none());

            let state =
                train.fetch_block_state(&env, &block).await.unwrap().state;

            assert_eq!(
                state,
                IncrementFold {
                    low_hash: block.hash.to_low_u64_be(),
                    n: i + INITIAL_VALUE,
                    initial_state: INITIAL_VALUE,
                }
            );

            assert_eq!(*train.earliest_block.read().await, U64::from(32));
        }

        for i in 16u64..=128 {
            let block = m.get_block_with_number(i.into()).await.unwrap();

            let state =
                train.fetch_block_state(&env, &block).await.unwrap().state;

            assert_eq!(
                state,
                IncrementFold {
                    low_hash: block.hash.to_low_u64_be(),
                    n: i + INITIAL_VALUE,
                    initial_state: INITIAL_VALUE,
                }
            );

            assert_eq!(*train.earliest_block.read().await, U64::from(16));
        }
    }

    #[tokio::test]
    async fn branching_blockchain_test() {
        async fn add_branch(
            m: &Arc<MockMiddleware>,
            base: H256,
            count: usize,
        ) -> H256 {
            let mut last_hash = base;
            for _ in 0..=count {
                last_hash = m.add_block(last_hash).await.unwrap();
            }

            last_hash
        }

        let (train, m, env) = instantiate_all().await;

        let base_b = m.get_block_with_number(32.into()).await.unwrap();
        let base_c = m.get_block_with_number(36.into()).await.unwrap();
        let base_d = m.get_block_with_number(65.into()).await.unwrap();

        let tip_a = m.get_block_with_number(128.into()).await.unwrap().hash;
        for i in 64u64..=128 {
            let block =
                m.get_block_with_number_from(i.into(), tip_a).await.unwrap();

            let state =
                train.fetch_block_state(&env, &block).await.unwrap().state;

            assert_eq!(
                state,
                IncrementFold {
                    low_hash: block.hash.to_low_u64_be(),
                    n: i + INITIAL_VALUE,
                    initial_state: INITIAL_VALUE,
                }
            );
        }

        let tip_b = add_branch(&m, base_b.hash, 128).await;
        for i in 80u64..=128 {
            let block =
                m.get_block_with_number_from(i.into(), tip_b).await.unwrap();

            let state =
                train.fetch_block_state(&env, &block).await.unwrap().state;

            assert_eq!(
                state,
                IncrementFold {
                    low_hash: block.hash.to_low_u64_be(),
                    n: i + INITIAL_VALUE,
                    initial_state: INITIAL_VALUE,
                }
            );
        }

        let tip_c = add_branch(&m, base_c.hash, 128).await;
        for i in 68u64..=128 {
            let block =
                m.get_block_with_number_from(i.into(), tip_c).await.unwrap();

            let state =
                train.fetch_block_state(&env, &block).await.unwrap().state;

            assert_eq!(
                state,
                IncrementFold {
                    low_hash: block.hash.to_low_u64_be(),
                    n: i + INITIAL_VALUE,
                    initial_state: INITIAL_VALUE,
                }
            );
        }

        let tip_d = add_branch(&m, base_d.hash, 128).await;
        for i in 90u64..=128 {
            let block =
                m.get_block_with_number_from(i.into(), tip_d).await.unwrap();

            let state =
                train.fetch_block_state(&env, &block).await.unwrap().state;

            assert_eq!(
                state,
                IncrementFold {
                    low_hash: block.hash.to_low_u64_be(),
                    n: i + INITIAL_VALUE,
                    initial_state: INITIAL_VALUE,
                }
            );
        }
    }
}
