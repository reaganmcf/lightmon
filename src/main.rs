#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
extern crate notify;
mod cli;
mod exec;
mod watcher;

use cli::Cli;
use std::sync::mpsc::channel;
use std::sync::Arc;

pub enum LightmonEvent {
    InitExec,
    KillChild,
    KillAndRestartChild,
}

fn main() {
    let cli_args = Arc::new(Cli::new());

    // get ligthmon event channel
    let (lightmon_event_sender, lightmon_event_receiver) = channel();

    // Send first dummy event
    match lightmon_event_sender.send(LightmonEvent::InitExec) {
        Ok(()) => {}
        Err(_) => {
            panic!("failed to send initial lightmon event. Something went seriously wrong!");
        }
    };

    watcher::start(cli_args.clone(), lightmon_event_sender);

    println!("lightmon started ({} mode)", cli_args.project_language);

    // event thread
    loop {
        if let Ok(lightmon_event) = lightmon_event_receiver.recv() {
            match lightmon_event {
                LightmonEvent::KillAndRestartChild => {
                    debug!("KILL AND RESTART RECEIEVED");
                    exec::start(cli_args.clone());
                }
                LightmonEvent::KillChild => {
                    debug!("KILL RECEIEVED");
                }
                LightmonEvent::InitExec => {
                    debug!("INIT EXEC RECEIVED");
                    exec::start(cli_args.clone());
                }
            }
        }
    }
}
