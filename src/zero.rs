/// Used to construct an element of a discrete space.
pub trait Zero<T, U> {
    /// Creates a default element.
    fn zero(&self, dim: T) -> U;
}
