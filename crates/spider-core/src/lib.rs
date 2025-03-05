//! # `spider-core`
//! Core apis and types

#![allow(async_fn_in_trait)]

pub mod beans;
pub mod lazy;
pub mod from_state;
pub mod invocation;
pub mod initialization;
mod project;

pub use project::*;


