use clap::{App, ArgMatches};

pub struct Cli {
  pub watch_pattern: String, // file pattern to watch
}

impl Cli {
  pub fn new() -> Self {
    let yaml = load_yaml!("cli.yaml");
    let matches: ArgMatches = App::from_yaml(yaml).get_matches();
    
    /*
     * To determine the watch pattern, we do the following:
     *  1. It is set manually via --watch "value"
     *  2. One of the language flags has been set
     *  3. We should determine the language automatically by scanning the directory
     */

    let mut watch_pattern: &str = "";
    
    // 1. Check if manually set
    if let Some(watch_pattern_arg) = matches.value_of("watch") {
      watch_pattern = &watch_pattern_arg;
    }
    
    // we can safely unwrap here because this argument is required
    Cli { watch_pattern: watch_pattern.to_string() }
  }
}
