use std::any::{Any, TypeId};

use serde::{Deserialize, Serialize};

use super::error_kind::ErrorKind;

#[macro_export]
macro_rules! params {
    ($($a:expr),*) => {{
        use zrpc::models::dt::ZRpcDtAuto;

        let mut res: Vec<ZRpcDt> = vec![];
        $(
            res.push($a.to_zdt());
        )*
        res
    }};
}

pub trait ZRpcDtAuto {
    fn to_zdt(&self) -> ZRpcDt;
}

impl ZRpcDtAuto for String {
    fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::String(self.clone())
    }
}

impl ZRpcDtAuto for i32 {
    fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::Int(*self)
    }
}

impl ZRpcDtAuto for f32 {
    fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::Float(*self)
    }
}

impl ZRpcDtAuto for Vec<u8> {
    fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::Serialized(self.clone())
    }
}

impl<T: Serialize> ZRpcDtAuto for T {
    default fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::serialize(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ZRpcDt {
    Int(i32),
    Float(f32),
    String(String),
    Serialized(Vec<u8>),
    Ok,
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
