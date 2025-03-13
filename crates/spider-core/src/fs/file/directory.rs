//! A directory within the file system

use crate::lazy::providers::{Provider, ProviderFactory};
use std::fmt::{Debug, Formatter};
use std::fs::Metadata;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use crate::fs::file::{FileSystemLocation, RegularFile};

/// simple wrapper over a regular file
#[derive(Clone)]
#[repr(transparent)]
pub struct Directory(RegularFile);

impl Directory {
    /// Creates a directory from a path
    pub(crate) fn new(path: &Path) -> io::Result<Self> {
        let regular_file = RegularFile::new(path)?;
        Directory::try_from(regular_file)
    }

    /// Creates a directory from the current directory
    pub(crate) fn current() -> io::Result<Self> {
        let current = std::env::current_dir()?;
        Self::new(current.as_path())
    }
}

impl FileSystemLocation for Directory {
    fn path(&self) -> &Path {
        self.0.path()
    }
}

impl TryFrom<RegularFile> for Directory {
    type Error = io::Error;

    fn try_from(value: RegularFile) -> Result<Self, Self::Error> {
        if !value.exists() {
            return Ok(Directory(value))
        }
        if value.metadata()?.is_dir() {
            Ok(Self(value))
        } else {
            Err(io::Error::new(ErrorKind::NotADirectory, "not a directory"))
        }
    }
}

impl Debug for Directory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}
