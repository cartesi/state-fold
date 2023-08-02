use crate::{Foldable, StateFoldEnvironment};

use eth_state_fold_types::ethers;
use eth_state_fold_types::Block;
use ethers::{contract::Contract, providers::Middleware, types::H256};

pub(crate) async fn set_value_get_block<F: Foldable, M: Middleware + Clone + 'static>(
    env: &StateFoldEnvironment<M, ()>,
    contract: &Contract<M>,
    value: &str,
) -> Block {
    let hash = contract
        .connect(env.inner_middleware())
        .method::<_, H256>("setValue", value.to_owned())
        .unwrap()
        .gas(8000000)
        .send()
        .await
        .unwrap()
        .await
        .unwrap()
        .unwrap()
        .block_hash
        .unwrap();

    env.block_with_hash(&hash).await.unwrap().as_ref().clone()
}
