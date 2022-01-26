use offchain_utils::offchain_core::types::Block;

use crate::foldable::Foldable;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockState<State: Foldable> {
    pub block: Block,
    pub state: State,
}
