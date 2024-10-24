use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct BlockForm {
    pub data: String,
}
