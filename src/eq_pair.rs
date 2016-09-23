use std::marker::PhantomData;

use Construct;
use Data;
use Count;
use Of;
use ToIndex;
use ToPos;
use Zero;

/// Dimension is natural number, position is (min, max).
pub struct EqPair<T = Data>(PhantomData<T>);

impl<T> Construct for EqPair<T> {
    fn new() -> EqPair<T> { EqPair(PhantomData) }
}

impl Count<usize> for EqPair<Data> {
    fn count(&self, dim: &usize) -> usize { dim * (dim + 1) / 2 }
}

impl<T, U> Count<U> for EqPair<Of<T>>
    where
        T: Construct + Count<U>
{
    fn count(&self, dim: &U) -> usize {
        let of: T = Construct::new();
        let data: EqPair<Data> = Construct::new();
        data.count(&of.count(dim))
    }
}


impl Zero<usize, (usize, usize)> for EqPair<Data> {
    fn zero(&self, _dim: &usize) -> (usize, usize) { (0, 0) }
}

impl<T, U, V>
Zero<U, (V, V)> for EqPair<Of<T>>
    where T: Construct + Zero<U, V>
{
    fn zero(&self, dim: &U) -> (V, V) {
        let of: T = Construct::new();
        (of.zero(dim), of.zero(dim))
    }
}

impl ToIndex<usize, (usize, usize)>
for EqPair<Data> {
    fn to_index(&self, _dim: &usize, &(min, max): &(usize, usize)) -> usize {
        min + max * (max + 1) / 2
    }
}

impl<T, U, V>
ToIndex<U, (V, V)> for EqPair<Of<T>>
    where T: Construct + ToIndex<U, V> + Count<U>
{
    fn to_index(
        &self,
        dim: &U,
        &(ref min, ref max): &(V, V)
    ) -> usize {
        let of: T = Construct::new();
        let data: EqPair<Data> = Construct::new();
        let min = of.to_index(dim, min);
        let max = of.to_index(dim, max);
        data.to_index(&self.count(dim), &(min, max))
    }
}

impl ToPos<usize, (usize, usize)> for EqPair<Data> {
    fn to_pos(&self, _dim: &usize, index: usize, pos: &mut (usize, usize)) {
        let max = ((-1f64 + (8f64 * index as f64 + 1f64).sqrt()) / 2f64) as usize;
        let min = index - max * (max + 1) / 2;
        *pos = (min, max)
    }
}

impl<T, U, V>
ToPos<U, (V, V)> for EqPair<Of<T>>
    where T: Construct + Count<U> + ToPos<U, V>
{
    fn to_pos(
        &self,
        dim: &U,
        index: usize,
        &mut (ref mut min, ref mut max): &mut (V, V)
    ) {
        let of: T = Construct::new();
        let data: EqPair<Data> = Construct::new();
        let count = self.count(dim);
        let mut pair = (0, 0);
        data.to_pos(&count, index, &mut pair);
        let (pair_min, pair_max) = pair;
        of.to_pos(dim, pair_min, min);
        of.to_pos(dim, pair_max, max);
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn features() {
        is_complete::<EqPair, usize, (usize, usize)>();
        is_complete::<EqPair<Of<EqPair>>, usize,
            ((usize, usize), (usize, usize))>();
    }

    #[test]
    fn test_eq_pair() {
        // 1 0 0 0
        // 2 3 0 0
        // 4 5 6 0
        // 7 8 9 10

        let eq_pair: EqPair = Construct::new();
        let ref n = 4;
        let count = eq_pair.count(n);
        assert_eq!(count, 10);

        let mut pos = (0, 0);
        for x in 0..count {
            eq_pair.to_pos(n, x, &mut pos);
            println!("{:?}", pos);
            assert_eq!(eq_pair.to_index(n, &pos), x);
        }
    }
}
