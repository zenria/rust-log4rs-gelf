extern crate log4rs;
extern crate serde;

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

pub mod append;

//#[test]
//fn test_env() {
//    log4rs::init_file(env!("LDP_LOG_FILE"), Default::default()).unwrap();
//    warn!("test!")
//}

#[test]
fn test_gelf() {
    use append::tcp::TcpSocketAppender;

    let socket = TcpSocketAppender::builder()
        .address("127.0.0.1:8080".parse().unwrap())
        .build();
}
