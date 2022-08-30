use common::error::ButlerError;

pub mod flunkey;

pub type ButlerResult<T> = std::result::Result<T, ButlerError>;
