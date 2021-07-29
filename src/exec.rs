// Contains the method for starting a thread that will run the exec commands in parallel.

use colored::*;
use std::sync::{mpsc::Sender, Arc};
use std::{
    io,
    process::{Child, Command},
    thread,
};

use crate::cli::Cli;
use crate::LightmonEvent;

// Start an exec thread that will run the exec commands
//
// Returns a handler to the thread
pub(crate) fn start(
    cli_args: Arc<Cli>,
    lightmon_event_sender: Sender<LightmonEvent>,
    exec_child_process_sender: Sender<Child>,
) -> std::thread::JoinHandle<()> {
    thread::spawn(move || {
        // Build commands from exec commands
        for exec_command in &cli_args.exec_commands {
            // split into components
            let split: Vec<&str> = exec_command.split(' ').collect();
            let mut cmd = Command::new(split[0]);
            for argument in split.iter().skip(1) {
                cmd.arg(argument);
            }

            println!(
                "{}",
                format!(
                    "[lightmon] starting `{}`",
                    &cli_args.exec_commands.join(" ")
                )
                .green()
            );

            let child = cmd.spawn().expect("Unable to spawn process");

            exec_child_process_sender
                .send(child)
                .expect("Unable to send event to main loop. Something seriously went wrong!");

            let mut input = String::new();
            loop {
                if io::stdin().read_line(&mut input).is_ok() && input.eq("rs\n") {
                    lightmon_event_sender.send(LightmonEvent::KillAndRestartChild).expect("Unable to kill and restart the process. Something seriously went wrong!");
                }
            }
        }
    })
}
