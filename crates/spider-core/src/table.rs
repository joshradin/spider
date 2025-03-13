//! The [`Table`] struct, which provides a sort of pseudo inheritance mechanism

use std::any::{Any, TypeId, type_name};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Table {
    metatable: Option<Box<Table>>,
    entries: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
struct Value {
    type_id: TypeId,
    type_name: &'static str,
    value: Arc<dyn Any + Send + Sync>,
}

impl Value {
    fn new<T>(t: T) -> Self
    where
        T: Send + Sync + 'static,
    {
        Self {
            type_id: TypeId::of::<T>(),
            type_name: type_name::<T>(),
            value: Arc::new(t),
        }
    }

    fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.value.downcast_ref::<T>()
    }
}

impl Table {
    /// Creates a new table
    pub fn new() -> Self {
        Table {
            metatable: None,
            entries: HashMap::new(),
        }
    }

    pub fn set<T>(&mut self, key: &str, value: T)
    where
        T: 'static + Send + Sync,
    {
        self.entries.insert(key.to_string(), Value::new(value));
    }
    pub fn get<T>(&self, key: &str) -> Result<&T, TableError>
    where
        T: 'static,
    {
        if let Some(val) = self.entries.get(key) {
            match val.downcast_ref::<T>() {
                None => Err(TableError::TypeMismatch {
                    key: key.to_string(),
                    type_name: type_name::<T>(),
                    actual_type_name: val.type_name,
                }),
                Some(v) => Ok(v),
            }
        } else {
            match &self.metatable {
                None => Err(TableError::KeyNotFound {
                    key: key.to_string(),
                }),
                Some(mt) => mt.get(key),
            }
        }
    }

    /// Checks if this table contains the given key
    pub fn contains_key(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    /// Gets the length of this metatable
    pub fn len(&self) -> usize {
        self.entries.len() + self.metatable.as_ref().map_or(0, |t| t.len())
    }

    /// Set the metatable of this table
    pub fn set_metatable(&mut self, table: Table) {
        self.metatable = Some(Box::new(table));
    }

    /// Gets the metatable of this table
    pub fn metatable(&self) -> Option<&Table> {
        self.metatable.as_ref().map(|t| &**t)
    }
}

#[derive(Debug, Error)]
pub enum TableError {
    /// If the key was not found
    #[error("{key:?} not found in table")]
    KeyNotFound { key: String },
    /// If the given entry was found, but the requested type was wrong
    #[error("{key:?} is not of type {type_name}")]
    TypeMismatch {
        key: String,
        type_name: &'static str,
        actual_type_name: &'static str,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table() {
        let mut table = Table::new();
        table.set::<fn() -> Table>("new", Table::new);
        let new_table = table.get::<fn() -> Table>("new").unwrap()();
    }

    #[test]
    fn test_table_with_metadata() {
        let mut metatable = Table::new();
        metatable.set::<&'static str>("v", "hello, world!");
        let mut table = Table::new();
        assert!(!table.contains_key("v"));
        table.set_metatable(metatable);
        assert_eq!(*table.get::<&str>("v").unwrap(), "hello, world!");
    }
}
