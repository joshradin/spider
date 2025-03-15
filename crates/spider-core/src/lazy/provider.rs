//! The traits for all value providers.

mod properties;
mod providers;
mod provider_factory;

use crate::lazy::provider::providers::{AndThenProvider, FlatMapProvider, MapProvider};
pub(crate) use properties::*;
pub use providers::BoxProvider;
pub use provider_factory::ProviderFactory;
use std::collections::HashSet;

/// Source of value in a provider
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProviderSource {}

/// A container object that provides a value of a specific type
pub trait Provider<T>: Clone + Send + Sync + 'static {
    /// Tries to get the value stored in this container, if available
    fn try_get(&self) -> Option<T>;
    /// Gets the value stored in this container, panicking if no value is present
    fn get(&self) -> T {
        self.try_get().expect("value is not set")
    }

    /// Gets the sources of this provider's value
    fn sources(&self) -> HashSet<ProviderSource>;
}

/// Provider extension trait
pub trait ProviderExt<T>: Provider<T>
where
    T: Send + Sync + 'static,
{
    /// Map the output of a type into a different type
    fn map<U, F>(self, f: F) -> impl Provider<U>
    where
        U: Send + Sync + 'static,
        F: Send + Sync,
        F: Fn(T) -> U + 'static,
        Self: Sized,
    {
        MapProvider::new(self, f)
    }
    /// Fallibly maps the output
    fn and_then<U, F>(self, f: F) -> impl Provider<U>
    where
        U: Send + Sync + 'static,
        F: Fn(T) -> Option<U> + Send + Sync + 'static,
        Self: Sized,
    {
        AndThenProvider::new(self, f)
    }
    /// Output into a different provider
    fn flat_map<P2, U, F>(self, f: F) -> impl Provider<U>
    where
        U: Send + Sync + 'static,
        P2: Provider<U>,
        F: Fn(T) -> P2 + Send + Sync + 'static,
        Self: Sized,
    {
        FlatMapProvider::new(self, f)
    }
}

impl<T: Send + Sync + 'static, P: Provider<T>> ProviderExt<T> for P {}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;
    use crate::lazy::provider::ProviderExt;
    use crate::lazy::value_source::ValueSource;

    #[test]
    fn test_map_provider() {
        let factory = ProviderFactory::default();
        let provider = factory.just(13);
        let mapped = provider.map(|i| i * i);
        assert_eq!(mapped.get(), 169);
    }

    #[test]
    fn test_flat_map_provider() {
        let factory = ProviderFactory::default();
        let provider = factory.just(13);
        let mapped = provider.flat_map(move |i| factory.just(i * i));
        assert_eq!(mapped.get(), 169);
    }

    #[test]
    fn test_and_then_provider() {
        let factory = ProviderFactory::default();
        let provider = factory.just(13);
        let mapped = provider.and_then(|i| Some(i * i));
        assert_eq!(mapped.get(), 169);
    }

    #[test]
    fn test_value_source_provider() {
        #[derive(Default)]
        struct NowValueSource;
        impl ValueSource for NowValueSource {
            type Properties = ();
            type Output = Instant;

            fn get(self, _properties: &Self::Properties) -> Option<Self::Output> {
                Some(Instant::now())
            }
        }

        let factory = ProviderFactory::default();
        let provider = factory.value_source(NowValueSource);
        let p = provider.get();
        let p2 = provider.get();
        assert_eq!(p, p2);
    }

    #[test]
    fn test_value_source_provider_with() {
        #[derive(Default)]
        struct SimpleValueSource<T: Clone>(T);
        impl<T: Clone + Send + Sync + 'static> ValueSource for SimpleValueSource<T> {
            type Properties = ();
            type Output = T;

            fn get(self, _properties: &Self::Properties) -> Option<Self::Output> {
                Some(self.0.clone())
            }
        }

        let factory = ProviderFactory::default();
        let provider = factory.value_source_with(SimpleValueSource(32i32), |mut props| {
            println!("config")
        });
        let p = provider.get();
        let p2 = provider.get();
        assert_eq!(p, p2);
    }

    #[test]
    fn test_boxed_flat_map_provider() {
        let factory = ProviderFactory::default();
        let provider = factory.just(13);
        let mapped = BoxProvider::new(provider.flat_map(move |i| factory.just(i * i)));
        assert_eq!(mapped.get(), 169);
    }

    #[test]
    fn test_set_property() {
        // let mut property = Property::new("test");
    }
}
