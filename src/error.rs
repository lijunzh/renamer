//! Error module for the renamer tool.
//!
//! # Examples
//!
//! ```
//! # use renamer::RenamerError;
//! let err = RenamerError::InvalidPattern;
//! assert_eq!(format!("{}", err), "Invalid pattern provided");
//! ```
use std::fmt;

/// Custom error type for the Renamer tool.
/// 
/// This enum defines possible errors that can occur during file renaming operations.
#[derive(Debug)]
pub enum RenamerError {
    /// The provided regex pattern did not match the file name.
    InvalidPattern,
    IOError(std::io::Error),
    // ... possible additional errors ...
}

impl fmt::Display for RenamerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenamerError::InvalidPattern => write!(f, "Invalid pattern provided"),
            RenamerError::IOError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for RenamerError {}

impl From<std::io::Error> for RenamerError {
    fn from(error: std::io::Error) -> Self {
        RenamerError::IOError(error)
    }
}
