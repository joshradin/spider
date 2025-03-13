//! Build errors

use crate::table::TableError;
use std::backtrace::Backtrace;
use std::fmt::{Debug, Display, Formatter};
use std::panic::Location;
use thiserror::Error;

/// Some error occurred
#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error(transparent)]
    TableError(#[from] TableError),
    #[error(transparent)]
    Custom { error: CustomError },
}

impl ErrorKind {
    pub fn custom<E: ToString + Send + Sync + 'static>(error: E) -> Self {
        Self::Custom {
            error: CustomError(Box::new(error)),
        }
    }
}

pub struct CustomError(pub Box<dyn ToString + Send + Sync>);

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl Debug for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl std::error::Error for CustomError {}

pub struct Error {
    pub kind: ErrorKind,
    pub thread_name: String,
    pub location: Location<'static>,
    pub trace: Backtrace,
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Error")
            .field("kind", &self.kind)
            .field("location", &self.location)
            .finish_non_exhaustive()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "error in thread {thread} at {location}: {error}",
            thread = self.thread_name,
            location = self.location,
            error = self.kind
        )?;
        write!(f, "{}", self.trace)
    }
}

macro_rules! create_error {
    ($kind:expr) => {{
        let thread = std::thread::current();
        Self {
            kind: $kind,
            thread_name: thread
                .name()
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("{:?}", thread.id())),
            location: Location::caller().clone(),
            trace: Backtrace::capture(),
        }
    }};
}

impl Error {
    #[track_caller]
    pub fn new(kind: ErrorKind) -> Self {
        create_error!(kind)
    }
}

impl<E> From<E> for Error
where
    ErrorKind: From<E>,
{
    #[track_caller]
    fn from(value: E) -> Self {
        create_error!(ErrorKind::from(value))
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_error() {
        let error = Error::from(ErrorKind::custom("error!"));
        println!("{}", error);
    }
}
