//! Lazy apis

use futures::future::BoxFuture;
use pin_project::pin_project;
use crate::from_state::FromState;

pub mod value_source;
pub mod providers;
pub mod properties;

#[cfg(test)]
mod tests {
    use tokio::test;


    #[test]
    async fn test_provider() {

    }
}
