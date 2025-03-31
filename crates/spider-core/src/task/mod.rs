//! Represents an atomic piece of work in a project

use std::cell::Ref;
use crate::action::Action;
use crate::error::Error;
use crate::finalized::Finalize;
use crate::project::Project;
use crate::shared::{shared, Shared};
use std::pin::Pin;
use std::sync::Arc;
use sync_wrapper::SyncWrapper;
use tokio::sync::Mutex;

pub type Result = std::result::Result<(), TaskError>;

#[derive(Debug)]
struct TaskInner {
    path: String,
}

/// A task
#[derive(Debug, Clone)]
pub struct Task {
    inner: Shared<Finalize<TaskInner>>,
}

impl Task {
    /// Creates a new task
    pub fn new<S: AsRef<str>>(path: S) -> Self {
        Self {
            inner: shared(Finalize::new(TaskInner {
                path: path.as_ref().to_string(),
            })),
        }
    }

    pub async fn path(&self) -> String {
        self.inner.read().await.path.clone()
    }
}

/// Convenience struct for stopping a task early.
pub struct TaskActions;

impl TaskActions {
    /// Stop the task action
    pub fn stop_action(&self) -> Result {
        Err(TaskError::StopAction(None))
    }

    /// Stop the task
    pub fn stop_task(&self) -> Result {
        Err(TaskError::StopTask(None))
    }

    pub fn fail<E: Into<Error>>(err: E) -> Result {
        Err(TaskError::fail(err))
    }
}

/// A single action in a task
pub trait TaskAction: Send {
    fn execute<'a>(
        &'a mut self,
        task: Task,
        project: Project,
    ) -> impl Future<Output = Result> + Send + 'a;
}

pub struct BoxTaskAction {
    inner: SyncWrapper<
        Box<dyn FnMut(Task, Project) -> Pin<Box<dyn Future<Output = Result> + Send>> + Send>,
    >,
}

impl BoxTaskAction {
    pub fn new<A>(inner: A) -> Self
    where
        A: TaskAction + Send + 'static,
    {
        let mut inner = Arc::new(Mutex::new(inner));
        let inner: Box<
            dyn FnMut(Task, Project) -> Pin<Box<dyn Future<Output = Result> + Send>> + Send,
        > = Box::new(move |task, project| {
            let mut inner = inner.clone();
            Box::pin(async move {
                let arc = inner;
                let mut inner = arc.lock().await;
                inner.execute(task, project).await
            })
        });

        let sync_wrapper = SyncWrapper::new(inner);
        Self {
            inner: sync_wrapper,
        }
    }
}

/// Creates a [`TaskAction`] from a function
pub fn from_fn<F, Fut>(f: F) -> BoxTaskAction
where
    F: FnMut(Task, Project) -> Fut + Send + 'static,
    Fut: Future<Output = Result> + Send + 'static,
{
    let mut inner = Arc::new(Mutex::new(f));
    let inner: Box<
        dyn FnMut(Task, Project) -> Pin<Box<dyn Future<Output = Result> + Send>> + Send,
    > = Box::new(move |task, project| {
        let mut inner = inner.clone();
        Box::pin(async move {
            let arc = inner;
            let mut inner = arc.lock().await;
            inner(task, project).await
        })
    });

    let sync_wrapper = SyncWrapper::new(inner);
    BoxTaskAction {
        inner: sync_wrapper,
    }
}

impl TaskAction for BoxTaskAction {
    async fn execute(&mut self, task: Task, project: Project) -> Result {
        self.inner.get_mut()(task, project).await
    }
}

pub enum TaskError {
    Fail(Error),
    StopTask(Option<Error>),
    StopAction(Option<Error>),
}

impl TaskError {
    pub fn fail<E: Into<Error>>(e: E) -> Self {
        Self::Fail(e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn run(task: Task, project: Project) -> Result {
        Ok(())
    }

    #[tokio::test]
    async fn test_task_action() {
        let mut task_action = from_fn(run);
        // let boxed = BoxTaskAction::new(task_action);
        let task = Task::new(":default");
        let project = Project {};
        let result = task_action.execute(task, project).await;
    }
}
