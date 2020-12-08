use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LecarData {
    pub key: usize,
    #[serde(with = "serde_bytes")]
    pub value: Vec<u8>
}
