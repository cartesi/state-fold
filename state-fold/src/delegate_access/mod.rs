// (c) Cartesi and individual authors (see AUTHORS)
// SPDX-License-Identifier: Apache-2.0 (see LICENSE)

pub mod error;
pub mod fold_middleware;
pub mod sync_middleware;

pub use error::AccessError;
pub use fold_middleware::FoldMiddleware;
pub use sync_middleware::SyncMiddleware;

mod partition_events;
mod utils;

#[cfg(test)]
mod tests {
    use crate::test_utils;
    use crate::StateFoldEnvironment;
    use std::sync::Arc;

    use eth_state_fold_types::ethers;
    use ethers::providers::Middleware;

    use super::{fold_middleware, sync_middleware};

    #[tokio::test]
    async fn test_sync_fold() {
        let (_handle, provider) = eth_state_fold_test::utils::new_geth().await;
        let genesis = provider.get_block_number().await.unwrap();
        let contract =
            eth_state_fold_test::simple_storage::deploy_simple_storage(Arc::clone(&provider)).await;
        let account = provider.get_accounts().await.unwrap()[0];
        let deployed_address = contract.address();

        let env = StateFoldEnvironment::new(
            Arc::clone(&provider),
            None,
            4,
            genesis,
            vec![],
            1,
            usize::MAX,
            (),
        );

        let block0 = eth_state_fold_test::utils::get_current_block(provider.as_ref()).await;
        let block1 = test_utils::set_value_get_block::<_>(&env, &contract, "this").await;
        let block2 = test_utils::set_value_get_block::<_>(&env, &contract, "that").await;
        let block3 = test_utils::set_value_get_block::<_>(&env, &contract, "other").await;

        sync_middleware::tests::sync_query_test(
            account,
            deployed_address,
            &env,
            (&block0, &block1, &block2, &block3),
        )
        .await;

        fold_middleware::tests::fold_query_test(
            account,
            deployed_address,
            &env,
            (&block0, &block1, &block2, &block3),
        )
        .await;
    }
}
