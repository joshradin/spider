use crate::action::Action;
use crate::project::Project;
use crate::table::Table;
use crate::task::{Task, TaskAction, TaskError};
use static_assertions::assert_impl_all;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub type ConstructTask = dyn for<'a, 'b, 'c> Fn(&'a mut Task, &'b TaskPrototype) -> crate::error::Result<()>
    + Send
    + Sync;

/// A task prototype
pub struct TaskPrototype {
    parent: Option<Arc<TaskPrototype>>,
    table: Table,
    constructor: Box<ConstructTask>,
    task_action: Option<Arc<TaskAction>>,
}

assert_impl_all!(TaskPrototype: Sync);

impl TaskPrototype {
    /// Creates a new task prototype with no configuration
    pub fn new(parent: impl Into<Option<Arc<TaskPrototype>>>) -> Self {
        let mut table = Table::new();
        let parent = parent.into();
        if let Some(parent) = &parent {
            table.set_metatable(parent.table.clone());
        }
        Self {
            parent,
            table,
            constructor: create_constructor(|_, _| Ok(())),
            task_action: None,
        }
    }

    /// Sets the constructor for this task
    pub fn set_constructor<F>(&mut self, f: F)
    where
        for<'a, 'b, 'c> F:
            Fn(&'a mut Task, &'b TaskPrototype) -> crate::error::Result<()> + Send + Sync + 'static,
    {
        self.constructor = create_constructor(f)
    }

    /// Gets the task action for this task prototype
    ///
    /// Searches up the task prototype inheritance tree
    pub fn task_action(&self) -> Option<&Arc<TaskAction>> {
        match (&self.task_action, &self.parent) {
            (Some(t), _) => Some(t),
            (_, Some(parent)) => parent.task_action(),
            _ => None,
        }
    }

    /// Sets the task action
    pub fn set_task_action<A>(&mut self, a: A)
    where
        for<'a> A: Action<&'a mut Task, std::result::Result<(), TaskError>> + Send + Sync + 'static,
    {
        self.task_action = Some(Arc::new(a));
    }

    pub(crate) fn build(
        &self,
        name: impl AsRef<str>,
        project: &Project,
    ) -> crate::error::Result<Task> {
        let mut task = Task::new(name.as_ref().to_string());
        task.set_metatable(self.table.clone());
        if let Some(action) = self.task_action() {
            let action = action.clone();
            task.do_first(move |task: &mut Task| action.execute(task));
        }
        (&self.constructor)(&mut task, &self)?;
        Ok(task)
    }
}

fn create_constructor<F>(cons: F) -> Box<ConstructTask>
where
    F: for<'a, 'b, 'c> Fn(&'a mut Task, &'b TaskPrototype) -> crate::error::Result<()>
        + Send
        + Sync
        + 'static,
{
    let func = move |task: &mut Task, prototype: &TaskPrototype| {
        if let Some(parent) = &prototype.parent {
            (&parent.constructor)(task, &*parent)?;
            let table = task.table().clone();
            let mut new_table = Table::new();
            new_table.set_metatable(table);
            task.set_table(new_table);
        }
        cons(task, prototype)?;
        Ok(())
    };
    Box::new(func) as Box<ConstructTask>
}

impl Deref for TaskPrototype {
    type Target = Table;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl DerefMut for TaskPrototype {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_prototype() {
        let project = Project::default();
        let mut prototype = TaskPrototype::new(None);
        prototype.set_task_action(|task: &mut Task| {
            let x = task.get::<i32>("v").unwrap();
            println!("v: {:?}", x);
            assert_eq!(*x, 3);
            Ok(())
        });
        prototype.set_constructor(|mut task, _| {
            task.set("v", 3);
            Ok(())
        });
        let mut spawn = prototype
            .build("task", &project)
            .expect("could not build task");
        let a: i32 = *spawn.get("v").unwrap();
        assert_eq!(a, 3);
        spawn.run().expect("could not run task");
    }

    #[test]
    fn test_task_prototype_parent() {
        let project = Project::default();
        let mut prototype = TaskPrototype::new(None);
        prototype.set_task_action(|task: &mut Task| {
            let x = task.get::<i32>("v").unwrap();
            println!("v: {:?}", x);
            assert_eq!(*x, 3);
            Ok(())
        });
        prototype.set_constructor(|mut task, _| {
            task.set("v", 3);
            Ok(())
        });

        let mut child = TaskPrototype::new(Some(Arc::new(prototype)));
        child.set_task_action(|task: &mut Task| {
            let x = task.get::<i32>("v").unwrap();
            println!("v: {:?}", x);
            assert_eq!(*x, 9);
            Ok(())
        });
        child.set_constructor(|mut task, _| {
            let v = *task.get::<i32>("v")?;
            task.set("v", v * v);
            Ok(())
        });

        let mut spawn = child.build("task", &project).expect("could not build task");
        let a: i32 = *spawn.get("v").unwrap();
        assert_eq!(a, 9);
        spawn.run().expect("could not run task");
    }
}
