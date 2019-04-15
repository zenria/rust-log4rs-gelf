use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread;
use std::time::Duration;

use serde_gelf::level::GelfLevel;
use serde_gelf::record::{GelfRecord, GelfRecordGetter};

use crate::result::Result;
use result::Error;
use output::GelfTcpOutput;

static mut BATCH_PROCESSOR: &'static Batch = &NoProcessor;

#[derive(Clone, Debug)]
pub enum Event {
    Send,
    Data(GelfRecord),
}


#[derive(Clone, Debug)]
pub struct Metronome {
    frequency: u64,
}


impl Metronome {
    pub fn new(frequency: u64) -> Metronome {
        Metronome { frequency }
    }
    pub fn start(&self, chan: SyncSender<Event>) {
        let frequency = self.frequency;
        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(frequency));
            let _ = chan.send(Event::Send);
        });
    }
}


#[derive(Debug)]
pub struct Buffer {
    items: Vec<GelfRecord>,
    arx: Arc<Mutex<Receiver<Event>>>,
    errors: Vec<Error>,
    output: GelfTcpOutput,
}

impl Buffer {
    pub fn new(arx: Arc<Mutex<Receiver<Event>>>, output: GelfTcpOutput) -> Buffer {
        Buffer { items: Vec::new(), arx, errors: Vec::new(), output }
    }
    pub fn run(&mut self) {
        loop {
            match { self.arx.lock().unwrap().recv() } {
                Ok(event) => {
                    match event {
                        Event::Send => match self.output.send(&self.items) {
                            Ok(_) => self.items.clear(),
                            Err(exc) => {
                                self.errors.push(exc);
                                if self.errors.len() >= 5 {
                                    println!("Too many errors !");
                                    for err in self.errors.iter() {
                                        println!("{:?}", err);
                                    }
                                    std::process::exit(0x0100);
                                }
                                thread::sleep(Duration::from_millis(100));
                            }
                        },
                        Event::Data(record) => self.items.push(record),
                    }
                }
                Err(_) => return,
            };
        }
    }
}


pub fn set_boxed_processor(processor: Box<Batch>) -> Result<()> {
    set_processor_inner(|| unsafe { &*Box::into_raw(processor) })
}

fn set_processor_inner<F>(make_processor: F) -> Result<()> where F: FnOnce() -> &'static Batch {
    unsafe {
        BATCH_PROCESSOR = make_processor();
        Ok(())
    }
}


pub trait Batch {
    fn send(&self, rec: &GelfRecord) -> Result<()>;
    fn flush(&self) -> Result<()>;
}


pub struct NoProcessor;

impl Batch for NoProcessor {
    fn send(&self, _rec: &GelfRecord) -> Result<()> { Ok(()) }
    fn flush(&self) -> Result<()> { Ok(()) }
}


pub struct BatchProcessor {
    tx: SyncSender<Event>,
    level: GelfLevel,
}

impl BatchProcessor {
    pub fn new(tx: SyncSender<Event>, level: GelfLevel) -> BatchProcessor {
        BatchProcessor { tx, level }
    }
}

impl Batch for BatchProcessor {
    fn send(&self, rec: &GelfRecord) -> Result<()> {
        if self.level >= rec.level() {
            self.tx.send(Event::Data(rec.clone()))?;
        }
        Ok(())
    }
    fn flush(&self) -> Result<()> {
        let _ = self.tx.send(Event::Send)?;
        Ok(thread::sleep(Duration::from_secs(2)))
    }
}



pub fn processor() -> &'static Batch { unsafe { BATCH_PROCESSOR } }
