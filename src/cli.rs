use clap::{App, ArgMatches};
use env_logger::Builder;
use log::LevelFilter;
use std::path::Path;

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

#[derive(Debug)]
pub struct Cli {
    pub watch_patterns: Vec<String>, // file patterns to watch
    pub project_language: SupportedLanguage,
    pub exec_commands: Vec<String>, // list of commands to run
}

impl Cli {
    pub fn new() -> Self {
        let yaml = load_yaml!("cli.yaml");
        let matches: ArgMatches = App::from_yaml(yaml).get_matches();

        if matches.is_present("verbose") {
            Builder::new().filter_level(LevelFilter::Debug).init();
        } else {
            Builder::new().filter_level(LevelFilter::Info).init();
        }

        let config: Option<Cli> = match matches.subcommand() {
            ("rust", Some(_)) => Some(Self::build_rust_config()),
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
                    Some(Self::build_rust_config())
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

    pub fn build_node_config() -> Self {
        debug!("Configuring for node mode...");
        Cli {
            watch_patterns: vec!["*.jsx".to_string(), ".js".to_string()],
            project_language: SupportedLanguage::Node,
            exec_commands: vec!["npm start".to_string()],
        }
    }

    pub fn build_rust_config() -> Self {
        debug!("Configuring for rust mode...");
        Cli {
            watch_patterns: vec!["*.rs".to_string(), "Cargo.toml".to_string()],
            project_language: SupportedLanguage::Rust,
            exec_commands: vec!["cargo build".to_string(), "cargo run".to_string()],
        }
    }
    pub fn build_shell_config(sub_matcher: &ArgMatches) -> Self {
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
