use test_domain::User;
use zrpc::{
    client::ZRpcClient,
    models::{dt::ZRpcDt, req::ZRpcReq},
    params,
};

fn main() {
    let mut client = ZRpcClient::new("127.0.0.1:3000").expect("Failed to initialize ZRpcClient");

    match client.call(ZRpcReq::new(
        "create_user",
        // vec![ZRpcDt::String("John".to_string()), ZRpcDt::Int(18)],
        params!("John".to_string(), 18),
    )) {
        Ok(v) => {
            println!("user_created: {:#?}", v.deserialize::<User>().unwrap());

            match client.call(ZRpcReq::new("list_users", vec![])) {
                Ok(v) => println!("users_list: {:#?}", v.deserialize::<Vec<User>>().unwrap()),
                Err(_) => eprintln!("Failed to call 'list_users' procedure"),
            }
        }

        Err(_) => eprintln!("Failed to call 'create_user' procedure"),
    }
}
