use std::sync::Mutex;

use lazy_static::lazy_static;
use rdev::{listen, Event, EventType, Key};
use zrpc::{
    client::ZRpcClient,
    models::{dt::ZRpcDt, req::ZRpcReq},
    server::ZRpcServer,
};

lazy_static! {
    static ref SERVER: Mutex<ZRpcClient> =
        Mutex::new(ZRpcClient::new("127.0.0.1:13331").expect("Failed to initialize ZRpcClient"));
}

fn callback(e: Event) {
    match e.event_type {
        EventType::KeyPress(key) => match key == Key::End {
            true => match SERVER
                .lock()
                .unwrap()
                .call(ZRpcReq::new("list_keys", vec![]))
            {
                Ok(v) => println!("Keys: {:?}", v.deserialize::<Vec<String>>()),
                Err(_) => eprintln!("Failed to list keys"),
            },
            false => match SERVER.lock().unwrap().call(ZRpcReq::new(
                "push_key",
                vec![ZRpcDt::String(format!("{:?}", key))],
            )) {
                Ok(v) => println!("Reponse: {:?}", v),
                Err(_) => eprintln!("Failed to push key"),
            },
        },
        EventType::ButtonPress(button) => {
            match SERVER.lock().unwrap().call(ZRpcReq::new(
                "push_key",
                vec![ZRpcDt::String(format!("{:?}", button))],
            )) {
                Ok(v) => println!("Reponse: {:?}", v),
                Err(_) => eprintln!("Failed to push key"),
            }
        }
        _ => {}
    }
}
fn main() {
    match SERVER
        .lock()
        .unwrap()
        .call(ZRpcReq::new("list_keys", vec![]))
    {
        Ok(v) => println!("Keys: {:?}", v.deserialize::<Vec<String>>()),
        Err(_) => eprintln!("Failed to list keys"),
    }

    if let Err(e) = listen(callback) {
        eprintln!("Error: {:?}", e);
    }
}
