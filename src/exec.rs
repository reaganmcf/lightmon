use std::io::{self};
use std::process::Command;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;

pub use crate::cli::Cli;
pub use crate::LightmonEvent;

pub fn start(
    cli_args: Arc<Cli>,
    lightmon_event_sender: Sender<LightmonEvent>,
) -> std::thread::JoinHandle<()> {
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
            let mut child = cmd.spawn().unwrap();
            loop {
                let mut input = String::new();
                if let Ok(n) = io::stdin().read_line(&mut input) {
                    if input.eq("rs\n") {
                        debug!("rs RECEIEVED");
                        match lightmon_event_sender.send(LightmonEvent::KillAndRestartChild) {
                            Ok(()) => {
                                let child_killed = child.kill();
                                match child_killed {
                                    Ok(()) => {}
                                    Err(e) => {
                                        error!("program cannot be quit and restarted: {:?}", e);
                                        std::process::exit(1);
                                    }
                                }
                            }
                            Err(_) => {
                                panic!("failed to send initial lightmon event. Something went seriously wrong!");
                            }
                        };
                    } else {
                        debug!("unknown input, bits read from input {:?}", n);
                        debug!("input = {:?}", input);
                    }
                }
            }
        }
    })
}
