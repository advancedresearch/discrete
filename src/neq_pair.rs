use std::marker::PhantomData;

use Construct;
use Data;
use Count;
use Of;
use ToIndex;
use ToPos;
use Zero;

/// Dimension is natural number, position is (a, b).
/// Represents all directional pairs that has not same element for `a` and `b`.
pub struct NeqPair<T = Data>(PhantomData<T>);

impl<T> Construct for NeqPair<T> {
    fn new() -> NeqPair<T> { NeqPair(PhantomData) }
}

impl Count<usize> for NeqPair<Data> {
    fn count(&self, dim: usize) -> usize { dim * (dim - 1) }
}

impl<T, U> Count<U> for NeqPair<Of<T>>
    where
        T: Construct + Count<U>
{
    fn count(&self, dim: U) -> usize {
        let of: T = Construct::new();
        let data: NeqPair<Data> = Construct::new();
        data.count(of.count(dim))
    }
}

impl Zero<usize, (usize, usize)> for NeqPair<Data> {
    fn zero(&self, _dim: usize) -> (usize, usize) { (0, 0) }
}

impl<T, U, V>
Zero<U, (V, V)> for NeqPair<Of<T>>
    where
        T: Construct + Zero<U, V>,
        U: Copy
{
    fn zero(&self, dim: U) -> (V, V) {
        let of: T = Construct::new();
        (of.zero(dim), of.zero(dim))
    }
}

impl ToIndex<usize, (usize, usize)>
for NeqPair<Data> {
    fn to_index(&self, dim: usize, &(a, b): &(usize, usize)) -> usize {
        use Pair;

        let pair: Pair<Data> = Construct::new();
        if a < b {
            pair.to_index(dim, &(a, b)) * 2
        } else {
            pair.to_index(dim, &(b, a)) * 2 + 1
        }
    }
}

impl<T, U, V>
ToIndex<U, (V, V)> for NeqPair<Of<T>>
    where
        T: Construct + ToIndex<U, V> + Count<U>,
        U: Copy
{
    fn to_index(
        &self,
        dim: U,
        &(ref min, ref max): &(V, V)
    ) -> usize {
        let of: T = Construct::new();
        let data: NeqPair<Data> = Construct::new();
        let min = of.to_index(dim, min);
        let max = of.to_index(dim, max);
        data.to_index(self.count(dim), &(min, max))
    }
}

impl ToPos<usize, (usize, usize)> for NeqPair<Data> {
    fn to_pos(&self, dim: usize, index: usize, pos: &mut (usize, usize)) {
        use Pair;

        let pair: Pair<Data> = Construct::new();
        if index % 2 == 0 {
            pair.to_pos(dim, index / 2, pos);
        } else {
            pair.to_pos(dim, (index - 1) / 2, pos);
            let tmp = pos.1;
            pos.1 = pos.0;
            pos.0 = tmp;
        }
    }
}

impl<T, U, V>
ToPos<U, (V, V)> for NeqPair<Of<T>>
    where
        T: Construct + Count<U> + ToPos<U, V>,
        U: Copy
{
    fn to_pos(
        &self,
        dim: U,
        index: usize,
        &mut (ref mut min, ref mut max): &mut (V, V)
    ) {
        let of: T = Construct::new();
        let data: NeqPair<Data> = Construct::new();
        let count = self.count(dim);
        let mut pair = (0, 0);
        data.to_pos(count, index, &mut pair);
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
        is_complete::<NeqPair, usize, (usize, usize)>();
        is_complete::<NeqPair<Of<NeqPair>>, usize,
            ((usize, usize), (usize, usize))>();
    }

    #[test]
    fn data() {
        let x: NeqPair = Construct::new();
        let dim = 4;
        assert_eq!(x.count(dim), 12);
        assert_eq!(x.to_index(dim, &(0, 1)), 0);
        assert_eq!(x.to_index(dim, &(1, 0)), 1);
        assert_eq!(x.to_index(dim, &(0, 2)), 2);
        assert_eq!(x.to_index(dim, &(2, 0)), 3);
        assert_eq!(x.to_index(dim, &(1, 2)), 4);
        assert_eq!(x.to_index(dim, &(2, 1)), 5);
        assert_eq!(x.to_index(dim, &(0, 3)), 6);
        let mut new_pos = (0, 0);
        x.to_pos(dim, 6, &mut new_pos);
        assert_eq!(new_pos, (0, 3));
        x.to_pos(dim, 5, &mut new_pos);
        assert_eq!(new_pos, (2, 1));
    }

    #[test]
    fn of() {
        let x: NeqPair<Of<DimensionN>> = Construct::new();
        let dim = [2, 2];
        assert_eq!(x.count(&dim), 12);
        assert_eq!(x.to_index(&dim, &(vec![0, 0], vec![1, 0])), 0);
        assert_eq!(x.to_index(&dim, &(vec![0, 0], vec![0, 1])), 2);
        assert_eq!(x.to_index(&dim, &(vec![1, 0], vec![0, 1])), 4);
        assert_eq!(x.to_index(&dim, &(vec![0, 0], vec![1, 1])), 6);
        assert_eq!(x.to_index(&dim, &(vec![1, 0], vec![1, 1])), 8);
        assert_eq!(x.to_index(&dim, &(vec![0, 1], vec![1, 1])), 10);
        let mut pos = (Vec::new(), Vec::new());
        for i in 0..6 {
            x.to_pos(&dim, i, &mut pos);
            // println!("{} {}", &min[], &max[]);
        }
        x.to_pos(&dim, 10, &mut pos);
        assert_eq!(&pos.0, &[0, 1]);
        assert_eq!(&pos.1, &[1, 1]);
    }
}
