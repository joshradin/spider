use crate::beans::{BeanError, BeansParam};

pub mod functions_cons;

pub trait BeanConstructor<Marker> {
    type Params: BeansParam;
    type Out: Send + Sync + 'static;

    fn build<'beans, 'state>(self, params: <Self::Params as BeansParam>::Item<'_, '_>) -> Result<Self::Out, BeanError>;
}

