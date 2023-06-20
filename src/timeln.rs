use std::io::{self, BufRead, StdinLock};
use std::time::{Instant, Duration};
use colored::*;
use regex::Regex;

use std::sync::{Arc, Mutex, PoisonError, MutexGuard};
use std::sync::mpsc::{self, Receiver, Sender, SendError};

use crate::annotator::{TimelnAnnotation, SimpleAnnotator};
use crate::time_formatter::{SecondsFormat};
use crate::summarizer::{Summarizer, SimpleSummarizer};
use crate::plotter::plot_deltas;
use crate::argopt::{TimelnOpt};

#[derive(Debug)]
pub enum TimelnError {
    Io(std::io::Error),
    Regex(regex::Error),
    SendError(SendError<Duration>),
    MutexPoisonedError(String),
    BoxError(Box<dyn std::error::Error>),
}

impl From<std::io::Error> for TimelnError {
    fn from(err: std::io::Error) -> Self {
        TimelnError::Io(err)
    }
}

impl From<regex::Error> for TimelnError {
    fn from(err: regex::Error) -> Self {
        TimelnError::Regex(err)
    }
}

impl From<SendError<Duration>> for TimelnError {
    fn from(err: SendError<Duration>) -> Self {
        TimelnError::SendError(err)
    }
}

impl From<Box<dyn std::error::Error>> for TimelnError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        TimelnError::BoxError(err)
    }
}

impl<T> From<PoisonError<MutexGuard<'_, T>>> for TimelnError {
    fn from(err: PoisonError<MutexGuard<'_, T>>) -> Self {
        TimelnError::MutexPoisonedError(format!("Mutex was poisoned: {}", err))
    }
}

pub struct TimelnContext {
    stdin: StdinLock<'static>,
    annotator: SimpleAnnotator,
    summarizer: Arc<Box<dyn Summarizer>>,
    total_lines: Arc<Mutex<usize>>,
    total_matches: Arc<Mutex<usize>>,
    regex: Option<Regex>,
    tx: Sender<Duration>,
    rx: Arc<Mutex<Receiver<Duration>>>,
    start_time: Instant,
    plot: bool,
}

impl TimelnContext {
    pub fn new(opt: TimelnOpt) -> Result<Self, TimelnError> {
        let stdin = io::stdin();
        let start_time = Instant::now();
        let time_format = SecondsFormat{};
        let annotator = SimpleAnnotator { color: opt.color, time_format: Arc::new(Box::new(time_format.clone())) };

        let regex = if let Some(r) = opt.regex {
            Some(Regex::new(&r)?)
        } else {
            None
        };

        let summarizer: Arc<Box<dyn Summarizer>> = Arc::new(Box::new(SimpleSummarizer {color: opt.color}));

        let total_lines = Arc::new(Mutex::new(0));
        let total_matches = Arc::new(Mutex::new(0));

        let (tx, rx) = mpsc::channel::<Duration>();
        let rx = Arc::new(Mutex::new(rx));

        Ok(Self {
            stdin: stdin.lock(),
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
            let total_matches= total_matches_ctrlc.lock().unwrap();
            println!("{}", summarizer_ctrlc.summarize(*total_lines, *total_matches, &Instant::now().duration_since(start_time_ctrlc), &**time_format_ctrlc));
            
            let rx_lock = rx_ctrlc.lock().unwrap();
            let durations: Vec<_> = rx_lock.try_iter().collect();
            let deltas: Vec<f64> = durations.iter().map(|&dur| dur.as_secs_f64()).collect();
            plot_deltas(&deltas, "deltas.png").unwrap();
            std::process::exit(0);
        }).expect("Error setting Ctrl-C handler");

        loop {
            buffer.clear();
            let bytes_read = self.stdin.read_line(&mut buffer)?;
            if bytes_read == 0 { // EOF
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

                        self.tx.send(delta)?;

                        let mut total_matches_guard = self.total_matches.lock().unwrap();
                        *total_matches_guard += 1;
                        
                        let line = String::from(buffer.trim().replace(&cap[0], &format!("{}", &cap[0].red())));
                        let output = self.annotator.format_line(&line, &now.duration_since(self.start_time), &delta);
                        println!("{}", output);
                    },
                    None => {}
                }
            } else {
                let delta = now.duration_since(last_time);
                last_time = now;

                self.tx.send(delta)?;
                
                let line = String::from(buffer.trim());
                let output = self.annotator.format_line(&line, &now.duration_since(self.start_time), &delta);
                println!("{}", output);
            }
        }

        Ok(())
    }

    pub fn summarize_and_plot(&self) -> Result<(), TimelnError> {
        let now = Instant::now();
        let total_lines_final = self.total_lines.lock()?;
        let total_matches_final= self.total_matches.lock()?;
        println!("{}", self.summarizer.summarize(*total_lines_final, *total_matches_final, &now.duration_since(self.start_time), &**self.annotator.time_format));

        if self.plot {
            let rx_lock = self.rx.lock()?;
            let durations: Vec<_> = rx_lock.try_iter().collect();
            let deltas: Vec<f64> = durations.iter().map(|&dur| dur.as_secs_f64()).collect();
            plot_deltas(&deltas, "deltas.png")?;
        }

        Ok(())
    }
}
