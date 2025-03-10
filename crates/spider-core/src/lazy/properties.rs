use crate::lazy::providers::{BoxProvider, Provider};
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use parking_lot::Mutex;

struct ProviderInner<T> {
    state: ProviderState<T>,
}

enum ProviderState<T> {
    Empty,
    Provider(BoxProvider<T>),
    Ready(T),
}

/// A property that stores a value of some type `T`, which can either be directly set or have it set
/// from a [`Provider`].
pub struct Property<T> {
    id: Option<String>,
    inner: Arc<Mutex<ProviderInner<T>>>,
}

impl<T: Send + Sync + Clone> Property<T> {
    pub(crate) fn empty(id: impl Into<Option<String>>) -> Self {
        let id = id.into();
        Self {
            id,
            inner: Arc::new(Mutex::new(ProviderInner {
                state: ProviderState::Empty,
            })),
        }
    }

    pub(crate) fn immediate(inner: T, id: impl Into<Option<String>>) -> Self {
        let id = id.into();
        Self {
            id,
            inner: Arc::new(Mutex::new(ProviderInner {
                state: ProviderState::Ready(inner),
            })),
        }
    }

    pub(crate) fn from(inner: impl Provider<T> + 'static, id: impl Into<Option<String>>) -> Self
    where
        T: 'static,
    {
        let id = id.into();
        Self {
            id,
            inner: Arc::new(Mutex::new(ProviderInner {
                state: ProviderState::Provider(inner.into_boxed()),
            })),
        }
    }
}

impl<T> Clone for Property<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl<T: Send + Sync + Clone> Provider<T> for Property<T> {
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
        match &mut inner.state {
            ProviderState::Empty => None,
            ProviderState::Provider(p) => {
                let value = p.get();
                inner.state = ProviderState::Ready(value.clone());
                Some(value)
            }
            ProviderState::Ready(value) => Some(value.clone()),
        }
    }
}

impl<T> Debug for Property<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "property {}", self.id.as_ref().map(String::as_str).unwrap_or("?"))
    }
}

impl<T> Display for Property<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lazy::providers::ProviderFactory;

    #[test]
    fn test_prop_from_provider() {
        let factory = ProviderFactory::new();
        let provider = factory.provider(|| { "hello, world!" });
        let property = Property::from(provider, None);
        assert_eq!(property.id, None);
        assert_eq!(property.get(), "hello, world!");
    }

    #[test]
    fn test_prop_from_prop() {
        let factory = ProviderFactory::new();
        let provider = Property::immediate("hello, world!", None);
        let property = Property::from(provider, "provider2".to_string());
        assert_eq!(property.get(), "hello, world!");
    }
}