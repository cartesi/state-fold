mod common;

use common::mock_provider::*;

use offchain_core::types::Block;
use offchain_utils::offchain_core;

use state_fold::error::*;
use state_fold::{
    types::BlockState, FoldAccess, StateFold, StateFoldDelegate, SyncAccess,
};

use async_trait::async_trait;
use ethers::providers::Middleware;
use ethers::types::{Address, TransactionRequest, H256, U64};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    branch: String,
    number: U64,
}

/// Mock delegate
struct Delegate {}

#[async_trait]
impl StateFoldDelegate for Delegate {
    type InitialState = ();
    type Accumulator = State;
    type State = BlockState<Self::Accumulator>;

    async fn sync<A: SyncAccess + Send + Sync>(
        &self,
        _: &Self::InitialState,
        block: &Block,
        access: &A,
    ) -> SyncResult<Self::Accumulator, A> {
        // Silly way of getting the value from middleware.
        let m = access
            .build_sync_contract(Address::zero(), block.number, |_, m| m)
            .await;
        let typed_tx = TransactionRequest::default().into();
        let v = m.call(&typed_tx, None).await.unwrap().to_vec();
        let (_h, number, branch): (H256, U64, String) =
            serde_json::de::from_slice(&v).unwrap();

        Ok(State { number, branch })
    }

    async fn fold<A: FoldAccess + Send + Sync>(
        &self,
        previous_state: &Self::Accumulator,
        block: &Block,
        access: &A,
    ) -> FoldResult<Self::Accumulator, A> {
        let m = access
            .build_fold_contract(Address::zero(), block.hash, |_, m| m)
            .await;
        let typed_tx = TransactionRequest::default().into();
        let v = m.call(&typed_tx, None).await.unwrap().to_vec();
        let (_h, _number, branch): (H256, U64, String) =
            serde_json::de::from_slice(&v).unwrap();

        Ok(State {
            number: previous_state.number + 1,
            branch,
        })
    }

    fn convert(&self, state: &BlockState<Self::Accumulator>) -> Self::State {
        state.clone()
    }
}

/// Tests
#[tokio::test]
async fn latest_state_test() {
    let (fold, access) = create_fold().await;
    let latest_hash = access.get_latest_hash().await;
    let state = fold
        .get_state_for_block(&(), Some(latest_hash))
        .await
        .unwrap()
        .state;

    assert_eq!(
        state,
        State {
            branch: "A".to_string(),
            number: 128.into()
        }
    );
}

#[tokio::test]
async fn straight_blockchain_test() {
    let (fold, access) = create_fold().await;

    for i in 64u64..=128 {
        let hash = access.get_block_with_number(i.into()).await.unwrap().hash;
        let state = fold
            .get_state_for_block(&(), Some(hash))
            .await
            .unwrap()
            .state;

        assert_eq!(
            state,
            State {
                branch: "A".to_string(),
                number: i.into()
            }
        );
    }

    for i in 32u64..=50 {
        let hash = access.get_block_with_number(i.into()).await.unwrap().hash;
        let state = fold
            .get_state_for_block(&(), Some(hash))
            .await
            .unwrap()
            .state;

        assert_eq!(
            state,
            State {
                branch: "A".to_string(),
                number: i.into()
            }
        );
    }

    for i in 58u64..=64 {
        let hash = access.get_block_with_number(i.into()).await.unwrap().hash;
        let state = fold
            .get_state_for_block(&(), Some(hash))
            .await
            .unwrap()
            .state;

        assert_eq!(
            state,
            State {
                branch: "A".to_string(),
                number: i.into()
            }
        );
    }

    for i in 16u64..=128 {
        let hash = access.get_block_with_number(i.into()).await.unwrap().hash;
        let state = fold
            .get_state_for_block(&(), Some(hash))
            .await
            .unwrap()
            .state;

        assert_eq!(
            state,
            State {
                branch: "A".to_string(),
                number: i.into()
            }
        );
    }
}

