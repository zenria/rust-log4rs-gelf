use log::Record;
use log4rs::encode::Write;
use std::error::Error;

pub mod gelf;

pub trait Encoder {
    fn encode(&self, w: &mut Write, record: &Record) -> Result<(), Box<Error + Sync + Send>>;
}