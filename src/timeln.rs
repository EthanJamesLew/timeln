//! This module provides the `TimelnContext` struct and related types for running the timeln module.
//!
//! The `TimelnContext` struct is the main context for executing the timeln functionality. It holds the state of the input and the options for processing the input. The module also defines the `TimeSnapshot` struct, which represents the information collected at each line.
//!
//! # Usage
//!
//! To use the timeln functionality, create a `TimelnContext` instance with the desired options using the `new` method. Then, call the `run` method to start the main loop of reading from stdin, annotating the lines, and sending the duration to the receiver. Finally, you can call the `summarize_and_plot` method to print a summary of the results and optionally plot the data.
//!
//! ## Example
//!
//! ```
//! use crate::timeln::{TimelnContext, TimeSnapshot};
//! use crate::argopt::TimelnOpt;
//! use crate::reader::StdinReadData;
//! use std::time::Duration;
//!
//! let opt = TimelnOpt {
//!     color: false,
//!     regex: None,
//!     plot: false,
//! };
//!
//! let mut context = TimelnContext::new(opt).unwrap();
//!
//! // Override the default stdin reader with a custom reader
//! let custom_reader = StdinReadData { /* custom reader implementation */ };
//! context.stdin = Box::new(custom_reader);
//!
//! // Run the timeln module
//! context.run().unwrap();
//!
//! // Send a duration to the receiver
//! let duration = Duration::from_secs(1);
//! context.tx.send(TimeSnapshot { delta: duration, elapsed: duration }).unwrap();
//!
//! // Receive and process the duration
//! let rx_lock = context.rx.lock().unwrap();
//! let received_snapshot = rx_lock.try_recv().unwrap();
//! // ...
//!
//! // Print a summary and plot the data
//! context.summarize_and_plot().unwrap();
//! ```
//!
//! # Testing
//!
//! The module includes unit tests for the `TimelnContext` struct and its methods. The tests cover the creation of a new context, sending and receiving durations, and running the main loop with test data. These tests ensure the correctness and functionality of the timeln module.
//!
//! Note: The `TimelnContext` struct and related types are intended for demonstration purposes and may require additional error handling and customization for your application's specific needs.
//!
//! Note: The example and test code snippets assume the existence of certain types, such as `TimelnOpt`, `StdinReadData`, and `TimeSnapshot`. Please adjust the code according to your project structure and dependencies.
//!
//! # Dependencies
//!
//! This module relies on several external dependencies:
//! - `std::io::{self}`: Provides input/output functionality.
//! - `std::time::{Instant, Duration}`: Enables time-related operations and measurements.
//! - `colored::*`: Facilitates text coloring for line annotations.
//! - `regex::Regex`: Supports regular expression matching for line filtering.
//! - `std::sync::{Arc, Mutex}`: Provides synchronization primitives for multi-threaded environments.
//! - `std::sync::mpsc::{self, Receiver, Sender}`: Implements message passing between threads.
//! - `crate::annotator::{TimelnAnnotation, SimpleAnnotator}`: Provides line annotation functionality.
//! - `crate::formatter::{SecondsFormat}`: Defines formatting options for time durations.
//! - `crate::summarizer::{Summarizer, SimpleSummarizer}`: Implements result summarization.
//! - `crate::plot::{plot_deltas, plot_times}`: Offers plotting capabilities for duration
use colored::*;
use regex::Regex;
use std::io::{self};
use std::time::{Duration, Instant};

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

use crate::annotator::{SimpleAnnotator, TimelnAnnotation};
use crate::argopt::TimelnOpt;
use crate::error::TimelnError;
use crate::formatter::SecondsFormat;
use crate::plot::{plot_deltas, plot_times};
use crate::reader::{ReadData, StdinReadData};
use crate::summarizer::{SimpleSummarizer, Summarizer};

