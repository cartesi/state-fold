use std::path::Path;

use offchain_utils::offchain_core::ethers::{contract::Abigen, utils::Solc};

fn main() {
    let bindings_dest_path1 =
        Path::new("./tests/common/").join("test_contract.rs");
    let bindings_dest_path2 =
        Path::new("./tests/common/").join("simple_storage.rs");

    // TestContract
    let contract_name = "TestContract";
    let path = "./tests/common/contract/TestContract.sol";
    let contracts = Solc::new(&path).build_raw().unwrap();
    let contract = contracts.get(contract_name).unwrap();
    let abi = contract.abi.clone();

    let bindings = Abigen::new(&contract_name, abi)
        .unwrap()
        .generate()
        .unwrap();

    bindings.write_to_file(&bindings_dest_path1).unwrap();
    //

    // SimpleStorage
    let contract_name = "SimpleStorage";
    let path = "./tests/common/contract/SimpleStorage.sol";
    let contracts = Solc::new(&path).build_raw().unwrap();
    let contract = contracts.get(contract_name).unwrap();
    let abi = contract.abi.clone();

    let bindings = Abigen::new(&contract_name, abi)
        .unwrap()
        .generate()
        .unwrap();

    bindings.write_to_file(&bindings_dest_path2).unwrap();
    //

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=./tests/common/contract/TestContract.sol");
    println!("cargo:rerun-if-changed=./tests/common/test_contract.rs");
    println!(
        "cargo:rerun-if-changed=./tests/common/contract/SimpleStorage.sol"
    );
    println!("cargo:rerun-if-changed=./tests/common/simple_storage.rs");
}
