//! From beans trait

use std::marker::PhantomData;
use crate::beans::{BeanError, Beans};

pub mod functions_cons;

/// Create from beans trait
pub trait FromBeans: Sized {
    type Output: Send + Sync + 'static;

    fn create_from_beans(&mut self, beans: &Beans) -> Result<Self::Output, BeanError>;
}

pub trait IntoFromBeans<O: Send + Sync + 'static, Marker> {
    type IntoCreateFromBeans: FromBeans<Output = O>;

    fn into_create_from_beans(self) -> Self::IntoCreateFromBeans;
}

impl<O: Send + Sync + 'static, C: FromBeans<Output=O>> IntoFromBeans<O, ()> for C {
    type IntoCreateFromBeans = Self;

    fn into_create_from_beans(self) -> Self::IntoCreateFromBeans {
        self
    }
}

