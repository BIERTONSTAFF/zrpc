use zrpc::models::error_kind::ErrorKind;
use zrpc::server::ZRpcServer;
use zrpc::models::dt::ZRpcDt;

fn sum_f32(p: &Vec<ZRpcDt>) -> ZRpcDt {
    match (&p[0], &p[1]) {
        (ZRpcDt::Float(a), ZRpcDt::Float(b)) => ZRpcDt::Float(a + b),
        _ => ZRpcDt::Error(ErrorKind::InvalidParameters),
    }
}

fn mul(p: &Vec<ZRpcDt>) -> ZRpcDt {
    match (&p[0], &p[1]) {
        (ZRpcDt::Float(a), ZRpcDt::Float(b)) => ZRpcDt::Float(a * b),
        _ => ZRpcDt::Error(ErrorKind::InvalidParameters),
    }
}

fn say_hello(p: &Vec<ZRpcDt>) -> ZRpcDt {
    match &p[0] {
        ZRpcDt::String(name) => ZRpcDt::String(format!("Hello, {}!", name)),
        _ => ZRpcDt::Error(ErrorKind::InvalidParameters),
    }
}

fn main() {
    let mut server = ZRpcServer::new("127.0.0.1:12520").expect("Failed to initialize ZRpcServer");
    server.add_proc("sum_f32".to_string(), sum_f32);
    server.add_proc("mul".to_string(), mul);
    server.add_proc("say_hello".to_string(), say_hello);

    let server_handle = std::thread::spawn(move || server.start());

    server_handle
        .join()
        .expect("Server thread has panicked")
        .expect("Failed to start RpcServer");
}
