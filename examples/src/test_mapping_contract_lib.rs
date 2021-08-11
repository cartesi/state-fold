use ethers::types::U256;
use snafu::Snafu;
use std::collections::HashMap;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("Delete on an empty entry"))]
    DeleteEmptyArray,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Default)]
pub struct ContractCtx {
    mapping: HashMap<U256, U256>,
}

impl ContractCtx {
    pub fn modify(&self, key: U256, value: U256) -> Self {
        let mut mapping = self.mapping.clone();
        mapping.insert(key, value);

        println!("Data after modify: {:?}", mapping);

        Self { mapping }
    }
    pub fn remove(&self, key: U256) -> Result<Self> {
        let mut mapping = self.mapping.clone();
        if !mapping.contains_key(&key) {
            return Err(Error::DeleteEmptyArray);
        }
        mapping.remove(&key);

        println!("Data after delete {:?}", mapping);

        Ok(Self { mapping })
    }
}
