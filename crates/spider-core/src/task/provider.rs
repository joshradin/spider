//! a [`Task`] [`Provider`]

use crate::beans::{BeanProvider, BeanProviderExt, FromBeanProvider, NamedFromBeanProvider};
use crate::lazy::configure::ConfigureOnCreate;
use crate::lazy::providers::Provider;
use crate::named::Named;
use crate::shared::{shared, Shared};
use crate::tasks::DefaultTask;
use crate::{Task, TaskPrototype};
use static_assertions::assert_impl_all;

assert_impl_all!(TaskProvider<DefaultTask>: Clone);

pub struct TaskProvider<T: TaskPrototype + ?Sized + 'static> {
    name: String,
    inner: Shared<ConfigureOnCreate<Task<T>>>,
}

impl<T: TaskPrototype + ?Sized + 'static> TaskProvider<T> {
    /// Create a new task provider with a given construction
    pub(crate) fn new(
        name: impl AsRef<str>,
        configure_on_create: ConfigureOnCreate<Task<T>>,
    ) -> Self {
        let provider = Self {
            name: name.as_ref().to_string(),
            inner: shared(configure_on_create),
        };
        provider
    }

    /// Configures this task provider
    pub fn configure<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut Task<T>) + Send + Sync + 'static,
    {
        self.inner.write().configure(f);
        self
    }
}

impl<T: TaskPrototype + ?Sized + 'static> Clone for TaskProvider<T> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl<T: TaskPrototype + ?Sized + 'static> Named for TaskProvider<T> {
    fn name(&self) -> &str {
        &self.name
    }
}

// impl<T: TaskPrototype + 'static> Provider<Task<T>> for TaskProvider<T> {
//     fn try_get(&self) -> Option<&Task<T>> {
//         todo!()
//     }
// }

#[cfg(test)]
mod tests {
    use crate::Project;

    #[test]
    fn test_create_task_provider() {
        let project = Project::new();
        // let task_provider = project.tasks_mut().register::<DefaultTask, _>("");
    }
}
