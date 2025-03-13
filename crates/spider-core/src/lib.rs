//! # `spider-core`
//! Core apis and types

#![allow(async_fn_in_trait)]

pub mod beans;
pub mod initialization;
pub mod invocation;
pub mod lazy;
mod project;
mod task;
pub mod named;
pub mod shared;
pub mod error;
pub mod action;
pub mod table;


// crate level re-exports
#[doc(inline)]
pub use crate::{project::*, task::*,};
