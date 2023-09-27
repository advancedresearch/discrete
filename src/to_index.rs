/// Implemented by spaces that can convert position to index.
pub trait ToIndex<T, U> {
    /// The numeric type.
    type N;
    /// Converts position to index.
    fn to_index(&self, dim: &T, pos: &U) -> Self::N;
}
