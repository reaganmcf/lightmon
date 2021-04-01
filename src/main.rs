#[macro_use]
extern crate clap;
mod cli;

use cli::Cli;

fn main() {
  let cli_args: Cli = Cli::new();
  println!("lightmon started ({} mode)", cli_args.project_language);
  println!("Parsed / configured args: {:?}", (cli_args.watch_patterns, cli_args.exec_command, cli_args.entry_file));
 
}

