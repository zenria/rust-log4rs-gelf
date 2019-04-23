# log4rs_gelf [![Build Status](https://travis-ci.org/ovh/log4rs_gelf.svg?branch=master)](https://travis-ci.org/ovh/log4rs_gelf) [![Latest version](https://img.shields.io/crates/v/log4rs_gelf.svg)](https://crates.io/crates/log4rs_gelf) [![Documentation](https://docs.rs/log4rs_gelf/badge.svg)](https://docs.rs/log4rs_gelf) ![License](https://img.shields.io/crates/l/log4rs_gelf.svg)

`log4rs_gelf` - A TCP/Gelf appender for [log4rs](https://github.com/sfackler/log4rs) based on 
[serde_gelf](https://github.com/ovh/serde_gelf) and [gelf_logger](https://github.com/ovh/gelf_logger).


## Examples

Configuration via a YAML file:

```yaml
appenders:
  ldp:
    additional_fields:
      component: rust-cs
    buffer_duration: 5
    buffer_size: 5
    hostname: 127.0.0.1
    kind: buffer
    level: Informational
    null_character: true
    port: 12202
    use_tls: false
root:
  appenders:
  - ldp
  level: info
```

```rust,no_run
    log4rs_gelf::init_file("/tmp/log4rs.yml", None).unwrap();
```

Programmatically constructing a configuration:

```rust,no_run
use serde_gelf::GelfLevel;
use serde_value::Value;
use log4rs::config::{Config, Appender, Root};
use log::LevelFilter;

fn main() {
   let buffer = log4rs_gelf::BufferAppender::builder()
       .set_level(GelfLevel::Informational)
       .set_hostname("localhost")
       .set_port(12202)
       .set_use_tls(false)
       .set_null_character(true)
       .set_buffer_size(Some(5))
       .set_buffer_duration(Some(5))
       .put_additional_field("component", Value::String("rust-cs".to_string()))
       .build()
       .unwrap();

   let config = Config::builder()
       .appender(Appender::builder().build("gelf", Box::new(buffer)))
       .build(Root::builder().appender("gelf").build(LevelFilter::Info))
       .unwrap();

   log4rs_gelf::init_config(config).unwrap();

   // Do whatever

   log4rs_gelf::flush().expect("Failed to send buffer, log records can be lost !");
}
```

## OVH Log Data Platform

You can activate the OVH [LDP](https://docs.ovh.com/gb/en/logs-data-platform/) 
feature including field typing and an preconfigured handler:

```toml
[dependencies]
log4rs_gelf = { git = "https://github.com/ovh/log4rs_gelf", features = ["ovh-ldp"] }
```

And then build the appender:

```rust,no_run
let buffer = BufferAppender::builder("gra1.logs.ovh.com","XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX")
    .put_additional_field("component", Value::String("rust-cs".to_string()))
    .build()
    .unwrap();
```

## License

Licensed under [BSD 3-Clause License](./LICENSE) or (https://opensource.org/licenses/BSD-3-Clause)