//! <p align="center">
//!   <img height="250px" src="https://raw.githubusercontent.com/reaganmcf/lightmon/master/assets/logo.png"/>
//! </p>
//!
//! # lightmon
//! A lightweight, cross-platform, language-agnostic "run code on file change" tool, inspired by Nodemon
//! <p align="left">
//!  <img src="https://img.shields.io/static/v1?label=status&message=In%20Development&color=critical"/>
//!  <img src="https://img.shields.io/crates/v/lightmon"/>
//!  <img src="https://github.com/reaganmcf/lightmon/actions/workflows/ci.yml/badge.svg"/>
//!  <img src="https://shields.io/github/license/reaganmcf/lightmon"/>
//! </p>
//!
//! ###  Why lightmon over nodemon?
//! There are many reasons to use lightmon over nodemon: __it's faster, lighter, and can be used for all types of projects__. Not only this, but lightmon is a **drag and drop  replacement** for projects that use `nodemon` because lightman can parse existing `nodemon.json` config files.
//! - Note: [Parse nodemon.json config is still WIP](https://github.com/reaganmcf/lightmon/issues/3)
//!
//! ## Usage
//! ```
//! lightmon
//! ```
//! By default, `lightmon` will automatically determine what kind of files it should watch based upon your project structure. For example, if a `node_modules` folder is present in the directory, `lightmon` will run in the `node` configuration, parsing your `package.json` to infer the correct command to run.
//!
//! ## Supported languages
//!
//! Watch patterns are the file patterns that lightmon will watch for file changes, and Exec commands are the list of commands that are executed when those events happen.
//!
//! ### Rust
//! ```
//! lightmon rust [cargo_subcommand]?
//! ```
//!
//! ##### Watch Patterns
//! [`Cargo.toml`, `.rs`]
//!
//! ##### Exec Commands
//! By default, the `rust` configuration will set the Exec command to `cargo run` if it's a binary project, and `cargo test` if it's a library.
//!
//! However, you can override this behavior by specifying the subcommand manually. For example, you want to do `cargo test` on a binary project instead of `cargo run` on file change events, you should do the following:
//! ```
//! lightmon rust test
//! ```
//!
//! Refer to `lightmon help rust` for more information.
//! ### Node.js
//! **Note: This configuration also works for React, React-Native, TypeScript, etc. Anything with a package.json!**
//!
//! ```
//! lightmon node
//! ```
//!
//! ##### Watch Patterns
//!
//! [`.jsx`, `.js`, `.css`, `.html`]
//! ##### Exec Commands
//!
//! If there is a package.json in the root directory, lightmon attempts to resolve the exec command in the following order:
//!
//! - The value at `scripts.start`
//! - `node main` where main is the value of the main key in package.json (the entry point of the project).
//!
//! **NOTE:** The Exec command will fallback to `node index.js` if all of the above fail.
//!
//! For example, the following package.json will result in the Exec command resolving to `react-scripts start`:
//! ```json
//! {
//!     "name": "calculator",
//!     "main": "index.js",
//!     "scripts": {
//!         "start": "react-scripts start",
//!         "build": "react-scripts build"
//!     }
//! }
//! ```
//!
//! ### C/C++
//! It's very tricky to infer what the patterns and exec commands could be, so we recommend using `shell` mode with a custom script (see below).
//!
//! ### Shell (for unsupported languages or complicated builds)
//! `lightmon shell -s <path> -w <patterns>`
//! Here users can specify the path to the shell script and which file types to watch for seperated by commas.
//!
//! For example, let's say you have a python project with a file named `start.py` at the root of the project. Whenever you edit any `.py` files in the project, you want to
//! re-run `python start.py`. To accomplish this, you could create a simple script called `run.sh` with the following contents:
//! ```sh
//! python start.py
//! ```
//!
//! Now, you just run the following:
//! ```
//! lightmon shell -s run.sh -w .py,.ipynb
//! ```

#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
extern crate notify;
mod cli;
mod exec;
mod watcher;

use cli::Cli;
use std::sync::Arc;
use std::{
    process::Child,
    sync::mpsc::{channel, Receiver, Sender},
};

/// Type of events that lightmon handles.
pub enum LightmonEvent {
    /// When lightmon first starts up successfully, it forces the exec thread to go off once
    InitExec,
    KillAndRestartChild,
}

/// Entry point for the entire binary.
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
