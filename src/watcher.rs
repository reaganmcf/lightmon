extern crate notify;
extern crate walkdir;

use notify::{Watcher, RecursiveMode, watcher};
use std::{process::Command, sync::mpsc::channel};
use std::time::Duration;
use walkdir::WalkDir;
use std::collections::HashSet;

pub use crate::cli::Cli; 

pub fn start(cli_args: Cli) {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();

    let mut explicit_files_to_watch: HashSet<String> = HashSet::new();
    let mut file_types_to_watch: HashSet<String> = HashSet::new();
  
    for pattern in cli_args.watch_patterns {
      // *.xxx pattern
      if pattern.starts_with("*.") {
        file_types_to_watch.insert(pattern[pattern.find(".").unwrap()..pattern.len()].to_string());
      } else {
        explicit_files_to_watch.insert(pattern.to_string());
      }
    }

    for entry in WalkDir::new(".").follow_links(true).into_iter().filter_map(|e| e.ok()) {
      let mut should_watch: bool;
      // Check if the file is an explicit one we should watch
      should_watch = explicit_files_to_watch.contains(entry.path().to_str().unwrap());
      // Check if the file ends in a type we should watch
      let f_name = entry.file_name().to_string_lossy();
      should_watch = should_watch || f_name.ends_with(".rs");
      
      if should_watch {
        println!("Started watch on {:?}", entry.path().to_str().unwrap());
        watcher.watch(entry.path().to_str().unwrap(),RecursiveMode::NonRecursive).unwrap();
      }
    }

    // TODO use cli args instead
    let mut exec_cmd = Command::new("cargo");
    exec_cmd.arg("build");
    
    loop {
      match rx.recv() {
       Ok(event) => {
        println!("Changes detected... Restarting using {:?}", exec_cmd);
        let res = exec_cmd.spawn();
        if let Ok(child) = res {
          println!("Started child {:?}", child);
        } else {
          println!("FAILED {:?}", res.err());
        }
       },
       Err(e) => {
         println!("watch error: {:?}", e);
       }
      }
    }
}
