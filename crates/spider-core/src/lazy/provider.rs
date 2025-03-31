//! The traits for all value providers.

mod providers;

use crate::lazy::provider::providers::{AndThenProvider, FlatMapProvider, MapProvider};
use std::collections::HashSet;

/// Source of value in a provider
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProviderSource {
    Task(),
}

/// A container object that provides a value of a specific type
pub trait Provider<T: Send + Sync + 'static>: Clone + Send + Sync + 'static {
    /// Tries to get the value stored in this container, if available
    fn try_get(&self) -> impl Future<Output = Option<T>> + Send;
    /// Gets the value stored in this container, panicking if no value is present
    fn get(&self) -> impl Future<Output = T> {
        async { self.try_get().await.expect("value is not set") }
    }

    /// Gets the sources of this provider's value
    fn sources(&self) -> HashSet<ProviderSource>;
}

/// A property represents a configurable value of type `T`
pub trait Property<T: Send + Sync + 'static>: Provider<T> {
    /// Sets the value of this property
    fn set(&mut self, value: T) -> impl Future<Output = ()> + Send + Sync;
    /// Sets the value of this property from a provider
    fn set_from(&mut self, provider: &impl Provider<T>) -> impl Future<Output = ()> + Send + Sync
    where
        T: 'static;
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
