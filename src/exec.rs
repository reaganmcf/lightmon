use std::{thread};
use std::process::{Command, Stdio};

pub use crate::cli::Cli;
pub use crate::LightmonEvent;

pub fn start(exec_command: String) -> std::thread::JoinHandle<()> {
  let exec_thread = thread::spawn(move|| {
    println!("thread started");
    let cmd = Command::new("echo").arg("test output from cmd").stdout(Stdio::inherit()).output().unwrap();
    println!("{:?}", cmd);
    
  }); 
  return exec_thread;
}

