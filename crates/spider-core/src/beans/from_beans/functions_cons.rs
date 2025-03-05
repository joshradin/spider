use std::marker::PhantomData;
use crate::beans::{BeanError, Beans, FromBeans, IntoFromBeans};

pub struct FromFunction<Arg>(PhantomData<Arg>);


pub struct FunctionCons<Marker, F>
where F: FromBeansFunction<Marker>
{
    _todo: PhantomData<(F, Marker)>
}

impl<Marker, F> FromBeans for FunctionCons<Marker, F> where
    F: FromBeansFunction<Marker, Output: Send + Sync + 'static> {
    type Output = F::Output;

    fn create_from_beans(&mut self, beans: &Beans) -> Result<Self::Output, BeanError> {
        todo!()
    }
}
//
// impl<T: Send + Sync + 'static, F: FromBeansFunction<fn() -> T>> IntoFromBeans<T, FromFunction<()>> for F {
//     type IntoCreateFromBeans = FunctionCons<FromFunction<()>, F>;
//
//     fn into_create_from_beans(self) -> Self::IntoCreateFromBeans {
//         todo!()
//     }
// }

pub trait FromBeansFunction<Marker>: Send + Sync + 'static {
    type Output;
    type Param: FromBeans;

    fn run(&mut self, param: Self::Param) -> Result<Self::Output, BeanError>;
}

