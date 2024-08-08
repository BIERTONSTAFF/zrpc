use std::sync::Mutex;

use test_domain::User;
use zrpc::{
    add_procs,
    models::{dt::ZRpcDt, error_kind::ErrorKind},
    server::ZRpcServer,
};

lazy_static::lazy_static! {
    static ref USERS: Mutex<Vec<User>> = Mutex::new(vec![]);
}

fn create_user(p: &Vec<ZRpcDt>) -> ZRpcDt {
    if p.len() != 2 {
        return ZRpcDt::Error(ErrorKind::InvalidParameters);
    }

    match (&p[0], &p[1]) {
        (ZRpcDt::String(name), ZRpcDt::Int(age)) => {
            let user = User {
                name: name.clone(),
                age: *age,
            };

            USERS.lock().unwrap().push(user.clone());

            ZRpcDt::serialize(user)
        }
        _ => ZRpcDt::Error(ErrorKind::InvalidParameters),
    }
}

fn list_users(_: &Vec<ZRpcDt>) -> ZRpcDt {
    ZRpcDt::serialize(USERS.lock().unwrap().clone())
}

fn main() {
    let mut server = ZRpcServer::new("127.0.0.1:3000").expect("Failed to initialize ZRpcServer");

    add_procs!(server, create_user, list_users);

    server.start().expect("Failed to start server");
}
