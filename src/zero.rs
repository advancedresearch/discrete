/// Used to construct an uninitialized element of a discrete space.
pub trait Zero<T, U, N> {
    /// Creates a default element.
    fn zero(&self, dim: &T) -> U;
}
