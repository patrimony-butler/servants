use common::member::ConfigName;
use common::member::ConfigResolver;

pub struct ButlerType;

impl ConfigResolver for ButlerType {
    fn get_config_name(&self) -> ConfigName {
        "butler.conf".to_string()
    }
}
