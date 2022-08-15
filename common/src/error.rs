use std::fmt;
use std::net::AddrParseError;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ButlerError {
    ConfigError(String),
    Error(std::io::Error),
    AddrParseError(AddrParseError),
    ParseIntError(ParseIntError),
}

impl fmt::Display for ButlerError {
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

impl From<std::io::Error> for ButlerError {
    fn from(err: std::io::Error) -> Self {
        ButlerError::Error(err)
    }
}

impl From<String> for ButlerError {
    fn from(err: String) -> Self {
        ButlerError::ConfigError(err)
    }
}

impl From<AddrParseError> for ButlerError {
    fn from(err: AddrParseError) -> Self {
        ButlerError::AddrParseError(err)
    }
}

impl From<ParseIntError> for ButlerError {
    fn from(err: ParseIntError) -> Self {
        ButlerError::ParseIntError(err)
    }
}
