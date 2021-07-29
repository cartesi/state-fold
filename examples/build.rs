use std::path::Path;

use offchain_utils::offchain_core::ethers::{contract::Abigen, utils::Solc};

fn main() {
    let bindings_dest_path1 =
        Path::new("./src/fold/contracts/").join("test_contract.rs");

    // TestContract
    let contract_name = "TestContract";
    let path = "./common/contract/TestContract.sol";
    let contracts = Solc::new(&path).build_raw().unwrap();
    let contract = contracts.get(contract_name).unwrap();
    let abi = contract.abi.clone();

    let bindings = Abigen::new(&contract_name, abi)
        .unwrap()
        .generate()
        .unwrap();

    bindings.write_to_file(&bindings_dest_path1).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=./common/contract/TestContract.sol");
    println!("cargo:rerun-if-changed=./src/fold/contracts/test_contract.rs");
}
