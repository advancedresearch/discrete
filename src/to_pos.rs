/// Implemented for spaces which can convert an index to position type.
pub trait ToPos<T, U, N> {
    /// Converts index to position.
    fn to_pos(&self, dim: &T, index: N, pos: &mut U);
}
