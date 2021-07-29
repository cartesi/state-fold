mod common;
use common::*;
pub use simple_storage::*;

use ethers::{
    contract::Contract,
    providers::Middleware,
    types::{Address, H256, U256, U64},
};
use std::sync::Arc;

use state_fold::{utils, Access, FoldAccess, SyncAccess};

#[tokio::test]
async fn fold_query_test() {
    let (_handle, provider) = new_geth().await;
    let genesis = provider.get_block_number().await.unwrap();
    let contract = deploy_simple_storage(Arc::clone(&provider)).await;
    let account = provider.get_accounts().await.unwrap()[0];

    let block_hash0 = get_current_block_hash(provider.as_ref()).await;
    let block_hash1 = set_value_get_hash(&provider, &contract, "this").await;
    let block_hash2 = set_value_get_hash(&provider, &contract, "that").await;
    let block_hash3 = set_value_get_hash(&provider, &contract, "other").await;

    let deployed_address = contract.address();
    let access = Access::new(Arc::clone(&provider), genesis, vec![], 4);

    // Test at block_hash0
    {
        let simple_storage = access
            .build_fold_contract(
                deployed_address,
                block_hash0,
                SimpleStorage::new,
            )
            .await;
        let value = simple_storage.get_value().call().await.unwrap();
        assert_eq!(value, "initial value");

        println!("{:?}", simple_storage.value_changed_filter().query().await);

        let event =
            simple_storage.value_changed_filter().query().await.unwrap();
        assert_eq!(event.len(), 1);
        assert_eq!(event[0].old_author, Address::zero());
        assert_eq!(event[0].author, account);
        assert_eq!(event[0].n, 0.into());
        assert_eq!(event[0].old_value, "");
        assert_eq!(event[0].new_value, "initial value");

        let bloom = provider
            .get_block(block_hash0)
            .await
            .unwrap()
            .unwrap()
            .logs_bloom
            .unwrap();

        assert!(utils::contains_address(&bloom, &deployed_address));
        assert!(utils::contains_topic(&bloom, &account));
        assert!(utils::contains_topic(&bloom, &Address::zero()));
        assert!(utils::contains_topic(&bloom, &U256::from(0)));
    }

    // Test at block_hash1
    {
        let simple_storage = access
            .build_fold_contract(
                deployed_address,
                block_hash1,
                SimpleStorage::new,
            )
            .await;

        let value = simple_storage.get_value().call().await.unwrap();
        assert_eq!(value, "this");

        let event =
            simple_storage.value_changed_filter().query().await.unwrap();
        assert_eq!(event.len(), 1);
        assert_eq!(event[0].old_author, account);
        assert_eq!(event[0].author, account);
        assert_eq!(event[0].old_value, "initial value");
        assert_eq!(event[0].new_value, "this");

        let event = simple_storage
            .value_changed_filter()
            .topic1(account)
            .query()
            .await
            .unwrap();
        assert_eq!(event.len(), 1);

        let event = simple_storage
            .value_changed_filter()
            .topic1(Address::zero())
            .query()
            .await
            .unwrap();
        assert_eq!(event.len(), 0);

        let bloom = provider
            .get_block(block_hash1)
            .await
            .unwrap()
            .unwrap()
            .logs_bloom
            .unwrap();

        assert!(utils::contains_address(&bloom, &deployed_address));
        assert!(utils::contains_topic(&bloom, &account));
        assert!(utils::contains_topic(&bloom, &U256::from(1)));
    }

    // Test at block_hash2
    {
        let simple_storage = access
            .build_fold_contract(
                deployed_address,
                block_hash2,
                SimpleStorage::new,
            )
            .await;
        let value = simple_storage.get_value().call().await.unwrap();
        assert_eq!(value, "that");

        let event =
            simple_storage.value_changed_filter().query().await.unwrap();
        assert_eq!(event.len(), 1);
        assert_eq!(event[0].old_author, account);
        assert_eq!(event[0].author, account);
        assert_eq!(event[0].old_value, "this");
        assert_eq!(event[0].new_value, "that");

        let bloom = provider
            .get_block(block_hash2)
            .await
            .unwrap()
            .unwrap()
            .logs_bloom
            .unwrap();

        assert!(utils::contains_address(&bloom, &deployed_address));
        assert!(utils::contains_topic(&bloom, &account));
        assert!(utils::contains_topic(&bloom, &U256::from(2)));
    }

    // Test at block_hash3
    {
        let simple_storage = access
            .build_fold_contract(
                deployed_address,
                block_hash3,
                SimpleStorage::new,
            )
            .await;
        let value = simple_storage.get_value().call().await.unwrap();
        assert_eq!(value, "other");

        let event =
            simple_storage.value_changed_filter().query().await.unwrap();
        assert_eq!(event.len(), 1);
        assert_eq!(event[0].old_author, account);
        assert_eq!(event[0].author, account);
        assert_eq!(event[0].old_value, "that");
        assert_eq!(event[0].new_value, "other");

        // test override block
        let value = simple_storage
            .get_value()
            .block(block_hash0)
            .call()
            .await
            .unwrap();
        assert_eq!(value, "initial value");

        // Default overrides given block.
        let event = simple_storage
            .value_changed_filter()
            .at_block_hash(block_hash0)
            .query()
            .await
            .unwrap();
        assert_eq!(event.len(), 1);
        assert_eq!(event[0].old_author, account);
        assert_eq!(event[0].author, account);
        assert_eq!(event[0].old_value, "that");
        assert_eq!(event[0].new_value, "other");

        let bloom = provider
            .get_block(block_hash3)
            .await
            .unwrap()
            .unwrap()
            .logs_bloom
            .unwrap();

        assert!(utils::contains_address(&bloom, &deployed_address));
        assert!(utils::contains_topic(&bloom, &account));
        assert!(utils::contains_topic(&bloom, &U256::from(3)));
    }
}

