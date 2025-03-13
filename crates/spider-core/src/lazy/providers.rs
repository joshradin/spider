use crate::lazy::value_source::ValueSource;
use std::any::Any;
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, LazyLock};

mod factory;
use crate::lazy::properties::Property;
pub use factory::*;

/// A provider of a value of type `T`.
#[derive(Clone)]
pub struct Provider<T: Clone> {
    pub(super) kind: Arc<ProviderKind<T>>,
}

impl<T: Clone + Debug> Debug for Provider<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Provider").field(&self.try_get()).finish()
    }
}

#[derive(Clone)]
pub(super) enum ProviderKind<T: Clone> {
    None,
    Just(T),
    Callable(Arc<LazyLock<Option<T>, Box<dyn FnOnce() -> Option<T> + Send + Sync>>>),
    Provider(Provider<T>),
}

impl<T: Clone + Sync + Send + 'static> From<Property<T>> for Provider<T> {
    fn from(value: Property<T>) -> Self {
        from_fallible_callable(move || value.try_get())
    }
}

impl<T: Clone> Provides for Provider<T> {
    type Output = T;

    fn provider(&self) -> Provider<Self::Output> {
        self.clone()
    }

    fn try_get(&self) -> Option<T> {
        match &*self.kind {
            ProviderKind::None => None,
            ProviderKind::Just(j) => Some(j.clone()),
            ProviderKind::Callable(c) => {
                let lazy = Option::<T>::clone(c);
                lazy
            }
            ProviderKind::Provider(p) => p.try_get(),
        }
    }

    fn map<U>(
        self,
        f: impl FnOnce(Self::Output) -> U + Send + Sync + Sized + 'static,
    ) -> Provider<U>
    where
        Self: Sized + Send + Sync + 'static,
        U: Clone + Send + Sync + 'static,
    {
        wrap(map(self, f))
    }

    fn and_then<U>(
        self,
        f: impl FnOnce(Self::Output) -> Option<U> + Send + Sync + 'static,
    ) -> Provider<U>
    where
        Self: Sized + Send + Sync + 'static,
        U: Clone + Send + Sync + 'static,
    {
        wrap(and_then(self, f))
    }

    fn zip<U>(
        self,
        other: impl Provides<Output = U> + Send + Sync + 'static,
    ) -> Provider<(Self::Output, U)>
    where
        Self::Output: Clone + Send + Sync + 'static,
        Self: Sized + Send + Sync + 'static,
        U: Clone + Send + Sync + 'static,
    {
        todo!()
    }
}

/// Maps the output of one provider into another
pub fn map<P, T, U, F>(p: P, f: F) -> impl Provides<Output = U>
where
    P: Provides<Output = T> + Send + Sync + 'static,
    U: Clone + Send,
    F: FnOnce(T) -> U + Send + Sync + 'static,
{
    from_fallible_callable(move || {
        let t = p.try_get()?;
        let u = f(t);
        Some(u)
    })
}

/// Maps the output of one provider into another
pub fn and_then<P, T, U, F>(p: P, f: F) -> impl Provides<Output = U>
where
    P: Provides<Output = T> + Send + Sync + 'static,
    U: Clone + Send,
    F: FnOnce(T) -> Option<U> + Send + Sync + 'static,
{
    from_fallible_callable(move || {
        let t = p.try_get()?;
        let u = f(t)?;
        Some(u)
    })
}

/// Maps the output of one provider into another
pub fn zip<P1, T, P2, U>(p: P1, f: P2) -> impl Provides<Output = (T, U)>
where
    P1: Provides<Output = T> + Send + Sync + 'static,
    P2: Provides<Output = U> + Send + Sync + 'static,
    U: Clone + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{
    from_fallible_callable(move || {
        let t = p.try_get()?;
        let u = f.try_get()?;
        Some((t, u))
    })
}

/// A type which provides a value of type `t`.
pub trait Provides {
    type Output;

    fn provider(&self) -> Provider<Self::Output>
    where
        Self::Output: Clone;

    fn get(&self) -> Self::Output
    where
        Self: Debug,
    {
        match self.try_get() {
            None => {
                panic!("{:?} has no value available", self)
            }
            Some(t) => t,
        }
    }

    fn try_get(&self) -> Option<Self::Output>;

    fn map<U>(
        self,
        f: impl FnOnce(Self::Output) -> U + Send + Sync + Sized + 'static,
    ) -> impl Provides<Output = U>
    where
        Self: Sized + Send + Sync + 'static,
        U: Clone + Send + Sync + 'static,
    {
        map(self, f)
    }

    /// Maps the output of one provider into another
    fn and_then<U>(
        self,
        f: impl FnOnce(Self::Output) -> Option<U> + Send + Sync + 'static,
    ) -> impl Provides<Output = U>
    where
        Self: Sized + Send + Sync + 'static,
        U: Clone + Send + Sync + 'static,
    {
        and_then(self, f)
    }

    /// Zips two providers together
    fn zip<U>(
        self,
        other: impl Provides<Output = U> + Send + Sync + 'static,
    ) -> impl Provides<Output = (Self::Output, U)>
    where
        Self::Output: Clone + Send + Sync + 'static,
        Self: Sized + Send + Sync + 'static,
        U: Clone + Send + Sync + 'static,
    {
        from_fallible_callable(move || {
            let t = self.try_get()?;
            let u = other.try_get()?;
            Some((t, u))
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::lazy::providers::Provides;
    use crate::lazy::providers::factory::ProviderFactory;
    use crate::lazy::value_source::ValueSource;
    use std::time::Instant;

    #[test]
    fn test_just_provider() {
        let factory = ProviderFactory::new(());
        let s = factory.provider(|| 13);
        let p = s.clone();
        assert_eq!(s.get(), 13);
        assert_eq!(p.get(), 13);
    }

    #[test]
    fn test_just_provider_to_boxed() {
        let factory = ProviderFactory::new(());
        let s = factory.provider(|| 13);
        let p = s.clone();
        assert_eq!(s.get(), 13);
        assert_eq!(p.get(), 13);
    }

    #[derive(Default)]
    struct InstantValueSource;
    impl ValueSource for InstantValueSource {
        type Properties = ();
        type Output = Instant;
        fn get(self, properties: &Self::Properties) -> Option<Instant> {
            Some(Instant::now())
        }
    }

    #[test]
    fn test_value_source_provider() {
        let factory = ProviderFactory::new(());
        let vs = factory.of::<InstantValueSource, _>();
        let now = vs.get();
        let now2 = vs.get();
        assert_eq!(now, now2);
    }

    #[test]
    fn test_provider_map() {
        let factory = ProviderFactory::new(());
        let s = factory.provider(|| 0);
        let mapped = s.map(|i| i + 2);
        assert_eq!(mapped.get(), 2);
    }

    #[test]
    fn test_provider_and_then() {
        let factory = ProviderFactory::new(());
        let s = factory.provider(|| 0);
        let mapped = s.and_then(|i| Some(i + 2));
        assert_eq!(mapped.get(), 2);
    }

    #[test]
    fn test_provider_zip() {
        let factory = ProviderFactory::new(());
        let s = factory.provider(|| 1);
        let t = factory.provider(|| 2);
        let mapped = s.zip(t);
        assert_eq!(mapped.get(), (1, 2));
    }
}
