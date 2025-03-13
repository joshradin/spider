//! Collection based properties

pub mod vec;
pub mod hashmap;
pub mod hashset;

#[doc(inline)]
pub use self::{vec::*, hashmap::*, hashset::*};