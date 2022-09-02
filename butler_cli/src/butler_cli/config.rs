use std::net::SocketAddrV4;

use configparser::ini::Ini;

use common::app::{ConfigName, ServantResult};
use common::config::{ConfigLoader, ConfigReader};

pub struct ConfigData {
    pub butler_addr: SocketAddrV4,
}

impl ConfigLoader for ConfigData {
    fn load(config_name: ConfigName) -> ServantResult<Box<ConfigData>> {
        let mut config = Ini::new();
        let _map = config.load(config_name)?;

        let port = ConfigReader::read_port(&config, "default", "butler_port", 6100_u16)?;
        let host = ConfigReader::read_host(&config, "default", "butler_host", "0.0.0.0")?;
        let butler_addr = SocketAddrV4::new(host, port);

        Ok(Box::new(ConfigData { butler_addr }))
    }
}
