use std::error::Error as stdError;
use std::{fmt, net};
use serde_json;

#[derive(Debug)]
pub enum Error {
    InvalidConfiguration(String),
    InvalidAddress(net::AddrParseError),
    Parsing(serde_json::Error),
}


impl stdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidConfiguration(ref data) => data,
            Error::InvalidAddress(ref err) => err.description(),
            Error::Parsing(ref err) => err.description()
        }
    }
}

impl From<net::AddrParseError> for Error {
    fn from(err: net::AddrParseError) -> Error {
        Error::InvalidAddress(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Parsing(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description())
    }
}