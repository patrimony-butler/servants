use std::{
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
};

use configparser::ini::Ini;

use crate::ButlerResult;
use common::member::ConfigResolver;

pub struct Config {
    pub addr: SocketAddrV4,
}

impl Config {
    pub fn load(member_type: impl ConfigResolver) -> ButlerResult<Self> {
        let mut config = Ini::new();
        let _map = config.load(member_type.get_config_name())?;
        let port = match config.getuint("default", "port")? {
            Some(p) => p as u16,
            None => 6000_u16,
        };
        let host = match config.get("default", "host") {
            Some(h) => Ipv4Addr::from_str(&(h as String))?,
            None => Ipv4Addr::from_str("0.0.0.0")?,
        };
        let addr = SocketAddrV4::new(host, port);
        Ok(Config { addr })
    }
}
