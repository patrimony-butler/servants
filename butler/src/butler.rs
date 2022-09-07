use std::io::{Read, Write};
use std::net::{SocketAddrV4, TcpListener, TcpStream};
use std::str;
use std::thread;

use common::app::ServantApp;
use common::app::ServantResult;

pub mod config;

pub struct ButlerApp {
    addr: SocketAddrV4,
}

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

impl ServantApp for ButlerApp {
    fn new(addr: SocketAddrV4) -> Self {
        ButlerApp { addr }
    }

    fn run(&mut self) -> ServantResult<()> {
        let listener = TcpListener::bind(self.addr)?;

        println!("Butler listening on port {}", self.addr.port());

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr()?);
                    thread::spawn(move || handle_client(stream));
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        drop(listener);

        Ok(())
    }
}

fn handle_client(mut stream: TcpStream) -> ServantResult<()> {
    let mut data = [0_u8; 500];

    while match stream.read(&mut data) {
        Ok(size) => {
            if size > 0 {
                println!("Received: '{}'", str::from_utf8(&data[0..size]).unwrap());
                let data = str::from_utf8(&data[0..size]).unwrap();
                let message = if data == "version" {
                    VERSION.unwrap()
                } else {
                    data
                };
                println!("Sent: {}", message);
                let _write_size = stream.write(message.as_bytes())?;
            }
            true
        }
        Err(_) => {
            println!(
                "An error occured, termination connection with {}",
                stream.peer_addr()?
            );
            false
        }
    } {}
    Ok(())
}
