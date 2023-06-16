//! This is a command-line utility written in Rust that reads input from stdin line by line,
//! and prints out the time elapsed since the start of the program and the delta time between lines or regex matches.
//!
//! It provides options to enable colorization of the timing information and to match lines based on a regex pattern.
//! The timing is colorized green by default, and matched strings are colorized red.
//! If no regex pattern is provided, the program times every line.
//! If a regex pattern is provided, it times and prints only the lines that match the regex pattern.
//!
//! The utility uses the StructOpt crate for parsing command line arguments and the regex crate for matching regular expressions.
//! It also uses the colored crate to colorize the output.
//!
//! Usage: 
//!     To use this utility, compile it and run it from the command line. 
//!     You can provide input directly from the command line or pipe input from another command.
//!
//!     You can use the `-c` or `--color` option to enable colorization of the timing information.
//!     Use the `-r` or `--regex` option followed by a regex pattern to time and print only the lines that match the pattern.
//!
//! Example:
//!     python your_script.py | timeln -c
//!     python your_script.py | timeln -r "your_regex_pattern"
//!
//! The script prints the elapsed time and the delta time between lines or regex matches in the format "[time: XX.XX s, delta: XX.XX s]".
//! If colorization is enabled, the timing information is printed in green and the matched strings are printed in red.

use std::io::{self, BufRead};
use std::time::{Instant, Duration};
use colored::*;
use structopt::StructOpt;
use regex::Regex;

#[derive(Debug, StructOpt)]
#[structopt(name = "time-reader", about = "A utility that times lines/regex from stdin.")]
struct Opt {
    #[structopt(short = "c", long = "color")]
    color: bool,
    #[structopt(short = "r", long = "regex")]
    regex: Option<String>,
}

#[derive(Debug)]
enum CustomError {
    Io(std::io::Error),
    Regex(regex::Error),
}

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> Self {
        CustomError::Io(err)
    }
}

impl From<regex::Error> for CustomError {
    fn from(err: regex::Error) -> Self {
        CustomError::Regex(err)
    }
}

fn main() -> Result<(), CustomError> {
    let opt = Opt::from_args();
    let stdin = io::stdin();
    let mut last_time = Instant::now();
    let start_time = Instant::now();
    let mut buffer = String::new();

    let regex = if let Some(r) = opt.regex {
        Some(Regex::new(&r)?)
    } else {
        None
    };

    let mut stdin = stdin.lock();
    loop {
        buffer.clear();
        let bytes_read = stdin.read_line(&mut buffer)?;
        if bytes_read == 0 { // EOF
            break;
        }

        if let Some(re) = &regex {
            for cap in re.captures_iter(&buffer) {
                let now = Instant::now();
                let delta = now.duration_since(last_time);
                let elapsed = now.duration_since(start_time);

                let elapsed_seconds = duration_to_seconds(&elapsed);
                let delta_seconds = duration_to_seconds(&delta);

                last_time = now;

                let timestamp = format!(
                    "[time: {:.2} s, delta: {:.2} s]", 
                    elapsed_seconds, delta_seconds
                );

                let output = if opt.color {
                    format!("{} {}",
                            timestamp.green(),
                            buffer.trim().replace(&cap[0], &format!("{}", &cap[0].red()))
                    )
                } else {
                    format!("{} {}", timestamp, buffer.trim())
                };

                println!("{}", output);
            }
        } else {
            let now = Instant::now();
            let delta = now.duration_since(last_time);
            let elapsed = now.duration_since(start_time);

            let elapsed_seconds = duration_to_seconds(&elapsed);
            let delta_seconds = duration_to_seconds(&delta);

            last_time = now;

            let timestamp = format!(
                "[time: {:.2} s, delta: {:.2} s]", 
                elapsed_seconds, delta_seconds
            );

            let output = if opt.color {
                format!("{} {}", timestamp.green(), buffer.trim())
            } else {
                format!("{} {}", timestamp, buffer.trim())
            };

            println!("{}", output);
        }
    }

    Ok(())
}

fn duration_to_seconds(duration: &Duration) -> f64 {
    duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9
}
