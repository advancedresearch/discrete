use std::marker::PhantomData;

use Construct;
use Data;
use Count;
use Of;
use ToIndex;
use ToPos;
use Zero;

/// Dimension is natural number, position is the same as index.
pub struct Dimension<T = Data>(PhantomData<T>);

impl<T> Construct for Dimension<T> {
    fn new() -> Dimension<T> { Dimension(PhantomData) }
}

impl Count<usize> for Dimension<Data> {
    fn count(&self, dim: usize) -> usize { dim }
}

impl<T, U> Count<U> for Dimension<Of<T>>
    where
        T: Construct + Count<U>
{
    fn count(&self, dim: U) -> usize {
        let of: T = Construct::new();
        of.count(dim)
    }
}

impl Zero<usize, usize> for Dimension<Data> {
    fn zero(&self, _dim: usize) -> usize { 0 }
}

impl<T, U, V>
Zero<U, V> for Dimension<Of<T>>
    where
        T: Construct + Zero<U, V>,
        U: Copy
{
    fn zero(&self, dim: U) -> V {
        let of: T = Construct::new();
        of.zero(dim)
    }
}

impl ToIndex<usize, usize> for Dimension<Data> {
    fn to_index(&self, _dim: usize, pos: &usize) -> usize { *pos }
}

impl<T, U, V> ToIndex<U, V> for Dimension<Of<T>>
    where
        T: Construct + ToIndex<U, V>
{
    fn to_index(&self, dim: U, pos: &V) -> usize {
        let of: T = Construct::new();
        of.to_index(dim, pos)
    }
}

impl ToPos<usize, usize> for Dimension<Data> {
    fn to_pos(&self, _dim: usize, index: usize, pos: &mut usize) {
        *pos = index;
    }
}

impl<T, U, V> ToPos<U, V> for Dimension<Of<T>>
    where
        T: Construct + ToPos<U, V>
{
    fn to_pos(&self, dim: U, index: usize, pos: &mut V) {
        let of: T = Construct::new();
        of.to_pos(dim, index, pos);
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn features() {
        is_complete::<Dimension, usize, usize>();
        is_complete::<Dimension<Of<Pair>>, usize, (usize, usize)>();
        does_zero::<Dimension, usize, usize>();
        does_zero::<Dimension<Of<Pair>>,
            usize,
            (usize, usize)>();
    }
}
