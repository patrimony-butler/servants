use common::error::ButlerError;

pub mod butler_cli;

pub type ButlerResult<T> = std::result::Result<T, ButlerError>;
