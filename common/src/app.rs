use crate::error::ButlerError;
use std::net::SocketAddrV4;

pub trait ServantApp {
    fn new(addr: SocketAddrV4) -> Self;
    fn run(&self) -> Result<(), ButlerError>;
}
