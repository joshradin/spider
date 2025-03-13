//! # `spider-core`
//! Core apis and types

#![allow(async_fn_in_trait)]

pub mod action;
pub mod beans;
pub mod error;
pub mod initialization;
pub mod invocation;
pub mod lazy;
pub mod named;
mod project;
pub mod shared;
pub mod table;
mod task;

// crate level re-exports
#[doc(inline)]
pub use crate::{project::*, task::*};
