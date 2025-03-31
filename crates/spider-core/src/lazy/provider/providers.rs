use crate::lazy::provider::{Provider, ProviderSource};
use crate::lazy::value_source::ValueSource;
use crate::shared::Shared;
use futures::future::BoxFuture;
use std::any::Any;
use std::collections::HashSet;
use std::fmt::Pointer;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A provider of a value `T`
#[derive(Clone)]
pub struct JustProvider<T: Clone>(T);

impl<T: Clone + Send + Sync + 'static> JustProvider<T> {
    pub fn new(t: T) -> Self {
        Self(t)
    }
}

impl<T: Clone + Send + Sync + 'static> Provider<T> for JustProvider<T> {
    async fn try_get(&self) -> Option<T> {
        Some(self.0.clone())
    }

    fn sources(&self) -> HashSet<ProviderSource> {
        HashSet::new()
    }
}

/// A provider from a provider
pub struct ProviderProvider<P, T: Send + Sync + 'static>
where
    P: Provider<T>,
{
    provider: P,
    _ty: PhantomData<T>,
}

impl<P, T: Send + Sync + 'static> ProviderProvider<P, T>
where
    P: Provider<T>,
{
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            _ty: Default::default(),
        }
    }
}

impl<P, T: Send + Sync + 'static> Clone for ProviderProvider<P, T>
where
    P: Provider<T>,
{
    fn clone(&self) -> Self {
        Self {
            provider: self.provider.clone(),
            _ty: PhantomData,
        }
    }
}

impl<P, T: Send + Sync + 'static> Provider<T> for ProviderProvider<P, T>
where
    P: Provider<T>,
{
    async fn try_get(&self) -> Option<T> {
        self.provider.try_get().await
    }

    fn sources(&self) -> HashSet<ProviderSource> {
        self.provider.sources()
    }
}

pub struct MapProvider<P, T, F, U>
where
    P: Provider<T>,
    T: Send + Sync + 'static,
    U: Send + Sync + 'static,
    F: Fn(T) -> U + Send + Sync + 'static,
{
    provider: P,
    function: Arc<F>,
    _ty: PhantomData<(T, U)>,
}

impl<P, T, F, U> MapProvider<P, T, F, U>
where
    P: Provider<T>,
    T: Send + Sync + 'static,
    U: Send + Sync + 'static,
    F: Fn(T) -> U + Send + Sync + 'static,
{
    pub fn new(provider: P, function: F) -> Self {
        Self {
            provider,
            function: Arc::new(function),
            _ty: PhantomData,
        }
    }
}

impl<P, T, F, U> Clone for MapProvider<P, T, F, U>
where
    P: Provider<T>,
    T: Send + Sync + 'static,
    U: Send + Sync + 'static,
    F: Fn(T) -> U + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            provider: self.provider.clone(),
            function: self.function.clone(),
            _ty: Default::default(),
        }
    }
}

impl<P, T, F, U> Provider<U> for MapProvider<P, T, F, U>
where
    P: Provider<T>,
    T: Send + Sync + 'static,
    U: Send + Sync + 'static,
    F: Fn(T) -> U + Send + Sync + 'static,
{
    async fn try_get(&self) -> Option<U> {
        let t: T = self.provider.try_get().await?;
        let u = (&*self.function)(t);
        Some(u)
    }

    fn sources(&self) -> HashSet<ProviderSource> {
        self.provider.sources()
    }
}

/// A provider that emits based on the output of a provider
pub struct FlatMapProvider<P1, T, F, P2, U>
where
    T: Send + Sync + 'static,
    U: Send + Sync + 'static,
    P1: Provider<T>,
    P2: Provider<U>,
    F: Fn(T) -> P2 + Send + Sync + 'static,
{
    provider: P1,
    function: Arc<F>,
    _ty: PhantomData<(T, U, P2)>,
}

impl<P1, T, F, P2, U> Clone for FlatMapProvider<P1, T, F, P2, U>
where
    F: 'static + Fn(T) -> P2 + Send + Sync,
    P1: Provider<T>,
    P2: Provider<U>,
    T: 'static + Send + Sync,
    U: 'static + Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            provider: self.provider.clone(),
            function: self.function.clone(),
            _ty: PhantomData,
        }
    }
}

impl<P1, T, F, P2, U> Provider<U> for FlatMapProvider<P1, T, F, P2, U>
where
    T: Send + Sync + 'static,
    U: Send + Sync + 'static,
    P1: Provider<T>,
    P2: Provider<U>,
    F: Fn(T) -> P2 + Send + Sync + 'static,
{
    async fn try_get(&self) -> Option<U> {
        let p2 = self.try_get_inner_provider().await?;
        p2.try_get().await
    }

    fn sources(&self) -> HashSet<ProviderSource> {
        let mut sources = HashSet::new();
        sources.extend(self.provider.sources());
        sources
    }
}

impl<P1, T, F, P2, U> FlatMapProvider<P1, T, F, P2, U>
where
    T: Send + Sync + 'static,
    U: Send + Sync + 'static,
    P1: Provider<T>,
    P2: Provider<U>,
    F: Fn(T) -> P2 + Send + Sync + 'static,
{
    pub fn new(provider: P1, function: F) -> Self {
        Self {
            provider,
            function: Arc::new(function),
            _ty: Default::default(),
        }
    }

    async fn get_inner_provider(&self) -> P2 {
        self.try_get_inner_provider()
            .await
            .expect("could not get inner provider")
    }

    async fn try_get_inner_provider(&self) -> Option<P2> {
        let t = self.provider.try_get().await?;
        let p2 = (&*self.function)(t);
        Some(p2)
    }
}

