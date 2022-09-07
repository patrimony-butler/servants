use std::io::{Read, Write};
use std::net::{SocketAddrV4, TcpStream};
use std::str::from_utf8;

use common::app::ServantApp;
use common::app::ServantResult;

pub mod config;

pub struct FlunkeyApp {
    addr: SocketAddrV4,
}

impl ServantApp for FlunkeyApp {
    fn new(addr: SocketAddrV4) -> Self {
        FlunkeyApp { addr }
    }

    fn run(&mut self) -> ServantResult<()> {
        match TcpStream::connect(self.addr) {
            Ok(mut stream) => {
                println!(
                    "Successfully connected to server in port {}",
                    self.addr.port()
                );

                let msg = b"Hello!";

                stream.write(msg).unwrap();
                println!("Sent Hello, awaiting reply...");

                let mut data = [0_u8; 6];
                match stream.read_exact(&mut data) {
                    Ok(_) => {
                        if &data == msg {
                            println!("Reply is ok!");
                        } else {
                            let text = from_utf8(&data).unwrap();
                            println!("Unexpected reply: {}", text);
                        }
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
        println!("Terminated");
        Ok(())
    }
}
