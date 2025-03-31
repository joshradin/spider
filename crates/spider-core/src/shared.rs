//! Shared references

use std::sync::Arc;
use tokio::sync::RwLock;

/// A shared type that can be sent
pub type Shared<T> = Arc<RwLock<T>>;

/// Creates a shared object
pub(crate) fn shared<T>(t: T) -> Shared<T> {
    Arc::new(RwLock::new(t))
}
