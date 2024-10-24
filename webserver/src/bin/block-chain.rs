use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpServer};
use dotenv::dotenv;
use log::{debug, info, warn};
use sqlx::postgres::PgPoolOptions;
use std::convert::Into;
use std::env::set_var;
use std::ptr::null;
use std::{env, io, sync::Mutex};

/// 引入模块

// 错误处理
#[path = "../errors.rs"]
mod errors;
// 状态
#[path = "../state.rs"]
mod state;
// 路由
#[path = "../router.rs"]
mod router;
// 业务方法
#[path = "../handles/mod.rs"]
mod handles;
// 数据持久层
#[path = "../dbaccess/mod.rs"]
mod db_access;
// 模型
#[path = "../models/mod.rs"]
mod models;

use crate::handles::block_chain_service::load_block_chain;
use crate::models::block_chain_model::BlockChain;
use router::*;
use state::AppState;

///
/// 运行`cargo run --bin teacher-service`
/// 打印日志，需要添加环境变量： RUST_LOG=debug
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // 加载环境变量配置
    dotenv().ok();

    // 设置默认级别
    set_var("RUST_LOG", "debug");

    // 日期初始化
    env_logger::init();

    // 端口
    let server_port = env::var("SERVER_PORT").unwrap_or("8080".to_string());
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env文件里设置");
    // let database_url = "".to_string();
    info!("server_port: {}, database_url", server_port);

    // let db_pool: sqlx::Pool<sqlx::Postgres> =
    //     PgPoolOptions::new().connect(&database_url).await.unwrap();

    // 应用共享数据
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]),
        // db: db_pool,
        block_chain: Mutex::from(load_block_chain().unwrap()),
    });

    let app = move || {
        // 加载日志
        // let logger = Logger::default();

        // 支持跨域
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080;")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "PUT", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(36000);

        App::new()
            .wrap(Logger::new(""))
            .app_data(shared_data.clone())
            // 添加路由支持
            // 1.通用路由： 健康检查
            // curl localhost:3000/health
            .configure(general_routes)
            // 2.业务路由
            .configure(block_chain_routes)
            .wrap(cors)
    };
    info!("===> 应用启动 ====");
    // 应用启动，指定端口为3000
    HttpServer::new(app)
        .bind(format!("127.0.0.1:{}", server_port))?
        .run()
        .await
}
