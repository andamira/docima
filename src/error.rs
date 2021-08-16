//!
//!

use std::{fmt, io};

/// The docima `Result` type.
pub type DocimaResult<T> = std::result::Result<T, DocimaError>;

/// A standard `Result` type.
pub type StdResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// The docima `Error` type.
#[derive(Debug)]
#[non_exhaustive]
pub enum DocimaError {
    /// An IO error.
    IoError(io::Error),

    /// An `image` crate error.
    PngEncodingError(png::EncodingError),

    /// A dynamic `std` error.
    StdError(Box<dyn std::error::Error>),

    MissingField(String),

    /// A custom error, explained in the string.
    Custom(String),
}

impl fmt::Display for DocimaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DocimaError::*;
        match self {
            IoError(err) => write!(f, "{}", err),
            PngEncodingError(err) => write!(f, "{}", err),
            StdError(err) => write!(f, "{}", err),
            MissingField(err) => write!(f, "{}", err),
            Custom(err) => write!(f, "{}", err),
        }
    }
}

impl From<io::Error> for DocimaError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<png::EncodingError> for DocimaError {
    fn from(err: png::EncodingError) -> Self {
        Self::PngEncodingError(err)
    }
}

impl From<Box<dyn std::error::Error>> for DocimaError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Self::StdError(err)
    }
}
