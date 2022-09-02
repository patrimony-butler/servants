use crate::app::ConfigName;
use crate::app::ServantResult;
use std::{net::Ipv4Addr, str::FromStr};

use configparser::ini::Ini;

pub const DEFAULT_BUTLER_PORT: u16 = 6000;
pub const DEFAULT_BUTLER_HOST: &str = "0.0.0.0";
pub const DEFAULT_FLUNKEY_PORT: u16 = 6100_u16;
pub const DEFAULT_FLUNKEY_HOST: &str = "0.0.0.0";

pub trait ConfigLoader {
    fn load(member_type: ConfigName) -> ServantResult<Box<Self>>;
}

pub struct ConfigReader;

impl ConfigReader {
    pub fn read_port(
        config: &Ini,
        section: &str,
        field_name: &str,
        default_value: u16,
    ) -> ServantResult<u16> {
        Ok(match config.getuint(section, field_name)? {
            Some(p) => p as u16,
            None => default_value,
        })
    }

    pub fn read_host(
        config: &Ini,
        section: &str,
        field_name: &str,
        default_value: &str,
    ) -> ServantResult<Ipv4Addr> {
        Ok(match config.get(section, field_name) {
            Some(h) => Ipv4Addr::from_str(&(h as String))?,
            None => Ipv4Addr::from_str(default_value)?,
        })
    }
}
