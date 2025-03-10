
use crate::lazy::value_source::ValueSource;
use std::any::Any;
use std::sync::Arc;
use parking_lot::Mutex;

mod factory;
pub use factory::*;

/// Provides a value
pub trait Provider<T: Send>: Clone + Send + Sync {
    fn get(&self) -> T {
        self.try_get().expect("value is not ready yet")
    }
    fn try_get(&self) -> Option<T>;

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
        properties: Vs::Properties,
        vs: Vs,
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
    fn try_get(&self) -> Option<Vs::Output> {
        let mut lock = self.inner.lock();
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
                cfg_cb(&mut properties);

                let value = vs.get(&properties);
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
enum JustProviderInner<T> {
    Future(Box<dyn Fn() -> T + Send>),
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
    fn try_get(&self) -> Option<T> {
        let mut inner = self.inner.lock();
        match &mut *inner {
            JustProviderInner::Future(fut) => {
                let gotten = fut();
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
        Arc<dyn for<'a> Fn(&'a (dyn Any + Send + Sync)) ->  Option<T> + Send + Sync>,
}

impl<T: Send + Sync> Provider<T> for BoxProvider<T> {
    fn try_get(&self) -> Option<T> {
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
                {
                    let as_p: &P = any.downcast_ref().expect("should not fail");
                    let t = as_p.try_get();
                    t
                }
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lazy::providers::Provider;
    use crate::lazy::value_source::ValueSource;
    use std::time::Instant;
    use crate::lazy::providers::factory::ProviderFactory;

    #[test]
     fn test_just_provider() {
        let factory = ProviderFactory::new();
        let s = factory.provider(|| { 13 });
        let p = s.clone();
        assert_eq!(s.get(), 13);
        assert_eq!(p.get(), 13);
    }

    #[test]
    fn test_just_provider_to_boxed() {
        let factory = ProviderFactory::new();
        let s = factory.provider(|| { 13 }).into_boxed();
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
        let factory = ProviderFactory::new();
        let vs = factory.of::<InstantValueSource>();
        let now = vs.get();
        let now2 = vs.get();
        assert_eq!(now, now2);
    }
}
