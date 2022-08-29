use std::{
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
};

use configparser::ini::Ini;

use common::config::ConfigReader;
use common::member::{ButlerResult, ConfigResolver};

pub struct Config {
    pub butler_addr: SocketAddrV4,
}

impl ConfigReader for Config {
    fn load(member_type: impl ConfigResolver) -> ButlerResult<Box<Config>> {
        let mut config = Ini::new();
        let _map = config.load(member_type.get_config_name())?;
        let port = match config.getuint("default", "butler_port")? {
            Some(p) => p as u16,
            None => 6100_u16,
        };
        let host = match config.get("default", "butler_host") {
            Some(h) => Ipv4Addr::from_str(&(h as String))?,
            None => Ipv4Addr::from_str("0.0.0.0")?,
        };
        let butler_addr = SocketAddrV4::new(host, port);

        Ok(Box::new(Config { butler_addr }))
    }
}
