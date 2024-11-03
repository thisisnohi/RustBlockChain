use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub(crate) index: u32,
    pub(crate) hash: String,
    pub(crate) prev_block_hash: String,
    pub difficulty: u32,
    pub nonce: u32,
    timestamp: DateTime<Utc>,
    data: String,
}

impl Block {
    // 初始化节点
    pub(crate) fn init(index: u32, data: String, prev_block_hash: String) -> Block {
        let mut block = Block {
            index,
            prev_block_hash,
            difficulty: 0,
            nonce: 0,
            timestamp: if index == 0 {
                // 创世区块时间固定
                Utc.with_ymd_and_hms(1986, 4, 3, 14, 0, 0).unwrap()
            } else {
                Utc::now()
            },
            data,
            hash: "".to_string(), // 计算hash值
        };
        // 计算hash
        block.hash = Block::calculate_valid_hash(&block);

        block
    }

    // 判断hash是否符合规则
    fn is_hash_valid(&self, hash: String) -> bool {
        hash.starts_with("000")
    }

    // 计算hash
    pub fn calculate_valid_hash(block: &Block) -> String {
        let mut hash = String::from("");
        let mut nonce = 0;

        while !block.is_hash_valid(hash.clone()) {
            let temp = format!("{}{}", block.to_string(), nonce);
            // println!("pre hash:{}", temp);
            hash = sha256::digest(temp);
            nonce += 1;
        }
        println!("nonce:{} to hash", nonce);
        hash.clone()
    }

    // 判断是否合格的区块
    fn is_valid_block(new_block: &Block) -> bool {
        new_block.index >= 0
            && !new_block.prev_block_hash.is_empty()
            && !new_block.hash.is_empty()
            && !new_block.data.is_empty()
    }
}
impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&*format!("prev_block_hash[{}]\n", self.prev_block_hash));
        f.write_str(&*format!("index[{}]\n", self.index));
        f.write_str(&*format!("timestamp[{}]\n", self.timestamp));
        f.write_str(&*format!("data[{}]\n", self.data))
    }
}
