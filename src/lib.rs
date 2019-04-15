extern crate log;
extern crate log4rs;
extern crate native_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_gelf;

pub use log4rs::init_config;

pub use batch::processor;

pub mod appender;
mod batch;
mod result;
mod formatter;
mod output;
mod macros;
mod file;

pub fn init_file<P>(path: P, deserializers: Option<log4rs::file::Deserializers>) -> Result<(), log4rs::Error> where P: AsRef<std::path::Path> {
    log4rs::init_file(path, deserializers.unwrap_or(file::deserializers()))
}


pub fn flush() -> result::Result<()> {
    processor().flush()
}