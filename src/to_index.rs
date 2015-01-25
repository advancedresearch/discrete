pub trait ToIndex<T, U> {
    fn to_index(&self, dim: T, pos: U) -> usize;
}
