//! Structs and functions for invoking spider

use crate::lazy::providers::ProviderFactory;
use std::env::current_dir;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::beans::{BeanConstructor, BeanParamSet, Beans, BeansParam};

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
    beans: Beans,
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
            beans: Beans::new(),
        })
    }

    /// Gets the beans object
    pub(crate) fn beans(&self) -> &Beans {
        &self.beans
    }

    /// Gets a mutable reference to the beans object
    pub(crate) fn beans_mut(&mut self) -> &mut Beans {
        &mut self.beans
    }

    /// Gets an objects creator
    pub fn objects(&self) -> Objects {
        Objects(&self.beans)
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


pub struct Objects<'a>(&'a Beans);



impl<'a> Objects<'a> {

    /// Create an object
    pub fn create<T: Send + Sync + 'static, Marker>(&self, cons: impl BeanConstructor<Marker, Out=T>) -> T {
        let Objects(beans) = self;

        beans.create(cons).unwrap()
    }
}