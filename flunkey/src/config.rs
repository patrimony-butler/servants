use common::member::{ConfigName, ConfigResolver};

pub struct FlunkeyType {
    config_name: ConfigName,
}

impl ConfigResolver for FlunkeyType {
    fn new(config_name: ConfigName) -> Self {
        FlunkeyType { config_name }
    }

    fn get_config_name(&self) -> ConfigName {
        self.config_name.clone()
    }
}
