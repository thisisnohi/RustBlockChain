use crate::models::block_model::Block;
use log::info;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BlockChain {
    pub blocks: Vec<Block>,
}
