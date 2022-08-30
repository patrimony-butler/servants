use crate::member::ConfigName;
use crate::member::ServantResult;

pub trait ConfigReader {
    fn load(member_type: ConfigName) -> ServantResult<Box<Self>>;
}