#[tokio::test]
async fn branching_blockchain_test() {
    async fn add_branch(
        access: &Arc<MockAccess>,
        branch: &str,
        base: H256,
        count: usize,
    ) -> H256 {
        let mut last_hash = base;
        for _ in 0..=count {
            last_hash = access.add_block(last_hash, branch).await.unwrap();
        }

        last_hash
    }

    let (fold, access) = create_fold().await;

    let base_b = access.get_block_with_number(32.into()).await.unwrap().hash;
    let base_c = access.get_block_with_number(36.into()).await.unwrap().hash;
    let base_d = access.get_block_with_number(65.into()).await.unwrap().hash;

    let tip_a = access.get_block_with_number(128.into()).await.unwrap().hash;
    for i in 64u64..=128 {
        let hash = access
            .get_block_with_number_from(i.into(), tip_a)
            .await
            .unwrap()
            .hash;

        let state = fold
            .get_state_for_block(&(), Some(hash))
            .await
            .unwrap()
            .state;

        assert_eq!(
            state,
            State {
                branch: "A".to_string(),
                number: i.into()
            }
        );
    }

    let tip_b = add_branch(&access, "B", base_b, 128).await;
    for i in 80u64..=128 {
        let hash = access
            .get_block_with_number_from(i.into(), tip_b)
            .await
            .unwrap()
            .hash;

        let state = fold
            .get_state_for_block(&(), Some(hash))
            .await
            .unwrap()
            .state;

        assert_eq!(
            state,
            State {
                branch: "B".to_string(),
                number: i.into()
            }
        );
    }

    let tip_c = add_branch(&access, "C", base_c, 128).await;
    for i in 68u64..=128 {
        let hash = access
            .get_block_with_number_from(i.into(), tip_c)
            .await
            .unwrap()
            .hash;

        let state = fold
            .get_state_for_block(&(), Some(hash))
            .await
            .unwrap()
            .state;

        assert_eq!(
            state,
            State {
                branch: "C".to_string(),
                number: i.into()
            }
        );

        let tip_d = add_branch(&access, "D", base_d, 128).await;
        for i in 90u64..=128 {
            let hash = access
                .get_block_with_number_from(i.into(), tip_d)
                .await
                .unwrap()
                .hash;

            let state = fold
                .get_state_for_block(&(), Some(hash))
                .await
                .unwrap()
                .state;

            assert_eq!(
                state,
                State {
                    branch: "D".to_string(),
                    number: i.into()
                }
            );
        }
    }
}

/// Helpers
async fn create_fold() -> (StateFold<Delegate, MockAccess>, Arc<MockAccess>) {
    let access = Arc::new(MockAccess::new(128, "A").await);
    let delegate = Delegate {};
    let fold = StateFold::new(delegate, Arc::clone(&access), 8);

    (fold, access)
}

// #[tokio::test]
// async fn error_test() {
//     fn create_rep_vec(rep: usize) -> Vec<Option<Error>> {
//         let mut vec = vec![];

//         for _ in 0..rep {
//             vec.push(Some(BlockchainTemporaryError { err: "" }.build()));
//         }

//         vec
//     }

//     fn create_err_from_web3(err: web3::error::Error) -> Vec<Option<Error>> {
//         let web3_err: std::result::Result<(), web3::error::Error> = Err(err);
//         vec![Some(
//             web3_err
//                 .context(Web3Error { err: "web3 error" })
//                 .unwrap_err(),
//         )]
//     }

//     let (fold, mockchain) = create_fold();

//     let hash = access.get_block_with_number(64.into()).await.unwrap().hash;
//     let state = fold.get_state_for_block(&(), hash).await.unwrap().state;
//     assert_eq!(
//         state,
//         State {
//             branch: "A".to_string(),
//             number: 64.into()
//         }
//     );

//     mockchain.lock().await.set_errors(create_rep_vec(1));
//     let hash = access.get_block_with_number(65.into()).await.unwrap().hash;
//     let state = fold.get_state_for_block(&(), hash).await.unwrap().state;
//     assert_eq!(
//         state,
//         State {
//             branch: "A".to_string(),
//             number: 65.into()
//         }
//     );

//     mockchain.lock().await.set_errors(create_rep_vec(4));
//     let hash = access.get_block_with_number(66.into()).await.unwrap().hash;
//     let state = fold.get_state_for_block(&(), hash).await.unwrap().state;
//     assert_eq!(
//         state,
//         State {
//             branch: "A".to_string(),
//             number: 66.into()
//         }
//     );

//     mockchain.lock().await.set_errors(create_rep_vec(5));
//     let hash = access.get_block_with_number(66.into()).await.unwrap().hash;
//     let state = fold.get_state_for_block(&(), hash).await.unwrap().state;
//     assert_eq!(
//         state,
//         State {
//             branch: "A".to_string(),
//             number: 66.into()
//         }
//     );

//     mockchain.lock().await.set_errors(create_rep_vec(5));
//     let hash = access.get_block_with_number(67.into()).await.unwrap().hash;
//     let res = fold.get_state_for_block(&(), hash).await;
//     assert!(res.is_err());

//     mockchain
//         .lock()
//         .await
//         .set_errors(create_err_from_web3(web3::error::Error::Unreachable));
//     let hash = access.get_block_with_number(68.into()).await.unwrap().hash;
//     let res = fold.get_state_for_block(&(), hash).await;
//     assert!(res.is_err());
// }
