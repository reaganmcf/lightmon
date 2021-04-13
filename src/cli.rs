use clap::{App, ArgMatches};
use env_logger::Builder;
use log::LevelFilter;

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

        let mut watch_patterns: Vec<String> = Vec::new();
        let project_language: SupportedLanguage;
        let mut exec_commands: Vec<String> = Vec::new();

        match matches.subcommand() {
            ("rust", Some(_)) => {
                debug!("Configuring for rust mode...");
                project_language = SupportedLanguage::Rust;
                watch_patterns.push("*.rs".to_string());
                watch_patterns.push("Cargo.toml".to_string());
                exec_commands.push("cargo build".to_string());
                exec_commands.push("cargo run".to_string());
            }
            ("node", Some(_)) => {
                debug!("Configuring for node mode...");
                project_language = SupportedLanguage::Node;
                watch_patterns.push("*.js".to_string());
                watch_patterns.push("*.jsx".to_string());
                exec_commands.push("npm start".to_string());
            }
            ("python", Some(_)) => {
                error!("Argument configuration not yet supported!");
                std::process::exit(1);
            }
            ("shell", Some(sub_matcher)) => {
                debug!("Configuring for shell mode...");
                project_language = SupportedLanguage::Shell;
                debug!("Script Path = {:?}", sub_matcher.value_of("script"));
                debug!("Watch Pattern = {:?}", sub_matcher.value_of("watch"));
                let split = sub_matcher.value_of("watch").unwrap().split(',');
                for s in split {
                    watch_patterns.push(format!("*{}", s.to_string()));
                }
                exec_commands.push(format!("bash {}", sub_matcher.value_of("script").unwrap()));
                debug!("{:?}", exec_commands);
            }
            _ => {
                //automatic lang detection
                error!("Argument configuration not yet supported!");
                std::process::exit(1);
            }
        }

        let ret = Cli {
            watch_patterns,
            project_language,
            exec_commands,
        };
        debug!("Parsed params = {:?}", ret);

        ret
    }
}
