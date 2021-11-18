#![allow(dead_code)]
pub mod mock_provider;

use ethers::contract::{abigen, Contract, ContractFactory};
use ethers::core::utils::{Geth, GethInstance};
use ethers::providers::{Http, Middleware, Provider};
use std::convert::TryFrom;
use std::sync::Arc;

abigen!(
    SimpleStorage,
    "./state-fold/tests/common/contract/SimpleStorage.abi",
);

pub async fn new_geth() -> (GethInstance, Arc<Provider<Http>>) {
    let geth = Geth::new().spawn();
    let provider = Provider::<Http>::try_from(geth.endpoint()).unwrap();
    let deployer = provider.get_accounts().await.unwrap()[0];
    (geth, Arc::new(provider.with_sender(deployer)))
}

pub async fn deploy_simple_storage<M: Middleware>(
    client: Arc<M>,
) -> Contract<M> {
    let bytecode = hex::decode(include_bytes!("./contract/SimpleStorage.bin"))
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
