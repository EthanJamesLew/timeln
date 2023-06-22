use std::io::BufRead;

use crate::error::TimelnError;

// New trait for reading data
pub trait ReadData {
    fn read_line(&mut self, buf: &mut String) -> Result<usize, TimelnError>;
}

// Stdin implementation
pub struct StdinReadData {
    pub stdin: std::io::StdinLock<'static>,
}

impl ReadData for StdinReadData {
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
    fn read_line(&mut self, buf: &mut String) -> Result<usize, TimelnError> {
        match self.data.read_line(buf) {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(TimelnError::Io(e)),
        }
    }
}
