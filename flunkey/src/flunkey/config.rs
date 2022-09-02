use std::net::SocketAddrV4;

use configparser::ini::Ini;

use common::app::{ConfigName, ServantResult};
use common::config::{
    ConfigLoader, ConfigReader, DEFAULT_BUTLER_HOST, DEFAULT_BUTLER_PORT, DEFAULT_FLUNKEY_HOST,
    DEFAULT_FLUNKEY_PORT,
};

pub struct ConfigData {
    pub addr: SocketAddrV4,
    pub butler_addr: SocketAddrV4,
}

impl ConfigLoader for ConfigData {
    fn load(config_name: ConfigName) -> ServantResult<Box<ConfigData>> {
        let mut config = Ini::new();
        let _map = config.load(config_name)?;

        let port = ConfigReader::read_port(&config, "default", "port", DEFAULT_FLUNKEY_PORT)?;
        let host = ConfigReader::read_host(&config, "default", "host", DEFAULT_FLUNKEY_HOST)?;
        let addr = SocketAddrV4::new(host, port);

        let butler_port =
            ConfigReader::read_port(&config, "default", "butler_port", DEFAULT_BUTLER_PORT)?;
        let butler_host =
            ConfigReader::read_host(&config, "default", "butler_host", DEFAULT_BUTLER_HOST)?;
        let butler_addr = SocketAddrV4::new(butler_host, butler_port);

        Ok(Box::new(ConfigData { addr, butler_addr }))
    }
}
