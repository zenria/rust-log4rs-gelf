// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.
// Copyright 2009 The log4rs-gelf Authors. All rights reserved.

use std::collections::BTreeMap;
use std::fmt;

use gelf_logger::{Batch, BatchProcessor, Config};
use log::Record;
use log4rs::append::Append;
use serde_gelf::{GelfLevel, GelfRecord};

/// Struct to handle the GELF buffer.
///
/// ## Example
///
/// ```rust
/// use serde_gelf::GelfLevel;
/// use serde_value::Value;
///
/// fn main() {
///     let appender = log4rs_gelf::BufferAppender::builder()
///         .set_level(GelfLevel::Informational)
///         .set_hostname("localhost")
///         .set_port(12202)
///         .set_use_tls(false)
///         .set_null_character(true)
///         .set_buffer_size(Some(5))
///         .set_buffer_duration(Some(5))
///         .put_additional_field("component", Value::String("rust-cs".to_string()))
///         .build()
///         .expect("Failed to create appender")
///         ;
/// }
/// ```
pub struct BufferAppender {
    processor: BatchProcessor,
}

/// Builder for [`BufferAppender`](struct.BufferAppender.html).
///
/// ## Example
///
/// ```rust
/// use serde_gelf::GelfLevel;
/// use serde_value::Value;
///
/// fn main() {
///     let builder = log4rs_gelf::BufferAppenderBuilder::default()
///         .set_level(GelfLevel::Informational)
///         .set_hostname("localhost")
///         .set_port(12202)
///         .set_use_tls(false)
///         .set_null_character(true)
///         .set_buffer_size(Some(5))
///         .set_buffer_duration(Some(5))
///         .put_additional_field("component", Value::String("rust-cs".to_string()))
///         ;
/// }
/// ```

#[derive(Debug)]
pub struct BufferAppenderBuilder {
    level: GelfLevel,
    hostname: String,
    port: u64,
    use_tls: bool,
    null_character: bool,
    buffer_size: Option<usize>,
    buffer_duration: Option<u64>,
    additional_fields: BTreeMap<serde_value::Value, serde_value::Value>,
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
                additional_fields.insert(
                    serde_value::Value::String("pkg_name".into()),
                    serde_value::Value::String(env!("CARGO_PKG_NAME").into()),
                );
                additional_fields.insert(
                    serde_value::Value::String("pkg_version".into()),
                    serde_value::Value::String(env!("CARGO_PKG_VERSION").into()),
                );
                additional_fields
            },
        }
    }
}

impl BufferAppenderBuilder {
    /// Sets threshold for this logger to level. Logging messages which are less severe than level
    /// will be ignored.
    pub fn set_level(mut self, level: GelfLevel) -> BufferAppenderBuilder {
        self.level = level;
        self
    }
    /// Sets the hostname of the remote server.
    pub fn set_hostname(mut self, hostname: &str) -> BufferAppenderBuilder {
        self.hostname = hostname.to_string();
        self
    }
    /// Sets the port of the remote server.
    pub fn set_port(mut self, port: u64) -> BufferAppenderBuilder {
        self.port = port;
        self
    }
    /// Activate transport security.
    pub fn set_use_tls(mut self, use_tls: bool) -> BufferAppenderBuilder {
        self.use_tls = use_tls;
        self
    }
    /// Adds a NUL byte (`\0`) after each entry.
    pub fn set_null_character(mut self, null_character: bool) -> BufferAppenderBuilder {
        self.null_character = null_character;
        self
    }
    /// Sets the upperbound limit on the number of records that can be placed in the buffer, once
    /// this size has been reached, the buffer will be sent to the remote server.
    pub fn set_buffer_size(mut self, buffer_size: Option<usize>) -> BufferAppenderBuilder {
        self.buffer_size = buffer_size;
        self
    }
    /// Sets the maximum lifetime (in milli seconds) of the buffer before send it to the remote
    /// server.
    pub fn set_buffer_duration(mut self, buffer_duration: Option<u64>) -> BufferAppenderBuilder {
        self.buffer_duration = buffer_duration;
        self
    }
    /// Adds an additional data which will be append to each log entry.
    pub fn put_additional_field(
        mut self,
        key: &str,
        value: serde_value::Value,
    ) -> BufferAppenderBuilder {
        self.additional_fields
            .insert(serde_value::Value::String(key.into()), value);
        self
    }
    /// Adds multiple additional data which will be append to each log entry.
    pub fn extend_additional_field(
        mut self,
        additional_fields: BTreeMap<serde_value::Value, serde_value::Value>,
    ) -> BufferAppenderBuilder {
        self.additional_fields.extend(additional_fields);
        self
    }
    /// Invoke the builder and return a [`BufferAppender`](struct.BufferAppender.html).
    pub fn build(self) -> Result<BufferAppender, gelf_logger::Error> {
        let config = Config::builder()
            .set_level(self.level)
            .set_hostname(self.hostname)
            .set_port(self.port)
            .set_use_tls(self.use_tls)
            .set_null_character(self.null_character)
            .set_buffer_size(self.buffer_size.unwrap_or(100))
            .set_buffer_duration(self.buffer_duration.unwrap_or(500))
            .extend_additional_fields(self.additional_fields)
            .build();

        Ok(BufferAppender {
            processor: gelf_logger::init_processor(&config)?,
        })
    }
}

impl BufferAppender {
    /// Creates a new [`BufferAppenderBuilder`](struct.BufferAppenderBuilder.html).
    #[cfg(not(feature = "ovh-ldp"))]
    pub fn builder() -> BufferAppenderBuilder {
        BufferAppenderBuilder::default()
    }
    /// Creates a new [`BufferAppenderBuilder`](struct.BufferAppenderBuilder.html) preconfigured for OVH [LDP](https://docs.ovh.com/gb/en/logs-data-platform/).
    #[cfg(feature = "ovh-ldp")]
    pub fn builder(hostname: &str, token: &str) -> BufferAppenderBuilder {
        BufferAppenderBuilder::default()
            .set_hostname(hostname)
            .set_level(GelfLevel::Informational)
            .put_additional_field("X-OVH-TOKEN", serde_value::Value::String(token.to_string()))
    }
}

impl fmt::Debug for BufferAppender {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("GelfAppender").finish()
    }
}

impl Append for BufferAppender {
    fn append(&self, record: &Record) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        Ok(self.processor.send(&GelfRecord::from(record))?)
    }

    fn flush(&self) {}
}
