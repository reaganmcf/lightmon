#![allow(non_snake_case)]

mod utils;

use serial_test::serial;
use std::time::Duration;
use utils::*;

const EP_NODE_BASIC_SCRIPT_START_PATH: &str = "./tests/example_projects/node_basic/script_start";
const EP_NODE_BASIC_MAIN_ENTRY_POINT_PATH: &str =
    "./tests/example_projects/node_basic/main_entry_point";
const EP_NODE_BASIC_FALLBACK_PATH: &str = "./tests/example_projects/node_basic/fallback";

fn BASIC_START_SCRIPT_RESOLUTION_EXPECTED() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!(
        r#"[lightmon] {}
[lightmon] enter `rs` at any time to restart
[lightmon] running in Node.js mode
[lightmon] watching ["*.jsx", "*.js", "*.html", "*.css"]
[lightmon] starting `node script_start.js`
called by script.start
"#,
        version
    )
}

fn BASIC_MAIN_RESOLUTION_EXPECTED() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!(
        r#"[lightmon] {}
[lightmon] enter `rs` at any time to restart
[lightmon] running in Node.js mode
[lightmon] watching ["*.jsx", "*.js", "*.html", "*.css"]
[lightmon] starting `node main.js`
called by main entry point
"#,
        version
    )
}

fn BASIC_FALLBACK_RESOLUTION_EXPECTED() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!(
        r#"[lightmon] {}
[lightmon] enter `rs` at any time to restart
[lightmon] running in Node.js mode
[lightmon] watching ["*.jsx", "*.js", "*.html", "*.css"]
[lightmon] starting `node index.js`
called by fallback
"#,
        version
    )
}

fn TEST_WITH_FILE_EDITS_EXPECTED() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!(
        r#"[lightmon] {}
[lightmon] enter `rs` at any time to restart
[lightmon] running in Node.js mode
[lightmon] watching ["*.jsx", "*.js", "*.html", "*.css"]
[lightmon] starting `node script_start.js`
called by script.start
[lightmon] Changes detected, Restarting...
[lightmon] starting `node script_start.js`
called by script.start
"#,
        version
    )
}

// node configuration where script.start is in package.json
#[test]
#[serial(node)]
fn node_basic_script_start_resolution() -> TestResult {
    let output = run_example(
        EP_NODE_BASIC_SCRIPT_START_PATH,
        Duration::from_secs(10),
        None,
        None,
    )
    .unwrap();
    assert_eq!(output.stdout, BASIC_START_SCRIPT_RESOLUTION_EXPECTED());
    Ok(())
}

// node configuration where main is in package.json
#[test]
#[serial(node)]
fn node_basic_main_resolution() -> TestResult {
    let output = run_example(
        EP_NODE_BASIC_MAIN_ENTRY_POINT_PATH,
        Duration::from_secs(10),
        None,
        None,
    )
    .unwrap();
    assert_eq!(output.stdout, BASIC_MAIN_RESOLUTION_EXPECTED());
    Ok(())
}

// node configuration where nothing can be resolved
#[test]
#[serial(node)]
fn node_basic_fallback_resolution() -> TestResult {
    let output = run_example(
        EP_NODE_BASIC_FALLBACK_PATH,
        Duration::from_secs(10),
        None,
        None,
    )
    .unwrap();
    assert_eq!(output.stdout, BASIC_FALLBACK_RESOLUTION_EXPECTED());
    Ok(())
}

#[test]
#[serial(node)]
fn node_test_with_js_file_edits() -> TestResult {
    let output = run_example_with_file_change(
        EP_NODE_BASIC_SCRIPT_START_PATH,
        Duration::from_secs(10),
        None,
        "test.js",
    )
    .unwrap();
    assert_eq!(output.stdout, TEST_WITH_FILE_EDITS_EXPECTED());
    Ok(())
}

#[test]
#[serial(node)]
fn node_test_with_jsx_file_edits() -> TestResult {
    let output = run_example_with_file_change(
        EP_NODE_BASIC_SCRIPT_START_PATH,
        Duration::from_secs(10),
        None,
        "test.jsx",
    )
    .unwrap();
    assert_eq!(output.stdout, TEST_WITH_FILE_EDITS_EXPECTED());
    Ok(())
}

#[test]
#[serial(node)]
fn node_test_with_css_file_edits() -> TestResult {
    let output = run_example_with_file_change(
        EP_NODE_BASIC_SCRIPT_START_PATH,
        Duration::from_secs(10),
        None,
        "test.css",
    )
    .unwrap();
    assert_eq!(output.stdout, TEST_WITH_FILE_EDITS_EXPECTED());
    Ok(())
}

#[test]
#[serial(node)]
fn node_test_with_html_file_edits() -> TestResult {
    let output = run_example_with_file_change(
        EP_NODE_BASIC_SCRIPT_START_PATH,
        Duration::from_secs(10),
        None,
        "test.html",
    )
    .unwrap();
    assert_eq!(output.stdout, TEST_WITH_FILE_EDITS_EXPECTED());
    Ok(())
}
