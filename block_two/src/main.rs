use block_two::BlockChain;
use std::fmt::Display;

fn main() {
    println!("hello BlockChain");

    // 初始化链
    let mut chain = BlockChain::init();
    println!("===> init chain:{:?}", chain);

    chain.add_block("first Block".to_string());
    println!("===> init chain:{:?}", chain);

    println!(
        "===> chain is valid:{}",
        BlockChain::is_valid_chain(&chain.blocks)
    );
}
