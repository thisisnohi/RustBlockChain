use actix_web::web;

use crate::handles::block_chain_service::add_block_data;
use crate::handles::{block_chain_service::get_all_blocks, general::health_check_handler};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

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
