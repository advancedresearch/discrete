/// Constructs a new space.
///
/// Since spaces are combines as phantom types,
/// an instance must be constructed to call methods on it.
pub trait Construct {
    /// Constructs a new Self.
    fn new() -> Self;
}
