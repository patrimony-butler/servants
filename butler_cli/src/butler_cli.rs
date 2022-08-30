use std::io::{Read, Write};
use std::net::{SocketAddrV4, TcpStream};
use std::str::from_utf8;

use common::app::ServantApp;
use common::error::ButlerError;

pub mod config;

pub struct ButlerCliApp {
    addr: SocketAddrV4,
}

impl ServantApp for ButlerCliApp {
    fn new(addr: SocketAddrV4) -> Self {
        ButlerCliApp { addr }
    }

    fn run(&self) -> Result<(), ButlerError> {
        println!("{:?}", self.addr);
        match TcpStream::connect(self.addr) {
            Ok(mut stream) => {
                println!(
                    "Successfully connected to server in port {}",
                    self.addr.port()
                );

                let msg = b"Hello from cli!";

                stream.write(msg).unwrap();
                println!("Sent Hello from cli!, awaiting reply...");

                let mut data = [0_u8; 15];
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
