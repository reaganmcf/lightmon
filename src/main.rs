#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;

mod cli;
mod exec;
mod watcher;

use cli::Cli;
use std::sync::Arc;
use std::{
    process::Child,
    sync::mpsc::{channel, Receiver, Sender},
};

pub(crate) enum LightmonEvent {
    InitExec,
    KillAndRestartChild,
}

// Entry point for the entire binary.
fn main() {
    let cli_args = Arc::new(Cli::new());

    // get ligthmon event channel
    let (lightmon_event_sender, lightmon_event_receiver) = channel();

    // exec children references channel. Used to kill already running exec threads
    let (exec_child_process_sender, exec_child_process_receiver): (Sender<Child>, Receiver<Child>) =
        channel();

    // Send first dummy event
    match lightmon_event_sender.send(LightmonEvent::InitExec) {
        Ok(()) => {}
        Err(_) => {
            panic!("failed to send initial lightmon event. Something went seriously wrong!");
        }
    };

    watcher::start(cli_args.clone(), lightmon_event_sender.clone());

    println!("lightmon started ({} mode)", cli_args.project_language);

    // event thread
    loop {
        if let Ok(lightmon_event) = lightmon_event_receiver.recv() {
            match lightmon_event {
                LightmonEvent::KillAndRestartChild => {
                    debug!("KILL AND RESTART RECEIEVED");

                    // kill child
                    if let Ok(mut exec_child) = exec_child_process_receiver.recv() {
                        match exec_child.kill() {
                            Ok(_) => debug!("Killed child process."),
                            Err(e) => debug!("Failed to kill child process! {:?}", e),
                        }

                        // waiting after killing sounds weird because it is. But, the following is
                        // from the rust doc:
                        //
                        // On some systems, calling wait or similar is necessary for the OS to release resources.
                        // A process that terminated but has not been waited on is still around as a “zombie”.
                        // Leaving too many zombies around may exhaust global resources (for example process IDs).
                        // The standard library does not automatically wait on child processes (not even if the Child is dropped), it is up to the application developer to do so.
                        // As a consequence, dropping Child handles without waiting on them first is not recommended in long-running applications.
                        let _ = exec_child.try_wait();
                    }

                    // Restart
                    exec::start(
                        cli_args.clone(),
                        lightmon_event_sender.clone(),
                        exec_child_process_sender.clone(),
                    );
                }
                LightmonEvent::InitExec => {
                    debug!("INIT EXEC RECEIVED");
                    exec::start(
                        cli_args.clone(),
                        lightmon_event_sender.clone(),
                        exec_child_process_sender.clone(),
                    );
                }
            }
        }
    }
}
