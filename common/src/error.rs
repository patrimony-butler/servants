use std::fmt;
use std::net::AddrParseError;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ServantError {
    ConfigError(String),
    Error(std::io::Error),
    AddrParseError(AddrParseError),
    ParseIntError(ParseIntError),
}

impl fmt::Display for ServantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Butler error!")
    }
}

#[derive(Debug, Clone)]
struct ConfigError;

impl std::error::Error for ConfigError {}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Config error!")
    }
}

impl From<std::io::Error> for ServantError {
    fn from(err: std::io::Error) -> Self {
        ServantError::Error(err)
    }
}

impl From<String> for ServantError {
    fn from(err: String) -> Self {
        ServantError::ConfigError(err)
    }
}

impl From<AddrParseError> for ServantError {
    fn from(err: AddrParseError) -> Self {
        ServantError::AddrParseError(err)
    }
}

impl From<ParseIntError> for ServantError {
    fn from(err: ParseIntError) -> Self {
        ServantError::ParseIntError(err)
    }
}
