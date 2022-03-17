use crate::{Foldable, StateFoldEnvironment};

use ethers::{contract::Contract, providers::Middleware, types::H256};
use state_fold_types::ethers;
use state_fold_types::Block;

use std::convert::TryInto;

pub(crate) async fn set_value_get_block<F: Foldable, M: Middleware + Clone + 'static>(
    env: &StateFoldEnvironment<M, ()>,
    contract: &Contract<M>,
    value: &str,
) -> Block {
    let hash = contract
        .connect(env.inner_middleware())
        .method::<_, H256>("setValue", value.to_owned())
        .unwrap()
        .send()
        .await
        .unwrap()
        .await
        .unwrap()
        .unwrap()
        .block_hash
        .unwrap();

    env.inner_middleware()
        .get_block(hash)
        .await
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap()
}
