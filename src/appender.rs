use std::{fmt, thread};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, sync_channel, SyncSender};

use log4rs::append::Append;
use log::Record;
use serde_gelf::level::GelfLevel;
use serde_gelf::record::GelfRecord;

use batch::{BatchProcessor, Buffer, Event, Metronome, processor, set_boxed_processor};
use formatter::GelfFormatter;
use output::GelfTcpOutput;
use result::Error;

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

#[derive(Debug)]
pub struct BufferAppenderBuilder {
    level: GelfLevel,
    hostname: String,
    port: u64,
    use_tls: bool,
    null_character: bool,
    buffer_size: Option<usize>,
    buffer_duration: Option<u64>,
    additional_fields: BTreeMap<String, serde_value::Value>,
}

impl Default for BufferAppenderBuilder {
    fn default() -> BufferAppenderBuilder {
        BufferAppenderBuilder {
            level: GelfLevel::default(),
            hostname: "127.0.0.1".to_string(),
            port: 12202,
            use_tls: true,
            null_character: true,
            buffer_size: Some(100),
            buffer_duration: Some(500),
            additional_fields: {
                let mut additional_fields = BTreeMap::new();
                additional_fields.insert("pkg_name".into(), serde_value::Value::String(env!("CARGO_PKG_NAME").into()));
                additional_fields.insert("pkg_version".into(), serde_value::Value::String(env!("CARGO_PKG_VERSION").into()));
                additional_fields
            },
        }
    }
}


impl BufferAppenderBuilder {
    pub fn set_level(mut self, level: GelfLevel) -> BufferAppenderBuilder {
        self.level = level;
        self
    }
    pub fn set_hostname(mut self, hostname: &str) -> BufferAppenderBuilder {
        self.hostname = hostname.to_string();
        self
    }
    pub fn set_port(mut self, port: u64) -> BufferAppenderBuilder {
        self.port = port;
        self
    }
    pub fn set_use_tls(mut self, use_tls: bool) -> BufferAppenderBuilder {
        self.use_tls = use_tls;
        self
    }
    pub fn set_null_character(mut self, null_character: bool) -> BufferAppenderBuilder {
        self.null_character = null_character;
        self
    }
    pub fn set_buffer_size(mut self, buffer_size: Option<usize>) -> BufferAppenderBuilder {
        self.buffer_size = buffer_size;
        self
    }
    pub fn set_buffer_duration(mut self, buffer_duration: Option<u64>) -> BufferAppenderBuilder {
        self.buffer_duration = buffer_duration;
        self
    }
    pub fn put_additional_field(mut self, key: &str, value: serde_value::Value) -> BufferAppenderBuilder {
        self.additional_fields.insert(key.to_string(), value);
        self
    }
    pub fn extend_additional_field(mut self, additional_fields: BTreeMap<String, serde_value::Value>) -> BufferAppenderBuilder {
        self.additional_fields.extend(additional_fields);
        self
    }
    pub fn build(self) -> Result<BufferAppender, Error> {
        let (tx, rx): (SyncSender<Event>, Receiver<Event>) = sync_channel(10_000_000);

        if let Some(duration) = self.buffer_duration {
            let ctx = tx.clone();
            Metronome::new(duration).start(ctx);
        }

        let _ = set_boxed_processor(Box::new(BatchProcessor::new(tx, self.level)))?;
        let arx = Arc::new(Mutex::new(rx));
        thread::spawn(move || {
            let _ = Buffer::new(
                arx,
                GelfTcpOutput::new(
                    self.hostname,
                    self.port,
                    GelfFormatter::new(self.null_character, self.additional_fields),
                    self.use_tls,
                ),
            ).run();
        });


        Ok(BufferAppender {})
    }
}


pub struct BufferAppender;

impl BufferAppender {
    #[cfg(not(feature = "ovh-ldp"))]
    pub fn builder() -> BufferAppenderBuilder {
        BufferAppenderBuilder::default()
    }
    #[cfg(feature = "ovh-ldp")]
    pub fn builder(hostname: &str, token: &str) -> BufferAppenderBuilder {
        BufferAppenderBuilder::default()
            .set_hostname(hostname)
            .set_level(GelfLevel::Informational)
            .put_additional_field("X-OVH-TOKEN", serde_value::Value::String(token.to_string()))
    }
}

impl Drop for BufferAppender {
    fn drop(&mut self) {
        println!("Exiting, purging buffer...");
        let _ = processor().flush();
    }
}

impl fmt::Debug for BufferAppender {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("GelfAppender").finish()
    }
}

impl Append for BufferAppender {
    fn append(&self, record: &Record) -> Result<(), Box<std::error::Error + Sync + Send>> {
        match processor().send(&GelfRecord::from(record)) {
            Ok(()) => Ok(()),
            Err(exc) => Err(Box::new(exc))
        }
    }

    fn flush(&self) {}
}
