//! Regular files

use crate::fs::file::FileSystemLocation;
use std::fmt::{Debug, Formatter};
use std::fs::{File, Metadata};
use std::path::{Path, PathBuf};
use std::{fs, io};

/// A regular file path, it's normalized path if applicable, and its metadata
#[derive(Clone)]
pub struct RegularFile {
    path: PathBuf,
}

impl RegularFile {
    /// Creates a regular file from a path
    pub(crate) fn new(path: &Path) -> io::Result<Self> {
        if !path.is_absolute() {
            return Err(io::ErrorKind::InvalidData.into());
        }
        Ok(Self {
            path: path.to_path_buf(),
        })
    }

    /// Opens this file
    pub fn open(&self) -> io::Result<File> {
        File::open(&self.path)
    }

    /// Creates this file, truncating it if it already exists
    pub fn create(&self) -> io::Result<File> {
        File::create(&self.path)
    }
}

impl FileSystemLocation for RegularFile {
    fn path(&self) -> &Path {
        self.path.as_path()
    }
}

impl Debug for RegularFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.path, f)
    }
}
