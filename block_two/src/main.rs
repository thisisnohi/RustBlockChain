use chrono::prelude::*;
use std::fmt;
use std::fmt::{Display, Formatter};
#[derive(Debug)]
struct Block {
    timestamp: DateTime<Local>,
    data: String,
    prev_block_hash: String,
    hash: String,
}

impl Block {
    fn init(data: String, prev_block_hash: String) -> Block {
        let mut block = Block {
            timestamp: Local::now(),
            data,
            prev_block_hash,
            hash: "".to_string(), // 计算hash值
        };
        // 计算hash
        block.calculate_valid_hash();

        block
    }

    // 判断hash是否符合规则
    fn is_hash_valid(&self, hash: String) -> bool {
        hash.starts_with("000")
    }

    // 计算hash
    fn calculate_valid_hash(&mut self) -> String {
        let mut hash = String::from("");
        let mut nonce = 0;

        while !self.is_hash_valid(hash.clone()) {
            let temp = format!("{}{}", self.to_string(), nonce);
            // println!("pre hash:{}", temp);
            hash = sha256::digest(temp);
            nonce += 1;
        }
        println!("nonce:{} to hash", nonce);
        self.hash = hash.clone();
        hash.clone()
    }
}
impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&*format!("prev_block_hash[{}]\n", self.prev_block_hash));
        f.write_str(&*format!("data[{}]\n", self.data))
    }
}

#[derive(Debug)]
struct BlockChain {
    blocks: Vec<Block>,
}

impl BlockChain {
    // 初始化区块链
    fn init() -> BlockChain {
        let mut blocks: Vec<Block> = Vec::new();
        blocks.push(Self::set_genesis_block());
        BlockChain { blocks }
    }
    // 创时区块
    fn set_genesis_block() -> Block {
        Block::init("Genesis".to_string(), "0".repeat(64))
    }
    // 获取上一节点hash
    fn get_last_hash(&self) -> String {
        self.blocks.last().unwrap().hash.clone()
    }

    // 添加一个区块
    fn add_block(&mut self, data: String) {
        self.blocks.push(Block::init(data, self.get_last_hash()));
    }
}

fn main() {
    println!("hello world")
}
