use common::member::{ConfigName, ConfigResolver};

pub struct FlunkeyType;

impl ConfigResolver for FlunkeyType {
    fn get_config_name(&self) -> ConfigName {
        "flunkey.conf".to_string()
    }
}
