use std::sync::{mpsc::SendError, MutexGuard, PoisonError};

use crate::{timeln::TimeSnapshot};

/// This enum defines the various types of errors that could occur within the timeln module.
#[derive(Debug)]
pub enum TimelnError {
    Io(std::io::Error),
    Regex(regex::Error),
    SendError(SendError<TimeSnapshot>),
    MutexPoisonedError(String),
    BoxError(Box<dyn std::error::Error>),
}

/// Implementations of From trait for TimelnError. 
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

impl From<SendError<TimeSnapshot>> for TimelnError {
    fn from(err: SendError<TimeSnapshot>) -> Self {
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
