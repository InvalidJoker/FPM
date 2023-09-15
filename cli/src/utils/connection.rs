use std::io::prelude::*;
use std::net::TcpStream;

use crate::{_color_output, error};

pub struct Connection {
    stream: TcpStream,
}
impl Connection {
    pub fn new(ip: &str) -> Connection {
        let stream = TcpStream::connect(ip);

        // check if connection was successful
        if stream.is_err() {
            error!("Failed to connect to the backend");
        }

        Connection {
            stream: stream.unwrap(),
        }
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