#[tokio::test]
async fn sync_query_test() {
    let (_handle, provider) = new_geth().await;
    let genesis = provider.get_block_number().await.unwrap();
    let contract = deploy_simple_storage(Arc::clone(&provider)).await;
    let account = provider.get_accounts().await.unwrap()[0];

    let block_number0 = provider.get_block_number().await.unwrap();
    let block_number1 = set_value_get_block(&provider, &contract, "this").await;
    let block_number2 = set_value_get_block(&provider, &contract, "that").await;
    let block_number3 =
        set_value_get_block(&provider, &contract, "other").await;

    let deployed_address = contract.address();
    let access = Access::new(Arc::clone(&provider), genesis, vec![], 4);

    // Test at block_hash0
    {
        let simple_storage = access
            .build_sync_contract(
                deployed_address,
                block_number0,
                SimpleStorage::new,
            )
            .await;
        let value = simple_storage.get_value().call().await.unwrap();
        assert_eq!(value, "initial value");

        let event =
            simple_storage.value_changed_filter().query().await.unwrap();
        assert_eq!(event.len(), 1);
        assert_eq!(event[0].old_author, Address::zero());
        assert_eq!(event[0].author, account);
        assert_eq!(event[0].old_value, "");
        assert_eq!(event[0].new_value, "initial value");
    }

    // Test at block_hash1
    {
        let simple_storage = access
            .build_sync_contract(
                deployed_address,
                block_number1,
                SimpleStorage::new,
            )
            .await;
        let value = simple_storage.get_value().call().await.unwrap();
        assert_eq!(value, "this");

        let event =
            simple_storage.value_changed_filter().query().await.unwrap();
        assert_eq!(event.len(), 2);
        assert_eq!(event[1].old_author, account);
        assert_eq!(event[1].author, account);
        assert_eq!(event[1].old_value, "initial value");
        assert_eq!(event[1].new_value, "this");

        let event = simple_storage
            .value_changed_filter()
            .topic2(Address::zero())
            .query()
            .await
            .unwrap();
        assert_eq!(event.len(), 1);

        let event = simple_storage
            .value_changed_filter()
            .topic1(account)
            .query()
            .await
            .unwrap();
        assert_eq!(event.len(), 2);
    }

    // Test at block_hash2
    {
        let simple_storage = access
            .build_sync_contract(
                deployed_address,
                block_number2,
                SimpleStorage::new,
            )
            .await;
        let value = simple_storage.get_value().call().await.unwrap();
        assert_eq!(value, "that");

        let event =
            simple_storage.value_changed_filter().query().await.unwrap();
        assert_eq!(event.len(), 3);
        assert_eq!(event[2].old_author, account);
        assert_eq!(event[2].author, account);
        assert_eq!(event[2].old_value, "this");
        assert_eq!(event[2].new_value, "that");
    }

    // Test at block_hash3
    {
        let simple_storage = access
            .build_sync_contract(
                deployed_address,
                block_number3,
                SimpleStorage::new,
            )
            .await;
        let value = simple_storage.get_value().call().await.unwrap();
        assert_eq!(value, "other");

        let event =
            simple_storage.value_changed_filter().query().await.unwrap();
        assert_eq!(event.len(), 4);
        assert_eq!(event[3].old_author, account);
        assert_eq!(event[3].author, account);
        assert_eq!(event[3].old_value, "that");
        assert_eq!(event[3].new_value, "other");

        let value = simple_storage
            .get_value()
            .block(block_number0)
            .call()
            .await
            .unwrap();
        assert_eq!(value, "initial value");

        // Defualt overrides given block.
        let event = simple_storage
            .value_changed_filter()
            .to_block(block_number0)
            .query()
            .await
            .unwrap();
        assert_eq!(event.len(), 4);
    }
}

async fn get_current_block_hash<M: Middleware>(provider: &M) -> H256 {
    provider
        .get_block(provider.get_block_number().await.unwrap())
        .await
        .unwrap()
        .unwrap()
        .hash
        .unwrap()
}

async fn set_value_get_block<M: Middleware + Clone>(
    provider: &Arc<M>,
    contract: &Contract<M>,
    value: &str,
) -> U64 {
    contract
        .connect(provider.clone())
        .method::<_, H256>("setValue", value.to_owned())
        .unwrap()
        .send()
        .await
        .unwrap()
        .await
        .unwrap()
        .block_number
        .unwrap()
}

async fn set_value_get_hash<M: Middleware + Clone>(
    provider: &Arc<M>,
    contract: &Contract<M>,
    value: &str,
) -> H256 {
    contract
        .connect(provider.clone())
        .method::<_, H256>("setValue", value.to_owned())
        .unwrap()
        .send()
        .await
        .unwrap()
        .await
        .unwrap()
        .block_hash
        .unwrap()
}
