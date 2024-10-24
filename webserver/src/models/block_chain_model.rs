use crate::models::block_model::Block;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    // 初始化区块链
    pub fn init() -> BlockChain {
        let mut blocks: Vec<Block> = Vec::new();
        blocks.push(Self::get_genesis_block());
        BlockChain { blocks }
    }
    // 创时区块
    fn get_genesis_block() -> Block {
        Block::init(0, "Genesis".to_string(), "0".repeat(64))
    }
    // 获取上一节点hash
    fn get_last_hash(&self) -> String {
        self.blocks.last().unwrap().hash.clone()
    }
    // 获取上一节点hash
    fn get_last_index(&self) -> u32 {
        self.blocks.last().unwrap().index
    }

    // 添加一个区块
    pub fn add_block(&mut self, data: String) {
        self.blocks.push(Block::init(
            self.get_last_index() + 1,
            data,
            self.get_last_hash(),
        ));
    }

    // 判断是否合格的区块
    pub fn is_valid_new_block(new_block: &Block, prev_block: &Block) -> bool {
        let hash = new_block.hash.clone();

        if new_block.index != prev_block.index + 1 {
            println!(
                "新区块索引[{}]与上一区块索引[{}]不匹配",
                new_block.index, prev_block.index
            );
            return false;
        } else if prev_block.hash != new_block.prev_block_hash {
            println!(
                "新区块指向上一区块索引[{}]与上一区块索引[{}]不匹配",
                new_block.prev_block_hash, prev_block.hash
            );
            return false;
        } else if hash != Block::calculate_valid_hash(&new_block) {
            println!("新区块Hash[{}]与计算hash[{}]不匹配", hash, new_block.hash);
            return false;
        }

        true
    }

    // 判断是否合格链
    pub fn is_valid_chain(blocks: &Vec<Block>) -> bool {
        // 判断创世块是否正确
        let block = BlockChain::get_genesis_block();
        if block.to_string() != blocks[0].to_string() {
            return false;
        }
        // 如果只有一个块(创世块)，则返回true
        if blocks.len() == 1 {
            return true;
        }
        // 循环判断每一区块是否正确
        // 从一开始
        let mut index = 1;
        while index < blocks.len() {
            if !BlockChain::is_valid_new_block(&blocks[index], &blocks[index - 1]) {
                println!("第[{}]区块，不符合规则", index);
                return false;
            }

            index += 1;
        }

        true
    }

    // 替换为最长链
    pub fn replace_chain(&mut self, blocks: Vec<Block>) {
        if BlockChain::is_valid_chain(&blocks) && blocks.len() > self.blocks.len() {
            self.blocks = blocks;

            // 广播
            broadcast_latest();
        } else {
            println!("Received blockchain invalid...")
        }
    }
}

fn broadcast_latest() {
    println!("===> broadcast_latest");

    todo!()
}
