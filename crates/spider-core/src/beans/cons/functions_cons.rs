use crate::beans::cons::BeanConstructor;
use crate::beans::{BeanError, BeanParamSet, BeansParam};
use spider_proc_macros::all_tuples;
use static_assertions::assert_impl_all;
use std::marker::PhantomData;

impl<F, Marker, O, P> BeanConstructor<Marker> for F
where
    F: FromBeansFunction<Marker, Output = O, Params = P>,
    P: BeansParam,
    O: Send + Sync + 'static,
{
    type Params = F::Params;
    type Out = O;

    fn build<'beans, 'state>(mut self, params: <Self::Params as BeansParam>::Item<'_, '_>) -> Result<Self::Out, BeanError> {
        let b = self.run(params)?;
        Ok(b)
    }
}

pub struct FromFunction<Arg>(PhantomData<Arg>);

pub trait FromBeansFunction<Marker>: Send + Sync + 'static {
    type Output;
    type Params: BeansParam;

    fn run<'beans, 'state>(
        &mut self,
        param: <Self::Params as BeansParam>::Item<'_, '_>,
    ) -> Result<Self::Output, BeanError>;
}

macro_rules! impl_from_beans_function {
    ($($T:ident),*) => {
        impl<Out, Func, $($T),*> FromBeansFunction<fn($($T),*) -> Out> for Func
            where
                $($T: BeansParam,)*
                Func: Send + Sync + 'static,
                for<'a> &'a mut Func: FnMut($($T),*) -> Result<Out, BeanError> +  FnMut($(<$T as BeansParam>::Item<'_, '_>),*) -> Result<Out, BeanError>,
                Out: 'static,

        {
            type Output = Out;
            type Params = BeanParamSet<($($T,)*)>;

            #[inline]
            fn run<'beans, 'state>(&mut self, param: <Self::Params as BeansParam>::Item<'_, '_>) -> Result<Out, BeanError> {
                #[allow(clippy::too_many_arguments)]
                fn call_inner<'b, 's, Out, $($T: BeansParam,)*>(
                    mut f: impl FnMut($($T,)*)->Result<Out, BeanError>,
                    $($T: $T,)*
                )->Result<Out, BeanError>{
                    f($($T,)*)
                }
                #[allow(non_snake_case)]
                let ($($T,)*) = param;
                call_inner(self, $($T),*)
            }
        }
    }
}

all_tuples!(impl_from_beans_function, 0, 16, T);

assert_impl_all!(fn() -> Result<(), BeanError>: FromBeansFunction<fn() -> ()>);
assert_impl_all!(fn((), ()) -> Result<(), BeanError>: FromBeansFunction<fn((), ()) -> ()>);
