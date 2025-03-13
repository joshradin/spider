use std::env::current_dir;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct SpiderInvocationDetails {
    cwd: PathBuf,
}

impl SpiderInvocationDetails {
    fn new(cwd: PathBuf) -> Self {
        Self { cwd }
    }
}

/// Represents an invocation of Spider.
#[derive(Debug)]
pub struct Spider {
    details: SpiderInvocationDetails,
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
            details: SpiderInvocationDetails::new(path.to_path_buf()),
        })
    }
}

/// Some type that is aware of spider
pub trait SpiderAware {
    /// Gets a reference to the spider instance
    fn spider(&self) -> &Spider;
}

/// A type that's aware of spider
impl SpiderAware for Spider {
    fn spider(&self) -> &Spider {
        self
    }
}
