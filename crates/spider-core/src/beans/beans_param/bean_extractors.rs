//! Default bean constructor

use crate::beans::beans_query::{BeansQuery, QueryData};
use crate::beans::{BeanError, BeanMut, BeanResult, Beans, BeansParam};
use static_assertions::assert_impl_all;
use std::marker::PhantomData;
use crate::beans::bean_query_iter::BeanQueryIter;

pub struct Single<'beans, 'state, T>(T, PhantomData<&'beans ()>, PhantomData<&'state ()>);



pub struct Multi<'beans, 'state, T: QueryData> {
    fetch: T::Fetch<'beans>,
    _lifetime: PhantomData<(T, &'state (), &'beans ())>,
}

impl<'beans, 'state, T: QueryData> Multi<'beans, 'state, T> {
    pub fn get(&self, id: impl AsRef<str>) -> BeanResult<<<T as QueryData>::ReadOnly as BeansQuery>::Item<'beans>>
    {
        todo!()
    }

    pub fn get_mut(&mut self, id: impl AsRef<str>) -> BeanResult<BeanMut<T>> {
        todo!()
    }

    pub fn iter(&self) -> BeanQueryIter<'beans, '_, T> {
        let x = self.fetch.clone();
        let ids = T::ids(&x);
        BeanQueryIter::new(x, ids)
    }
}

impl<T: QueryData> BeansParam for Multi<'_, '_, T> {
    type State = ();
    type Item<'beans, 'state> = Multi<'beans, 'state, T>;

    fn init_state(_beans: &mut Beans) -> Self::State {
    }

    fn get_param<'beans, 'state>(_state: &'state mut Self::State, beans: &'beans Beans) -> Result<Self::Item<'beans, 'state>, BeanError> {
        let fetch = T::init_fetch(beans);
        Ok(Multi {
            fetch,
            _lifetime: PhantomData,
        })
    }
}

assert_impl_all!(Multi<&String>: BeansParam);
assert_impl_all!((Multi<&String>, Multi<&i32>): BeansParam);

pub struct IdBeans<T>(&'static str, PhantomData<T>);

