use std::env;
use std::io::{Read, Write};
use std::net::{SocketAddrV4, TcpStream};
use std::str::from_utf8;

use common::app::ServantApp;
use common::app::ServantResult;

pub mod config;

pub struct ButlerCliApp {
    addr: SocketAddrV4,
}

impl ServantApp for ButlerCliApp {
    fn new(addr: SocketAddrV4) -> Self {
        ButlerCliApp { addr }
    }

    fn run(&self) -> ServantResult<()> {
        let args = self.read_args();
        let message = self.process_args(args);

        match TcpStream::connect(self.addr) {
            Ok(mut stream) => {
                println!(
                    "Successfully connected to server in port {}",
                    self.addr.port()
                );

                stream.write(message.as_bytes()).unwrap();

                let mut data = [0_u8; 500];
                match stream.read(&mut data) {
                    Ok(size) => {
                        println!("Received: {}", from_utf8(&data[0..size]).unwrap());
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

impl ButlerCliApp {
    fn read_args(&self) -> CLIArgs {
        let args: Vec<String> = env::args().collect();
        let args = CLIArgs::new(args);
        println!("{:?}", args);
        args
    }

    fn process_args(&self, args: CLIArgs) -> String {
        match &args.command[..] {
            "version" => "version".to_owned(),
            _ => "wrong command".to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct CLIArgs {
    pub program: String,
    pub command: String,
    pub arguments: Vec<String>,
}

impl CLIArgs {
    fn new(args: Vec<String>) -> Self {
        let program = (&args[0]).to_string();
        let command = (&args[1]).to_string();
        let arguments = (&args[2..]).to_vec();
        CLIArgs {
            program,
            command,
            arguments,
        }
    }
}
