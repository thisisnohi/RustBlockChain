// use crate::models::block_chain_model::BlockChain;
use log::Level::Debug;
use log::{debug, info, log, log_enabled, trace, warn, Level};
// #[path = "./models/mod.rs"]
// mod models;

/// 第一步： 引入依赖
///     log = "0.4.22"
///     env_logger = "0.11.5"
/// 第二步： 初始化
///     env_logger::init();
/// 第三步： 环境变量添加日志级别：RUST_LOG=debug
fn main() {
    env_logger::init();

    trace!("Commencing yak shaving");

    // 判断能否记录 Debug 消息
    if log_enabled!(Debug) {
        // 下面的日志记录较为昂贵，因此我们先在前面判断了是否能够记录，能，才继续这里的逻辑
        debug!("expensive debug data: {} {}", 1, 2);
    }
    if log_enabled!(target: "Global", Debug) {
        debug!(target: "Global", "expensive debug data: {} {}", 11,22);
    }

    log!(Level::Error, "Received errors: {}, {}", 11, 22);
    log!(target: "app_events", Level::Warn, "App warning: {}, {}, {}",
   11,22,33);

    println!("Hello, world!");
    // // 初始化链
    // let mut chain = BlockChain::init();
    // println!("===> init chain:{:?}", chain);
    //
    // chain.add_block("first Block".to_string());
    // println!("===> init chain:{:?}", chain);
    //
    // println!(
    //     "===> chain is valid:{}",
    //     BlockChain::is_valid_chain(&chain.blocks)
    // );
}
