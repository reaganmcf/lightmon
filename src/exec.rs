use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::thread;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub use crate::cli::Cli;
pub use crate::LightmonEvent;

pub fn start(cli_args: Arc<Cli>) -> std::thread::JoinHandle<()> {
    thread::spawn(move || {
        debug!("thread started");

        // Build commands from exec commands
        for exec_command in &cli_args.exec_commands {
            // split into components
            let split: Vec<&str> = exec_command.split(' ').collect();
            let mut cmd = Command::new(split[0]);
            for argument in split.iter().skip(1) {
                cmd.arg(argument);
            }
            debug!("final cmd = {:?}", cmd);
            let output = cmd.output().unwrap();
            cmd.stdout(Stdio::inherit());
            cmd.stdout(Stdio::inherit());
            //write stdout to stdout
            io::stdout().write_all(&output.stdout).unwrap();
            if !output.stderr.is_empty() {
                //if stderr
                //change color to red
                let mut error = StandardStream::stderr(ColorChoice::Always);
                error
                    .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                    .unwrap();
                //write to stderr
                io::stderr().write_all(&output.stderr).unwrap();
                //reset terminal output color
                let color_reset = WriteColor::reset(&mut error).unwrap();
                debug!("reset color? {:?}", color_reset);
            }
        }
    })
}
