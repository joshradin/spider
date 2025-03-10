//! Build errors

use thiserror::Error;

/// Some error occurred
#[derive(Debug, Error)]
pub enum Error {

    #[error(transparent)]
    Custom {
        error: Box<dyn std::error::Error + Send + Sync>,
    }
}

/// An error occurred in a task
#[derive(Debug)]
pub enum Err<E = Error> {
    Fail(E),
    StopAction(Option<E>),
    StopTask(Option<E>),
}

impl<E> Err<E> {
    /// Maps the inner error
    pub fn map<E2, F>(self, f: F) -> Err<E2>
        where F: FnOnce(E) -> E2
    {
        match self {
            Err::Fail(e) => {
                Err::Fail(f(e))
            }
            Err::StopAction(e) => {
                Err::StopAction(e.map(f))
            }
            Err::StopTask(e) => {
                Err::StopTask(e.map(f))
            }
        }
    }

    pub fn into<E2>(self) -> Err<E2>
        where E: Into<E2>
    {
        self.map(E::into)
    }

    pub fn from<E2>(this: Err<E2>) -> Err<E>
        where E: From<E2>
    {
        this.map(|e| E::from(e))
    }
}

impl<E> From<E> for Err<E> {
    fn from(value: E) -> Self {
        Self::Fail(value)
    }
}

pub type BuildResult<T = (), E = Error> = Result<T, Err<E>>;