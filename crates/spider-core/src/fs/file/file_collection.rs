//! A collection of files

use crate::fs::file::FileSystemLocation;
use crate::lazy::provider::{BoxProvider, Provider};
use std::sync::Arc;

/// Represents a collection of [`FileSystemLocation`]s.
// #[derive(Debug)]
pub struct FileCollection {
    providers: Vec<BoxProvider<Arc<dyn FileSystemLocation>>>,
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
