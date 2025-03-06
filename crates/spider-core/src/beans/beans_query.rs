//! Beans query

use std::marker::PhantomData;
use futures::TryFutureExt;
use crate::beans::{BeanRef, Beans, IdBeans};

pub trait BeansQuery {
    type Item<'a>;
    type Fetch<'a>: Clone;

    fn init_fetch(beans: &Beans) -> Self::Fetch<'_>;
    fn ids<'w>(fetch: &Self::Fetch<'w>) -> Vec<String>;
    fn fetch<'w>(fetch: &mut Self::Fetch<'w>, id: String) -> Self::Item<'w>;
}

pub trait QueryData : BeansQuery {
    type ReadOnly: ReadOnlyQueryData;
}
impl<T: Send + Sync + 'static> QueryData for &T {
    type ReadOnly = Self;
}

pub trait ReadOnlyQueryData: QueryData<ReadOnly = Self> {

}
impl<T: Send + Sync + 'static> ReadOnlyQueryData for &T {
}


pub struct ReadFetch<'beans, T> {
    beans: &'beans Beans,
    ids: Vec<String>,
    _bean_ty: PhantomData<fn () -> &'beans T>,
}

impl<'beans, T> Clone for ReadFetch<'beans, T> {
    fn clone(&self) -> Self {
        Self {
            beans: self.beans.clone(),
            ids: self.ids.clone(),
            _bean_ty: PhantomData,
        }
    }
}


impl<T: Send + Sync + 'static> BeansQuery for &T {
    type Item<'a> = BeanRef<'a, T>;
    type Fetch<'a> = ReadFetch<'a, T>;

    fn init_fetch(beans: &Beans) -> Self::Fetch<'_> {
        let ids = beans.get_ids_of::<T>();
        ReadFetch {
            beans,
            ids,
            _bean_ty: PhantomData,
        }
    }

    fn ids(fetch: &Self::Fetch<'_>) -> Vec<String> {
        fetch.ids
            .iter()
            .map(|id| id.clone())
            .collect()
    }

    fn fetch<'w>(fetch: &mut Self::Fetch<'w>, id: String) -> Self::Item<'w> {
        fetch.beans.get(&id).unwrap_or_else(|e| {
            panic!("Could not get bean with id {id:?}: {e}")
        })
    }
}