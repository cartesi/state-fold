use offchain_utils::offchain_core::ethers;
use offchain_utils::offchain_core::types::Block;

use ethers::core::types::{H256, U64};

#[derive(Clone, Debug)]
pub struct BlockState<State: Clone> {
    pub block: Block,
    pub state: State,
}

#[derive(Clone, Debug)]
pub enum QueryBlock {
    Latest,
    BlockHash(H256),
    BlockNumber(U64),
    BlockDepth(usize),
    Block(Block),
}

impl From<H256> for QueryBlock {
    fn from(h: H256) -> Self {
        QueryBlock::BlockHash(h)
    }
}

impl From<&H256> for QueryBlock {
    fn from(h: &H256) -> Self {
        QueryBlock::BlockHash(*h)
    }
}

impl From<U64> for QueryBlock {
    fn from(n: U64) -> Self {
        QueryBlock::BlockNumber(n)
    }
}

impl From<&U64> for QueryBlock {
    fn from(n: &U64) -> Self {
        QueryBlock::BlockNumber(*n)
    }
}

impl From<Block> for QueryBlock {
    fn from(b: Block) -> Self {
        QueryBlock::Block(b)
    }
}

impl From<&Block> for QueryBlock {
    fn from(b: &Block) -> Self {
        QueryBlock::from(b.clone())
    }
}
