use zrpc::models::{dt::ZRpcDt, req::ZRpcReq};
use zrpc::client::ZRpcClient;

fn main() {
    let mut client = ZRpcClient::new("127.0.0.1:12520").expect("Failed to initialize ZRpcClient");
    let client_handle = std::thread::spawn(move || {
        match client.call(ZRpcReq::new(
            "say_hello".to_string(),
            vec![ZRpcDt::String("John".to_string())],
        )) {
            Ok(v) => println!("Response: {:#?}", v),
            Err(_) => eprintln!("Failed to call remote procedure"),
        }
    });

    client_handle.join().expect("Client thread has panicked");
}
