#[macro_use]
extern crate log;
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
  
  // get ligthmon event channel
  let (lightmon_event_sender, lightmon_event_receiver) = channel();

  watcher::start(cli_args.watch_patterns, lightmon_event_sender);
  
  println!("lightmon started ({} mode)", cli_args.project_language);

  loop {
    if let Ok(lightmon_event) = lightmon_event_receiver.recv() {
      match lightmon_event {
        LightmonEvent::KillAndRestartChild => {
          debug!("KILL AND RESTART RECEIEVED");
          exec::start(cli_args.exec_command.clone());
        },
        LightmonEvent::KillChild => {
          debug!("KILL RECEIEVED");
        }
      }
    }
  }


}

