mod utils;

#[macro_use]
extern crate serial_test;
use std::time::Duration;
use utils::*;

const EP_RUST_BASIC_BIN_PATH: &str = "./tests/example_projects/rust_basic_bin";
const EP_RUST_BASIC_LIB_PATH: &str = "./tests/example_projects/rust_basic_lib";
const EP_RUST_INVALID_PATH: &str = "./tests/example_projects/rust_invalid";

static BASIC_BIN_CONFIGURATION_EXPECTED: &str = "lightmon started (rust mode)
Hello, World!
";

static BASIC_BIN_WITH_FILE_EDITS_EXPECTED: &str = "lightmon started (rust mode)
Hello, World!
Changes detected, Restarting...
Hello, World!
";

#[test]
#[serial(rust)]
fn rust_basic_bin_configuration() -> TestResult {
    // Spawn child lightmon process at rust directory
    let output = run_example(EP_RUST_BASIC_BIN_PATH, Duration::from_secs(5), None, None).unwrap();
    assert_eq!(output.stdout, BASIC_BIN_CONFIGURATION_EXPECTED);
    Ok(())
}

#[test]
#[serial(rust)]
fn rust_basic_lib_configuration() -> TestResult {
    // Spawn child lightmon process at rust directory
    let output = run_example(EP_RUST_BASIC_LIB_PATH, Duration::from_secs(5), None, None).unwrap();
    assert!(output.stdout.contains("tests::it_works"));
    Ok(())
}

#[test]
#[serial(rust)]
fn rust_invalid_configuration_errors_out() -> TestResult {
    // Spawn child lightmon process at rust directory
    let output = run_example(
        EP_RUST_INVALID_PATH,
        Duration::from_secs(5),
        None,
        Some(true),
    )
    .unwrap();
    assert!(output
        .stderr
        .contains("ERROR lightmon::cli] Could not find which type of rust project this is."));
    assert!(output.stdout.is_empty());
    Ok(())
}

#[test]
#[serial(rust)]
fn rust_subcommand_override_with_args() -> TestResult {
    // Spawn child lightmon process at rust directory
    let output = run_example(
        EP_RUST_BASIC_BIN_PATH,
        Duration::from_secs(5),
        Some(vec!["rust", "build", "--bin", "foo"]),
        None,
    )
    .unwrap();
    assert!(output.stderr.contains("no bin target named `foo`"));

    Ok(())
}

#[test]
#[serial(rust)]
fn rust_subcommand_override_in_bin_configuration() -> TestResult {
    // Spawn child lightmon process at rust directory
    let output = run_example(
        EP_RUST_BASIC_BIN_PATH,
        Duration::from_secs(5),
        Some(vec!["rust", "doc"]),
        None,
    )
    .unwrap();
    assert!(!output.stdout.contains("Hello, World!"));
    Ok(())
}

#[test]
#[serial(rust)]
fn rust_basic_bin_test_with_rs_file_edits() -> TestResult {
    let output = run_example_with_file_change(
        EP_RUST_BASIC_BIN_PATH,
        Duration::from_secs(10),
        None,
        "src/test.rs",
    )
    .unwrap();
    assert_eq!(output.stdout, BASIC_BIN_WITH_FILE_EDITS_EXPECTED);
    Ok(())
}
