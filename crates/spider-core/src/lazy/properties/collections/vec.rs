//! Contains the [`VecProperty`]

use crate::lazy::properties::collections::AddProperty;
use crate::lazy::properties::{Property, SetProperty};
use crate::lazy::providers::{wrap, Provider, Provides};
use parking_lot::Mutex;
use smallvec::SmallVec;
use std::sync::Arc;

struct VecPropertyInner<T: Clone> {
    values: Vec<Provider<SmallVec<[T; 1]>>>,
}

impl<T: Clone> Default for VecPropertyInner<T> {
    fn default() -> Self {
        Self { values: Vec::new() }
    }
}

/// A vec property
#[derive(Clone)]
pub struct VecProperty<T: Clone> {
    id: Option<String>,
    inner: Arc<Mutex<VecPropertyInner<T>>>,
}

impl<T: Clone> VecProperty<T> {
    pub(crate) fn new(id: impl Into<Option<String>>) -> Self {
        Self {
            id: None,
            inner: Default::default(),
        }
    }

    pub fn add(&mut self, value: T) {

    }

    pub fn add_all<I, P>(
        &mut self,
        item: &P,
    ) where
        T: Send + Sync + 'static,
        I: IntoIterator<Item=T> + Clone + Send + Sync + 'static,
        P: Provides<Output=I> + Clone + Send + Sync + 'static,
    {
        let provider = wrap(item.clone())
            .map(|v| {
                SmallVec::<[T; 1]>::from_iter(v)
            });

        let mut lock = self.inner.lock();
        lock.values.push(provider);
    }
}

impl<T: Clone> Extend<T> for VecProperty<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            // self.add(x)
        }
    }
}
impl<'a, T: Clone> Extend<&'a Provider<T>> for VecProperty<T> {
    fn extend<I: IntoIterator<Item = &'a Provider<T>>>(&mut self, iter: I) {
        todo!()
    }
}
impl<'a, T: Clone> Extend<&'a Property<T>> for VecProperty<T> {
    fn extend<I: IntoIterator<Item = &'a Property<T>>>(&mut self, iter: I) {
        todo!()
    }
}

impl<T: Clone> SetProperty<Vec<T>> for VecProperty<T> {
    fn set(&mut self, value: Vec<T>) {
        todo!()
    }
}

impl<T: Clone> SetProperty<&[T]> for VecProperty<T> {
    fn set(&mut self, value: &[T]) {
        todo!()
    }
}

impl<T: Clone, P: Provides<Output = Vec<T>>> SetProperty<P> for VecProperty<T> {
    fn set(&mut self, value: P) {
        todo!()
    }
}


impl<T: Clone> Provides for VecProperty<T> {
    type Output = Vec<T>;

    fn provider(&self) -> Provider<Self::Output>
    where
        Self::Output: Clone,
    {
        todo!()
    }

    fn try_get(&self) -> Option<Self::Output> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lazy::providers::provider;

    #[test]
    fn test_add() {
        let mut vec_prop: VecProperty<i32> = VecProperty::new(None);
        vec_prop.add(1);
        vec_prop.extend([1, 2, 3]);
        vec_prop.add_all(&provider(vec![]));
    }
}
