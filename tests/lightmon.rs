mod utils;

use assert_cmd::prelude::*;
use std::process::Command;
use utils::*;

#[test]
fn unsupported_configuration_fails() -> TestResult {
    let mut cmd = Command::cargo_bin("lightmon")?;
    cmd.arg("java").assert().failure();

    Ok(())
}
