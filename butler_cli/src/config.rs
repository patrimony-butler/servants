use common::member::{ConfigName, ConfigResolver};

pub struct ButlerCliType {
    config_name: ConfigName,
}

impl ConfigResolver for ButlerCliType {
    fn new(config_name: ConfigName) -> Self {
        ButlerCliType { config_name }
    }

    fn get_config_name(&self) -> ConfigName {
        self.config_name.clone()
    }
}
