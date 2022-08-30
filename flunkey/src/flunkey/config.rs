use std::{
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
};

use configparser::ini::Ini;

use common::config::ConfigReader;
use common::member::{ConfigName, ServantResult};

pub struct Config {
    pub addr: SocketAddrV4,
    pub butler_addr: SocketAddrV4,
}

impl ConfigReader for Config {
    fn load(config_name: ConfigName) -> ServantResult<Box<Config>> {
        let mut config = Ini::new();
        let _map = config.load(config_name)?;
        let port = match config.getuint("default", "port")? {
            Some(p) => p as u16,
            None => 6100_u16,
        };
        let host = match config.get("default", "host") {
            Some(h) => Ipv4Addr::from_str(&(h as String))?,
            None => Ipv4Addr::from_str("0.0.0.0")?,
        };
        let addr = SocketAddrV4::new(host, port);

        let butler_port = match config.getuint("default", "butler_port")? {
            Some(p) => p as u16,
            None => 6000_u16,
        };
        let butler_host = match config.get("default", "butler_host") {
            Some(h) => Ipv4Addr::from_str(&(h as String))?,
            None => Ipv4Addr::from_str("127.0.0.1")?,
        };
        let butler_addr = SocketAddrV4::new(butler_host, butler_port);

        Ok(Box::new(Config { addr, butler_addr }))
    }
}
