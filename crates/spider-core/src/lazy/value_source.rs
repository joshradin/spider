/// A value source
pub trait ValueSource {
    type Properties: Send + Sync;
    type Output: Send + Sync;

    /// Get a value from the given properties
    fn get(self, properties: &Self::Properties) -> Option<Self::Output>;
}

