use serde::{Deserialize, Serialize};

/// 节点健康信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeHealth {
    pub node: String,
    pub health: bool,
    pub visit_count: u32,
    pub message: String,
}

/// 节点信息
#[derive(Deserialize, Debug, Clone)]
pub struct NodeForm {
    pub node: String,
}
