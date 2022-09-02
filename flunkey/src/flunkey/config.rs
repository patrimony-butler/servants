use std::net::SocketAddrV4;

use configparser::ini::Ini;

use common::app::{ConfigName, ServantResult};
use common::config::{read_host, read_port, ConfigReader};

pub struct Config {
    pub addr: SocketAddrV4,
    pub butler_addr: SocketAddrV4,
}

impl ConfigReader for Config {
    fn load(config_name: ConfigName) -> ServantResult<Box<Config>> {
        let mut config = Ini::new();
        let _map = config.load(config_name)?;

        let port = read_port(&config, "default", "port", 6100_u16)?;
        let host = read_host(&config, "default", "host", "0.0.0.0")?;
        let addr = SocketAddrV4::new(host, port);

        let butler_port = read_port(&config, "default", "butler_port", 6000_u16)?;
        let butler_host = read_host(&config, "default", "butler_host", "0.0.0.0")?;
        let butler_addr = SocketAddrV4::new(butler_host, butler_port);

        Ok(Box::new(Config { addr, butler_addr }))
    }
}
