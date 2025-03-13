use std::path::{Path, PathBuf};
use crate::fs::file::{FileSystemLocation, RegularFile};
use crate::lazy::properties::{Property, SetProperty};
use crate::lazy::providers::Provides;


pub trait FileSystemLocationProperty<T>: Provides<Output = T>
where
    T: FileSystemLocation + Clone + Send + Sync + 'static,
{
}

impl<T> SetProperty<PathBuf> for Property<T>
where
    T: FileSystemLocation + Clone + Send + Sync + 'static,
{
    fn set(&mut self, value: PathBuf) {
        todo!()
    }
}

impl<T> SetProperty<&Path> for Property<T>
where
    T: FileSystemLocation + Clone + Send + Sync + 'static,
{
    fn set(&mut self, value: &Path) {
        todo!()
    }
}

impl<T: FileSystemLocation + Clone + Send + Sync + 'static> FileSystemLocationProperty<T>
for Property<T>
{
}

/// A regular file provider
pub trait RegularFileProperty: FileSystemLocationProperty<RegularFile> {
}

impl RegularFileProperty for Property<RegularFile> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_regular_file_property() {
        let mut property = Property::<RegularFile>::empty(None);


    }
}