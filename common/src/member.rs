use crate::error::ButlerError;

pub trait ConfigResolver {
    fn new(config_name: ConfigName) -> Self;
    fn get_config_name(&self) -> ConfigName;
}

pub type ConfigName = String;

pub type ButlerResult<T> = std::result::Result<T, ButlerError>;
