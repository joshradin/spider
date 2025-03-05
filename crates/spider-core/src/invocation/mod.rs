//! Structs and functions for invoking spider

use crate::lazy::providers::ProviderFactory;
use std::env::current_dir;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub trait InvocationDetails {
    /// The current directory of the invocation
    fn current_dir(&self) -> &Path;
}

#[derive(Debug)]
struct SpiderInvocationDetails {
    cwd: PathBuf,
}

impl SpiderInvocationDetails {
    fn new(cwd: PathBuf) -> Self {
        Self { cwd }
    }
}

impl InvocationDetails for SpiderInvocationDetails {
    fn current_dir(&self) -> &Path {
        &self.cwd
    }
}

/// Represents an invocation of Spider.
#[derive(Debug)]
pub struct Spider {
    details: Arc<SpiderInvocationDetails>,
}

impl Default for Spider {
    fn default() -> Self {
        Self::new().expect("could not create a default Spider instance")
    }
}

impl Spider {
    /// Creates a new spider instance in the working direcroty
    pub fn new() -> io::Result<Self> {
        let path = current_dir()?;
        Self::in_path(path)
    }

    pub fn in_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        if !std::fs::exists(path)? {
            Err(io::ErrorKind::NotFound)?;
        }
        Ok(Spider {
            details: Arc::new(SpiderInvocationDetails::new(path.to_path_buf())),
        })
    }
}


impl SpiderAware for Spider {
    fn get_spider(&self) -> &Spider {
        self
    }
}

/// Some type that is aware of spider
pub trait SpiderAware {
    /// Gets a reference to the spider instance
    fn get_spider(&self) -> &Spider;
}
