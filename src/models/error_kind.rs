use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorKind {
    ProcedureNotFound,
    InvalidParameters,
    InternalError,
}
