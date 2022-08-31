use crate::error::ServantError;
use std::net::SocketAddrV4;

pub trait ServantApp {
    fn new(addr: SocketAddrV4) -> Self;
    fn run(&self) -> Result<(), ServantError>;
}

pub type ConfigName = String;

pub type ServantResult<T> = std::result::Result<T, ServantError>;
