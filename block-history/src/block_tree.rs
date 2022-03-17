use state_fold_types::ethereum_types::{H256, U64};
use state_fold_types::Block;

use std::collections::HashMap;

pub(crate) struct BlockTree {
    tree: HashMap<H256, Block>,
    number_map: HashMap<U64, H256>,
    latest_hash: H256,
}

impl BlockTree {
    pub fn new(start_block: Block) -> Self {
        Self {
            latest_hash: start_block.hash,
            number_map: HashMap::from([(start_block.number, start_block.hash)]),
            tree: HashMap::from([(start_block.hash, start_block)]),
        }
    }

    pub fn block_with_hash(&self, hash: &H256) -> Option<Block> {
        self.tree.get(hash).cloned()
    }

    pub fn block_with_number(&self, number: &U64) -> Option<Block> {
        let hash = self.number_map.get(number)?;
        self.block_with_hash(hash)
    }

    pub fn insert_block(&mut self, block: Block) {
        self.number_map.insert(block.number, block.hash);
        self.tree.insert(block.hash, block);
    }

    pub fn latest_block(&self) -> Block {
        self.block_with_hash(&self.latest_hash).unwrap()
    }

    pub fn update_latest_block(&mut self, block: Block) {
        self.latest_hash = block.hash;
        self.insert_block(block);
    }
}
