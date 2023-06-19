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
use std::time::{Instant};
use colored::*;
use structopt::StructOpt;
use regex::Regex;

use std::sync::{Arc, Mutex};

mod annotator;
mod time_formatter;
mod summarizer;
mod plotter;

use crate::annotator::{TimelnAnnotation, SimpleAnnotator};
use crate::time_formatter::{SecondsFormat};
use crate::summarizer::{Summarizer, SimpleSummarizer};
use crate::plotter::plot_deltas;

#[derive(Debug, StructOpt)]
#[structopt(name = "timeln", about = "A utility that times lines/regex from stdin.")]
struct Opt {
    #[structopt(short = "c", long = "color")]
    color: bool,
    #[structopt(short = "r", long = "regex")]
    regex: Option<String>,
    #[structopt(short = "p", long = "plot")]
    plot: bool,
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
    let time_format = SecondsFormat{};
    let annotator = SimpleAnnotator { color: opt.color, time_format: Box::new(time_format.clone()) };

    let regex = if let Some(r) = opt.regex {
        Some(Regex::new(&r)?)
    } else {
        None
    };

    let summarizer: Arc<Box<dyn Summarizer>> = Arc::new(Box::new(SimpleSummarizer {color: opt.color})); // or DetailedSummarizer

    let total_lines = Arc::new(Mutex::new(0));
    let total_matches = Arc::new(Mutex::new(0));

    let total_lines_ctrlc = total_lines.clone();
    let total_matches_ctrlc = total_matches.clone();
    let summarizer_ctrlc = summarizer.clone();
    let start_time_ctrlc = start_time.clone();
    let time_format_ctrlc = time_format.clone();

    ctrlc::set_handler(move || {
        let total_lines = total_lines_ctrlc.lock().unwrap();
        let total_matches= total_matches_ctrlc.lock().unwrap();
        println!("{}", summarizer_ctrlc.summarize(*total_lines, *total_matches, &Instant::now().duration_since(start_time_ctrlc), &time_format_ctrlc));
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    let mut deltas = Vec::new();

    let mut stdin = stdin.lock();
    loop {
        buffer.clear();
        let bytes_read = stdin.read_line(&mut buffer)?;
        if bytes_read == 0 { // EOF
            break;
        }
        let mut total_lines_guard = total_lines.lock().unwrap();
        *total_lines_guard += 1;
        
        let now = Instant::now();
        
        if let Some(re) = &regex {
            match re.captures_iter(&buffer).next() {
                Some(cap) => {
                    let delta = now.duration_since(last_time);
                    last_time = now;

                    deltas.push(delta);

                    let mut total_matches_guard = total_matches.lock().unwrap();
                    *total_matches_guard += 1;
                    
                    let line = String::from(buffer.trim().replace(&cap[0], &format!("{}", &cap[0].red())));
                    let output = annotator.format_line(&line, &now.duration_since(start_time), &delta);
                    println!("{}", output);
                },
                None => {}
            }
        } else {
            let delta = now.duration_since(last_time);
            last_time = now;

            deltas.push(delta);
            
            let line = String::from(buffer.trim());
            let output = annotator.format_line(&line, &now.duration_since(start_time), &delta);
            println!("{}", output);
        }
    }

    let now = Instant::now();
    let total_lines_final = total_lines.lock().unwrap();
    let total_matches_final= total_matches.lock().unwrap();
    println!("{}", summarizer.summarize(*total_lines_final, *total_matches_final, &now.duration_since(start_time), &time_format));

    if opt.plot {
        // Convert durations to f64 values in seconds
        let deltas: Vec<f64> = deltas.iter().map(|&dur| dur.as_secs_f64()).collect();
        plot_deltas(&deltas, "deltas.png").unwrap();
    }

    Ok(())
}
