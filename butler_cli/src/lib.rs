use common::error::ButlerError;

pub mod butler_cli;
pub mod config;

pub type ButlerResult<T> = std::result::Result<T, ButlerError>;
