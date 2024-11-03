use crate::models::block_model::Block;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockForm {
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BroadcastBlockForm {
    pub block: Block,
}
