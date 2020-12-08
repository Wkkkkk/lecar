use rocket::data::{Outcome, FromData, Transformed, Transform, FromDataSimple};
use rocket::{Request, Data};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LecarData {
    pub key: usize,
    #[serde(with = "serde_bytes")]
    pub value: Vec<u8>
}
