//! Performs some action against the given object

use crate::err::BuildResult;
use std::any::Any;

/// Performs some action against the given object
pub trait Action<T: ?Sized>: Send {
    type Output;
    /// Performs this action against the given object
    fn execute(&mut self, t: &mut T) -> Self::Output;
}

#[diagnostic::do_not_recommend]
impl<T: ?Sized + Send, R, F: FnMut(&mut T) -> R + Send> Action<T> for F {
    type Output = R;
    fn execute(&mut self, t: &mut T) -> R {
        self(t)
    }
}

pub struct BoxAction<'a, T: ?Sized + 'a, R: 'a> {
    execute: Box<dyn FnMut(&mut T) -> R + Send + Sync + 'a>,
}

impl<'a, T: ?Sized + 'a, O: 'a> Action<T> for BoxAction<'a, T, O> {
    type Output = O;

    fn execute(&mut self, t: &mut T) -> Self::Output {
        todo!()
    }
}

impl<'a, T: ?Sized, R> BoxAction<'a, T, R> {
    pub fn new<A>(mut action: A) -> Self
    where
        A: Action<T, Output=R> + Send + Sync + 'a,
    {
        let execute = move |t: &mut T| {
            (&mut action).execute(t)
        };
        Self {
            execute: Box::new(execute),
        }
    }
}

