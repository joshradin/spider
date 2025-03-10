//! Simple [`BeanProvider`] and [`Inject`] traits

use crate::named::{CreateNamed, Named};
use spider_proc_macros::all_tuples;
use std::sync::Arc;

/// Gets a bean of type `T`
pub trait BeanProvider<T> {
    /// Gets the given bean
    fn get_bean(&self) -> T;
}

macro_rules! impl_bean_provider {
    ($($T:ident),*) => {
        #[automatically_derived]
        impl <U, $($T),*> BeanProvider<($($T,)*)> for U
            where
                $(U : BeanProvider<$T>,)*
        {
            fn get_bean(&self) -> ($($T,)*) {
                (
                    $(BeanProvider::<$T>::get_bean(self),)*
                )
            }
        }
    };
}

all_tuples!(impl_bean_provider, 0, 16, T);

/// Provides extension methods for [`BeanProvider`]
pub trait BeanProviderExt<T>: BeanProvider<T> {
    /// Injects into a mutable bean
    fn inject_into(&self, b: &mut impl Inject<T>) -> &Self {
        b.inject(self);
        self
    }
}
impl<T, U> BeanProviderExt<U> for T where T: BeanProvider<U> {}

/// A type that can be created by
pub trait Inject<U> {
    fn inject<P>(&mut self, bean_provider: &P)
    where
        P: BeanProvider<U> + ?Sized;
}

/// Runs injection code with the given bean provider, returning the self object
pub trait WithInject<U>: Inject<U> {
    /// Injects an existing object with data
    fn with_inject<P>(self, f: &P) -> Self
    where
        P: BeanProvider<U> + ?Sized;
}

impl<T, U> WithInject<U> for T
where
    T: Inject<U>,
{
    fn with_inject<P>(mut self, f: &P) -> Self
    where
        P: BeanProvider<U> + ?Sized,
    {
        self.inject(f);
        self
    }
}

/// An object that must be created from a bean provider
#[diagnostic::on_unimplemented(
    message = "`FromBeanProvider<{U}>` must be implemented for `{Self}` in order for it to be constructed",
    label = "Bean",
    note = "For `Named` types, you should implement `Inject<{U}>`"
    note = "If no bean injection is needed, you can implemented `NoBeans`"
)]
pub trait FromBeanProvider<U> {
    fn from_bean_provider<P>(f: &P) -> Self
    where
        P: BeanProvider<U> + ?Sized;
}

impl<T: Default + Inject<U>, U> FromBeanProvider<U> for T {
    fn from_bean_provider<P>(f: &P) -> Self
    where
        P: BeanProvider<U> + ?Sized,
    {
        T::default().with_inject(f)
    }
}

/// Creates a named object with a given bean provider
pub trait NamedFromBeanProvider<U> {
    fn named_from_bean_provider<P>(name: impl AsRef<str>, p: &P) -> Self
    where
        P: BeanProvider<U> + ?Sized;
}
impl<T: CreateNamed + Inject<U>, U> NamedFromBeanProvider<U> for T {
    fn named_from_bean_provider<P>(name: impl AsRef<str>, p: &P) -> Self
    where
        P: BeanProvider<U> + ?Sized,
    {
        T::with_name(name).with_inject(p)
    }
}

pub trait NoBeans {}
impl<T: NoBeans> Inject<()> for T {
    fn inject<P>(&mut self, _bean_provider: &P)
    where
        P: BeanProvider<()> + ?Sized,
    {
    }
}

impl NoBeans for () {}

#[cfg(test)]
mod tests {
    use super::*;
    struct StringProvider;
    impl BeanProvider<String> for StringProvider {
        fn get_bean(&self) -> String {
            "hello world".to_string()
        }
    }

    #[derive(Default, PartialEq, Debug)]
    struct StringBean(String);

    impl Inject<String> for StringBean {
        fn inject<P>(&mut self, bean_provider: &P)
        where
            P: BeanProvider<String> + ?Sized,
        {
            self.0 = bean_provider.get_bean();
        }
    }

    #[test]
    fn test_bean_provider() {
        let (a, b): (String, String) = StringProvider.get_bean();
        assert_eq!(a, b);
    }

    #[test]
    fn test_bean_provider_ext() {
        let (mut a, mut b): (StringBean, StringBean) = (Default::default(), Default::default());
        StringProvider.inject_into(&mut a).inject_into(&mut b);
        assert!(!a.0.is_empty());
        assert_eq!(a, b);
    }

    #[test]
    fn test_inject() {
        let provider = StringProvider;
        let mut bean = StringBean::from_bean_provider(&provider);
        assert_eq!(&bean.0, "hello world");
    }
}
