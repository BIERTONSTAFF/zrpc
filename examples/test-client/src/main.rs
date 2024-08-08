use test_domain::User;
use zrpc::client::ZRpcClient;
use zrpc::models::{dt::ZRpcDt, req::ZRpcReq};
use zrpc::params;

fn main() {
    let mut client = ZRpcClient::new("127.0.0.1:12520").expect("Failed to initialize ZRpcClient");
    let client_handle = std::thread::spawn(move || {
        match client.call(ZRpcReq::new(
            "user_info",
            // vec![ZRpcDt::serialize(User {
            //     name: "John".to_string(),
            //     age: 50,
            // })],
            params!(User {
                name: "John".to_string(),
                age: 50,
            }),
        )) {
            Ok(v) => {
                println!("Response: {:?}", v);
            }
            Err(_) => eprintln!("Failed to call remote procedure"),
        }
    });

    client_handle.join().expect("Client thread has panicked");
}
