use crate::models::block_chain_model::BlockChain;
use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    // pub courses: Mutex<Vec<Course>>
    // pub db: PgPool,
    pub block_chain: Mutex<BlockChain>,
}
