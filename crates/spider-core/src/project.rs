use crate::beans::BeanProvider;
use crate::lazy::providers::Provider;
use crate::shared::{Ref, RefMut, Shared, shared};
use std::sync::Arc;

#[derive(Default)]
pub struct Project {
    // tasks: Shared<TaskContainer>,
}

impl Project {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_register_task() {}
}
