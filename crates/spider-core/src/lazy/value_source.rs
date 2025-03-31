//! Defines the [`ValueSource`] trait

/// A value source
pub trait ValueSource {
    type Properties: Send + Sync;
    type Output: Send + Sync;

    /// Get a value from the given properties
    fn get(
        self,
        properties: &Self::Properties,
    ) -> impl Future<Output = Option<Self::Output>> + Send + Sync;
}
//
// /// Creates a callable from a value source
// #[inline]
// pub fn into_callable<Vs>(vs: Vs) -> impl FnOnce() -> Option<Vs::Output>
// where
//     Vs: ValueSource<Properties: Default>,
// {
//     into_callable_with(vs, Vs::Properties::default())
// }
//
// /// Creates a callable with the given properties
// pub fn into_callable_with<Vs>(vs: Vs, props: Vs::Properties) -> impl FnOnce() -> Option<Vs::Output>
// where
//     Vs: ValueSource<Properties: Sized>,
// {
//     move || vs.get(&props)
// }
