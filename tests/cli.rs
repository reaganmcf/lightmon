extern crate assert_cmd;
extern crate predicates;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn no_configuration_fails() -> Result<() , Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("lightmon")?;
  cmd.assert().failure();

  Ok(())
}

#[test]
fn unsupported_configuration_fails() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("lightmon")?;
  cmd.arg("java").assert().failure();

  Ok(())
}

#[test]
fn rust_mode_parses_correctly() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("lightmon")?;

  cmd.arg("rust");
  cmd.assert()
    .success()
    .stdout(predicate::str::contains("rust"));

  Ok(())
}

#[test]
fn node_mode_parses_correctly() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("lightmon")?;

  cmd.arg("node");
  cmd.assert()
    .success()
    .stdout(predicate::str::contains("node"));

  Ok(())
}
