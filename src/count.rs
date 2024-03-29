/// Implemented by spaces that can count the number of objects.
pub trait Count<T, N> {
    /// Counts the size of space given the dimensions.
    fn count(&self, dim: &T) -> N;
}
