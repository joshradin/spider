//! A trait for creating a value from a stateful object

/// Create some value of type [`Self::Output`] from state `S`
pub trait FromState<S> {
    async fn from_state(state: &S) -> Self;
}

impl<T: Default> FromState<()> for T {
    async fn from_state(_state: &()) -> Self {
        Self::default()
    }
}

/// Create the specified object from this state
pub trait Create<T> {
    /// Create a value of type T from input states
    async fn create(&self) -> T;
}



impl<T: FromState<S>, S> Create<T> for S {
    async fn create(&self) -> T {
        T::from_state(self).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct World(u32);
    struct Value(u32);
    impl FromState<World> for Value {
        async fn from_state(state: &World) -> Self {
            Self(state.0)
        }
    }
    #[tokio::test]
    async fn test_from_state() {
        let world = World(13);
        let s: Value = world.create().await;
        assert_eq!(s.0, world.0)
    }
}