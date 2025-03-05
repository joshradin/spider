use std::fs::Metadata;
use std::path::Path;
use tokio::fs::File;
use tokio::io;

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

    /// Opens the file
    async fn as_file(&self) -> io::Result<File> {
        let path = self.path();
        File::open(path).await
    }

}