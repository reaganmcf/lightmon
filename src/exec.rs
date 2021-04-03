extern crate notify;
use notify::DebouncedEvent;
use std::{sync::mpsc::Receiver, thread};

pub use crate::cli::Cli;

pub fn start(exec_command: String, rx: Receiver<DebouncedEvent>) -> std::thread::JoinHandle<()> {
  let exec_thread = thread::spawn(move|| {
    println!("thread started");
    loop {
      println!("checking events...");
      match rx.recv() {
        Ok(event) => {
          println!("changes detected {:?}", event);
        },
        Err(e) => {
          println!("err {:?}", e);
        }
      }
    }
  }); 
  return exec_thread;
}


