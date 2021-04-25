//! Responsible for parsing command line arguments and returning information needed by the
//! watcher and exec threads.

extern crate serde_json;
use clap::{App, AppSettings, ArgMatches};
use env_logger::Builder;
use log::LevelFilter;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Enum definitions of all supported languages
#[derive(Debug)]
pub enum SupportedLanguage {
    Rust,
    Node,
    Shell,
}

impl std::fmt::Display for SupportedLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedLanguage::Rust => write!(f, "rust"),
            SupportedLanguage::Node => write!(f, "node"),
            SupportedLanguage::Shell => write!(f, "shell"),
        }
    }
}

/// Struct that contains all parsed data necessary for `exec` and `watcher`
#[derive(Debug)]
pub struct Cli {
    pub watch_patterns: Vec<String>, // file patterns to watch
    pub project_language: SupportedLanguage,
    pub exec_commands: Vec<String>, // list of commands to run
}

impl Cli {
    /// Entry point for generating a new Cli struct. Uses clap, as well as automatic language
    /// project detection to determine which config to build.
    pub fn new() -> Self {
        let yaml = load_yaml!("cli.yaml");
        let matches: ArgMatches = App::from_yaml(yaml)
            .global_setting(AppSettings::AllowExternalSubcommands)
            .get_matches();

        if matches.is_present("verbose") {
            Builder::new().filter_level(LevelFilter::Debug).init();
        } else {
            Builder::new().filter_level(LevelFilter::Info).init();
        }

        let config: Option<Cli> = match matches.subcommand() {
            ("rust", Some(sub_matcher)) => Some(Self::build_rust_config(Some(sub_matcher))),
            ("node", Some(_)) => Some(Self::build_node_config()),
            ("python", Some(_)) => None,
            ("shell", Some(sub_matcher)) => Some(Self::build_shell_config(sub_matcher)),
            _ => {
                //automatic lang detection
                // if Path::new("lightmon.toml").exists(){
                //     //TODO
                // } else if Path::new("nodemon.json").exists() {
                //TODO
                // }
                if Path::new("package.json").exists() {
                    Some(Self::build_node_config())
                } else if Path::new("Cargo.toml").exists() {
                    Some(Self::build_rust_config(None))
                } else {
                    None
                }
            }
        };

        if config.is_none() {
            error!("Argument configuration not yet supported!");
            std::process::exit(1);
        }
        config.unwrap()
    }

    /// Build the `nodejs` configuration.
    ///
    /// ### Watch Patterns
    /// [`.jsx`, `.js`, `.html`, `.css`]
    ///
    /// ### Exec Commands
    /// If there is a `package.json` in the root directory, lightmon attempts to resolve the exec command
    /// in the following order:
    ///  1. The value at `scripts.start`
    ///  2. `node main` where `main` is the value of the `main` key in `package.json` (the entry point of the project).
    ///
    /// **NOTE:** exec command will fallback to `node index.js` if all of the above fail.
    ///
    /// For example, the following `package.json` will result in the exec command resolving to
    /// `react-scripts start`:
    /// ```json
    /// {
    ///     "name": "calculator",
    ///     "main": "index.js",
    ///     "scripts": {
    ///         "start": "react-scripts start"
    ///         "build": "react-scripts build"
    ///     }
    /// }
    /// ```
    ///
    /// In this example, the exec command will resolve to `node my_entry_point.js`:
    /// ```json
    /// {
    ///     "name": "bar",
    ///     "main": "my_entry_point.js"
    /// }
    /// ```
    fn build_node_config() -> Self {
        debug!("Configuring for node mode...");
        let watch_patterns: Vec<String> = vec![
            "*.jsx".to_string(),
            "*.js".to_string(),
            "*.html".to_string(),
            "*.css".to_string(),
        ];
        let mut exec_commands: Vec<String> = Vec::new();

        if Path::new("package.json").exists() {
            let file = File::open("package.json").unwrap();
            let reader = BufReader::new(file);
            let values: Value = serde_json::from_reader(reader).unwrap();

            if values.is_object() {
                if let Some(scripts) = values.get("scripts") {
                    debug!("scripts found! Value is = {}", scripts);
                    if let Some(scripts_start) = scripts.get("start") {
                        debug!(
                            "scripts.start found! Resolving exec_commands as '{}'",
                            scripts_start
                        );
                        exec_commands.push(scripts_start.as_str().unwrap().to_string());
                    }
                }

                // If scripts resolution failed, try getting main entry point
                if exec_commands.is_empty() {
                    if let Some(main_entry_point) = values.get("main") {
                        debug!(
                            "main found! Resolving exec_commands as '{}'",
                            main_entry_point
                        );
                        // main_entry_point has a " on either end, so we need to trim
                        exec_commands.push(format!("node {}", main_entry_point.as_str().unwrap()));
                    }
                }
            }
        }

        // exec commands resolution fallback
        if exec_commands.is_empty() {
            debug!("Failed to resolve exec command using package.json, falling back to 'node index.js'");
            exec_commands.push("node index.js".to_string());
        }

        Cli {
            watch_patterns,
            project_language: SupportedLanguage::Node,
            exec_commands,
        }
    }

