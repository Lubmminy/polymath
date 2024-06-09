#![forbid(unsafe_code)]
#![deny(
    dead_code,
    unused_imports,
    unused_mut,
    missing_docs,
    missing_debug_implementations
)]
//! internal library to provide structures for errors in polymath.

use std::error::Error as StdError;
use std::fmt;

/// Boxed error to bypass specific [Error](StdError).
type BError = Box<dyn StdError + Send + Sync>;

/// The struct that represents an error.
#[derive(Debug)]
pub struct Error {
    /// The error type.
    pub etype: ErrorType,
    /// The cause of this error.
    pub cause: Option<BError>,
    /// Explains the context in which the error occurs.
    pub context: Option<String>,
}

impl Error {
    /// Throw an [`Error`].
    pub fn new(
        etype: ErrorType,
        cause: Option<BError>,
        context: Option<String>,
    ) -> Self {
        Error {
            etype,
            cause,
            context,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.etype)
    }
}

impl StdError for Error {}

/// Errors in polymath.
#[derive(Debug)]
pub enum ErrorType {
    /// Generic error that returns no additional information.
    Unspecified,
    /// Errors related to databases/message brokers.
    Database(DatabaseError),
    /// Errors related to databases/message brokers.
    Crawler(CrawlerError),
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorType::Unspecified => {
                write!(f, "An error has occurred, but no further information is provided.")
            },
            ErrorType::Database(error) => write!(f, "{:?}", error),
            ErrorType::Crawler(error) => write!(f, "{:?}", error),
        }
    }
}
impl StdError for ErrorType {}

/// Errors related to databases/message brokers.
#[derive(Debug)]
pub enum DatabaseError {
    /// The connection pool has not been created correctly.
    PoolCreation,
    /// The connection pool could not be obtained.
    PoolObtention,
    /// The message for the broker has not been sent.
    MessageNotSent,
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseError::PoolCreation => {
                write!(f, "The connection pool has not been created correctly.")
            },
            DatabaseError::PoolObtention => {
                write!(f, "The connection pool could not be obtained.")
            },
            DatabaseError::MessageNotSent => {
                write!(f, "The message for the broker has not been sent.")
            },
        }
    }
}

impl StdError for DatabaseError {}

/// Errors related to `polymath-crawler`.
#[derive(Debug)]
pub enum CrawlerError {
    /// The domain is outside the allowed domains.
    InvalidDomain,
}

impl fmt::Display for CrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CrawlerError::InvalidDomain => {
                write!(f, "The domain is outside the allowed domains.")
            },
        }
    }
}

impl StdError for CrawlerError {}
