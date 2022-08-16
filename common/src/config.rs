use crate::member::ButlerResult;
use crate::member::ConfigResolver;

pub trait ConfigReader {
    fn load(member_type: impl ConfigResolver) -> ButlerResult<Box<Self>>;
}
