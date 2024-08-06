use serde::{Deserialize, Serialize};

use super::error_kind::ErrorKind;

#[derive(Debug, Serialize, Deserialize)]
pub enum ZRpcDt {
    Int(i32),
    Float(f32),
    String(String),
    Serialized(Vec<u8>),
    Error(ErrorKind),
}

impl ZRpcDt {
    pub fn serialize<T: Serialize>(t: T) -> Self {
        Self::Serialized(bincode::serialize(&t).expect("Failed to serialize type"))
    }

    pub fn deserialize<T: for<'de> Deserialize<'de>>(&self) -> Result<T, ()> {
        if let Self::Serialized(bytes) = self {
            bincode::deserialize::<T>(bytes).map_err(|_| ())
        } else {
            Err(())
        }
    }
}
