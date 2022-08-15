use crate::error::ButlerError;

pub mod error;
pub mod flunkey;
pub mod butler;
pub mod member;

pub type ButlerResult<T> = std::result::Result<T, ButlerError>;

