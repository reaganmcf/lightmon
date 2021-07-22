#![allow(non_snake_case)]

mod utils;

use serial_test::serial;
use std::time::Duration;
use utils::*;

const EP_SHELL_BASIC_PATH: &str = "./tests/example_projects/shell_basic";

fn BASIC_CONFIGURATION_EXPECTED() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!(
        r#"[lightmon] {}
[lightmon] enter `rs` at any time to restart
[lightmon] running in shell mode
[lightmon] watching ["*.sh"]
[lightmon] starting `bash script.sh`
Hello, World!
"#,
        version
    )
}

fn BASIC_WITH_FILE_EDITS_EXPECTED() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!(
        r#"[lightmon] {}
[lightmon] enter `rs` at any time to restart
[lightmon] running in shell mode
[lightmon] watching ["*.c"]
[lightmon] starting `bash script.sh`
Hello, World!
[lightmon] Changes detected, Restarting...
[lightmon] starting `bash script.sh`
Hello, World!
"#,
        version
    )
}

static NO_SCRIPT_PATH_EXPECTED: &str = "error: The following required arguments were not provided:
    --script-path <path>

USAGE:
    lightmon shell --script-path <path> --watch-patterns <patterns>

For more information try --help
";

static NO_WATCH_PATTERNS_EXPECTED: &str =
    "error: The following required arguments were not provided:
    --watch-patterns <patterns>

USAGE:
    lightmon shell --script-path <path> --watch-patterns <patterns>

For more information try --help
";

#[cfg(not(target_os = "windows"))]
#[test]
#[serial(shell)]
fn shell_basic_configuration() -> TestResult {
    let output = run_example(
        EP_SHELL_BASIC_PATH,
        Duration::from_secs(5),
        Some(vec!["shell", "-s", "script.sh", "-w", ".sh"]),
        None,
    )
    .unwrap();
    assert_eq!(output.stdout, BASIC_CONFIGURATION_EXPECTED());
    Ok(())
}

#[cfg(not(target_os = "windows"))]
#[test]
#[serial(shell)]
fn shell_should_error_with_no_script_path() -> TestResult {
    let output = run_example(
        EP_SHELL_BASIC_PATH,
        Duration::from_secs(5),
        Some(vec!["shell", "-w", ".sh"]),
        None,
    )
    .unwrap();
    assert_eq!(output.stderr, NO_SCRIPT_PATH_EXPECTED);
    Ok(())
}

#[cfg(not(target_os = "windows"))]
#[test]
#[serial(shell)]
fn shell_should_error_with_no_watch_patterns() -> TestResult {
    let output = run_example(
        EP_SHELL_BASIC_PATH,
        Duration::from_secs(5),
        Some(vec!["shell", "-s", "script.sh"]),
        Some(true),
    )
    .unwrap();
    assert_eq!(output.stderr, NO_WATCH_PATTERNS_EXPECTED);
    Ok(())
}

#[cfg(not(target_os = "windows"))]
#[test]
//#[serial(shell)]
fn shell_basic_with_file_changes() -> TestResult {
    let output = run_example_with_file_change(
        EP_SHELL_BASIC_PATH,
        Duration::from_secs(5),
        Some(vec!["shell", "-s", "script.sh", "-w", ".c"]),
        "test.c",
    )
    .unwrap();
    assert_eq!(output.stdout, BASIC_WITH_FILE_EDITS_EXPECTED());
    Ok(())
}
