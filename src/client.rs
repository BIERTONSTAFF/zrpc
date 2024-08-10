use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::{
    log,
    models::{dt::ZRpcDt, req::ZRpcReq},
};

pub struct ZRpcClient {
    stream: TcpStream,
}

impl ZRpcClient {
    pub fn new(addr: &str) -> Result<Self, ()> {
        let stream = TcpStream::connect(addr).map_err(|_| ())?;

        Ok(Self { stream })
    }

    pub fn call(&mut self, req: ZRpcReq) -> Result<ZRpcDt, ()> {
        let bytes = bincode::serialize(&req).map_err(|_| ())?;
        let len = (bytes.len() as u32).to_be_bytes();

        self.stream.write_all(&len).map_err(|_| ())?;
        self.stream.write_all(&bytes).map_err(|_| ())?;

        log!(
            "[ZRpcClient] {} bytes were written",
            len.len() + bytes.len()
        );

        let mut len = [0u8; 4];
        self.stream.read_exact(&mut len).map_err(|_| ())?;

        let mut buf = vec![0u8; u32::from_be_bytes(len) as usize];
        self.stream.read_exact(&mut buf).map_err(|_| ())?;

        Ok(bincode::deserialize::<ZRpcDt>(&buf).map_err(|_| ())?)
    }
}
