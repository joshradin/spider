//! A directory within the file system

use std::fmt::{Debug, Formatter};
use std::fs::Metadata;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use spider_core::invocation::InvocationDetails;
use spider_core::lazy::providers::{Provider, ProviderFactory};
use crate::file::{FileSystemLocation, RegularFile};

/// simple wrapper over a regular file
#[derive(Clone)]
#[repr(transparent)]
pub struct Directory(RegularFile);

impl FileSystemLocation for Directory {
    fn path(&self) -> &Path {
        self.0.path()
    }

    fn metadata(&self) -> &Metadata {
        self.0.metadata()
    }
}

impl TryFrom<RegularFile> for Directory {
    type Error = io::Error;

    fn try_from(value: RegularFile) -> Result<Self, Self::Error> {
        if value.metadata().is_dir() {
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


