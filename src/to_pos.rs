pub trait ToPos<T, U> {
    fn to_pos(&self, dim: T, index: usize, pos: U);
}
