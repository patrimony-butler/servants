use common::error::ButlerError;

pub mod butler;

pub type ButlerResult<T> = std::result::Result<T, ButlerError>;
