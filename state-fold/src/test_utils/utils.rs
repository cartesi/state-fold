use crate::{Foldable, StateFoldEnvironment};

use ethers::{
    contract::{abigen, Contract, ContractFactory},
    core::utils::{Geth, GethInstance},
    providers::{Http, Middleware, Provider},
    types::H256,
};
use offchain_utils::offchain_core::ethers;
use offchain_utils::offchain_core::types::Block;

use hex;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::sync::Arc;

abigen!(
    SimpleStorage,
    "./state-fold/src/test_utils/contracts/SimpleStorage.abi",
);

pub(crate) async fn new_geth() -> (GethInstance, Arc<Provider<Http>>) {
    let geth = Geth::new().spawn();
    let provider = Provider::<Http>::try_from(geth.endpoint()).unwrap();
    let deployer = provider.get_accounts().await.unwrap()[0];
    (geth, Arc::new(provider.with_sender(deployer)))
}

pub(crate) async fn deploy_simple_storage<M: Middleware>(
    client: Arc<M>,
) -> Contract<M> {
    let bytecode = hex::decode(include_bytes!("./contracts/SimpleStorage.bin"))
        .unwrap()
        .into();
    let abi = simplestorage_mod::SIMPLESTORAGE_ABI.clone();

    let factory = ContractFactory::new(abi, bytecode, client);

    // This is what we wanted to write, but there's a bug in ethers preventing
    // it.
    /*
    factory
        .deploy("initial value".to_string())
        .unwrap()
        .send()
        .await
        .unwrap()
    */

    let mut deployer = factory.deploy("initial value".to_string()).unwrap();
    deployer.tx.set_gas(8000000);
    deployer.send().await.unwrap()
}

pub(crate) async fn get_current_block<M: Middleware>(provider: &M) -> Block {
    provider
        .get_block(provider.get_block_number().await.unwrap())
        .await
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap()
}

pub(crate) async fn set_value_get_block<
    F: Foldable,
    M: Middleware + Clone + 'static,
>(
    env: &StateFoldEnvironment<M>,
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
