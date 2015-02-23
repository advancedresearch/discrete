use std::num::Float;
use std::marker::PhantomData;

use Construct;
use Data;
use Count;
use Of;
use Subspace;
use ToIndex;
use ToPos;

/// Dimension is natural number, position is (min, max).
pub struct Pair<T>(PhantomData<T>);

impl<T> Construct for Pair<T> {
    fn new() -> Pair<T> { Pair(PhantomData) }
}

impl Count<usize> for Pair<Data> {
    fn count(&self, dim: usize) -> usize { dim * (dim - 1) / 2 }
}

impl<T, U> Count<(usize, U)> for Pair<Subspace<T>>
    where
        T: Construct + Count<U>
{
    fn count(&self, (a, b): (usize, U)) -> usize {
        let subspace: T = Construct::new();
        let data: Pair<Data> = Construct::new();
        data.count(a) * subspace.count(b)
    }
}

impl<T, U> Count<U> for Pair<Of<T>>
    where
        T: Construct + Count<U>
{
    fn count(&self, dim: U) -> usize {
        let of: T = Construct::new();
        let data: Pair<Data> = Construct::new();
        data.count(of.count(dim))
    }
}

impl ToIndex<usize, (usize, usize)>
for Pair<Data> {
    fn to_index(&self, _dim: usize, (min, max): (usize, usize)) -> usize {
        min + max * (max - 1) / 2
    }
}

impl<T, U, V>
ToIndex<(usize, U), ((usize, usize), V)>
for Pair<Subspace<T>>
    where
        T: Construct + Count<U> + ToIndex<U, V>,
        U: Copy
{
    fn to_index(
        &self,
        (a, b): (usize, U),
        (pa, pb): ((usize, usize), V)
    ) -> usize {
        let subspace: T = Construct::new();
        let count = subspace.count(b);
        let data: Pair<Data> = Construct::new();
        data.to_index(a, pa) * count + subspace.to_index(b, pb)
    }
}

impl<T, U, V>
ToIndex<U, (V, V)> for Pair<Of<T>>
    where
        T: Construct + ToIndex<U, V> + Count<U>,
        U: Copy
{
    fn to_index(
        &self,
        dim: U,
        (min, max): (V, V)
    ) -> usize {
        let of: T = Construct::new();
        let data: Pair<Data> = Construct::new();
        let min = of.to_index(dim, min);
        let max = of.to_index(dim, max);
        data.to_index(self.count(dim), (min, max))
    }
}

impl<'a> ToPos<usize, &'a mut (usize, usize)> for Pair<Data> {
    fn to_pos(&self, _dim: usize, index: usize, pos: &'a mut (usize, usize)) {
        let max = ((-1f64 + (8f64 * index as f64 + 1f64).sqrt()) / 2f64) as usize + 1;
        let min = index - max * (max + 1) / 2 + max;
        *pos = (min, max)
    }
}

impl<'a, T, U, V>
ToPos<(usize, U), &'a mut ((usize, usize), V)> for Pair<Subspace<T>>
    where
        T: Construct + Count<U> + ToPos<U, &'a mut V>,
        U: Copy
{
    fn to_pos(
        &self,
        (a, b): (usize, U),
        index: usize,
        &mut (ref mut head, ref mut tail): &'a mut ((usize, usize), V)
    ) {
        let subspace: T = Construct::new();
        let count = subspace.count(b);
        let data: Pair<Data> = Construct::new();
        let x = index / count;
        data.to_pos(a, x, head);
        subspace.to_pos(b, index - x * count, tail)
    }
}

impl<'a, T, U, V>
ToPos<U, (V, V)> for Pair<Of<T>>
    where
        T: Construct + Count<U> + ToPos<U, V>,
        U: Copy
{
    fn to_pos(
        &self,
        dim: U,
        index: usize,
        (min, max): (V, V)
    ) {
        let of: T = Construct::new();
        let data: Pair<Data> = Construct::new();
        let count = self.count(dim);
        let mut pair = (0, 0);
        data.to_pos(count, index, &mut pair);
        let (pair_min, pair_max) = pair;
        of.to_pos(dim, pair_min, min);
        of.to_pos(dim, pair_max, max);
    }
}
