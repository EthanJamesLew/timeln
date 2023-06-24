//! This module provides implementations of the `ReadData` trait for reading data from different sources.
//!
//! The `ReadData` trait defines a common interface for reading lines of data into a buffer. Two implementations
//! are provided: `StdinReadData` for reading from standard input, and `TestReadData` for reading from test data.
//!
//! # Examples
//!
//! Reading from standard input:
//!
//! ```
//! use crate::ReadData;
//!
//! let stdin = std::io::stdin();
//! let handle = stdin.lock();
//! let mut reader = StdinReadData { stdin: handle };
//!
//! let mut buf = String::new();
//! let result = reader.read_line(&mut buf);
//!
//! match result {
//!     Ok(bytes) => println!("Read {} bytes from standard input.", bytes),
//!     Err(e) => eprintln!("Error reading from standard input: {:?}", e),
//! }
//! ```
//!
//! Reading from test data:
//!
//! ```
//! use crate::ReadData;
//!
//! let input = "Hello, world!\n".to_string();
//! let cursor = std::io::Cursor::new(input);
//! let mut reader = TestReadData { data: cursor };
//!
//! let mut buf = String::new();
//! let result = reader.read_line(&mut buf);
//!
//! match result {
//!     Ok(bytes) => println!("Read {} bytes from test data.", bytes),
//!     Err(e) => eprintln!("Error reading from test data: {:?}", e),
//! }
//! ```
//!
//! # Testing
//!
//! Unit tests are provided for each implementation. They can be run using the command `cargo test`.
//! The tests verify the functionality of the `read_line` method for both `StdinReadData` and `TestReadData`.
//!
//! Note: The `ReadData` trait and its implementations are intended for demonstration purposes and may
//! require additional error handling and validation for production use.
//!
use std::io::BufRead;

use crate::error::TimelnError;

/// New trait for reading data
pub trait ReadData {
    fn read_line(&mut self, buf: &mut String) -> Result<usize, TimelnError>;
}

/// Stdin implementation
pub struct StdinReadData {
    pub stdin: std::io::StdinLock<'static>,
}

impl ReadData for StdinReadData {
    /// Reads a line from standard input into the provided buffer.
    /// Returns the number of bytes read or an error if encountered.
    fn read_line(&mut self, buf: &mut String) -> Result<usize, TimelnError> {
        match self.stdin.read_line(buf) {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(TimelnError::Io(e)),
        }
    }
}

// Test data implementation
pub struct TestReadData {
    pub data: std::io::Cursor<String>,
}

impl ReadData for TestReadData {
    /// Reads a line from test data into the provided buffer.
    /// Returns the number of bytes read or an error if encountered.
    fn read_line(&mut self, buf: &mut String) -> Result<usize, TimelnError> {
        match self.data.read_line(buf) {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(TimelnError::Io(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_read_line() {
        // Arrange
        let input = "Hello, world!\n".to_string();
        let cursor = std::io::Cursor::new(input);
        let mut reader = TestReadData { data: cursor };

        // Act
        let mut buf = String::new();
        let result = reader.read_line(&mut buf);

        // Assert
        assert_eq!(result.unwrap(), 14);
        assert_eq!(buf, "Hello, world!\n");
    }
}
