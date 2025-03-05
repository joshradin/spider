/// A value source
pub trait ValueSource<T: Send, P: Sync = ()> {
    type Fut: Future<Output = T> + Send;

    /// Get a value from the given properties
    fn get(self, properties: &P) -> Self::Fut;
}

