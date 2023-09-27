/// Implemented for spaces which can convert an index to position type.
pub trait ToPos<T, U> {
    /// The numeric type.
    type N;
    /// Converts index to position.
    fn to_pos(&self, dim: &T, index: Self::N, pos: &mut U);
}
