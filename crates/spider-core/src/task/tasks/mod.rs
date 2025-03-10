//! The standard [`TaskPrototype`] implementations.

use std::collections::VecDeque;
use crate::{Action, BoxAction, Task, TaskPrototype};
use crate::beans::{BeanProvider, Inject, NoBeans};
use crate::err::BuildResult;
use crate::named::{CreateNamed, Named};

/// Default task, no-op by default
#[derive(Default)]
pub struct DefaultTask;
impl NoBeans for DefaultTask {}
impl TaskPrototype for DefaultTask {
    type Inject = ();

    fn setup(_this: &mut Task<Self>) -> BuildResult {
        Ok(())
    }

    fn task_action(this: &mut Task<Self>) -> BuildResult {
        todo!()
    }
}
