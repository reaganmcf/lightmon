mod utils;

#[macro_use]
extern crate serial_test;
use std::time::Duration;
use utils::*;

const EP_NODE_BASIC_SCRIPT_START_PATH: &str = "./tests/example_projects/node_basic/script_start";
const EP_NODE_BASIC_MAIN_ENTRY_POINT_PATH: &str =
    "./tests/example_projects/node_basic/main_entry_point";
const EP_NODE_BASIC_FALLBACK_PATH: &str = "./tests/example_projects/node_basic/fallback";

static BASIC_START_SCRIPT_RESOLUTION_EXPECTED: &str = "lightmon started (node mode)
called by script.start
";

static BASIC_MAIN_RESOLUTION_EXPECTED: &str = "lightmon started (node mode)
called by main entry point
";

static BASIC_FALLBACK_RESOLUTION_EXPECTED: &str = "lightmon started (node mode)
called by fallback
";

static TEST_WITH_FILE_EDITS_EXPECTED: &str = "lightmon started (node mode)
called by script.start
Changes detected, Restarting...
called by script.start
";

// node configuration where script.start is in package.json
#[test]
#[serial(node)]
fn node_basic_script_start_resolution() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_example(
        EP_NODE_BASIC_SCRIPT_START_PATH,
        Duration::from_secs(10),
        None,
    )
    .unwrap();
    assert_eq!(output.stdout, BASIC_START_SCRIPT_RESOLUTION_EXPECTED);
    Ok(())
}

// node configuration where main is in package.json
#[test]
#[serial(node)]
fn node_basic_main_resolution() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_example(
        EP_NODE_BASIC_MAIN_ENTRY_POINT_PATH,
        Duration::from_secs(10),
        None,
    )
    .unwrap();
    assert_eq!(output.stdout, BASIC_MAIN_RESOLUTION_EXPECTED);
    Ok(())
}

// node configuration where nothing can be resolved
#[test]
#[serial(node)]
fn node_basic_fallback_resolution() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_example(EP_NODE_BASIC_FALLBACK_PATH, Duration::from_secs(10), None).unwrap();
    assert_eq!(output.stdout, BASIC_FALLBACK_RESOLUTION_EXPECTED);
    Ok(())
}

#[test]
#[serial(node)]
fn node_test_with_js_file_edits() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_example_with_file_change(
        EP_NODE_BASIC_SCRIPT_START_PATH,
        Duration::from_secs(10),
        None,
        "test.js",
    )
    .unwrap();
    assert_eq!(output.stdout, TEST_WITH_FILE_EDITS_EXPECTED);
    Ok(())
}

#[test]
#[serial(node)]
fn node_test_with_jsx_file_edits() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_example_with_file_change(
        EP_NODE_BASIC_SCRIPT_START_PATH,
        Duration::from_secs(10),
        None,
        "test.jsx",
    )
    .unwrap();
    assert_eq!(output.stdout, TEST_WITH_FILE_EDITS_EXPECTED);
    Ok(())
}

#[test]
#[serial(node)]
fn node_test_with_css_file_edits() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_example_with_file_change(
        EP_NODE_BASIC_SCRIPT_START_PATH,
        Duration::from_secs(10),
        None,
        "test.css",
    )
    .unwrap();
    assert_eq!(output.stdout, TEST_WITH_FILE_EDITS_EXPECTED);
    Ok(())
}

#[test]
#[serial(node)]
fn node_test_with_html_file_edits() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_example_with_file_change(
        EP_NODE_BASIC_SCRIPT_START_PATH,
        Duration::from_secs(10),
        None,
        "test.html",
    )
    .unwrap();
    assert_eq!(output.stdout, TEST_WITH_FILE_EDITS_EXPECTED);
    Ok(())
}
