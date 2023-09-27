use std::marker::PhantomData;

use num::BigUint;

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

impl Count<BigUint> for Pair<Data> {
    type N = BigUint;
    fn count(&self, dim: &BigUint) -> BigUint {
        let _1: BigUint = 1usize.into();
        let _2: BigUint = 2usize.into();
        dim * (dim - _1) / _2
    }
}

impl<T, U> Count<U> for Pair<Of<T>>
    where
        T: Construct + Count<U>,
        Pair: Count<<T as Count<U>>::N, N = <T as Count<U>>::N>
{
    type N = <T as Count<U>>::N;
    fn count(&self, dim: &U) -> Self::N {
        let of: T = Construct::new();
        let data: Pair<Data> = Construct::new();
        data.count(&of.count(dim))
    }
}

impl Zero<usize, (usize, usize)> for Pair<Data> {
    fn zero(&self, _dim: &usize) -> (usize, usize) { (0, 0) }
}

impl Zero<BigUint, (BigUint, BigUint)> for Pair<Data> {
    fn zero(&self, _dim: &BigUint) -> (BigUint, BigUint) { (0usize.into(), 0usize.into()) }
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
    type N = usize;
    fn to_index(&self, _dim: &usize, &(min, max): &(usize, usize)) -> usize {
        min + max * (max - 1) / 2
    }
}

impl<T, U, V>
ToIndex<U, (V, V)> for Pair<Of<T>>
    where T: Construct + ToIndex<U, V, N = usize> + Count<U, N = usize>
{
    type N = usize;
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
    type N = usize;
    fn to_pos(&self, _dim: &usize, index: usize, pos: &mut (usize, usize)) {
        use num::integer::Roots;
        let max = (1 + (8 * index as u128 + 1).sqrt()) / 2;
        let min = (index as u128).wrapping_sub(max * (max + 1) / 2).wrapping_add(max);
        *pos = (min as usize, max as usize)
    }
}

impl<T, U, V>
ToPos<U, (V, V)> for Pair<Of<T>>
    where T: Construct + Count<U, N = usize> + ToPos<U, V, N = usize>
{
    type N = usize;
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
