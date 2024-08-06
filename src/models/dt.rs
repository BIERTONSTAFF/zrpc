use serde::{Deserialize, Serialize};

use super::error_kind::ErrorKind;

#[derive(Debug, Serialize, Deserialize)]
pub enum ZRpcDt {
    Int(i32),
    Float(f32),
    String(String),
    Error(ErrorKind),
}
