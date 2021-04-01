#[macro_use]
extern crate clap;
mod cli;

use cli::Cli;

fn main() {
  let cli_args: Cli = Cli::new();

  println!("Using watch pattern {}", cli_args.watch_pattern);
}

