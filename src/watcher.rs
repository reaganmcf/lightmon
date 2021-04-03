extern crate notify;
extern crate walkdir;

use notify::{DebouncedEvent, RecursiveMode, Watcher, watcher};
use std::{sync::mpsc::{Sender}, thread::{JoinHandle}};
use std::time::Duration;
use walkdir::WalkDir;
use std::collections::HashSet;

pub use crate::cli::Cli;

pub fn start(watch_patterns: Vec<String>, tx: Sender<DebouncedEvent>) -> JoinHandle<()> {
  // Create a watcher object, delivering debounced events.
  // The notification back-end is selected based on the platform.
  let watch_thread = std::thread::spawn(move|| {
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    let mut explicit_files_to_watch: HashSet<String> = HashSet::new();
    let mut file_types_to_watch: HashSet<String> = HashSet::new();

    for pattern in watch_patterns {
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

    loop {
      
    }
  });

  watch_thread
}
