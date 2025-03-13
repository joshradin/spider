use crate::lazy::properties::sealed::Sealed;
use crate::lazy::providers::{Provider, Provides};
use parking_lot::Mutex;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

enum PropertyState<T: Clone> {
    None,
    Provider(Provider<T>),
    Immediate(T),
}

/// A property that stores a value of some type `T`, which can either be directly set or have it set
/// from a [`Provider`].
#[derive(Clone)]
pub struct Property<T: Clone> {
    id: Option<String>,
    inner: Arc<Mutex<PropertyState<T>>>,
}

impl<T: Send + Sync + Clone> Property<T> {
    pub(crate) fn empty(id: impl Into<Option<String>>) -> Self {
        let id = id.into();
        Self {
            id,
            inner: Arc::new(Mutex::new(PropertyState::None)),
        }
    }
}

impl<T: Send + Sync + Clone + 'static> Provides for Property<T> {
    type Output = T;

    fn provider(&self) -> Provider<Self::Output>
    where
        Self::Output: Clone,
    {
        Provider::from(self.clone())
    }

    fn get(&self) -> T {
        match self.try_get() {
            None => {
                panic!("{} has no value available", self)
            }
            Some(t) => t,
        }
    }

    fn try_get(&self) -> Option<T> {
        let mut inner = self.inner.lock();
        match &*inner {
            PropertyState::None => None,
            PropertyState::Provider(p) => p.try_get(),
            PropertyState::Immediate(i) => Some(i.clone()),
        }
    }
}

impl<T: Clone> Debug for Property<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "property {}",
            self.id.as_ref().map(String::as_str).unwrap_or("?")
        )
    }
}

impl<T: Clone> Display for Property<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

pub trait SetProperty<T>: Sealed {
    fn set(&mut self, value: T);
}

impl<T: Clone + Send + Sync> SetProperty<T> for Property<T> {
    fn set(&mut self, value: T) {
        *self.inner.lock() = PropertyState::Immediate(value);
    }
}

impl<T: Clone + Send + Sync + 'static> SetProperty<&Property<T>> for Property<T> {
    fn set(&mut self, value: &Property<T>) {
        *self.inner.lock() = PropertyState::Provider(Provider::from(value.clone()))
    }
}

impl<T: Clone + Send + Sync> SetProperty<&Provider<T>> for Property<T> {
    fn set(&mut self, value: &Provider<T>) {
        *self.inner.lock() = PropertyState::Provider(value.clone())
    }
}
mod sealed {
    use crate::lazy::properties::Property;

    pub trait Sealed {}
    impl<T: Clone> Sealed for Property<T> {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lazy::providers::ProviderFactory;

    #[test]
    fn test_prop_from_provider() {
        let factory = ProviderFactory::new(());
        let provider = factory.provider(|| "hello, world!");

        let mut property: Property<&str> = Property::empty(None);
        assert_eq!(property.try_get(), None);
        property.set(&provider);
        assert_eq!(property.try_get(), Some("hello, world!"));
    }

    #[test]
    fn test_prop_from_prop() {
        let factory = ProviderFactory::new(());
        let provider = factory.provider(|| "hello, world!");

        let mut property: Property<&str> = Property::empty(None);
        property.set(&provider);
        let mut property2: Property<&str> = Property::empty(None);
        property2.set(&property);
        assert_eq!(property.try_get(), Some("hello, world!"));
    }
}
