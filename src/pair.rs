use std::marker::PhantomData;

use crate::BigUint;

use Construct;
use Data;
use Of;
use space::Space;

/// Dimension is natural number, position is (min, max).
pub struct Pair<T = Data>(PhantomData<T>);

impl<T> Construct for Pair<T> {
    fn new() -> Self { Pair(PhantomData) }
}

impl Space<usize> for Pair<Data> {
    type Dim = usize;
    type Pos = (usize, usize);
    fn count(&self, dim: &usize) -> usize { dim * (dim - 1) / 2 }
    fn zero(&self, _dim: &usize) -> (usize, usize) { (0, 0) }
    fn to_index(&self, _dim: &usize, &(min, max): &(usize, usize)) -> usize {
        if max == 0 {0} else {
            min + max * (max - 1) / 2
        }
    }
    fn to_pos(&self, _dim: &usize, index: usize, pos: &mut (usize, usize)) {
        use num_integer::Roots;
        let index = index as u128;
        let max = (1 + (8 * index + 1).sqrt()) / 2;
        let d = max * (max + 1) / 2;
        let min = (index).wrapping_add(max).wrapping_sub(d);
        *pos = (min as usize, max as usize)
    }
}

impl Space<BigUint> for Pair<Data> {
    type Dim = BigUint;
    type Pos = (BigUint, BigUint);
    fn count(&self, dim: &BigUint) -> BigUint {
        dim * (dim - 1usize) / 2usize
    }
    fn zero(&self, _dim: &BigUint) -> (BigUint, BigUint) { (0usize.into(), 0usize.into()) }
    fn to_index(&self, _dim: &Self::Dim, (min, max): &Self::Pos) -> BigUint {
        let _0 = 0usize.into();
        if max == &_0 {_0}
        else {
            min + max * (max - 1usize) / 2usize
        }
    }
    fn to_pos(&self, _dim: &BigUint, index: BigUint, pos: &mut (BigUint, BigUint)) {
        let max: BigUint = (1usize + (8usize * &index + 1usize).sqrt()) / 2usize;
        let d = &max * (&max + 1usize) / 2usize;
        let min = &index + &max - d;
        *pos = (min, max)
    }
}

impl<N, T> Space<N> for Pair<Of<T>>
    where T: Space<N>,
          Pair<Data>: Space<N, Dim = N, Pos = (N, N)>,
{
    type Dim = T::Dim;
    type Pos = (T::Pos, T::Pos);
    fn count(&self, dim: &Self::Dim) -> N {
        let of: T = Construct::new();
        let data: Pair<Data> = Construct::new();
        data.count(&of.count(dim))
    }
    fn zero(&self, dim: &Self::Dim) -> Self::Pos {
        let of: T = Construct::new();
        (of.zero(dim), of.zero(dim))
    }
    fn to_index(
        &self,
        dim: &Self::Dim,
        &(ref min, ref max): &Self::Pos,
    ) -> N {
        let of: T = Construct::new();
        let data: Pair<Data> = Construct::new();
        let min = of.to_index(dim, min);
        let max = of.to_index(dim, max);
        data.to_index(&self.count(dim), &(min, max))
    }
    fn to_pos(
        &self,
        dim: &Self::Dim,
        index: N,
        &mut (ref mut min, ref mut max): &mut Self::Pos,
    ) {
        let of: T = Construct::new();
        let data: Pair<Data> = Construct::new();
        let count = of.count(dim);
        let mut pair = data.zero(&count);
        data.to_pos(&count, index, &mut pair);
        let (pair_min, pair_max) = pair;
        of.to_pos(dim, pair_min, min);
        of.to_pos(dim, pair_max, max);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn features() {
        is_complete::<usize, Pair>();
        is_complete::<usize, Pair<Of<Pair>>>();
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

    fn conv_pos((a, b): (usize, usize)) -> (BigUint, BigUint) {
        (a.into(), b.into())
    }

    #[test]
    fn data_big() {
        let x: Pair = Construct::new();
        let ref dim: BigUint = 4usize.into();
        assert_eq!(x.count(dim), 6usize.into());
        assert_eq!(x.to_index(dim, &conv_pos((0, 1))), 0usize.into());
        assert_eq!(x.to_index(dim, &conv_pos((0, 2))), 1usize.into());
        assert_eq!(x.to_index(dim, &conv_pos((1, 2))), 2usize.into());
        assert_eq!(x.to_index(dim, &conv_pos((0, 3))), 3usize.into());
        let mut new_pos = x.zero(dim);
        x.to_pos(dim, 3usize.into(), &mut new_pos);
        assert_eq!(new_pos, conv_pos((0, 3)));
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

    fn conv(v: Vec<usize>) -> Vec<BigUint> {
        v.into_iter().map(|n| n.into()).collect()
    }

    fn conv_pos_of((a, b): (Vec<usize>, Vec<usize>)) -> (Vec<BigUint>, Vec<BigUint>) {
        (conv(a), conv(b))
    }

    #[test]
    fn of_big() {
        let x: Pair<Of<DimensionN>> = Construct::new();
        let ref dim = conv(vec![2, 2]);
        assert_eq!(x.count(dim), 6usize.into());
        assert_eq!(x.to_index(dim, &conv_pos_of((vec![0, 0], vec![1, 0]))), 0usize.into());
        assert_eq!(x.to_index(dim, &conv_pos_of((vec![0, 0], vec![0, 1]))), 1usize.into());
        assert_eq!(x.to_index(dim, &conv_pos_of((vec![1, 0], vec![0, 1]))), 2usize.into());
        assert_eq!(x.to_index(dim, &conv_pos_of((vec![0, 0], vec![1, 1]))), 3usize.into());
        assert_eq!(x.to_index(dim, &conv_pos_of((vec![1, 0], vec![1, 1]))), 4usize.into());
        assert_eq!(x.to_index(dim, &conv_pos_of((vec![0, 1], vec![1, 1]))), 5usize.into());
        let mut pos = x.zero(dim);
        for i in 0usize..6 {
            x.to_pos(dim, i.into(), &mut pos);
            // println!("{} {}", &min[], &max[]);
            assert_eq!(x.to_index(dim, &pos), i.into());
        }
        x.to_pos(dim, 5usize.into(), &mut pos);
        assert_eq!(pos, conv_pos_of((vec![0, 1], vec![1, 1])));
    }
}
