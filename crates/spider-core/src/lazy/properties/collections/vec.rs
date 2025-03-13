//! Contains the [`VecProperty`]

use std::sync::Arc;
use parking_lot::Mutex;
use crate::lazy::properties::SetProperty;
use crate::lazy::providers::{Provider, Provides};



#[derive(Clone)]
struct VecPropertyInner<T: Clone> {
    values: Vec<Provider<T>>
}

/// A vec property
#[derive(Clone)]
pub struct VecProperty<T: Clone> {
    inner: Arc<Mutex<VecPropertyInner<T>>>,
}

impl<T: Clone> VecProperty<T> {
    /// Adds a value to this vec property
    pub fn push(&mut self, value: T) {

    }
}

impl<T: Clone> SetProperty<Vec<T>> for VecProperty<T> {
    fn set(&mut self, value: Vec<T>) {
        todo!()
    }
}

impl<T: Clone, P : Provides<Output=Vec<T>>> SetProperty<P> for VecProperty<T> {
    fn set(&mut self, value: P) {
        todo!()
    }
}

impl<T: Clone> Provides for VecProperty<T> {
    type Output = Vec<T>;

    fn provider(&self) -> Provider<Self::Output>
    where
        Self::Output: Clone
    {
        todo!()
    }

    fn try_get(&self) -> Option<Self::Output> {
        todo!()
    }
}
