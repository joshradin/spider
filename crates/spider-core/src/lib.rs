//! # `spider-core`
//! Core apis and types

#![allow(async_fn_in_trait)]

mod action;
pub mod beans;
pub mod initialization;
pub mod invocation;
pub mod lazy;
mod project;
mod task;
pub mod named;
pub mod shared;
pub mod err;


// crate level re-exports
#[doc(inline)]
pub use crate::{action::*, project::*, task::*, action::*};
