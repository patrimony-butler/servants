use crate::error::ServantError;

pub type ConfigName = String;

pub type ServantResult<T> = std::result::Result<T, ServantError>;
