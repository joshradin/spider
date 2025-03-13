//! All `Task` related modules

mod task;
mod task_prototypes;

pub use self::{task::*, task_prototypes::*};
use crate::error::Error;

/// An error occurred in a task
#[derive(Debug)]
pub enum TaskError {
    Error(Error),
    StopAction(Option<Error>),
    StopTask(Option<Error>),
}

impl TaskError {
    /// Creates a stop task
    pub fn stop_action(e: impl Into<Option<Error>>) -> Self {
        Self::StopAction(e.into())
    }

    /// Stops the entire task
    pub fn stop_task(e: impl Into<Option<Error>>) -> Self {
        Self::StopTask(e.into())
    }

    /// Converts this into a result
    pub fn fail<T>(self) -> Result<T, Self> {
        Err(self)
    }
}

impl<E> From<E> for TaskError
where
    Error: From<E>,
{
    fn from(value: E) -> Self {
        Self::Error(value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ErrorKind;

    #[test]
    fn test_fail() {
        let result: Result<(), TaskError> =
            TaskError::from(ErrorKind::custom("".to_string())).fail();
    }
}
