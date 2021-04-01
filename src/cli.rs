use clap::{App, ArgMatches};

pub enum SupportedLanguage {
  RUST,
  NODE
}

pub struct Cli {
  pub watch_patterns: Vec<String>, // file patterns to watch
  pub project_language: SupportedLanguage,
  pub exec_command: String, // subcommand that was used for
  pub entry_file: Option<String>, // entry file, only applicable for particular configuraitons
}

impl Cli {
  pub fn new() -> Self {
    let yaml = load_yaml!("cli.yaml");
    let matches: ArgMatches = App::from_yaml(yaml).get_matches();
    
    let mut watch_patterns: Vec<String> = Vec::new();
    let project_language: SupportedLanguage;
    let exec_command: String;
    let entry_file: Option<String> = None;
    
    if matches.is_present("rust") {
      watch_patterns.push("*.rust".to_string());
      watch_patterns.push("Cargo.toml".to_string());
      project_language = SupportedLanguage::RUST;
      exec_command = "cargo build; cargo run".to_string();
    } else if matches.is_present("node") {
      watch_patterns.push("*js".to_string());
      watch_patterns.push("*.jsx".to_string());
      project_language = SupportedLanguage::NODE;
      exec_command = "npm start".to_string();
    } else {
      eprintln!("Argument configuration not yet supported!");
      std::process::exit(1);
    }
    
    // we can safely unwrap here because this argument is required
    Cli { watch_patterns, project_language, exec_command, entry_file}
  }
}
