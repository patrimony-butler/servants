use std::net::SocketAddrV4;

use configparser::ini::Ini;

use common::app::{ConfigName, ServantResult};
use common::config::{
    ConfigLoader, ConfigReader, DEFAULT_BUTLER_HOST, DEFAULT_BUTLER_PORT,
    DEFAULT_CONFIG_SECTION_NAME,
};

pub struct ConfigData {
    pub addr: SocketAddrV4,
}

impl ConfigLoader for ConfigData {
    fn load(config_name: ConfigName) -> ServantResult<Box<ConfigData>> {
        let mut config = Ini::new();
        let _map = config.load(config_name)?;

        let port = ConfigReader::read_port(
            &config,
            DEFAULT_CONFIG_SECTION_NAME,
            "port",
            DEFAULT_BUTLER_PORT,
        )?;
        let host = ConfigReader::read_host(
            &config,
            DEFAULT_CONFIG_SECTION_NAME,
            "host",
            DEFAULT_BUTLER_HOST,
        )?;
        let addr = SocketAddrV4::new(host, port);

        Ok(Box::new(ConfigData { addr }))
    }
}
