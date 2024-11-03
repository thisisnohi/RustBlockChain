use crate::db_access::block_chain_db::all_blocks;
use crate::errors::MyError;
use crate::models::block_chain_model::BlockChain;
use crate::models::block_form::{BlockForm, BroadcastBlockForm};
use crate::models::block_model::Block;
use crate::state::AppState;
use crate::GLOBAL_APP_STATE;
use actix_web::{rt, web, HttpResponse};
use log::info;
use std::sync::Arc;

// 加载本地区块链
// 从本地数据获取，如果没有，则创建一个新的
pub fn load_block_chain() -> Result<BlockChain, MyError> {
    info!("load_block_chain... 暂且当没有{}", "".to_string());

    Ok(BlockChain::init())
}

/// 获取所有块
pub async fn get_all_blocks() -> Result<HttpResponse, MyError> {
    let mut app_state = GLOBAL_APP_STATE.lock().unwrap().clone();

    Ok(HttpResponse::Ok().json(&app_state.block_chain))
}

/// 获取最后一个块
pub async fn last_block() -> Result<HttpResponse, MyError> {
    let mut app_state = GLOBAL_APP_STATE.lock().unwrap().clone();

    let block = app_state
        .block_chain
        .lock()
        .unwrap()
        .blocks
        .last()
        .unwrap()
        .clone();

    Ok(HttpResponse::Ok().json(block))
}

/// 添加新的块，并返回所有块
pub async fn add_block_data(form: web::Json<BlockForm>) -> Result<HttpResponse, MyError> {
    info!("add_block_data...");
    let blocks;
    {
        let mut app_state = GLOBAL_APP_STATE.lock().unwrap().clone();
        // 获取最新的数据
        let mut chain = app_state.block_chain.lock().unwrap();
        chain.add_block_data(form.data.clone()).await;

        blocks = chain.blocks.clone();
    }

    info!("chain.add_block...");
    Ok(HttpResponse::Ok().json(blocks))
}

/// 添加新的块，并返回所有块
pub async fn broadcast_block_data(
    form: web::Json<BroadcastBlockForm>,
) -> Result<HttpResponse, MyError> {
    info!("接收节点发送的数据请求...");
    let mut app_state = GLOBAL_APP_STATE.lock().unwrap().clone();
    // 获取最新的数据
    let mut chain = app_state.block_chain.lock().unwrap();

    // 判断新增块是否符合要求
    if (BlockChain::is_valid_new_block(&form.block, &chain.get_last_block().unwrap())) {
        chain.add_block(form.block.clone())
    }

    info!("chain.add_block...");
    Ok(HttpResponse::Ok().json(&chain.blocks))
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
    fn get_last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }
    // 获取上一节点hash
    fn get_last_index(&self) -> u32 {
        self.blocks.last().unwrap().index
    }

    // 添加一个区块
    pub async fn add_block_data(&mut self, data: String) {
        let block = Block::init(self.get_last_index() + 1, data, self.get_last_hash());
        let broad_block = block.clone();

        self.add_block(block);

        // 异步任务，解决锁问题
        rt::spawn(async {
            let node_list;
            let cloned_broad_block;
            {
                // 解决 broad_block 值move问题
                cloned_broad_block = broad_block.clone();
                let mut app_state = GLOBAL_APP_STATE.lock().unwrap().clone();
                node_list = app_state.node_list.lock().unwrap().clone();
            }
            // 广播
            broadcast_latest(node_list, broad_block).await;
            info!("===> 调用 broadcast_latest 结束");
        });
        info!("===> 调用 异步任务结束");
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
        info!(
            "===> add_block: {}",
            self.blocks.last().unwrap().to_string()
        );
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
            // TODO 广播链
            // broadcast_latest();
        } else {
            println!("Received blockchain invalid...")
        }
    }
}

async fn broadcast_latest(node_list: Vec<String>, block: Block) {
    info!("===> broadcast_latest");
    // 获取所有节点信息
    // url 是否在列表中

    // 循环节点，发送最新区块信息
    for url in node_list.iter() {
        sync_node_list(url.clone(), &block).await
    }

    info!("===> 广播完成");
}

// 同步节点信息
async fn sync_node_list(node_url: String, block: &Block) {
    let form = crate::models::block_form::BroadcastBlockForm {
        block: block.clone(),
    };
    // 测试节点是否存在
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/blockchain/broadcast", &node_url))
        .json(&form)
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let body = response.text().await.unwrap();
        println!("Response body: {}", &body);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    info!("同步[{}] 结束", node_url);
}
