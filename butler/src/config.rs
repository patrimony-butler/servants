use common::member::ConfigName;
use common::member::ConfigResolver;

pub struct ButlerType {
    config_name: ConfigName,
}

impl ConfigResolver for ButlerType {
    fn new(config_name: ConfigName) -> Self {
        ButlerType { config_name }
    }

    fn get_config_name(&self) -> ConfigName {
        self.config_name.clone()
    }
}
