use crate::from_state::{Create, FromState};
use crate::lazy::value_source::ValueSource;
use futures::future::BoxFuture;
use pin_project::pin_project;
use std::any::Any;
use std::sync::Arc;
use tokio::sync::Mutex;

mod factory;
pub use factory::*;

/// Provides a value
pub trait Provider<T: Send>: Clone + Send + Sync {
    fn get(&self) -> impl Future<Output=T> + Send {
        async { self.try_get().await.expect("value is not ready yet") }
    }
    fn try_get(&self) -> impl Future<Output=Option<T>> + Send;

    fn into_boxed(self) -> BoxProvider<T>
    where
        T: 'static,
        Self: 'static,
    {
        BoxProvider::new(self)
    }
}

enum ValueSourceProviderInner<Vs: ValueSource> {
    Poisoned,
    Futures {
        properties: BoxFuture<'static, Vs::Properties>,
        vs: BoxFuture<'static, Vs>,
        cfg_cb: Box<dyn FnOnce(&mut Vs::Properties) + Send + Sync>,
    },
    Gotten(Option<Vs::Output>),
}

struct ValueSourceProvider<Vs: ValueSource> {
    inner: Arc<Mutex<ValueSourceProviderInner<Vs>>>,
}

impl<Vs: ValueSource> Clone for ValueSourceProvider<Vs> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<Vs: ValueSource + Send> Provider<Vs::Output> for ValueSourceProvider<Vs>
where
    Vs::Output: Clone,
{
    async fn try_get(&self) -> Option<Vs::Output> {
        let mut lock = self.inner.lock().await;
        match &mut *lock {
            ValueSourceProviderInner::Futures { .. } => {
                let ValueSourceProviderInner::Futures {
                    mut properties,
                    mut vs,
                    cfg_cb,
                } = std::mem::replace(&mut *lock, ValueSourceProviderInner::Poisoned)
                else {
                    unreachable!()
                };
                let mut properties = properties.as_mut().await;
                let vs = vs.await;

                cfg_cb(&mut properties);

                let value = vs.get(&properties).await;
                *lock = ValueSourceProviderInner::Gotten(value.clone());
                value
            }
            ValueSourceProviderInner::Gotten(t) => {
                t.clone()
            }
            _ => {
                panic!("value panicked")
            }
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
    async fn try_get(&self) -> Option<T> {
        let mut inner = self.inner.lock().await;
        match &mut *inner {
            JustProviderInner::Future(fut) => {
                let gotten = fut.await;
                *inner = JustProviderInner::Gotten(gotten.clone());
                Some(gotten)
            }
            JustProviderInner::Gotten(g) => Some(g.clone()),
        }
    }
}

/// A box provider, wrapping any provider type
pub struct BoxProvider<T> {
    any: Arc<dyn Any + Send + Sync>,
    vtable:
        Arc<dyn for<'a> Fn(&'a (dyn Any + Send + Sync)) -> BoxFuture<'a, Option<T>> + Send + Sync>,
}

impl<T: Send + Sync> Provider<T> for BoxProvider<T> {
    fn try_get(&self) -> impl Future<Output=Option<T>> + Send {
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
                    let t = as_p.try_get().await;
                    t
                })
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lazy::providers::Provider;
    use crate::lazy::value_source::ValueSource;
    use std::time::Instant;
    use tokio::test;
    use crate::lazy::providers::factory::ProviderFactory;

    #[test]
    async fn test_just_provider() {
        let factory = ProviderFactory::new();
        let s = factory.provider(async { 13 });
        let p = s.clone();
        assert_eq!(s.get().await, 13);
        assert_eq!(p.get().await, 13);
    }

    #[test]
    async fn test_just_provider_to_boxed() {
        let factory = ProviderFactory::new();
        let s = factory.provider(async { 13 }).into_boxed();
        let p = s.clone();
        assert_eq!(s.get().await, 13);
        assert_eq!(p.get().await, 13);
    }

    #[derive(Default)]
    struct InstantValueSource;
    impl ValueSource for InstantValueSource {
        type Properties = ();
        type Output = Instant;

        async fn get(self, properties: &Self::Properties) -> Option<Instant> {
            Some(Instant::now())
        }
    }

    #[test]
    async fn test_value_source_provider() {
        let factory = ProviderFactory::new();
        let vs = factory.of::<InstantValueSource>();
        let now = vs.get().await;
        let now2 = vs.get().await;
        assert_eq!(now, now2);
    }
}