    /// Build the `rust` configuration.
    ///
    /// ### Watch Patterns
    /// [`Cargo.toml`, `.rs`]
    ///
    /// ### Exec Commands
    /// If no subcommand is passed in, the exec command resolves with the following rules:
    ///  1. `cargo run` if `src/main.rs` exists
    ///  2. `cargo test` if `src/lib.rs` exists
    ///
    /// However, you can also specify any subcommand and custom arguments explicitly and they will
    /// be carried over to the exec command. For example
    /// ```
    /// lightmon rust build --bin my_bin --all-targets
    /// ```
    /// Will resolve the exec command to `cargo build --bin my_bin --all-targets`
    fn build_rust_config(sub_matcher: Option<&ArgMatches>) -> Self {
        let mut exec_commands: Vec<String> = Vec::new();
        debug!("Configuring for rust mode...");
        // attempt to match subcommand
        if let Some(sub_matcher) = sub_matcher {
            debug!("build_rust_config received subcommand override!");
            if let (external_name, Some(external_args)) = sub_matcher.subcommand() {
                if external_args.values_of("").is_some() {
                    let ext_args: Vec<&str> = external_args.values_of("").unwrap().collect();
                    debug!("External commands = {:?}", ext_args);
                    exec_commands.push(format!("cargo {} {}", external_name, ext_args.join(" ")));
                } else {
                    exec_commands.push(format!("cargo {}", external_name));
                }
            }
        } else {
            // Since no subcommand was given, resolve via project type
            // 1. Check if src/main.rs exists
            if Path::new("src/main.rs").exists() {
                debug!("Cargo bin project detected");
                exec_commands.push(format!("cargo {}", "run"));
            } else if Path::new("src/lib.rs").exists() {
                debug!("Cargo lib project detected");
                exec_commands.push(format!("cargo {}", "test"));
            } else {
                error!("Could not find which type of rust project this is. Consider overriding using a cargo subcommand. Run `lightmon help rust` for more information.");
                std::process::exit(1);
            }
        }

        Cli {
            watch_patterns: vec!["*.rs".to_string(), "Cargo.toml".to_string()],
            project_language: SupportedLanguage::Rust,
            exec_commands,
        }
    }

    /// Build the `shell` configuration.
    /// The watch patterns and exec commands are determined by the arguments passed in.
    fn build_shell_config(sub_matcher: &ArgMatches) -> Self {
        let mut watch_patterns: Vec<String> = Vec::new();
        let mut exec_commands: Vec<String> = Vec::new();
        debug!("Configuring for shell mode...");
        debug!("Script Path = {:?}", sub_matcher.value_of("script"));
        debug!("Watch Pattern = {:?}", sub_matcher.value_of("watch"));
        let split = sub_matcher.value_of("watch").unwrap().split(',');
        for s in split {
            watch_patterns.push(format!("*{}", s.to_string()));
        }
        exec_commands.push(format!("bash {}", sub_matcher.value_of("script").unwrap()));
        debug!("{:?}", exec_commands);
        Cli {
            watch_patterns,
            project_language: SupportedLanguage::Shell,
            exec_commands,
        }
    }
}
