use crate::member::ButlerResult;
use crate::member::ConfigName;

pub trait ConfigReader {
    fn load(member_type: ConfigName) -> ButlerResult<Box<Self>>;
}
