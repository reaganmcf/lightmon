mod utils;

use assert_cmd::prelude::*;
use std::process::Command;
use std::time::Duration;
use utils::*;

const EP_SHELL_BASIC_PATH: &str = "./tests/example_projects/shell_basic";

#[test]
fn unsupported_configuration_fails() -> TestResult {
    let mut cmd = Command::cargo_bin("lightmon")?;
    cmd.arg("java").assert().failure();

    Ok(())
}

#[test]
fn verbose_shows_debug_statements() -> TestResult {
    // Spawn child lightmon process at
    let output = run_example(
        EP_SHELL_BASIC_PATH,
        Duration::from_secs(5),
        Some(vec!["-v", "shell", "-s", "script.sh", "-w", ".sh"]),
        None,
    )
    .unwrap();
    assert!(output.stderr.contains("DEBUG lightmon::cli]"));
    Ok(())
}
