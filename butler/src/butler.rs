use std::io::{Read, Write};
use std::net::{SocketAddrV4, TcpListener, TcpStream};
use std::thread;

use common::error::ButlerError;

pub mod config;

pub struct ButlerApp {
    addr: SocketAddrV4,
}

impl ButlerApp {
    pub fn new(addr: SocketAddrV4) -> Self {
        ButlerApp { addr }
    }

    pub fn run(&self) -> Result<(), ButlerError> {
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

fn handle_client(mut stream: TcpStream) -> Result<(), ButlerError> {
    let mut data = [0_u8; 500];

    while match stream.read(&mut data) {
        Ok(size) => {
            stream.write(&data[0..size])?;
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
