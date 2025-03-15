//! Collection based properties

pub mod vec;
pub mod hashmap;
pub mod hashset;

#[doc(inline)]
pub use self::{vec::*, hashmap::*, hashset::*};

pub trait AddProperty<T> {
    fn add_all(&mut self, item: T);
}

