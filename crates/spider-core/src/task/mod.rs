//! A [`TaskPrototype`] represents a single atomic piece of work for a build

use crate::beans::{BeanProvider, BeanProviderExt, FromBeanProvider, Inject};
use crate::err::BuildResult;
use crate::named::{CreateNamed, Named};
use crate::tasks::DefaultTask;
use crate::{Action, BoxAction, Project};
use std::any::Any;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Weak};

pub mod container;
pub mod provider;
pub mod tasks;

pub struct TaskId<T: TaskPrototype + ?Sized>(String, PhantomData<T>);


pub struct Task<T: TaskPrototype + ?Sized> {
    name: String,
    state: Box<T>,
    actions: VecDeque<BoxAction<'static, Task<T>, BuildResult>>,
    project: Weak<Project>,
}

impl<T: TaskPrototype + ?Sized> Deref for Task<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}
impl<T: TaskPrototype + ?Sized> DerefMut for Task<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}


impl<T: TaskPrototype > Task<T> {
    pub(crate) fn new(name: String, provider: &Arc<Project>, state: T) -> Self
    {
        // let obj_safe = object_safe_task_action();

        let project = Arc::downgrade(&provider.get_bean());

        Self {
            name,
            state: Box::new(state),
            actions: VecDeque::from([]),
            // _parent_ty: Default::default(),
            project,
        }
    }

    /// Get a reference to the owning project
    pub fn project(&self) -> Arc<Project> {
        self.project.upgrade().unwrap()
    }

    pub fn do_first<U>(&mut self, action: impl Action<U>) {}
}

impl<T: TaskPrototype + ?Sized> Inject<Arc<Project>> for Task<T> {
    fn inject<P>(&mut self, bean_provider: &P)
    where
        P: BeanProvider<Arc<Project>> + ?Sized,
    {
    }
}

/// A [`TaskPrototype`] represents a single atomic piece of work for a build
pub trait TaskPrototype: 'static
    where Self: FromBeanProvider<Self::Inject>
{
    type Inject;

    /// Setups the task
    fn setup(this: &mut Task<Self>) -> BuildResult;
    fn task_action(this: &mut Task<Self>) -> BuildResult;
}

fn build_task() {}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::err::BuildResult;
    use crate::{Project, Task, TaskPrototype};
    use crate::beans::NoBeans;

    #[derive(Default)]
    struct SubTask {
        pub output_dir: PathBuf,
    }
    impl NoBeans for SubTask {}
    impl TaskPrototype for SubTask {
        type Inject = ();

        fn setup(this: &mut Task<Self>) -> BuildResult {
            todo!()
        }

        fn task_action(this: &mut Task<Self>) -> BuildResult {
            todo!()
        }
    }

    #[test]
    fn test_create_task() {
        let project = Project::new();
        let mut task: Task<SubTask> = todo!();
        task.output_dir = PathBuf::new();
        assert_eq!(task.name, "");
    }
}
