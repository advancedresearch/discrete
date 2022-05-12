use std::marker::PhantomData;

use Construct;
use Data;
use Count;
use Of;
use ToIndex;
use ToPos;
use Zero;

/// Dimension is natural number, position is (min, max).
pub struct Pair<T = Data>(PhantomData<T>);

impl<T> Construct for Pair<T> {
    fn new() -> Pair<T> { Pair(PhantomData) }
}

impl Count<usize> for Pair<Data> {
    type N = usize;
    fn count(&self, dim: &usize) -> usize { dim * (dim - 1) / 2 }
}

impl<T, U> Count<U> for Pair<Of<T>>
    where
        T: Construct + Count<U, N = usize>
{
    type N = usize;
    fn count(&self, dim: &U) -> usize {
        let of: T = Construct::new();
        let data: Pair<Data> = Construct::new();
        data.count(&of.count(dim))
    }
}

impl Zero<usize, (usize, usize)> for Pair<Data> {
    fn zero(&self, _dim: &usize) -> (usize, usize) { (0, 0) }
}

impl<T, U, V>
Zero<U, (V, V)> for Pair<Of<T>>
    where T: Construct + Zero<U, V>
{
    fn zero(&self, dim: &U) -> (V, V) {
        let of: T = Construct::new();
        (of.zero(dim), of.zero(dim))
    }
}

impl ToIndex<usize, (usize, usize)>
for Pair<Data> {
    fn to_index(&self, _dim: &usize, &(min, max): &(usize, usize)) -> usize {
        min + max * (max - 1) / 2
    }
}

impl<T, U, V>
ToIndex<U, (V, V)> for Pair<Of<T>>
    where T: Construct + ToIndex<U, V> + Count<U, N = usize>
{
    fn to_index(
        &self,
        dim: &U,
        &(ref min, ref max): &(V, V)
    ) -> usize {
        let of: T = Construct::new();
        let data: Pair<Data> = Construct::new();
        let min = of.to_index(dim, min);
        let max = of.to_index(dim, max);
        data.to_index(&self.count(dim), &(min, max))
    }
}

impl ToPos<usize, (usize, usize)> for Pair<Data> {
    fn to_pos(&self, _dim: &usize, index: usize, pos: &mut (usize, usize)) {
        let max = ((-1f64 + (8f64 * index as f64 + 1f64).sqrt()) / 2f64) as usize + 1;
        let min = index.wrapping_sub(max * (max + 1) / 2).wrapping_add(max);
        *pos = (min, max)
    }
}

impl<T, U, V>
ToPos<U, (V, V)> for Pair<Of<T>>
    where T: Construct + Count<U, N = usize> + ToPos<U, V>
{
    fn to_pos(
        &self,
        dim: &U,
        index: usize,
        &mut (ref mut min, ref mut max): &mut (V, V)
    ) {
        let of: T = Construct::new();
        let data: Pair<Data> = Construct::new();
        let count = of.count(dim);
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
        is_complete::<Pair, usize, (usize, usize)>();
        is_complete::<Pair<Of<Pair>>, usize,
            ((usize, usize), (usize, usize))>();
    }

    #[test]
    fn data() {
        let x: Pair = Construct::new();
        let ref dim = 4;
        assert_eq!(x.count(dim), 6);
        assert_eq!(x.to_index(dim, &(0, 1)), 0);
        assert_eq!(x.to_index(dim, &(0, 2)), 1);
        assert_eq!(x.to_index(dim, &(1, 2)), 2);
        assert_eq!(x.to_index(dim, &(0, 3)), 3);
        let mut new_pos = (0, 0);
        x.to_pos(dim, 3, &mut new_pos);
        assert_eq!(new_pos, (0, 3));
    }

    #[test]
    fn of() {
        let x: Pair<Of<DimensionN>> = Construct::new();
        let ref dim = vec![2, 2];
        assert_eq!(x.count(dim), 6);
        assert_eq!(x.to_index(dim, &(vec![0, 0], vec![1, 0])), 0);
        assert_eq!(x.to_index(dim, &(vec![0, 0], vec![0, 1])), 1);
        assert_eq!(x.to_index(dim, &(vec![1, 0], vec![0, 1])), 2);
        assert_eq!(x.to_index(dim, &(vec![0, 0], vec![1, 1])), 3);
        assert_eq!(x.to_index(dim, &(vec![1, 0], vec![1, 1])), 4);
        assert_eq!(x.to_index(dim, &(vec![0, 1], vec![1, 1])), 5);
        let mut pos = (Vec::new(), Vec::new());
        for i in 0..6 {
            x.to_pos(dim, i, &mut pos);
            // println!("{} {}", &min[], &max[]);
        }
        x.to_pos(dim, 5, &mut pos);
        assert_eq!(&pos.0, &[0, 1]);
        assert_eq!(&pos.1, &[1, 1]);
    }
}
