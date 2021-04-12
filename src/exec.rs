use std::process::{Command, Stdio};
use std::sync::Arc;
use std::thread;

pub use crate::cli::Cli;
pub use crate::LightmonEvent;

pub fn start(cli_args: Arc<Cli>) -> std::thread::JoinHandle<()> {
    thread::spawn(move || {
        debug!("thread started");

        // Build commands from exec commands
        for exec_command in &cli_args.exec_commands {
            // split into components
            let split: Vec<&str> = exec_command.split(' ').collect();
            let mut cmd = Command::new(split[0]);
            for argument in split.iter().skip(1) {
                cmd.arg(argument);
            }

            debug!("final cmd = {:?}", cmd);
            let output = cmd.stdout(Stdio::inherit()).output().unwrap();
            debug!("{:?}", output);
        }
    })
}
