//! This module defines the `TimelnError` enum and its associated conversions, which represent the various types of errors that can occur within the timeln module.
//!
//! The `TimelnError` enum encapsulates different error types, including `std::io::Error`, `regex::Error`, `SendError<TimeSnapshot>`, `PoisonError<MutexGuard<'_, T>>`, and `Box<dyn std::error::Error>`. These error types cover common scenarios encountered in the timeln module, such as I/O errors, regular expression errors, channel send errors, mutex poisoning errors, and generic boxed errors.
//!
//! The `From` trait is implemented for each error type, allowing easy conversion of these errors into the `TimelnError` enum. This enables consistent error handling and propagation within the timeln module, simplifying error management for the caller.
//!
//! # Examples
//!
//! Converting a `std::io::Error` into a `TimelnError`:
//!
//! ```
//! use crate::TimelnError;
//!
//! let io_error = std::io::Error::new(std::io::ErrorKind::Other, "Custom I/O Error");
//! let timeln_error: TimelnError = io_error.into();
//!
//! println!("Converted TimelnError: {:?}", timeln_error);
//! ```
//!
//! Handling a `SendError<TimeSnapshot>` and converting it to `TimelnError`:
//!
//! ```
//! use crate::{TimelnError, timeln::TimeSnapshot};
//! use std::sync::mpsc::channel;
//!
//! let (sender, receiver) = channel::<TimeSnapshot>();
//!
//! // Attempt to send a TimeSnapshot
//! let snapshot = TimeSnapshot::new();
//! let send_result = sender.send(snapshot);
//!
//! let timeln_error: TimelnError = send_result.err().unwrap().into();
//!
//! println!("Converted TimelnError: {:?}", timeln_error);
//! ```
//!
//! # Error Handling
//!
//! When interacting with the timeln module, it is important to handle errors appropriately. The `TimelnError` enum provides a comprehensive set of error types that can occur within the module, allowing for granular error handling based on the specific error scenario.
//!
//! # Note
//!
//! The `TimelnError` enum and its conversions are specific to the timeln module and may require additional error handling and customization for your application's specific needs.
//!
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
    /// Converts a std::io::Error into a TimelnError.
    fn from(err: std::io::Error) -> Self {
        TimelnError::Io(err)
    }
}

impl From<regex::Error> for TimelnError {
    /// Converts a regex::Error into a TimelnError.
    fn from(err: regex::Error) -> Self {
        TimelnError::Regex(err)
    }
}

impl From<SendError<TimeSnapshot>> for TimelnError {
    /// Converts a SendError<TimeSnapshot> into a TimelnError.
    fn from(err: SendError<TimeSnapshot>) -> Self {
        TimelnError::SendError(err)
    }
}

impl From<Box<dyn std::error::Error>> for TimelnError {
    /// Converts a Box<dyn std::error::Error> into a TimelnError.
    fn from(err: Box<dyn std::error::Error>) -> Self {
        TimelnError::BoxError(err)
    }
}

impl<T> From<PoisonError<MutexGuard<'_, T>>> for TimelnError {
    /// Converts a PoisonError<MutexGuard<'_, T>> into a TimelnError.
    fn from(err: PoisonError<MutexGuard<'_, T>>) -> Self {
        TimelnError::MutexPoisonedError(format!("Mutex was poisoned: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_conversion_from_io_error() {
        // Arrange
        let io_error = io::Error::new(io::ErrorKind::Other, "Custom I/O Error");

        // Act
        let timeln_error: TimelnError = io_error.into();

        // Assert
        match timeln_error {
            TimelnError::Io(err) => assert_eq!(err.kind(), io::ErrorKind::Other),
            _ => panic!("Expected TimelnError::Io, but got a different variant."),
        }
    }

    #[test]
    fn test_conversion_from_send_error() {
        // Arrange
        let (sender, receiver) = std::sync::mpsc::channel::<TimeSnapshot>();
        let snapshot = TimeSnapshot::default();
    
        // Close the receiver end of the channel
        drop(receiver);
    
        // Attempt to send a TimeSnapshot (receiver dropped intentionally)
        let send_result = sender.send(snapshot);
    
        // Act
        let timeln_error: TimelnError = match send_result {
            Err(err) => err.into(),
            Ok(_) => panic!("Expected SendError, but send operation succeeded."),
        };
    
        // Assert
        match timeln_error {
            TimelnError::SendError(_) => assert!(true),
            _ => panic!("Expected TimelnError::SendError, but got a different variant."),
        }
    }
}
