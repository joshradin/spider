use crate::file::RegularFile;
use spider_core::lazy::providers::Provider;

/// A regular file provider
pub trait RegularFileProvider: Provider<RegularFile> {
}

impl<P: Provider<RegularFile>> RegularFileProvider for P {}

#[cfg(test)]
mod tests {
    use spider_core::invocation::Spider;

    #[test]
    fn test_regular_file_provider() {
        let spider = Spider::default();

    }
}