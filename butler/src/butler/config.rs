use std::net::SocketAddrV4;

use configparser::ini::Ini;

use common::app::{ConfigName, ServantResult};
use common::config::{read_host, read_port, ConfigReader};

pub struct Config {
    pub addr: SocketAddrV4,
}

impl ConfigReader for Config {
    fn load(config_name: ConfigName) -> ServantResult<Box<Config>> {
        let mut config = Ini::new();
        let _map = config.load(config_name)?;

        let port = read_port(&config, "default", "port", 6000_u16)?;
        let host = read_host(&config, "default", "host", "0.0.0.0")?;
        let addr = SocketAddrV4::new(host, port);

        Ok(Box::new(Config { addr }))
    }
}
