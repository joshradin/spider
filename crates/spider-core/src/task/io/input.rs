use serde::Serialize;
use crate::task::io::Digest;

/// Represents a task input
pub trait Input {
    /// Fingerprint this input
    fn fingerprint(&self) -> Digest;
    /// Check if input has changed
    fn is_changed(&self) -> bool;
    /// Change reasons
    fn change_reasons(&self) -> Vec<String>;
}

pub struct InputProperty<T: Serialize> {
    pub key: String,
    pub value: T,
}

impl<T: Serialize> Input for InputProperty<T> {
    fn fingerprint(&self) -> Digest {
        todo!()
    }

    fn is_changed(&self) -> bool {
        todo!()
    }

    fn change_reasons(&self) -> Vec<String> {
        todo!()
    }
}

/// Represents the inputs of a [`Task`](super::Task)
pub struct TaskInputs {
    inputs: Vec<Box<dyn Input + Send + Sync>>,
}

impl TaskInputs {
    /// Create a new task inputs
    pub(crate) fn new() -> Self {
        Self { inputs: Vec::new() }
    }

    pub fn add(&mut self, i: impl Input + Send + Sync + 'static) -> &mut Self {
        self
    }

    #[inline]
    pub fn property(
        &mut self,
        key: impl AsRef<str>,
        value: impl Serialize + Send + Sync + 'static,
    ) -> &mut Self {
        self.add(InputProperty {
            key: key.as_ref().to_string(),
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fingerprint() {
        let mut inputs = TaskInputs::new();
        inputs.property(
            "value",
            13
        );

    }
}
