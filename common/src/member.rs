use crate::error::ButlerError;

pub type ConfigName = String;

pub type ButlerResult<T> = std::result::Result<T, ButlerError>;
