//! Shared references

use std::sync::Arc;
use parking_lot::RwLock;

/// A shared type that can be sent
pub type Shared<T> = Arc<RwLock<T>>;

pub type Ref<'a, T> = parking_lot::RwLockReadGuard<'a, T>;
pub type RefMut<'a, T> = parking_lot::RwLockWriteGuard<'a, T>;

/// Creates a shared object
pub(crate) fn shared<T>(t: T) -> Shared<T> {
    Arc::new(RwLock::new(t))
}