use serde::{Deserialize, Serialize};

use super::dt::ZRpcDt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZRpcReq(pub String, pub Vec<ZRpcDt>);

impl ZRpcReq {
    pub fn new(proc: &str, params: Vec<ZRpcDt>) -> Self {
        Self(proc.to_string(), params)
    }
}
