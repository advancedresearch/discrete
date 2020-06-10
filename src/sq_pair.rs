use std::marker::PhantomData;

use Construct;
use Data;
use Count;
use Of;
use ToIndex;
use ToPos;
use Zero;

/// A discrete space that models a full square of NxN pairs.
///
/// Dimension is natural number, position is `(x, y)`.
pub struct SqPair<T = Data>(PhantomData<T>);

impl<T> Construct for SqPair<T> {
    fn new() -> SqPair<T> {SqPair(PhantomData)}
}

impl Count<usize> for SqPair<Data> {
    fn count(&self, dim: &usize) -> usize {dim * dim}
}

impl<T, U> Count<U> for SqPair<Of<T>>
    where
        T: Construct + Count<U>
{
    fn count(&self, dim: &U) -> usize {
        let of: T = Construct::new();
        let data: SqPair<Data> = Construct::new();
        data.count(&of.count(dim))
    }
}

impl Zero<usize, (usize, usize)> for SqPair<Data> {
    fn zero(&self, _: &usize) -> (usize, usize) {(0, 0)}
}

impl<T, U, V>
Zero<U, (V, V)> for SqPair<Of<T>>
    where
        T: Construct + Zero<U, V>
{
    fn zero(&self, dim: &U) -> (V, V) {
        let of: T = Construct::new();
        (of.zero(dim), of.zero(dim))
    }
}

impl ToIndex<usize, (usize, usize)> for SqPair<Data> {
    fn to_index(&self, dim: &usize, &(a, b): &(usize, usize)) -> usize {
        a + b * dim
    }
}

impl<T, U, V>
ToIndex<U, (V, V)> for SqPair<Of<T>>
    where
        T: Construct + ToIndex<U, V> + Count<U>
{
    fn to_index(
        &self,
        dim: &U,
        &(ref a, ref b): &(V, V)
    ) -> usize {
        let of: T = Construct::new();
        let data: SqPair<Data> = Construct::new();
        let a = of.to_index(dim, a);
        let b = of.to_index(dim, b);
        data.to_index(&self.count(dim), &(a, b))
    }
}

impl ToPos<usize, (usize, usize)> for SqPair<Data> {
    fn to_pos(&self, dim: &usize, index: usize, pos: &mut (usize, usize)) {
        pos.0 = index % dim;
        pos.1 = index / dim;
    }
}

impl<T, U, V>
ToPos<U, (V, V)> for SqPair<Of<T>>
    where
        T: Construct + Count<U> + ToPos<U, V>
{
    fn to_pos(
        &self,
        dim: &U,
        index: usize,
        &mut (ref mut a, ref mut b): &mut (V, V)
    ) {
        let of: T = Construct::new();
        let data: SqPair<Data> = Construct::new();
        let count = of.count(dim);
        let mut pair = (0, 0);
        data.to_pos(&count, index, &mut pair);
        let (pair_a, pair_b) = pair;
        of.to_pos(dim, pair_a, a);
        of.to_pos(dim, pair_b, b);
    }
}
