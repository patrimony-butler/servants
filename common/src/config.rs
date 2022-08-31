use crate::app::ConfigName;
use crate::app::ServantResult;

pub trait ConfigReader {
    fn load(member_type: ConfigName) -> ServantResult<Box<Self>>;
}
