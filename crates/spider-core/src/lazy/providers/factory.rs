use crate::lazy::providers::{JustProvider, JustProviderInner, Provider, ValueSourceProvider, ValueSourceProviderInner};
use crate::lazy::value_source::ValueSource;
use std::sync::Arc;
use parking_lot::Mutex;

#[derive(Debug)]
pub struct ProviderFactory();

impl ProviderFactory {
    /// Creates a new empty provider
    pub(crate) fn new() -> Self {
        Self()
    }
}

impl ProviderFactory {
    /// Creates a provider that just generates a value from a future
    pub fn provider<T, U>(&self, just: U) -> impl Provider<T> + Clone + use < T, U >
    where
        T: Send + Clone,
        U: Fn() -> T + Send + 'static,
    {
        JustProvider {
            inner: Arc::new(Mutex::new(JustProviderInner::Future(Box::new(just)),
            )),
        }
    }

    /// Create a provider of a value source
    pub fn of<Vs>(
        &self,
    ) -> impl Provider<Vs::Output>
    where
        Vs: ValueSource<Properties=(), Output: Clone>  + Send,
    {
        ValueSourceProvider::<Vs> {
            inner: Arc::new(Mutex::new(ValueSourceProviderInner::Futures {
                properties: {
                    todo!()
                },
                vs: {
                    // let state = self.state.clone();
                    // Box::pin(async move { (*state).create().await })
                    todo!()
                },
                cfg_cb: Box::new(|_| {}),
            })),
        }
    }

    /// Create a provider of a value source
    pub fn of_with<Vs>(
        &self,
        cfg: impl FnOnce(&mut Vs::Properties) + Send + Sync + 'static,
    ) -> impl Provider<Vs::Output>
    where
        Vs: ValueSource<Properties: Send, Output: Clone>  + Send,
    {
        ValueSourceProvider::<Vs> {
            inner: Arc::new(Mutex::new(ValueSourceProviderInner::Futures {
                properties: {
                    todo!()
                },
                vs: {
                    todo!()
                },
                cfg_cb: Box::new(cfg),
            })),
        }
    }
}

