use std::sync::Arc;
use crate::beans::BeanProvider;
use crate::container::TaskContainer;
use crate::lazy::providers::Provider;
use crate::shared::{shared, Ref, RefMut, Shared};


pub struct Project {
    tasks: Shared<TaskContainer>,
}

impl Project {
    pub(crate) fn new() -> Arc<Project> {
        Arc::<Project>::new_cyclic(|project| {
            let tasks = TaskContainer::new(project.clone());

            Self {
                tasks: shared(tasks),
            }
        })
    }

    pub fn tasks(&self) -> Ref<TaskContainer> {
        self.tasks.read()
    }

    pub fn tasks_mut(&self) -> RefMut<TaskContainer> {
        self.tasks.write()
    }
}

impl BeanProvider<Arc<Project>> for Arc<Project> {
    fn get_bean(&self) -> Arc<Project> {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::tasks::DefaultTask;
    use super::*;

    #[test]
    fn test_register_task() {
        let project = Project::new();
        // let mut task = project.tasks_mut().register::<DefaultTask, _>("example");
        // task.configure(|task| {
        //
        // });
    }
}