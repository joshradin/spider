/// A type with a name
pub trait Named {
    /// Gets the name of self
    fn name(&self) -> &str;
}

/// Create a named object
pub trait CreateNamed: Named {
    /// Creates a named object with the given name
    fn with_name(name: impl AsRef<str>) -> Self;
}
