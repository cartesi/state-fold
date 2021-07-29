use ethers::types::U256;
use im::Vector;
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("Index out of bounds"))]
    IndexOutOfBound {},
    #[snafu(display("Pop on empty array"))]
    PopEmptyArray {},
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Default)]
pub struct ContractCtx {
    data: Vector<U256>,
}

impl ContractCtx {
    pub fn push(&self, value: U256) -> Self {
        let mut data = self.data.clone();
        data.push_back(value);

        println!("Data after push: {:?}", data);

        Self { data }
    }

    pub fn pop(&self) -> Result<Self> {
        if self.data.is_empty() {
            PopEmptyArray {}.fail()
        } else {
            let mut data = self.data.clone();
            data.pop_back();

            println!("Data after pop: {:?}", data);

            Ok(Self { data })
        }
    }

    pub fn modify(&self, index: U256, value: U256) -> Result<Self> {
        let index = index.as_u32() as usize;

        if self.data.get(index).is_none() {
            IndexOutOfBound {}.fail()
        } else {
            let data = self.data.update(index, value);

            println!("Data after modify: {:?}", data);

            Ok(Self { data })
        }
    }
}
