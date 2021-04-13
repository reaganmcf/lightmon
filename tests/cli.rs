extern crate assert_cmd;
extern crate predicates;

use assert_cmd::prelude::*;
//use predicates::prelude::*;
use std::process::Command;

#[test]
fn unsupported_configuration_fails() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lightmon")?;
    cmd.arg("java").assert().failure();

    Ok(())
}

//TODO tests for valid configurations. The issue right now though is that
// when you spawn with a valid configuration, the process won't exit. This makes it a
// lot harder to test, so going to have to figure it out later
