use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpListener,
    process,
    sync::{Arc, Mutex},
};

use crate::models::{dt::ZRpcDt, error_kind::ErrorKind, req::ZRpcReq};

#[macro_export]
macro_rules! add_procs {
    ($server:expr, $($proc:ident),*) => {
        $(
            $server.add_proc(stringify!($proc), $proc);
        )*
    };
}

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
            let procs = self.procs.clone();

            std::thread::spawn(move || match res {
                Ok(mut stream) => {
                    if let Err(e) = Self::handle_stream(&mut stream, &procs) {
                        eprintln!("Failed to handle stream: {}", e);
                    }
                }
                Err(e) => eprintln!("Failed to accept connection: {}", e),
            });
        }
        Ok(())
    }

    pub fn add_proc<F>(&mut self, name: &str, proc: F)
    where
        F: Fn(&Vec<ZRpcDt>) -> ZRpcDt + 'static + Send + Sync,
    {
        println!("[ZRpcServer] '{}' procedure has been loaded", name);

        self.procs
            .lock()
            .unwrap()
            .insert(name.to_string(), Box::new(proc));
    }

    fn handle_stream(
        stream: &mut std::net::TcpStream,
        procs: &Arc<Mutex<HashMap<String, Box<dyn Fn(&Vec<ZRpcDt>) -> ZRpcDt + Send + Sync>>>>,
    ) -> Result<(), String> {
        let mut len = [0u8; 4];
        stream
            .read_exact(&mut len)
            .map_err(|_| "Failed to read buffer length")?;

        let mut buf = vec![0u8; u32::from_be_bytes(len) as usize];
        stream
            .read_exact(&mut buf)
            .map_err(|_| "Failed to read buffer")?;

        let req: ZRpcReq =
            bincode::deserialize(&buf).map_err(|_| "Failed to deserialize buffer")?;
        let bytes = match procs.lock().unwrap_or_else(|e| e.into_inner()).get(&req.0) {
            Some(proc) => {
                bincode::serialize(&proc(&req.1)).map_err(|_| "Failed to serialize result")?
            }
            None => bincode::serialize(&ZRpcDt::Error(ErrorKind::ProcedureNotFound))
                .map_err(|_| "Failed to serialize error")?,
        };

        stream
            .write_all(&(bytes.len() as u32).to_be_bytes())
            .map_err(|_| "Failed to write result length")?;

        println!(
            "[ZRpcServer:{:?}] {} bytes were written",
            std::thread::current().id(),
            stream.write(&bytes).map_err(|_| "Failed to write result")?,
        );

        Ok(())
    }
}
