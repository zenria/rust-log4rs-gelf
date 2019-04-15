use std::collections::btree_map::BTreeMap;

use log4rs::append::Append;
use log4rs::file::{Deserialize, Deserializers};
use serde_gelf::level::GelfLevel;

use appender::BufferAppenderBuilder;

#[derive(Deserialize)]
pub struct BufferAppenderConfig {
    level: GelfLevel,
    hostname: String,
    port: u64,
    use_tls: bool,
    null_character: bool,
    buffer_size: Option<usize>,
    buffer_duration: Option<u64>,
    additional_fields: BTreeMap<String, serde_value::Value>,
}


struct BufferAppenderDeserializer;

impl Deserialize for BufferAppenderDeserializer {
    type Trait = Append;

    type Config = BufferAppenderConfig;

    fn deserialize(
        &self,
        config: BufferAppenderConfig,
        deserializers: &Deserializers,
    ) -> Result<Box<Append>, Box<std::error::Error + Sync + Send>> {
        let mut appender = BufferAppenderBuilder::default()
            .set_level(config.level)
            .set_hostname(&config.hostname)
            .set_port(config.port)
            .set_use_tls(config.use_tls)
            .set_null_character(config.null_character)
            .set_buffer_size(config.buffer_size)
            .set_buffer_duration(config.buffer_duration)
            .extend_additional_field(config.additional_fields)
            ;

        Ok(Box::new(appender.build()?))
    }
}

pub fn deserializers() -> Deserializers {
    let mut d = Deserializers::default();
    d.insert("buffer", BufferAppenderDeserializer);
    d
}