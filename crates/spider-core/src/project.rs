use crate::beans::BeanProvider;
// use crate::lazy::provider::ProviderFactory;

#[derive(Debug)]
pub struct Project {
    // provider_factory: ProviderFactory,
    // tasks: Shared<TaskContainer>,
}

impl Project {}

// impl BeanProvider<ProviderFactory> for Project {
//     fn get_bean(&self) -> ProviderFactory {
//         ProviderFactory::new()
//     }
// }

#[cfg(test)]
mod tests {
    // use crate::beans::BeanProvider;
    // use crate::lazy::provider::ProviderFactory;
    // use crate::project::Project;
    //
    // #[test]
    // fn test_register_task() {
    //     let project = Project {
    //         provider_factory: ProviderFactory::new(),
    //     };
    //     let provider: ProviderFactory = project.get_bean();
    //     let provider = provider.provider(|| {});
    // }
}
