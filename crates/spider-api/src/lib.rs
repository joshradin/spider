//! # `spider-api`
//!
//! A container crate making it easier to consume Spider sub-crates
//!
//! ## Default Features
//! The default features set enables most of the expected features for spider
//!
//!
//! |**Feature name**|**Description**|
//! |-|-|
//! | fs | enables file system apis |

#[cfg(feature = "fs")]
pub mod fs {
    //! Provides file system types for spider

    pub use spider_fs::*;
}

pub use spider_core::*;