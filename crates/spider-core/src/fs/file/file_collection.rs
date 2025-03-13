//! A collection of files

use std::fs::File;
use std::sync::Arc;
use crate::fs::file::FileSystemLocation;
use crate::lazy::providers::Provider;

/// Represents a collection of [`FileSystemLocation`]s.
// #[derive(Debug)]
pub struct FileCollection {
    providers: Vec<Provider<Arc<dyn FileSystemLocation>>>
}
//
// impl IntoIterator for FileCollection {
//     type Item = File;
//     type IntoIter = Vec<File>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }

