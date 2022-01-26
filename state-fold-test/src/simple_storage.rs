use ethers::{
    contract::{abigen, Contract},
    providers::Middleware,
};
use offchain_utils::offchain_core::ethers;

use hex;
use std::sync::Arc;

#[cfg(not(doctest))]
abigen!(
    SimpleStorage,
    "./state-fold-test/src/contracts/bin/SimpleStorage.abi",
);

pub async fn deploy_simple_storage<M: Middleware>(
    client: Arc<M>,
) -> Contract<M> {
    let bytecode =
        hex::decode(include_bytes!("./contracts/bin/SimpleStorage.bin"))
            .unwrap()
            .into();
    let abi = simplestorage_mod::SIMPLESTORAGE_ABI.clone();

    crate::utils::deploy_contract(client, bytecode, abi).await
}
