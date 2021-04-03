use clap::{App, ArgMatches};

pub enum SupportedLanguage {
  Rust,
  Node
}

impl std::fmt::Display for SupportedLanguage {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SupportedLanguage::Rust => write!(f, "rust"),
      SupportedLanguage::Node => write!(f, "node") 
    }
  }
}

pub struct Cli {
  pub watch_patterns: Vec<String>, // file patterns to watch
  pub project_language: SupportedLanguage,
  pub exec_command: String, // subcommand that was used for
}

impl Cli {
  pub fn new() -> Self {
    let yaml = load_yaml!("cli.yaml");
    let matches: ArgMatches = App::from_yaml(yaml).get_matches();
    
    let mut watch_patterns: Vec<String> = Vec::new();
    let project_language: SupportedLanguage;
    let exec_command: String;
    
    if matches.is_present("rust") {
      watch_patterns.push("*.rs".to_string());
      watch_patterns.push("Cargo.toml".to_string());
      project_language = SupportedLanguage::Rust;
      exec_command = "cargo build; cargo run".to_string();
    } else if matches.is_present("node") {
      watch_patterns.push("*.js".to_string());
      watch_patterns.push("*.jsx".to_string());
      project_language = SupportedLanguage::Node;
      exec_command = "npm start".to_string();
    } else {
      eprintln!("Argument configuration not yet supported!");
      std::process::exit(1);
    }
    
    Cli { watch_patterns, project_language, exec_command }
  }
}
