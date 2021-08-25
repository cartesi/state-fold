use state_fold::types::StateFoldDelegate;

use ethers::types::Address;

pub trait ExampleContractFoldDelegate: StateFoldDelegate + Default {
    fn new(address: Address) -> Self;
}

pub enum Examples {
    Array(&'static str, &'static str),
}
