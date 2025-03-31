use crate::action::Action;
use crate::beans::{BeanProvider, FromBeanProvider};
use crate::lazy::provider::Provider;
use crate::lazy::provider::providers::{JustProvider, ProducerProvider, ValueSourceProvider};
use crate::lazy::value_source::ValueSource;

/// A provider factory
#[derive(Clone)]
pub struct ProviderFactory {}

impl ProviderFactory {
    pub(crate) fn new() -> Self {
        Self {}
    }

    /// Creates a provider that just returns a given value
    pub fn just<T: Clone + Send + Sync + 'static>(
        &self,
        t: T,
    ) -> impl Provider<T> + use<T> + 'static {
        JustProvider::new(t)
    }

    /// Creates a provider from a function
    pub fn provider<T: Send + Sync + 'static, F>(
        &self,
        p: F,
    ) -> impl Provider<T> + use<T, F> + 'static
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        ProducerProvider::new(p)
    }

    /// Creates a provider from a value source
    pub fn value_source<Vs, U>(&self, vs: Vs) -> impl Provider<Vs::Output> + use<Vs, U> + 'static
    where
        Vs: ValueSource + Send + Sync + 'static,
        ProviderFactory: BeanProvider<U>,
        Vs::Properties: FromBeanProvider<U>,
        Vs::Output: Clone + Send + Sync + 'static,
        U: Send + Sync + 'static,
    {
        let props = Vs::Properties::from_bean_provider(self);
        ValueSourceProvider::<Vs>::new(vs, props)
    }

    /// Creates a provider from a value source, configuring the created properties
    pub fn value_source_with<Vs, U, A: FnOnce(&mut Vs::Properties)>(
        &self,
        vs: Vs,
        action: A,
    ) -> impl Provider<Vs::Output> + use<Vs, U, A> + 'static
    where
        Vs: ValueSource + Send + Sync + 'static,
        ProviderFactory: BeanProvider<U>,
        Vs::Properties: FromBeanProvider<U>,
        Vs::Output: Clone + Send + Sync + 'static,
        U: Send + Sync + 'static,
    {
        let mut props = Vs::Properties::from_bean_provider(self);
        action(&mut props);
        ValueSourceProvider::<Vs>::new(vs, props)
    }
}
