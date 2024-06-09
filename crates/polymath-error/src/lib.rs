#![forbid(unsafe_code)]
#![deny(
    dead_code,
    unused_imports,
    unused_mut,
    missing_docs,
    missing_debug_implementations
)]
//! Internal library to provide structures for errors in Polymath.
//!
//! # Examples
//!
//! Basic usage of [`Error`]:
//! ```rust
//! use polymath_error::{Error, ErrorType};
//!
//! let error = Error::new(
//!     ErrorType::Unspecified,
//!     None,
//!     Some("An unspecified error occurred.".to_string()),
//! );
//! eprintln!("{}", error);
//! ```

use std::error::Error as StdError;
use std::fmt;

/// Boxed error to bypass specific [Error](StdError).
type BError = Box<dyn StdError + Send + Sync>;

/// Represents an error in Polymath.
#[derive(Debug)]
pub struct Error {
    /// The type of the error.
    pub error_type: ErrorType,
    /// The cause of this error.
    pub cause: Option<BError>,
    /// Contextual information about where the error occurred.
    pub context: Option<String>,
}

impl Error {
    /// Creates a new [`Error`].
    pub fn new(
        error_type: ErrorType,
        cause: Option<BError>,
        context: Option<String>,
    ) -> Self {
        Self {
            error_type,
            cause,
            context,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error_type)
    }
}

impl StdError for Error {}

/// Defines the types of errors in Polymath.
#[derive(Debug)]
pub enum ErrorType {
    /// A generic error with no additional information.
    Unspecified,
    /// Errors related to databases or message brokers.
    Database(DatabaseError),
    /// Errors related to the crawler.
    Crawler(CrawlerError),
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorType::Unspecified => {
                write!(f, "An unspecified error occurred.")
            },
            ErrorType::Database(ref error) => {
                write!(f, "{}", error)
            },
            ErrorType::Crawler(ref error) => {
                write!(f, "{}", error)
            },
        }
    }
}

impl StdError for ErrorType {}

/// Errors related to databases or message brokers.
#[derive(Debug)]
pub enum DatabaseError {
    /// The connection pool was not created successfully.
    PoolCreation,
    /// The connection pool could not be obtained.
    PoolObtention,
    /// The message for the broker was not sent.
    MessageNotSent,
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseError::PoolCreation => {
                write!(f, "Failed to create the connection pool.")
            },
            DatabaseError::PoolObtention => {
                write!(f, "Failed to obtain the connection pool.")
            },
            DatabaseError::MessageNotSent => {
                write!(f, "Failed to send the message to the broker.")
            },
        }
    }
}

impl StdError for DatabaseError {}

/// Errors related to the `polymath-crawler`.
#[derive(Debug)]
pub enum CrawlerError {
    /// The domain is not allowed.
    InvalidDomain,
}

impl fmt::Display for CrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CrawlerError::InvalidDomain => {
                write!(f, "The domain is not within the allowed domains.")
            },
        }
    }
}

impl StdError for CrawlerError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = Error::new(
            ErrorType::Unspecified,
            None,
            Some("An unspecified error occurred.".to_string()),
        );
        assert_eq!(error.to_string(), "An unspecified error occurred.");
    }

    #[test]
    fn test_database_error_display() {
        let db_error = Error::new(
            ErrorType::Database(DatabaseError::PoolCreation),
            None,
            Some("Failed to create database connection pool.".to_string()),
        );
        assert_eq!(
            db_error.to_string(),
            "Failed to create the connection pool."
        );
    }

    #[test]
    fn test_crawler_error_display() {
        let crawler_error = Error::new(
            ErrorType::Crawler(CrawlerError::InvalidDomain),
            None,
            Some("Invalid domain encountered.".to_string()),
        );
        assert_eq!(
            format!("{}", crawler_error),
            "The domain is not within the allowed domains."
        );
    }

    #[test]
    fn test_error_with_cause() {
        let cause: BError = Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Root cause",
        ));
        let error = Error::new(
            ErrorType::Unspecified,
            Some(cause),
            Some("An unspecified error occurred.".to_string()),
        );
        assert_eq!(error.to_string(), "An unspecified error occurred.");
    }
}