/// Information Collected at Each Line
#[derive(Debug, Copy, Clone)]
pub struct TimeSnapshot {
    delta: Duration,
    elapsed: Duration,
}

impl TimeSnapshot {
    /// Create a defualt instance of TimeSnapshot.
    pub fn default() -> Self {
        Self {
            delta: Duration::new(0, 0),
            elapsed: Duration::new(0, 0),
        }
    }
}

/// The main context struct for running the timeln module.
/// It holds the state of the input and the options for processing the input.
pub struct TimelnContext {
    stdin: Box<dyn ReadData>,
    annotator: SimpleAnnotator,
    summarizer: Arc<Box<dyn Summarizer>>,
    total_lines: Arc<Mutex<usize>>,
    total_matches: Arc<Mutex<usize>>,
    regex: Option<Regex>,
    tx: Sender<TimeSnapshot>,
    rx: Arc<Mutex<Receiver<TimeSnapshot>>>,
    start_time: Instant,
    plot: bool,
}

impl TimelnContext {
    /// Creates a new instance of TimelnContext from a given set of options.
    pub fn new(opt: TimelnOpt) -> Result<Self, TimelnError> {
        let stdin = io::stdin();
        let read_data: Box<dyn ReadData> = Box::new(StdinReadData {
            stdin: stdin.lock(),
        });
        let start_time = Instant::now();
        let time_format = SecondsFormat {};
        let annotator = SimpleAnnotator {
            color: opt.color,
            time_format: Arc::new(Box::new(time_format.clone())),
        };

        let regex = if let Some(r) = opt.regex {
            Some(Regex::new(&r)?)
        } else {
            None
        };

        let summarizer: Arc<Box<dyn Summarizer>> =
            Arc::new(Box::new(SimpleSummarizer { color: opt.color }));

        let total_lines = Arc::new(Mutex::new(0));
        let total_matches = Arc::new(Mutex::new(0));

        let (tx, rx) = mpsc::channel::<TimeSnapshot>();
        let rx = Arc::new(Mutex::new(rx));

        Ok(Self {
            stdin: read_data,
            annotator,
            summarizer,
            total_lines,
            total_matches,
            regex,
            tx,
            rx,
            start_time,
            plot: opt.plot,
        })
    }

    /// Runs the main loop of reading from stdin, annotating the lines and sending the duration to the receiver.
    pub fn run(&mut self) -> Result<(), TimelnError> {
        let mut last_time = Instant::now();
        let mut buffer = String::new();

        let total_lines_ctrlc = self.total_lines.clone();
        let total_matches_ctrlc = self.total_matches.clone();
        let summarizer_ctrlc = self.summarizer.clone();
        let start_time_ctrlc = self.start_time;
        let rx_ctrlc = Arc::clone(&self.rx);
        let time_format_ctrlc = self.annotator.time_format.clone();

        ctrlc::set_handler(move || {
            let total_lines = total_lines_ctrlc.lock().unwrap();
            let total_matches = total_matches_ctrlc.lock().unwrap();
            println!(
                "{}",
                summarizer_ctrlc.summarize(
                    *total_lines,
                    *total_matches,
                    &Instant::now().duration_since(start_time_ctrlc),
                    &**time_format_ctrlc
                )
            );

            let rx_lock = rx_ctrlc.lock().unwrap();
            let durations: Vec<_> = rx_lock.try_iter().collect();
            let deltas: Vec<f64> = durations
                .iter()
                .map(|&dur| dur.delta.as_secs_f64())
                .collect();
            let times: Vec<f64> = durations
                .iter()
                .map(|&dur| dur.elapsed.as_secs_f64())
                .collect();
            plot_deltas(&deltas, "deltas.svg").unwrap();
            plot_times(&times, "times.svg").unwrap();
            std::process::exit(0);
        })
        .expect("Error setting Ctrl-C handler");

        loop {
            buffer.clear();
            let bytes_read = self.stdin.read_line(&mut buffer)?;
            if bytes_read == 0 {
                // EOF
                break;
            }
            let mut total_lines_guard = self.total_lines.lock()?;
            *total_lines_guard += 1;

            let now = Instant::now();

            if let Some(re) = &self.regex {
                match re.captures_iter(&buffer).next() {
                    Some(cap) => {
                        let delta = now.duration_since(last_time);
                        last_time = now;

                        self.tx.send(TimeSnapshot {
                            delta: delta,
                            elapsed: now.duration_since(self.start_time),
                        })?;

                        let mut total_matches_guard = self.total_matches.lock().unwrap();
                        *total_matches_guard += 1;

                        let line = String::from(
                            buffer
                                .trim()
                                .replace(&cap[0], &format!("{}", &cap[0].red())),
                        );
                        let output = self.annotator.format_line(
                            &line,
                            &now.duration_since(self.start_time),
                            &delta,
                        );
                        println!("{}", output);
                    }
                    None => {}
                }
            } else {
                let delta = now.duration_since(last_time);
                last_time = now;

                self.tx.send(TimeSnapshot {
                    delta: delta,
                    elapsed: now.duration_since(self.start_time),
                })?;

                let line = String::from(buffer.trim());
                let output =
                    self.annotator
                        .format_line(&line, &now.duration_since(self.start_time), &delta);
                println!("{}", output);
            }
        }

        Ok(())
    }

