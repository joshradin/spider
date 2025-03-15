use std::fs::Metadata;
use std::path::Path;
use std::{fs, io};

/// An immutable location within a file system
pub trait FileSystemLocation {
    /// Gets the path of this file
    fn path(&self) -> &Path;

    fn metadata(&self) -> io::Result<Metadata> {
        fs::metadata(&self.path())
    }

    /// Checks if the given file system location actually exists
    fn exists(&self) -> bool {
        fs::exists(&self.path()).unwrap_or(false)
    }

    /// If this [`FileSystemLocation`] is a directory
    fn is_dir(&self) -> bool {
        self.metadata().map(|m| m.is_dir()).unwrap_or(false)
    }
}
