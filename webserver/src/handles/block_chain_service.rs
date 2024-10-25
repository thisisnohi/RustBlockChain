use crate::db_access::block_chain_db::all_blocks;
use crate::errors::MyError;
use crate::models::block_chain_model::BlockChain;
use crate::models::block_form::BlockForm;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use log::info;

// 加载本地区块链
// 从本地数据获取，如果没有，则创建一个新的
pub fn load_block_chain() -> Result<BlockChain, MyError> {
    info!("load_block_chain... 暂且当没有{}", "".to_string());

    Ok(BlockChain::init())
}

/// 获取所有块
pub async fn get_all_blocks(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    Ok(HttpResponse::Ok().json(&app_state.block_chain))
}

/// 添加新的块，并返回所有块
pub async fn add_block_data(
    form: web::Json<BlockForm>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    info!("add_block_data...");
    // 获取最新的数据
    let mut chain = app_state.block_chain.lock().unwrap();
    chain.add_block(form.data.clone());
    info!("chain.add_block...");
    Ok(HttpResponse::Ok().json(&chain.blocks))
}
