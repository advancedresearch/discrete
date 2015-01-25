pub trait Count<T> {
    fn count(&self, dim: T) -> usize;
}
