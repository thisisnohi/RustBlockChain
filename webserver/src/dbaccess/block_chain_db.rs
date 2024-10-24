use crate::errors::MyError;
use crate::models::block_model::Block;

/// 获取所有区块
pub async fn all_blocks() -> Result<Vec<Block>, MyError> {
    let rows: Vec<Block> = Vec::new();
    println!("===> all_blocks");
    Ok(rows)
}

/// 获取所有区块
pub async fn init_block_chain() -> Result<Vec<Block>, MyError> {
    let rows: Vec<Block> = Vec::new();
    println!("===> init_block_chain");

    Ok(rows)
}
