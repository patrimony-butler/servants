pub struct ButlerType;

pub struct FlunkeyType;

pub trait ConfigResolver {
    fn get_config_name(&self) -> ConfigName;
}

impl ConfigResolver for ButlerType {
    fn get_config_name(&self) -> ConfigName {
        "butler.conf".to_string()
    }
}

impl ConfigResolver for FlunkeyType {
    fn get_config_name(&self) -> ConfigName {
        "flunkey.conf".to_string()
    }
}

type ConfigName = String;
