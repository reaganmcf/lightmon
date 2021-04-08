use std::{thread};
use std::process::{Command, Stdio};
use std::sync::Arc;

pub use crate::cli::Cli;
pub use crate::LightmonEvent;

pub fn start(cli_args: Arc<Cli>) -> std::thread::JoinHandle<()> {
  let exec_thread = thread::spawn(move|| {
    debug!("thread started");

    // Build commands from exec commands
    for exec_command in &cli_args.exec_commands {
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

