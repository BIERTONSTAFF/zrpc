use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpListener,
    sync::{Arc, Mutex},
};

use crate::models::{dt::ZRpcDt, error_kind::ErrorKind, req::ZRpcReq};

pub struct ZRpcServer {
    listener: TcpListener,
    procs: Arc<Mutex<HashMap<String, Box<dyn Fn(&Vec<ZRpcDt>) -> ZRpcDt + Send + Sync>>>>,
}

impl ZRpcServer {
    pub fn new(addr: &str) -> Result<Self, ()> {
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

                    let req: ZRpcReq = bincode::deserialize(&buf).map_err(|_| ())?;
                    let bytes = match self.procs.lock().unwrap().get(&req.0) {
                        Some(proc) => bincode::serialize(&proc(&req.1)).map_err(|_| ())?,
                        None => bincode::serialize(&ZRpcDt::Error(ErrorKind::ProcedureNotFound)).map_err(|_| ())?,
                    };

                    stream
                        .write_all(&(bytes.len() as u32).to_be_bytes())
                        .map_err(|_| ())?;

                    println!(
                        "[ZRpcServer] {} bytes were written",
                        stream.write(&bytes).map_err(|_| ())?
                    );
                }
                Err(e) => eprintln!("Failed to handle stream: {}", e),
            }
        }
        Ok(())
    }

    pub fn add_proc<F>(&mut self, n: String, f: F)
    where F: Fn(&Vec<ZRpcDt>) -> ZRpcDt + 'static + Send + Sync {
        self.procs.lock().unwrap().insert(n, Box::new(f));
    }
}
