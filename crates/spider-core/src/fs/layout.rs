use crate::fs::file::{Directory, RegularFile};
use crate::fs::layout::to_regular_file_internal::ProjectFileInternal;
use crate::lazy::properties::Property;

/// Layout descriptor
pub struct ProjectLayout {
    project_dir: Directory,
    build_dir: Property<Directory>,
}

impl ProjectLayout {
    /// Creates a new layout directory
    pub(crate) fn new(project_dir: &Directory, build_dir: &Property<Directory>) -> Self {
        Self {
            project_dir: project_dir.clone(),
            build_dir: build_dir.clone(),
        }
    }

    pub fn file(&self, file: impl ProjectFile) -> RegularFile {
        let path = ProjectFileInternal::get_absolute_path(&file, self);
        RegularFile::new(&path).expect("Should never fail")
    }
}

#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a valid project file path",
    label = "Invalid project file path",
    note = "Implemented for `Path`-like types"
)]
pub trait ProjectFile: ProjectFileInternal {}

#[diagnostic::do_not_recommend]
impl<T: ProjectFileInternal> ProjectFile for T {}

mod to_regular_file_internal {
    use crate::fs::layout::ProjectLayout;
    use std::path::{Path, PathBuf};
    use crate::fs::file::{FileSystemLocation, RegularFile};

    pub trait ProjectFileInternal {
        fn get_absolute_path(self: &Self, layout: &ProjectLayout) -> PathBuf;
    }

    impl<P: AsRef<Path>> ProjectFileInternal for P {
        fn get_absolute_path(self: &Self, layout: &ProjectLayout) -> PathBuf {
            let path = self.as_ref().to_path_buf();
            if path.is_absolute() {
                path
            } else {
                layout.project_dir.path().join(path)
            }
        }
    }

    impl ProjectFileInternal for RegularFile {
        fn get_absolute_path(self: &Self, layout: &ProjectLayout) -> PathBuf {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use crate::fs::file::Directory;
    use crate::fs::layout::ProjectLayout;
    use crate::lazy::properties::Property;

    #[test]
    fn test_file() {
        let layout = ProjectLayout::new(
            &Directory::current().expect("could not create current dir directory"),
            &Property::empty(None),
        );
        let fs = layout.file("test_file");
    }
}
