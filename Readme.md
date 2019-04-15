# log4rs-gelf

`log4rs-gelf` - very a TCP/Gelf appender for [log4rs](https://github.com/sfackler/log4rs) based on 
[serde_gelf](https://github.com/cdumay/rust-serde_gelf).

**Work in progress, for testing only !**

## Examples

Configuration using a YAML file:

```yaml
appenders:
  gelf:
    kind: buffer
    level: "Informational"
    hostname: "localhost"
    port: 12202
    use_tls: false
    null_character: true
    buffer_size: 5
    buffer_duration: 5
    additional_fields:
      component: "rust-cs"
root:
  level: info
  appenders:
    - gelf
```

In code:

```rust,no_run
log4rs_gelf::init_file("log4rs.yml", None).unwrap();
```

Programmatically constructing a configuration:


```rust,no_run
#[macro_use]
extern crate log4rs_gelf;
#[macro_use]
extern crate serde_gelf;
extern crate log4rs;
extern crate log;

use log4rs::config::{Config, Appender, Root};
use log::LevelFilter;
use serde_value::Value;
use std::collections::btree_map::BTreeMap;
use log4rs_gelf::appender::BufferAppender;

fn main() {
    let buffer = BufferAppender::builder()
        .set_level(serde_gelf::level::GelfLevel::Informational)
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
        .build(Root::builder().appender("stdout").appender("gelf").build(LevelFilter::Info))
        .unwrap();
    let handle = log4rs_gelf::init_config(config).unwrap();
    
    // main code
    
    // before exiting, make sure to flush pending buffered log entries
    log4rs_gelf::flush().unwrap();
}
```

## OVH Log Data Platform

You can activate the OVH LDP feature including field typing and an pre-configured handler:

```toml
[dependencies]
log4rs_gelf = { git = "https://github.com/cdumay/log4rs-gelf", features = ["ovh-ldp"] }
```

And then build the appender:

```rust,no_run
let buffer = BufferAppender::builder("gra1.logs.ovh.com","XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX")
    .put_additional_field("component", Value::String("rust-cs".to_string()))
    .build()
    .unwrap();
```

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)
