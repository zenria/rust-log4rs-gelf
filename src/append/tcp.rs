//use append::StreamService;
use builder::Builder;
use log4rs::append::Append;
use std::io::{self, Write};
use std::net::{SocketAddr, TcpStream};
use std::thread;
use std::sync::mpsc::{Sender, channel};
use log4rs::encode::Encode;
use log::Record;
use error::Error;
use std::error::Error as stdError;
use log4rs::encode::pattern::PatternEncoder;
use std::sync::{Arc, Mutex};
use log4rs::encode::Write as log4rsWrite;
use std::io::ErrorKind;
use std::ops::DerefMut;

#[derive(Debug)]
struct ThreadWriter {
    sender: Sender<Vec<u8>>
}

impl Write for ThreadWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        match self.sender.send(Vec::from(buf)) {
            Ok(_) => Ok(1),
            Err(error) => Err(io::Error::new(ErrorKind::Other, error))
        }
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}

impl log4rsWrite for ThreadWriter {}

#[derive(Debug)]
pub struct TCPAppender {
    encoder: Box<Encode>,
    tx: Arc<Mutex<ThreadWriter>>,
}

#[derive(Debug)]
pub struct TCPAppenderBuilder {
    hosts: Vec<SocketAddr>,
    batch_size: usize,
    max_cache_size: usize,
    encoder: Option<Box<Encode>>,
}


impl TCPAppender {
    pub fn builder() -> TCPAppenderBuilder { TCPAppenderBuilder::new() }
    pub fn network_send(cache: &mut Vec<Vec<u8>>, hosts: &Vec<SocketAddr>) -> Result<(), io::Error> {
        let mut socket = TcpStream::connect(&hosts[..])?;
        println!("Pushing {} message(s)", cache.len());
        for item in cache.iter() {
            socket.write(&item)?;
        }
        Ok(cache.clear())
    }
}

impl TCPAppenderBuilder {
    pub fn host(mut self, host: &str) -> TCPAppenderBuilder {
        if let Ok(addr) = host.parse() {
            self.hosts.push(addr);
        }
        self
    }
    pub fn hosts(mut self, hosts: Vec<&str>) -> TCPAppenderBuilder {
        for host in hosts {
            if let Ok(addr) = host.parse() {
                self.hosts.push(addr);
            }
        }
        self
    }
    pub fn batch_size(mut self, batch_size: usize) -> TCPAppenderBuilder {
        self.batch_size = batch_size;
        self
    }
    pub fn max_cache_size(mut self, max_cache_size: usize) -> TCPAppenderBuilder {
        self.max_cache_size = max_cache_size;
        self
    }
    pub fn encoder(mut self, encoder: Box<Encode>) -> TCPAppenderBuilder {
        self.encoder = Some(encoder);
        self
    }
}

impl Builder for TCPAppenderBuilder {
    type TargetItem = TCPAppender;

    fn new() -> TCPAppenderBuilder {
        TCPAppenderBuilder { hosts: Vec::new(), batch_size: 1000, max_cache_size: 10000, encoder: None }
    }
    fn build(self) -> Result<TCPAppender, Error> {
        match self.hosts.len() {
            0 => Err(Error::InvalidConfiguration("No host set!".to_string())),
            _ => {
                let (tx, rx) = channel::<Vec<u8>>();
                let appender = TCPAppender {
                    encoder: self.encoder.unwrap_or_else(|| Box::new(PatternEncoder::default())),
                    tx: Arc::new(Mutex::new(ThreadWriter { sender: tx })),
                };

                let trx = Arc::new(Mutex::new(rx));
                let batch_size = self.batch_size.clone();
                let hosts = self.hosts.clone();
                let max_cache_size = self.max_cache_size.clone();
                thread::spawn(move || {
                    let mut cache: Vec<Vec<u8>> = Vec::new();
                    loop {
                        let record = match { trx.lock().unwrap().recv() } {
                            Ok(line) => line,
                            Err(_) => return,
                        };
                        match cache.len() >= max_cache_size {
                            true => println!("Error: cache is full !!!"),
                            false => {
                                cache.push(record.clone());
                                if cache.len() >= batch_size {
                                    if let Err(e) = TCPAppender::network_send(&mut cache, &hosts) {
                                        println!("Error: {}, {} object(s) in cache", e.description(), cache.len());
                                    }
                                }
                            }
                        };
                    }
                });
                Ok(appender)
            }
        }
    }
}

impl Append for TCPAppender {
    fn append(&self, record: &Record) -> Result<(), Box<stdError + Sync + Send>> {
        if let Ok(mut pipe) = self.tx.lock() {
            self.encoder.encode(pipe.deref_mut(), record)?;
        }
        Ok(())
    }
    fn flush(&self) {}
}