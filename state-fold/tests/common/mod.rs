#![allow(dead_code)]
pub mod mock_provider;
pub mod simple_storage;
pub mod test_contract;

use ethers::contract::{Contract, ContractFactory};
use ethers::core::utils::{Geth, GethInstance, Solc};
use ethers::providers::{Http, Middleware, Provider};
use std::convert::TryFrom;
use std::sync::Arc;

pub async fn new_geth() -> (GethInstance, Arc<Provider<Http>>) {
    let geth = Geth::new().spawn();
    let provider = Provider::<Http>::try_from(geth.endpoint()).unwrap();
    let deployer = provider.get_accounts().await.unwrap()[0];
    (geth, Arc::new(provider.with_sender(deployer)))
}

pub async fn deploy_simple_storage<M: Middleware>(
    client: Arc<M>,
) -> Contract<M> {
    let contract_name = "SimpleStorage";
    let path = "./tests/common/contract/SimpleStorage.sol";
    let contracts = Solc::new(&path).build().unwrap();
    let contract = contracts.get(contract_name).unwrap();
    let abi = contract.abi.clone();
    let bytecode = contract.bytecode.clone();

    let factory = ContractFactory::new(abi, bytecode, client);
    factory
        .deploy("initial value".to_string())
        .unwrap()
        .send()
        .await
        .unwrap()
}
