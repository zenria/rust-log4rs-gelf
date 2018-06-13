use log4rs::append::Append;
use log::Record;
use std::error::Error;
use std::io;
use std::net::SocketAddr;


#[derive(Deserialize)]
pub struct TcpSocketAppenderConfig {
    hosts: Vec<String>
}

pub struct TcpSocketAppenderBuilder {
    addrs: Vec<SocketAddr>
}


impl TcpSocketAppenderBuilder {
    pub fn build(self) -> io::Result<TcpSocketAppender> {
        Ok(TcpSocketAppender {})
    }
    pub fn address(mut self, address: SocketAddr) -> TcpSocketAppenderBuilder {
        self.addrs.push(address);
        self
    }
}

#[derive(Debug)]
pub struct TcpSocketAppender;

impl TcpSocketAppender {
    pub fn builder() -> TcpSocketAppenderBuilder {
        TcpSocketAppenderBuilder { addrs: vec![] }
    }
}

impl Append for TcpSocketAppender {
    fn append(&self, record: &Record) -> Result<(), Box<Error + Sync + Send>> {
        unimplemented!()
    }

    fn flush(&self) {
        unimplemented!()
    }
}