/// A provider that emits based on the output of a provider
pub struct AndThenProvider<P, T, F, U>
where
    T: Send + Sync + 'static,
    U: Send + Sync + 'static,
    P: Provider<T>,
    F: Fn(T) -> Option<U> + Send + Sync + 'static,
{
    provider: P,
    function: Arc<F>,
    _ty: PhantomData<(T, U)>,
}

impl<P, T, F, U> AndThenProvider<P, T, F, U>
where
    T: Send + Sync + 'static,
    U: Send + Sync + 'static,
    P: Provider<T>,
    F: Fn(T) -> Option<U> + Send + Sync + 'static,
{
    pub fn new(provider: P, function: F) -> Self {
        Self {
            provider,
            function: Arc::new(function),
            _ty: PhantomData,
        }
    }
}

impl<P, T, F, U> Clone for AndThenProvider<P, T, F, U>
where
    F: 'static + Fn(T) -> Option<U> + Send + Sync,
    P: Provider<T>,
    T: 'static + Send + Sync,
    U: 'static + Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            provider: self.provider.clone(),
            function: self.function.clone(),
            _ty: PhantomData,
        }
    }
}

impl<P, T, F, U> Provider<U> for AndThenProvider<P, T, F, U>
where
    T: Send + Sync + 'static,
    U: Send + Sync + 'static,
    P: Provider<T>,
    F: Fn(T) -> Option<U> + Send + Sync + 'static,
{
    async fn try_get(&self) -> Option<U> {
        let t = self.provider.try_get().await?;
        (&*self.function)(t)
    }

    fn sources(&self) -> HashSet<ProviderSource> {
        self.provider.sources()
    }
}

pub struct ProducerProvider<T, F>
where
    F: Send + Sync + 'static,
    F: Fn() -> T,
{
    function: Arc<F>,
    _ty: PhantomData<T>,
}

impl<T, F> ProducerProvider<T, F>
where
    F: Send + Sync + 'static,
    F: Fn() -> T,
    T: Send + Sync + 'static,
{
    pub fn new(function: F) -> Self {
        Self {
            function: Arc::new(function),
            _ty: PhantomData,
        }
    }
}

impl<T, F> Clone for ProducerProvider<T, F>
where
    F: Send + Sync + 'static,
    F: Fn() -> T,
{
    fn clone(&self) -> Self {
        Self {
            function: self.function.clone(),
            _ty: PhantomData,
        }
    }
}

impl<T: Send + Sync + 'static, F> Provider<T> for ProducerProvider<T, F>
where
    F: Send + Sync + 'static,
    F: Fn() -> T,
{
    async fn try_get(&self) -> Option<T> {
        let t: T = (self.function)();
        Some(t)
    }

    fn sources(&self) -> HashSet<ProviderSource> {
        HashSet::new()
    }
}

enum ValueSourceProviderInner<Vs>
where
    Vs: ValueSource + Send + Sync + 'static,
    Vs::Output: Clone + Send + Sync + 'static,
{
    Poisoned,
    ValueSource { vs: Vs, props: Vs::Properties },
    Value(Option<Vs::Output>),
}

pub struct ValueSourceProvider<Vs>
where
    Vs: ValueSource + Send + Sync + 'static,
    Vs::Output: Clone + Send + Sync + 'static,
{
    inner: Shared<ValueSourceProviderInner<Vs>>,
}

impl<Vs> Clone for ValueSourceProvider<Vs>
where
    Vs: ValueSource + Send + Sync + 'static,
    Vs::Output: Clone + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<Vs> Provider<Vs::Output> for ValueSourceProvider<Vs>
where
    Vs: ValueSource + Send + Sync + 'static,
    Vs::Output: Clone + Send + Sync + 'static,
{
    async fn try_get(&self) -> Option<Vs::Output> {
        let mut inner = self.inner.write().await;
        match &*inner {
            ValueSourceProviderInner::Value(vs) => vs.as_ref().cloned(),
            ValueSourceProviderInner::ValueSource { .. } => {
                let ValueSourceProviderInner::ValueSource { vs, props } =
                    std::mem::replace(&mut *inner, ValueSourceProviderInner::Poisoned)
                else {
                    unreachable!()
                };
                let output = vs.get(&props).await;
                *inner = ValueSourceProviderInner::Value(output.clone());
                output
            }
            ValueSourceProviderInner::Poisoned => {
                panic!("Poisoned value source provider");
            }
        }
    }

    fn sources(&self) -> HashSet<ProviderSource> {
        HashSet::new()
    }
}

impl<Vs> ValueSourceProvider<Vs>
where
    Vs: ValueSource + Send + Sync + 'static,
    Vs::Output: Clone + Send + Sync + 'static,
{
    pub fn new(vs: Vs, props: Vs::Properties) -> Self {
        Self {
            inner: Arc::new(RwLock::new(ValueSourceProviderInner::ValueSource {
                vs,
                props,
            })),
        }
    }
}
