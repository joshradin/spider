use std::fs::Metadata;
use std::path::Path;
/// An immutable location within a file system
pub trait FileSystemLocation {
    /// Gets the path of this file
    fn path(&self) -> &Path;

    /// The metadata of this file system location
    fn metadata(&self) -> &Metadata;

    /// If this [`FileSystemLocation`] is a directory
    fn is_dir(&self) -> bool {
        self.metadata().is_dir()
    }
}
