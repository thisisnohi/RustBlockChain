use actix_web::web;

use crate::handles::block_chain_service::add_block_data;
use crate::handles::node_service::{add_node, all_nodes};
use crate::handles::{block_chain_service::get_all_blocks, general::health_check_handler};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

/// 区块链相关路由
pub fn block_chain_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/blockchain")
            // 获取所有块
            .route("/", web::get().to(get_all_blocks))
            // 添加新的块
            .route("/", web::put().to(add_block_data))
            .route("/{data}", web::put().to(add_block_data)),
    );
}

/// 节点相关路由
pub fn node_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/nodes")
            // 添加新的节点
            .route("", web::put().to(add_node))
            // 删除节点
            // .route("/", web::delete().to(add_node))
            // 获取所有节点
            .route("", web::get().to(all_nodes)),
    );
}
