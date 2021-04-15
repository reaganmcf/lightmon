mod utils;

#[macro_use]
extern crate serial_test;
use std::time::Duration;
use utils::*;

const EP_RUST_BASIC_BIN_PATH: &str = "./tests/example_projects/rust_basic_bin";

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
fn rust_basic_bin_configuration() -> Result<(), Box<dyn std::error::Error>> {
    // Spawn child lightmon process at rust directory
    let output = run_example(EP_RUST_BASIC_BIN_PATH, Duration::from_secs(5), None).unwrap();
    assert_eq!(output.stdout, BASIC_BIN_CONFIGURATION_EXPECTED);
    Ok(())
}

#[test]
#[serial(rust)]
fn rust_basic_bin_test_with_rs_file_edits() -> Result<(), Box<dyn std::error::Error>> {
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
