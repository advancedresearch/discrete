use std::marker::PhantomData;

use Construct;
use Data;
use Count;
use Of;
use Subspace;
use ToIndex;
use ToPos;

/// Dimension is natural number, position is the same as index.
pub struct Dimension<T>(PhantomData<T>);

impl<T> Construct for Dimension<T> {
    fn new() -> Dimension<T> { Dimension(PhantomData) }
}

impl Count<usize> for Dimension<Data> {
    fn count(&self, dim: usize) -> usize { dim }
}

impl<T, U> Count<(usize, U)> for Dimension<Subspace<T>>
    where
        T: Construct + Count<U>
{
    fn count(&self, (a, b): (usize, U)) -> usize {
        let subspace: T = Construct::new();
        a * subspace.count(b)
    }
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

impl ToIndex<usize, usize> for Dimension<Data> {
    fn to_index(&self, _dim: usize, pos: usize) -> usize { pos }
}

impl<T, U: Copy, V> ToIndex<(usize, U), (usize, V)> for Dimension<Subspace<T>>
    where
        T: Construct + Count<U> + ToIndex<U, V>
{
    fn to_index(&self, (_a, b): (usize, U), (pa, pb): (usize, V)) -> usize {
        let subspace: T = Construct::new();
        let count = subspace.count(b);
        pa * count + subspace.to_index(b, pb)
    }
}

impl ToPos<usize, usize> for Dimension<Data> {
    fn to_pos(&self, _dim: usize, index: usize, pos: &mut usize) {
        *pos = index;
    }
}

impl<T, U: Copy, V>
ToPos<(usize, U), (usize, V)> for Dimension<Subspace<T>>
    where
        T: Construct + Count<U> + ToPos<U, V>
{
    fn to_pos(
        &self,
        (_a, b): (usize, U),
        index: usize,
        &mut (ref mut head, ref mut tail): &mut (usize, V)
    ) {
        let subspace: T = Construct::new();
        let count = subspace.count(b);
        let x = index / count;
        *head = x;
        subspace.to_pos(b, index - x * count, tail)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn features() {
        is_complete::<Dimension<Data>, usize, usize, usize>();
        does_count::<Dimension<Subspace<Pair<Data>>>, (usize, usize)>();
        does_count::<Dimension<Of<Pair<Data>>>, usize>();
    }

    #[test]
    fn subspace() {
        type D2 = Dimension<Subspace<Dimension<Data>>>;
        type D3 = Dimension<Subspace<D2>>;

        let x: D3 = Construct::new();
        let dim = (3, (3, 3));
        assert_eq!(x.count(dim), 27);
        let pos = (1, (0, 2));
        let index = x.to_index(dim, pos);
        assert_eq!(index, 11);
        let mut new_pos = pos;
        x.to_pos(dim, index, &mut new_pos);
        assert_eq!(pos, new_pos);
    }
}
