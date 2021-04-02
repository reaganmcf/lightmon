#[macro_use]
extern crate clap;
mod cli;
mod watcher;

use cli::Cli;

fn main() {
  let cli_args: Cli = Cli::new();
  println!("lightmon started ({} mode)", cli_args.project_language);
  //println!("Parsed / configured args: {:?}", (cli_args.watch_patterns, cli_args.exec_command));
  
  watcher::start(cli_args);
  
}

