use std::io::prelude::*;
use std::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(ip: &str) -> Connection {
        let stream = TcpStream::connect(ip)
            .unwrap_or_else(|error| panic!("Failed to connect to server: {}", error));
        Connection { stream }
    }

    pub fn send(&mut self, data: &str) {
        let mut data = data.to_string();
        data.push('\n');
        self.stream.write(data.as_bytes()).unwrap();
    }

    pub fn recv(&mut self) -> String {
        let mut data = String::new();
        self.stream.read_to_string(&mut data).unwrap();
        data
    }
}
