//! Regular files

use crate::file::FileSystemLocation;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::fs::Metadata;
use std::path::{Path, PathBuf};

/// A regular file path, it's normalized path if applicable, and its metadata
#[derive(Clone)]
pub struct RegularFile {
    path: PathBuf,
    metadata: Metadata,
}

impl RegularFile {
    /// Creates a regular file from a path
    pub(crate) fn new(path: &Path) -> std::io::Result<Self> {
        let abs_path = path.canonicalize()?;
        let metadata = fs::metadata(abs_path)?;
        Ok(Self {
            path: path.to_path_buf(),
            metadata,
        })
    }
}

impl FileSystemLocation for RegularFile {
    fn path(&self) -> &Path {
        self.path.as_path()
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl Debug for RegularFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.path, f)
    }
}
