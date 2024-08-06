use test_domain::User;
use zrpc::add_procs;
use zrpc::models::dt::ZRpcDt;
use zrpc::models::error_kind::ErrorKind;
use zrpc::server::ZRpcServer;

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

fn user_info(p: &Vec<ZRpcDt>) -> ZRpcDt {
    match &p[0] {
        ZRpcDt::Serialized(_) => {
            let user = p[0]
                .deserialize::<User>()
                .expect("Failed to deserialize User");

            ZRpcDt::String(format!("Name: {}, age: {}", user.name, user.age))
        }
        _ => ZRpcDt::Error(ErrorKind::InvalidParameters),
    }
}

fn main() {
    let mut server = ZRpcServer::new("127.0.0.1:12520").expect("Failed to initialize ZRpcServer");

    add_procs!(server, sum_f32, mul, say_hello, user_info);

    let server_handle = std::thread::spawn(move || server.start());

    server_handle
        .join()
        .expect("Server thread has panicked")
        .expect("Failed to start RpcServer");
}
