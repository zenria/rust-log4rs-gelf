use std::{fmt, io};
use std::sync::mpsc::SendError;

use batch::Event;

#[derive(Debug)]
pub enum Error {
    ChannelError(SendError<Event>),
    FileNotFound(String),
    InvalidOutput(String),
    IOError(io::Error),
    JsonSerializerError(serde_json::Error),
    LogError(log::SetLoggerError),
    NotInitialized(String),
    TLSError(native_tls::HandshakeError<std::net::TcpStream>),
    ValueSerializerError(serde_value::SerializerError),
}

pub type Result<S> = std::result::Result<S, Error>;

impl From<native_tls::HandshakeError<std::net::TcpStream>> for Error {
    fn from(err: native_tls::HandshakeError<std::net::TcpStream>) -> Error {
        Error::TLSError(err)
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(err: log::SetLoggerError) -> Error {
        Error::LogError(err)
    }
}


impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IOError(err)
    }
}

impl From<serde_value::SerializerError> for Error {
    fn from(err: serde_value::SerializerError) -> Error {
        Error::ValueSerializerError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::JsonSerializerError(err)
    }
}

impl From<SendError<Event>> for Error {
    fn from(err: SendError<Event>) -> Error {
        Error::ChannelError(err)
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ChannelError(event) => write!(f, "ChannelError: {}", event),
            Error::FileNotFound(text) => write!(f, "FileNotFound: {}", text),
            Error::InvalidOutput(text) => write!(f, "InvalidOutput: {}", text),
            Error::IOError(err) => write!(f, "IOError: {}", err),
            Error::JsonSerializerError(err) => write!(f, "JsonSerializerError: {}", err),
            Error::LogError(err) => write!(f, "LogError: {}", err),
            Error::NotInitialized(text) => write!(f, "NotInitialized: {}", text),
            Error::TLSError(err) => write!(f, "TLSError: {}", err),
            Error::ValueSerializerError(err) => write!(f, "ValueSerializerError: {}", err),
        }
    }
}