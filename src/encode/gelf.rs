use builder::Builder;
use log::{self, Record};
use serde_json::{self, Map, Value};
use chrono::Utc;
use hostname;
use log4rs::encode::Encode;
use log4rs::encode::Write;
use error::Error;
use std::error::Error as stdError;
use std::ops::Add;

#[derive(Debug, Serialize, Copy, Clone)]
pub enum GelfLevel {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Informational = 6,
    Debugging = 7,
}

impl From<log::Level> for GelfLevel {
    fn from(level: log::Level) -> GelfLevel {
        match level {
            log::Level::Trace => GelfLevel::Debugging,
            log::Level::Debug => GelfLevel::Debugging,
            log::Level::Info => GelfLevel::Informational,
            log::Level::Warn => GelfLevel::Warning,
            log::Level::Error => GelfLevel::Error
        }
    }
}

pub struct GelfEncoderBuilder {
    null_character: bool,
    additionnal_fields: Map<String, Value>,
}

impl GelfEncoderBuilder {
    pub fn null_character(mut self, null_character: bool) -> GelfEncoderBuilder {
        self.null_character = null_character;
        self
    }
    pub fn add_field(mut self, name: &str, value: Value) -> GelfEncoderBuilder {
        self.additionnal_fields.insert(format!("_{}", name), value);
        self
    }
}

impl Builder for GelfEncoderBuilder {
    type TargetItem = GelfEncoder;

    fn build(self) -> Result<GelfEncoder, Error> {
        Ok(GelfEncoder {
            null_character: self.null_character,
            hostname: hostname::get_hostname().unwrap_or("localhost".to_string()),
            additionnal_fields: self.additionnal_fields,
        })
    }
}

#[derive(Debug)]
pub struct GelfEncoder {
    additionnal_fields: Map<String, Value>,
    hostname: String,
    null_character: bool,
}


impl GelfEncoder {
    pub fn new(null_character: bool, additionnal_fields: Map<String, Value>) -> GelfEncoder {
        GelfEncoder {
            additionnal_fields,
            hostname: hostname::get_hostname().unwrap_or("localhost".to_string()),
            null_character,
        }
    }
    pub fn builder() -> GelfEncoderBuilder { GelfEncoderBuilder { null_character: false, additionnal_fields: Map::new() } }
    pub fn null_character(self) -> bool { self.null_character }
    fn current_timestamp() -> i64 { Utc::now().timestamp() }
}

impl Encode for GelfEncoder {
    fn encode(&self, w: &mut Write, record: &Record) -> Result<(), Box<stdError + Sync + Send>> {
        let mut map: Map<String, Value> = Map::new();
        map.insert("facility".to_string(), Value::from(record.module_path().unwrap_or("")));
        map.insert("file".to_string(), Value::from(record.file().unwrap_or("")));
        map.insert("host".to_string(), Value::from(self.hostname.clone()));
        map.insert("level".to_string(), Value::from(GelfLevel::from(record.level()) as u32));
        map.insert("line".to_string(), Value::from(record.line().unwrap_or(0)));
        map.insert("short_message".to_string(), Value::from(format!("{}", record.args())));
        map.insert("timestamp".to_string(), Value::from(GelfEncoder::current_timestamp()));
        map.insert("version".to_string(), Value::from("1.1"));
        map.extend(self.additionnal_fields.clone());

        let mut jdata = serde_json::to_string(&map)?;
        if self.null_character == true {
            jdata = jdata.add("\0");
        }
        w.write(&jdata.as_bytes())?;
        w.flush()?;
        Ok(())
    }
}
