use std::io::{Read, Write};
use std::net::{SocketAddrV4, TcpListener, TcpStream};
use std::thread;

use common::app::ServantApp;
use common::error::ServantError;

pub mod config;

pub struct ButlerApp {
    addr: SocketAddrV4,
}

impl ServantApp for ButlerApp {
    fn new(addr: SocketAddrV4) -> Self {
        ButlerApp { addr }
    }

    fn run(&self) -> Result<(), ServantError> {
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

fn handle_client(mut stream: TcpStream) -> Result<(), ServantError> {
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
