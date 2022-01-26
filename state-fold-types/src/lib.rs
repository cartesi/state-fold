use ethers::core::types::{H256, U64};
use offchain_core::ethers;
use offchain_core::types::Block;
use offchain_utils::offchain_core;

#[derive(Clone, Debug)]
pub struct BlockState<State> {
    pub block: Block,
    pub state: State,
}

#[derive(Clone, Debug)]
pub enum BlocksSince {
    Normal(Vec<Block>),
    Reorg(Vec<Block>),
}

#[derive(Clone, Debug)]
pub enum BlockStreamItem {
    NewBlock(Block),
    Reorg(Vec<Block>),
}

#[derive(Clone, Debug)]
pub enum StatesSince<T> {
    Normal(Vec<BlockState<T>>),
    Reorg(Vec<BlockState<T>>),
}

#[derive(Clone, Debug)]
pub enum StateStreamItem<T> {
    NewState(BlockState<T>),
    Reorg(Vec<BlockState<T>>),
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
