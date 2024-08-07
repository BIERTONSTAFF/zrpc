use std::time::Duration;

use test_domain::User;
use zrpc::{
    client::ZRpcClient,
    models::{dt::ZRpcDt, req::ZRpcReq},
};

fn main() {
    let mut client = ZRpcClient::new("127.0.0.1:3000").expect("Failed to initialize ZRpcClient");

    match client.call(ZRpcReq::new(
        "create_user".to_string(),
        vec![ZRpcDt::String("John".to_string()), ZRpcDt::Int(18)],
    )) {
        Ok(v) => {
            println!("user_created: {:#?}", v.deserialize::<User>());

            match client.call(ZRpcReq::new("list_users".to_string(), vec![])) {
                Ok(v) => println!("users_list: {:#?}", v.deserialize::<Vec<User>>()),
                Err(_) => eprintln!("Failed to call 'list_users' procedure"),
            }
        }

        Err(_) => eprintln!("Failed to call 'create_user' procedure"),
    }
}
