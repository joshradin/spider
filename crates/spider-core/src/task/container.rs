//! A container of [`TaskPrototype`] objects.
//!
//! All tasks should be registered

use crate::beans::{BeanProvider, FromBeanProvider, NamedFromBeanProvider};
use crate::lazy::configure::ConfigureOnCreate;
use crate::provider::TaskProvider;
use crate::{Project, Task, TaskPrototype};
use std::sync::{Arc, Weak};

/// A task container
pub struct TaskContainer {
    owner: Weak<Project>,
    // tasks: HashMap<String, TaskProvider<()>>,
}

impl TaskContainer {
    pub(crate) fn new(owner: Weak<Project>) -> Self {
        Self { owner }
    }

    pub fn register<T: TaskPrototype>(&mut self, name: impl AsRef<str>) -> TaskProvider<T>
    {
        let name = name.as_ref().to_string();
        let name_c = name.clone();

        let project = self.owner.clone();
        let configure_on_create = ConfigureOnCreate::<Task<T>>::new(move || {
            let project = project.upgrade().expect("Could not get project instance");
            // T::build(&project)
            todo!()
        });

        let provider = TaskProvider::<T>::new(name_c, configure_on_create);
        provider
    }
    //
    // pub fn tasks(&self) -> Vec<TaskProvider<BoxTask>> {
    //     todo!()
    // }
}

#[cfg(test)]
mod tests {
    use crate::named::Named;
    use crate::tasks::DefaultTask;
    use crate::Project;

    #[test]
    fn test_register_task() {
        let project = Project::new();
        let t = project.tasks_mut().register::<DefaultTask>("test");
        assert_eq!(t.name(), "test");
        // let get = t.get();
    }
}
