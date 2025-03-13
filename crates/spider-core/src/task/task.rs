//! A [`Task`] represents a single, atomic instance of work

use crate::action::Action;
use crate::error::Error;
use crate::table::Table;
use crate::task::TaskError::*;
use crate::Project;
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::sync::Weak;
use tracing::{info, trace};

pub type TaskAction = dyn for<'a> Action<&'a mut Task, Result<(), crate::task::TaskError>> + Sync + Send;

/// A fully configured task
pub struct Task {
    name: String,
    actions: VecDeque<Option<Box<TaskAction>>>,
    table: Table,
}

impl Task {
    /// Creates a new, empty task
    pub(crate) fn new(name: String) -> Self {
        Self {
            name,
            actions: Default::default(),
            table: Table::new(),
        }
    }

    pub fn table(&self) -> &Table {
        &self.table
    }

    pub(crate) fn set_table(&mut self, table: Table) {
        self.table = table;
    }

    /// Do the given action first
    pub fn do_first<A>(&mut self, action: A)
    where
        A: for<'a> Action<&'a mut Task, Result<(), crate::task::TaskError>> + Send + Sync + 'static,
    {
        self.actions.push_front(Some(Box::new(action)));
    }

    /// Do the given action last
    pub fn do_last<A>(&mut self, action: A)
    where
        A: for<'a> Action<&'a mut Task, Result<(), crate::task::TaskError>>+ Send + Sync + 'static,
    {
        self.actions.push_back(Some(Box::new(action)));
    }

    /// Runs this task
    pub(crate) fn run(&mut self) -> Result<(), Error> {
        let action_count = self.actions.len();
        for i in 0..action_count {
            let action = self.actions[i].take().expect("action must not missing");
            let result = action.execute(self);
            match result {
                Ok(_) => {}
                Err(Error(e)) => {
                    return Err(e);
                }
                Err(StopAction(e)) => {
                    trace!("action {} stopped: {e:?}", i);
                }
                Err(StopTask(e)) => {
                    trace!("task {} stopped: {e:?}", i);
                }
            }
        }
        Ok(())
    }
}

impl Deref for Task {
    type Target = Table;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}
impl DerefMut for Task {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_task() {
        let mut task = Task::new("test".to_string());
        task.set("hello","hello");
        task.do_first(|task: &mut Task| {
            task.get::<&str>("hello")?;
            Ok(())
        });
        task.run().expect("run failed");
    }
}
