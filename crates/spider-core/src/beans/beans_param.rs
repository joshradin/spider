//! From beans trait

use crate::beans::{BeanError, BeanRef, Beans};
use spider_proc_macros::all_tuples;

mod bean_extractors;
pub use bean_extractors::*;

/// Create from beans trait
pub trait BeansParam: Sized {
    type State: Send + Sync + 'static;
    type Item<'beans, 'state>: BeansParam<State = Self::State>;

    fn init_state(beans: &Beans) -> Self::State;

    fn get_param<'beans, 'state>(
        state: &'state mut Self::State,
        beans: &'beans Beans,
    ) -> Result<Self::Item<'beans, 'state>, BeanError>;
}

pub struct BeanParamSet<T: BeansParam>(pub(crate) T);

macro_rules! impl_bean_param {
    ($($T:ident),*) => {
        impl<$($T),*> BeansParam for ($($T,)*)
            where
                $(
                    $T: BeansParam,
                )*
        {
            type State = ($($T::State,)*);
            type Item<'beans, 'state> = ($($T::Item<'beans, 'state>,)*);

            fn init_state(beans: &Beans) -> Self::State {
                (
                    $($T::init_state(beans),)*
                )
            }

            fn get_param<'beans, 'state>(
                state: &'state mut Self::State,
                beans: &'beans Beans,
            ) -> Result<Self::Item<'beans, 'state>, BeanError> {
                let (
                    $($T,)*
                ) = state;
                let built = (
                    $($T::get_param($T, beans)?,)*
                );
                Ok(built)
            }
        }
    }
}

macro_rules! impl_bean_param_set {
    ($($T:ident),*) => {
        impl<$($T),*> BeansParam for BeanParamSet<($($T,)*)>
            where
                $(
                    $T: BeansParam,
                )*
        {
            type State = ($($T::State,)*);
            type Item<'beans, 'state> = ($($T::Item<'beans, 'state>,)*);

            fn init_state(beans: &Beans) -> Self::State {
                (
                    $($T::init_state(beans),)*
                )
            }

            fn get_param<'beans, 'state>(
                state: &'state mut Self::State,
                beans: &'beans Beans,
            ) -> Result<Self::Item<'beans, 'state>, BeanError> {
                let (
                    $($T,)*
                ) = state;
                let built = (
                    $($T::get_param($T, beans)?,)*
                );
                Ok(built)
            }
        }
    }
}

all_tuples!(impl_bean_param, 0, 16, T);
all_tuples!(impl_bean_param_set, 0, 16, T);
