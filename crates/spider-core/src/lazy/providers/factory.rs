use crate::beans::{BeanProvider, FromBeanProvider};
use crate::lazy::providers::{Provider, ProviderKind, Provides};
use crate::lazy::value_source::{ValueSource, into_callable_with};
use std::sync::{Arc, LazyLock};

#[derive(Debug)]
pub struct ProviderFactory<P>(P);

impl<P> ProviderFactory<P> {
    /// Creates a new empty provider
    pub(crate) fn new(t: P) -> Self {
        Self(t)
    }
}

impl<P> ProviderFactory<P> {
    /// Creates a provider that just generates a value from a future
    pub fn provider<T, U>(&self, just: U) -> Provider<T>
    where
        T: Send + Clone,
        U: FnOnce() -> T + Send + Sync + 'static,
    {
        from_callable(just)
    }

    pub fn wrap<T: Clone + Send>(
        &self,
        provides: impl Provides<Output = T> + Send + Sync + 'static,
    ) -> Provider<T> {
        wrap(provides)
    }

    /// Create a provider of a value source
    pub fn of<Vs, U>(&self) -> Provider<Vs::Output>
    where
        Vs: ValueSource<Output: Clone> + Default + Send + Sync + 'static,
        P: BeanProvider<U>,
        Vs::Properties: FromBeanProvider<U> + Send + Sync,
    {
        let vs = Vs::default();
        let properties: Vs::Properties = FromBeanProvider::from_bean_provider(&self.0);
        let callable = into_callable_with(vs, properties);
        let kind = ProviderKind::Callable(Arc::new(LazyLock::new(
            Box::new(callable) as Box<dyn FnOnce() -> Option<Vs::Output> + Send + Sync>
        )));
        Provider {
            kind: Arc::new(kind),
        }
    }

    /// Create a provider of a value source
    pub fn of_with<Vs, U>(
        &self,
        cfg: impl FnOnce(&mut Vs::Properties) + Send + Sync + 'static,
    ) -> Provider<Vs::Output>
    where
        Vs: ValueSource<Properties: Send, Output: Clone> + Default + Send + Sync + 'static,
        P: BeanProvider<U>,
        Vs::Properties: FromBeanProvider<U> + Send + Sync,
    {
        let vs = Vs::default();
        let mut properties: Vs::Properties = FromBeanProvider::from_bean_provider(&self.0);
        cfg(&mut properties);
        let callable = into_callable_with(vs, properties);
        let kind = ProviderKind::Callable(Arc::new(LazyLock::new(
            Box::new(callable) as Box<dyn FnOnce() -> Option<Vs::Output> + Send + Sync>
        )));
        Provider {
            kind: Arc::new(kind),
        }
    }
}

/// Creates a provider of `t`
pub(crate) fn provider<T: Clone>(t: T) -> Provider<T> {
    Provider {
        kind: Arc::new(
            ProviderKind::Just(t)
        )
    }
}

pub(crate) fn wrap<T: Clone + Send>(
    provides: impl Provides<Output = T> + Sync + Send + 'static,
) -> Provider<T> {
    from_fallible_callable(move || provides.try_get())
}

pub(crate) fn from_callable<T, U>(just: U) -> Provider<T>
where
    T: Send + Clone,
    U: FnOnce() -> T + Send + Sync + 'static,
{
    let kind = ProviderKind::Callable(Arc::new(LazyLock::new(
        Box::new(|| Some(just())) as Box<dyn FnOnce() -> Option<T> + Send + Sync>
    )));
    Provider {
        kind: Arc::new(kind),
    }
}

pub(crate) fn from_fallible_callable<T, U>(just: U) -> Provider<T>
where
    T: Send + Clone,
    U: FnOnce() -> Option<T> + Send + Sync + 'static,
{
    let kind = ProviderKind::Callable(Arc::new(LazyLock::new(
        Box::new(just) as Box<dyn FnOnce() -> Option<T> + Send + Sync>
    )));
    Provider {
        kind: Arc::new(kind),
    }
}
