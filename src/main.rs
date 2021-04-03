#[macro_use]
extern crate clap;
extern crate notify;
mod cli;
mod watcher;
mod exec;

use cli::Cli;
use notify::DebouncedEvent;
use std::{sync::mpsc::{Receiver, channel}};

fn main() {
  let cli_args: Cli = Cli::new();
  
  // get notify channels
  let (tx, rx) = channel();

  let watch_thread = watcher::start(cli_args.watch_patterns, tx.clone());
  let exec_thread = exec::start(cli_args.exec_command, rx);
  
  println!("lightmon started ({} mode)", cli_args.project_language);

  watch_thread.join();
  exec_thread.join();

}

