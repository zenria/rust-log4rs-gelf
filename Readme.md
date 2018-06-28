# log4rs-gelf

`log4rs-gelf` - very simple TCP/Gelf appender for [log4rs](https://github.com/sfackler/log4rs) based on 
[serde_json](https://github.com/serde-rs/json).

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
log4rs_gelf = "0.1"
```

Example code:

```rust,no_run
#[deny(warnings)]
extern crate log4rs;
extern crate serde_json;
extern crate log4rs_gelf;

#[macro_use]
extern crate log;

use log4rs::config::{Config, Appender, Root};
use log4rs_gelf::encode::gelf::GelfEncoder;
use log4rs_gelf::append::tcp::TCPAppender;
use log4rs_gelf::builder::Builder;
use log::LevelFilter;
use std::{thread, time};
use serde_json::Value;


fn main() {
    let gelf = GelfEncoder::builder()
        .null_character(true)
        .add_field("MyCustomField", Value::from("75874f9c-d4f9-45bd-a5fc-9a1ca201f70e"))
        .build().unwrap();

    let gelf_tcp_input = TCPAppender::builder()
        .encoder(Box::new(gelf))
        .hosts(vec!["127.0.0.1:7000", "192.168.0.1:12202"])
        .max_cache_size(10)
        .batch_size(5)
        .build().unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("gelf_tcp", Box::new(gelf_tcp_input)))
        .build(Root::builder().appender("gelf_tcp").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
    for idx in 0..10 {
        info!("Test {}", idx)
    }

    // We wait a little to exit...
    thread::sleep(time::Duration::from_secs(3));
}
```

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)
