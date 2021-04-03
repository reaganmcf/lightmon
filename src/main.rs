#[macro_use]
extern crate clap;
extern crate notify;
mod cli;
mod watcher;
mod exec;

use cli::Cli;
use std::sync::mpsc::channel;

pub enum LightmonEvent {
  KillChild,
  KillAndRestartChild
}

fn main() {
  let cli_args: Cli = Cli::new();
  
  // get kill_exec channel
  let (kill_exec_sender, kill_exec_receiver) = channel();

  watcher::start(cli_args.watch_patterns, kill_exec_sender);
  
  println!("lightmon started ({} mode)", cli_args.project_language);

  loop {
    if let Ok(kill_exec_receiever) = kill_exec_receiver.recv() {
      match kill_exec_receiever {
        LightmonEvent::KillAndRestartChild => {
          println!("KILL AND RESTART RECEIEVED");
          exec::start(cli_args.exec_command.clone());
        },
        LightmonEvent::KillChild => {
          println!("KILL RECEIEVED");
        }
      }
    }
  }


}

