use crate::lazy::provider::{BoxProvider, Provider, ProviderSource};
use crate::shared::Shared;
use static_assertions::assert_impl_all;
use std::collections::HashSet;

pub mod collections;

/// A [`Provider`] of type `T` that allows for setting contained value.
///
/// If not set, no value is returned.
///
///
pub struct Property<T: Sync> {
    inner: Shared<PropertyInner<T>>,
}

enum PropertyInner<T: Sync> {
    Empty,
    Just(T),
    Provided(BoxProvider<T>),
}

impl<T: Send + Sync + 'static> Property<T> {
    /// Sets the value of this property
    pub fn set(&mut self, value: T) {
        let mut write = self.inner.write();
        *write = PropertyInner::Just(value);
    }

    /// Sets the value of this property from a provider
    pub fn set_from(&mut self, provider: &impl Provider<T>)
    where
        T: 'static,
    {
        let mut write = self.inner.write();
        *write = PropertyInner::Provided(BoxProvider::new(provider.clone()));
    }
}

impl<T: Sync> Clone for Property<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
impl<T: Clone + Send + Sync + 'static> Provider<T> for Property<T> {
    fn try_get(&self) -> Option<T> {
        let read = self.inner.read();
        match &*read {
            PropertyInner::Just(just) => Some(just.clone()),
            PropertyInner::Provided(p) => p.try_get(),
            PropertyInner::Empty => None,
        }
    }

    fn sources(&self) -> HashSet<ProviderSource> {
        let read = self.inner.read();
        match &*read {
            PropertyInner::Provided(p) => p.sources(),
            _ => HashSet::new(),
        }
    }
}

assert_impl_all!(Property<i32>: Provider<i32>);
