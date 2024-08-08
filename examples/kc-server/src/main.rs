use std::sync::Mutex;

use lazy_static::lazy_static;
use zrpc::{
    add_procs,
    models::{dt::ZRpcDt, error_kind::ErrorKind},
    server::ZRpcServer,
};

lazy_static! {
    static ref K_STORE: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn push_key(p: &Vec<ZRpcDt>) -> ZRpcDt {
    match p.len() == 1 {
        true => match &p[0] {
            ZRpcDt::String(key) => {
                K_STORE.lock().unwrap().push(key.to_string());

                ZRpcDt::Ok
            }
            _ => ZRpcDt::Error(ErrorKind::InvalidParameters),
        },
        false => ZRpcDt::Error(ErrorKind::InvalidParameters),
    }
}

fn list_keys(_: &Vec<ZRpcDt>) -> ZRpcDt {
    ZRpcDt::serialize(K_STORE.lock().unwrap().clone())
}

fn main() {
    let mut server = ZRpcServer::new("127.0.0.1:13331").expect("Failed to initialize ZRpcServer");

    add_procs!(server, push_key, list_keys);

    server.start().expect("Failed to start ZRpcServer");
}
