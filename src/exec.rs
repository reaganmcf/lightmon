use std::{thread};
use std::process::{Command, Stdio};

pub use crate::cli::Cli;
pub use crate::LightmonEvent;

pub fn start(exec_commands: Vec<String>) -> std::thread::JoinHandle<()> {
  let exec_thread = thread::spawn(move|| {
    debug!("thread started");

    // Build commands from exec commands
    for exec_command in exec_commands.into_iter() {
      // split into components
      let split: Vec<&str> = exec_command.split(" ").collect();
      let mut cmd = Command::new(split[0]);
      for i in 1..split.len() {
        cmd.arg(split[i]);
      }

      debug!("final cmd = {:?}", cmd);
      let output = cmd.stdout(Stdio::inherit()).output().unwrap();
      debug!("{:?}", output);
    }
    
  }); 
  return exec_thread;
}