    /// Prints a summary of the results and optionally plots the data.
    pub fn summarize_and_plot(&self) -> Result<(), TimelnError> {
        let now = Instant::now();
        let total_lines_final = self.total_lines.lock()?;
        let total_matches_final = self.total_matches.lock()?;
        println!(
            "{}",
            self.summarizer.summarize(
                *total_lines_final,
                *total_matches_final,
                &now.duration_since(self.start_time),
                &**self.annotator.time_format
            )
        );

        if self.plot {
            let rx_lock = self.rx.lock()?;
            let durations: Vec<_> = rx_lock.try_iter().collect();
            let deltas: Vec<f64> = durations
                .iter()
                .map(|&dur| dur.delta.as_secs_f64())
                .collect();
            let times: Vec<f64> = durations
                .iter()
                .map(|&dur| dur.elapsed.as_secs_f64())
                .collect();
            plot_deltas(&deltas, "deltas.svg")?;
            plot_times(&times, "times.svg")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{argopt::TimelnOpt, reader::TestReadData};

    #[test]
    fn test_timeln_context_new() {
        let opt = TimelnOpt {
            color: false,
            regex: None,
            plot: false,
        };
        let context = TimelnContext::new(opt);
        assert!(context.is_ok());
    }

    #[test]
    fn test_send_duration() {
        let opt = TimelnOpt {
            color: false,
            regex: None,
            plot: false,
        };
        let context = TimelnContext::new(opt).unwrap();
        let duration = Duration::from_secs(1);
        assert!(context
            .tx
            .send(TimeSnapshot {
                delta: duration,
                elapsed: duration
            })
            .is_ok());
    }

    #[test]
    fn test_receive_duration() {
        let opt = TimelnOpt {
            color: false,
            regex: None,
            plot: false,
        };
        let context = TimelnContext::new(opt).unwrap();
        let duration = Duration::from_secs(1);
        context
            .tx
            .send(TimeSnapshot {
                delta: duration,
                elapsed: duration,
            })
            .unwrap();
        let rx_lock = context.rx.lock().unwrap();
        assert_eq!(rx_lock.try_recv().unwrap().delta, duration);
    }

    #[test]
    fn test_run() {
        let opt = TimelnOpt {
            color: false,
            regex: None,
            plot: false,
        };
        let mut context = TimelnContext::new(opt).unwrap();
        let test_data = TestReadData {
            data: std::io::Cursor::new("test\n".to_string()),
        };
        context.stdin = Box::new(test_data);
        assert!(context.run().is_ok());
    }
}
