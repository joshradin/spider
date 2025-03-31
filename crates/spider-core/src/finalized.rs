//! A type that's been "finalized", preventing further mutable access

use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Finalize<T> {
    finalized: bool,
    t: T,
}

impl<T> Finalize<T> {
    /// Creates a new, unfinalized object
    pub const fn new(t: T) -> Self {
        Self {
            finalized: false,
            t,
        }
    }

    /// Finalizes this object
    pub fn finalize(&mut self) {
        self.finalized = true;
    }
}

impl<T> Deref for Finalize<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

impl<T> DerefMut for Finalize<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.finalized {
            panic!("Can not mutably dereference an object that's already been finalized")
        }
        &mut self.t
    }
}
