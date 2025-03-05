use crate::lazy::value_source::ValueSource;
use futures::future::BoxFuture;
use pin_project::pin_project;
use std::any::Any;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Provides a value
pub trait Provider<T: Send>: Clone + Send + Sync {
    fn get(&self) -> impl Future<Output = T> + Send;

    fn into_boxed(self) -> BoxProvider<T>
    where
        T: 'static,
        Self: 'static,
    {
        BoxProvider::new(self)
    }
}

#[derive(Debug, Default)]
pub struct ProviderFactory<S> {
    state: S,
}

impl<S> ProviderFactory<S> {
    /// Creates a provider that just generates a value from a future
    pub fn provider<T, U>(&self, just: U) -> impl Provider<T> + Clone + use<T, S, U>
    where
        T: Send + Clone,
        U: IntoFuture<Output = T, IntoFuture: Send + 'static>,
    {
        JustProvider {
            inner: Arc::new(Mutex::new(JustProviderInner::Future(Box::pin(
                just.into_future(),
            )))),
        }
    }
}

/// Provide just a value

#[pin_project]
enum JustProviderInner<T> {
    Future(BoxFuture<'static, T>),
    Gotten(T),
}

struct JustProvider<T> {
    inner: Arc<Mutex<JustProviderInner<T>>>,
}

impl<T> Clone for JustProvider<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Send + Clone> Provider<T> for JustProvider<T> {
    async fn get(&self) -> T {
        let mut inner = self.inner.lock().await;
        match &mut *inner {
            JustProviderInner::Future(fut) => {
                let gotten = fut.await;
                *inner = JustProviderInner::Gotten(gotten.clone());
                gotten
            }
            JustProviderInner::Gotten(g) => g.clone(),
        }
    }
}

/// A box provider, wrapping any provider type
pub struct BoxProvider<T> {
    any: Arc<dyn Any + Send + Sync>,
    vtable: Arc<dyn for<'a> Fn(&'a (dyn Any + Send + Sync)) -> BoxFuture<'a, T> + Send + Sync>,
}

impl<T: Send + Sync> Provider<T> for BoxProvider<T> {
    fn get(&self) -> impl Future<Output = T> + Send {
        let as_any = &*self.any;
        (self.vtable)(as_any)
    }

    fn into_boxed(self) -> BoxProvider<T>
    where
        T: 'static,
        Self: 'static,
    {
        self
    }
}

impl<T> Clone for BoxProvider<T> {
    fn clone(&self) -> Self {
        Self {
            any: self.any.clone(),
            vtable: self.vtable.clone(),
        }
    }
}

impl<T: Send + 'static> BoxProvider<T> {
    pub fn new<P>(value: P) -> Self
    where
        P: Provider<T> + Send + Sync + 'static,
    {
        Self {
            any: Arc::new(value),
            vtable: Arc::new(|any: &(dyn Any + Send + Sync)| {
                Box::pin(async {
                    let as_p: &P = any.downcast_ref().expect("should not fail");
                    let t = as_p.get().await;
                    t
                })
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lazy::providers::{Provider, ProviderFactory};
    use tokio::test;

    #[test]
    async fn test_just_provider() {
        let factory = ProviderFactory::<()>::default();
        let s = factory.provider(async { 13 });
        let p = s.clone();
        assert_eq!(s.get().await, 13);
        assert_eq!(p.get().await, 13);
    }

    #[test]
    async fn test_just_provider_to_boxed() {
        let factory = ProviderFactory::<()>::default();
        let s = factory.provider(async { 13 }).into_boxed();
        let p = s.clone();
        assert_eq!(s.get().await, 13);
        assert_eq!(p.get().await, 13);
    }
}
