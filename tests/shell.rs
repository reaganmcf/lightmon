#[macro_use]
extern crate serial_test;

mod utils;
use std::time::Duration;
use utils::*;

const EP_SHELL_BASIC_PATH: &str = "./tests/example_projects/shell_basic";

static BASIC_CONFIGURATION_EXPECTED: &str = "lightmon started (shell mode)
Hello, World!
";

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

static BASIC_WITH_FILE_EDITS_EXPECTED: &str = "lightmon started (shell mode)
Hello, World!
Changes detected, Restarting...
Hello, World!
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
    assert_eq!(output.stdout, BASIC_CONFIGURATION_EXPECTED);
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
#[serial(shell)]
fn shell_basic_with_file_changes() -> TestResult {
    let output = run_example_with_file_change(
        EP_SHELL_BASIC_PATH,
        Duration::from_secs(5),
        Some(vec!["shell", "-s", "script.sh", "-w", ".c"]),
        "test.c",
    )
    .unwrap();
    assert_eq!(output.stdout, BASIC_WITH_FILE_EDITS_EXPECTED);
    Ok(())
}
