use crate::messages;
use std::io::Write;
use std::net::TcpStream;

pub trait Command {
    fn execute(&self, stream: &mut TcpStream);
}

pub struct Version;

impl Command for Version {
    fn execute(&self, stream: &mut TcpStream) {
        stream.write_all("version".as_bytes()).unwrap();
    }
}

impl Version {
    pub fn compose_message(&self) -> String {
        let message = messages::Version {
            app_id: String::from("app_id"),
            text: String::from("version"),
        };
        message.serialize()
    }
}
