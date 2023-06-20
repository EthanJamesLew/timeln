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

mod annotator;
mod time_formatter;
mod summarizer;
mod plotter;
mod timeln;
mod argopt;

use structopt::StructOpt;

use crate::argopt::TimelnOpt;
use crate::timeln::{TimelnContext, TimelnError};

fn main() -> Result<(), TimelnError> {
    let opt = TimelnOpt::from_args();
    let mut context = TimelnContext::new(opt)?;

    context.run()?;

    context.summarize_and_plot()?;

    Ok(())
}