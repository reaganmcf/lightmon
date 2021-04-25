#![allow(dead_code)]

extern crate assert_cmd;
use assert_cmd::prelude::*;
use std::fs::OpenOptions;
use std::io::{prelude::*, Read};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
}

pub fn get_path(relative_path: &str) -> Result<std::path::PathBuf, std::io::Error> {
    std::fs::canonicalize(Path::new(relative_path))
}

pub fn run_example(
    project_path: &str,
    sleep_time: Duration,
    arg_list: Option<Vec<&str>>,
    is_going_to_fail: Option<bool>,
) -> Result<CommandOutput, Box<dyn std::error::Error>> {
    // Spawn child lightmon process at node directory
    let mut cmd = Command::cargo_bin("lightmon").ok().unwrap();
    if let Some(args) = arg_list {
        for arg in args.iter() {
            cmd.arg(arg);
        }
    }

    cmd.current_dir(get_path(project_path)?);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    let mut child = cmd.spawn().unwrap();

    // Wait some time to collect stdout
    std::thread::sleep(sleep_time);

    // Kill it
    if let None = is_going_to_fail {
        assert!(
            child.kill().is_ok(),
            "child process should be able to be killed"
        );
    }

    // read stdout and stderr into strings
    let mut stdout = String::new();
    let mut stderr = String::new();
    let std_out_read_attempt = child.stdout.unwrap().read_to_string(&mut stdout);
    let std_err_read_attempt = child.stderr.unwrap().read_to_string(&mut stderr);
    assert!(
        std_out_read_attempt.is_ok(),
        "should always be able to read child stdout"
    );
    assert!(
        std_err_read_attempt.is_ok(),
        "should always be able to read child stderr"
    );

    println!("child stdout = '{}'\nchild stderr = '{}'", stdout, stderr);
    Ok(CommandOutput { stdout, stderr })
}

pub fn run_example_with_file_change(
    project_path: &str,
    sleep_time: Duration,
    arg_list: Option<Vec<&str>>,
    file_name: &str,
) -> Result<CommandOutput, Box<dyn std::error::Error>> {
    // Spawn child lightmon process at node directory
    let mut cmd = Command::cargo_bin("lightmon").ok().unwrap();
    if let Some(args) = arg_list {
        for arg in args.iter() {
            cmd.arg(arg);
        }
    }

    cmd.current_dir(get_path(project_path)?);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    let mut lightmon_child = cmd.spawn().unwrap();

    // Wait some time to collect stdout
    std::thread::sleep(sleep_time);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(get_path(format!("{}/{}", project_path, file_name).as_str()).unwrap())
        .unwrap();
    match file.write_all(b"some bytes") {
        Ok(_) => {}
        Err(_) => {
            println!("Failed to write to file (which triggers event)");
        }
    };

    // Wait again
    std::thread::sleep(sleep_time);

    // Kill it
    assert!(
        lightmon_child.kill().is_ok(),
        "child process should be able to be killed"
    );

    // read stdout into string
    let mut stdout = String::new();
    let mut stderr = String::new();
    let std_out_read_attempt = lightmon_child.stdout.unwrap().read_to_string(&mut stdout);
    let std_err_read_attempt = lightmon_child.stderr.unwrap().read_to_string(&mut stderr);
    assert!(
        std_out_read_attempt.is_ok(),
        "should always be able to read child stdout"
    );
    assert!(
        std_err_read_attempt.is_ok(),
        "should always be able to read child stderr"
    );

    println!("child stdout = '{}'\nchild stderr = '{}'", stdout, stderr);
    Ok(CommandOutput { stdout, stderr })
}
