use common::error::ButlerError;

pub mod config;
pub mod flunkey;

pub type ButlerResult<T> = std::result::Result<T, ButlerError>;
