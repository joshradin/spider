use std::marker::PhantomData;
use spider_proc_macros::all_tuples;

struct Foo<T> {
    phantom: PhantomData<T>,
}

trait WrappedInFoo {
    type Tup;
}

macro_rules! impl_wrapped_in_foo {
    ($($T:ident),*) => {
        impl<$($T),*> WrappedInFoo for ($($T,)*) {
            type Tup = ($(Foo<$T>,)*);
        }
    };
}

all_tuples!(impl_wrapped_in_foo, 0, 15, T);

#[test]
fn test_all_tuples() {

}