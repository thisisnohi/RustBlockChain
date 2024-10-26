use serde::Deserialize;

/// 节点信息
#[derive(Deserialize, Debug, Clone)]
pub struct NodeForm {
    pub node: String,
}
