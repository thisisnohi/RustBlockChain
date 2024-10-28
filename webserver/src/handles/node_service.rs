use crate::db_access::block_chain_db::all_blocks;
use crate::errors::MyError;
use crate::models::block_chain_model::BlockChain;
use crate::models::block_form::BlockForm;
use crate::models::node_info::{NodeForm, NodeHealth};
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use log::info;
use std::collections::HashMap;

/// 加载本地区块链
// 从本地数据获取，如果没有，则创建一个新的
pub async fn all_nodes(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    info!("获取所有链接的节点...{}", "".to_string());

    Ok(HttpResponse::Ok().json(app_state.node_list.lock().unwrap().clone()))
}

/// 添加节点
pub async fn add_node(
    app_state: web::Data<AppState>,
    node_form: web::Json<NodeForm>,
) -> Result<HttpResponse, MyError> {
    info!("连接新节点...{:?}", node_form);

    // 测试节点是否存在
    let url = node_form.node.clone();
    let resp = reqwest::get(format!("{}/health", &url))
        .await
        .unwrap()
        .json::<NodeHealth>()
        .await
        .unwrap();

    info!("{:#?}", resp);

    // url 是否在列表中
    let mut node_list = app_state.node_list.lock().unwrap();
    if node_list.contains(&url) {
        return Err(MyError::NodeExist("节点已经存在111".into()));
    }

    // 添加节点到列表中
    node_list.push(url);

    // 返回所有节点
    Ok(HttpResponse::Ok().json(node_list.clone()))
}
