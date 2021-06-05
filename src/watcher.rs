use notify::poll::PollWatcher;
use notify::{RecursiveMode, Watcher};
use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::Path;
use std::sync::Arc;
use std::{
    sync::mpsc::{channel, Sender},
    thread::JoinHandle,
};
use walkdir::WalkDir;

use crate::cli::Cli;
use crate::LightmonEvent;

// Start a new watcher thread that will send LightmonEvents back to the main thread.
// Returns a handle to the new thread
pub(crate) fn start(
    cli_args: Arc<Cli>,
    lightmon_event_sender: Sender<LightmonEvent>,
) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let (tx, rx) = channel();
        let mut watcher = PollWatcher::with_delay_ms(tx, 100).unwrap();

        let mut explicit_files_to_watch: HashSet<String> = HashSet::new();
        let mut file_types_to_watch: HashSet<String> = HashSet::new();

        for pattern in &cli_args.watch_patterns {
            // *.xxx pattern
            if pattern.starts_with("*.") {
                file_types_to_watch
                    .insert(pattern[pattern.find('.').unwrap() + 1..pattern.len()].to_string());
            } else {
                explicit_files_to_watch.insert(pattern.to_string());
            }
        }

        for entry in WalkDir::new(".")
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let mut should_watch: bool;
            // Check if the file is an explicit one we should watch
            should_watch = explicit_files_to_watch.contains(entry.path().to_str().unwrap());
            // Check if the file ends in a type we should watch
            let file_name = entry.file_name().to_str().unwrap();
            let file_ext = Path::new(file_name).extension().and_then(OsStr::to_str);
            if let Some(file_ext) = file_ext {
                should_watch = should_watch || file_types_to_watch.contains(file_ext);

                if should_watch {
                    debug!("Started watch on {:?}", entry.path().to_str().unwrap());
                    watcher
                        .watch(entry.path().to_str().unwrap(), RecursiveMode::NonRecursive)
                        .unwrap();
                }
            }
        }

        loop {
            debug!("checking events...");
            match rx.recv() {
                Ok(event) => {
                    println!("Changes detected, Restarting...");
                    debug!(
                        "changes detected {:?}\n Sending restart event to exec",
                        event
                    );
                    match lightmon_event_sender.send(LightmonEvent::KillAndRestartChild) {
                        Ok(_) => {}
                        Err(e) => error!("Failed to send event to exec thread! Reason: {:?}", e),
                    }
                }
                Err(e) => {
                    error!("Failed to receive event from channel {:?}", e);
                }
            }
        }
    })
}
