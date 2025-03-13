//! task inputs and outputs

use serde::Serialize;

pub mod input;
pub mod output;

#[derive(Debug, Clone, Copy)]
pub struct Digest(md5::Digest);

/// Represents a task output
pub trait Output {}



/// Represents the outputs of a [`Task`](super::Task)
pub struct TaskOutputs {

}