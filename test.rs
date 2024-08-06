const ADDR: &str = "127.0.0.1:3000";

#[derive(Debug, Serialize, Deserialize)]
enum RpcDT {
    Int(i32),
    Float(f32),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct RpcReq(String, Vec<RpcDT>);

impl RpcReq {
    pub fn new(procedure: String, params: Vec<RpcDT>) -> Self {
        Self(procedure, params)
    }
}

struct RpcServer {
    listener: TcpListener,
    procs: Arc<Mutex<HashMap<String, fn(f32, f32) -> f32>>>,
}

impl RpcServer {
    pub fn init(addr: &str) -> Result<Self, ()> {
        let listener = TcpListener::bind(addr).map_err(|_| ())?;

        Ok(Self {
            listener,
            procs: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub fn start(&mut self) -> Result<(), ()> {
        for res in self.listener.incoming() {
            match res {
                Ok(mut stream) => {
                    let mut len = [0u8; 4];
                    stream.read_exact(&mut len).map_err(|_| ())?;

                    let mut buf = vec![0u8; u32::from_be_bytes(len) as usize];
                    stream.read_exact(&mut buf).map_err(|_| ())?;

                    let req = serde_json::from_slice::<RpcReq>(&buf).map_err(|_| ())?;

                    if let Some(proc) = self.procs.lock().unwrap().get(&req.0) {
                        if let (RpcDT::Float(a), RpcDT::Float(b)) = (&req.1[0], &req.1[1]) {
                            let bytes =
                                serde_json::to_vec(&RpcDT::Float(proc(*a, *b))).map_err(|_| ())?;

                            stream
                                .write_all(&(bytes.len() as u32).to_be_bytes())
                                .map_err(|_| ())?;

                            println!(
                                "[RpcServer::start] {} bytes were written",
                                stream.write(&bytes).map_err(|_| ())?
                            );
                        }
                    }
                }
                Err(e) => eprintln!("Failed to handle stream: {}", e),
            }
        }

        Ok(())
    }

    pub fn add_proc(&mut self, n: String, f: fn(f32, f32) -> f32) {
        self.procs.lock().unwrap().insert(n, f);
    }
}

struct RpcClient {
    stream: TcpStream,
}

impl RpcClient {
    pub fn new(addr: &str) -> Result<Self, ()> {
        let stream = TcpStream::connect(addr).map_err(|_| ())?;

        Ok(Self { stream })
    }

    pub fn call(&mut self, req: RpcReq) -> Result<RpcDT, ()> {
        let bytes = serde_json::to_vec(&req).map_err(|_| ())?;
        let len = (bytes.len() as u32).to_be_bytes();

        self.stream.write_all(&len).map_err(|_| ())?;

        println!(
            "[RpcCliet::call] {} bytes were written",
            self.stream.write(&bytes).map_err(|_| ())?
        );

        let mut len = [0u8; 4];
        self.stream.read_exact(&mut len).map_err(|_| ())?;

        let mut buf = vec![0u8; u32::from_be_bytes(len) as usize];
        self.stream.read_exact(&mut buf).map_err(|_| ())?;

        Ok(serde_json::from_slice::<RpcDT>(&buf).map_err(|_| ())?)
    }
}

fn sum_f32(a: f32, b: f32) -> f32 {
    a + b
}

fn main() {
    let mut server = RpcServer::init(ADDR).expect("Failed to initialize RpcServer");
    server.add_proc("sum_f32".to_string(), sum_f32);

    let server_handle = std::thread::spawn(move || server.start());

    std::thread::sleep(Duration::from_secs(1));

    let mut client = RpcClient::new(ADDR).expect("Failed to create RpcClient");
    let client_handle = std::thread::spawn(move || {
        match client.call(RpcReq::new(
            "sum_f32".to_string(),
            vec![RpcDT::Float(1.0), RpcDT::Float(1.5)],
        )) {
            Ok(v) => println!("Response: {:#?}", v),
            Err(_) => eprintln!("Failed to call remote procedure"),
        }
    });

    server_handle
        .join()
        .expect("Failed to start RpcServer")
        .expect("Server thread panicked");
    client_handle.join().expect("Client thread panicked");
}
