use std::marker::PhantomData;

use crate::{
    BigUint,
    Construct,
    Data,
    Of,
    space::Space,
};

/// Dimension is natural number, position is (a, b).
/// Represents all directional pairs that has not same element for `a` and `b`.
pub struct NeqPair<T = Data>(PhantomData<T>);

impl<T> Construct for NeqPair<T> {
    fn new() -> Self { NeqPair(PhantomData) }
}

impl Space<usize> for NeqPair<Data> {
    type Dim = usize;
    type Pos = (usize, usize);
    fn count(&self, dim: &usize) -> usize { dim * (dim - 1) }
    fn zero(&self, _dim: &usize) -> (usize, usize) { (0, 0) }
    fn to_index(&self, dim: &usize, &(a, b): &(usize, usize)) -> usize {
        use crate::Pair;

        let pair: Pair<Data> = Construct::new();
        if a < b {
            <Pair as Space<usize>>::to_index(&pair, dim, &(a, b)) * 2
        } else {
            <Pair as Space<usize>>::to_index(&pair, dim, &(b, a)) * 2 + 1
        }
    }
    fn to_pos(&self, dim: &usize, index: usize, pos: &mut (usize, usize)) {
        use crate::Pair;

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

impl Space<BigUint> for NeqPair<Data> {
    type Dim = BigUint;
    type Pos = (BigUint, BigUint);
    fn count(&self, dim: &BigUint) -> BigUint {
        dim * (dim - 1usize)
    }
    fn zero(&self, _dim: &BigUint) -> (BigUint, BigUint) { (0usize.into(), 0usize.into()) }
    fn to_index(&self, dim: &Self::Dim, (a, b): &Self::Pos) -> BigUint {
        use crate::Pair;

        let pair: Pair<Data> = Construct::new();
        if a < b {
            <Pair as Space<BigUint>>::to_index(&pair, dim, &(a.clone(), b.clone())) * 2usize
        } else {
            <Pair as Space<BigUint>>::to_index(&pair, dim, &(b.clone(), a.clone())) * 2usize + 1usize
        }
    }
    fn to_pos(&self, dim: &Self::Dim, index: BigUint, pos: &mut Self::Pos) {
        use crate::Pair;

        let pair: Pair<Data> = Construct::new();
        if &index % 2usize == 0usize.into() {
            pair.to_pos(dim, index / 2usize, pos);
        } else {
            pair.to_pos(dim, (index - 1usize) / 2usize, pos);
            std::mem::swap(&mut pos.0, &mut pos.1);
        }
    }
}

impl<N, T> Space<N> for NeqPair<Of<T>>
    where T: Space<N>,
          NeqPair<Data>: Space<N, Dim = N, Pos = (N, N)>,
{
    type Dim = T::Dim;
    type Pos = (T::Pos, T::Pos);
    fn count(&self, dim: &Self::Dim) -> N {
        let of: T = Construct::new();
        let data: NeqPair<Data> = Construct::new();
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
        let data: NeqPair<Data> = Construct::new();
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
        let data: NeqPair<Data> = Construct::new();
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
    use super::super::*;

    #[test]
    fn features() {
        is_complete::<usize, NeqPair>();
        is_complete::<usize, NeqPair<Of<NeqPair>>>();
    }

    #[test]
    fn data() {
        let x: NeqPair = Construct::new();
        let ref dim = 4;
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

    fn conv_pos((a, b): (usize, usize)) -> (BigUint, BigUint) {
        (a.into(), b.into())
    }

    #[test]
    fn data_big() {
        let x: NeqPair = Construct::new();
        let ref dim: BigUint = 4usize.into();
        assert_eq!(x.count(dim), 12usize.into());
        assert_eq!(x.to_index(dim, &conv_pos((0, 1))), 0usize.into());
        assert_eq!(x.to_index(dim, &conv_pos((1, 0))), 1usize.into());
        assert_eq!(x.to_index(dim, &conv_pos((0, 2))), 2usize.into());
        assert_eq!(x.to_index(dim, &conv_pos((2, 0))), 3usize.into());
        assert_eq!(x.to_index(dim, &conv_pos((1, 2))), 4usize.into());
        assert_eq!(x.to_index(dim, &conv_pos((2, 1))), 5usize.into());
        assert_eq!(x.to_index(dim, &conv_pos((0, 3))), 6usize.into());
        let mut new_pos = x.zero(dim);
        x.to_pos(dim, 6usize.into(), &mut new_pos);
        assert_eq!(new_pos, conv_pos((0, 3)));
        x.to_pos(dim, 5usize.into(), &mut new_pos);
        assert_eq!(new_pos, conv_pos((2, 1)));
    }

    #[test]
    fn of() {
        let x: NeqPair<Of<DimensionN>> = Construct::new();
        let ref dim = vec![2, 2];
        assert_eq!(x.count(dim), 12);
        assert_eq!(x.to_index(dim, &(vec![0, 0], vec![1, 0])), 0);
        assert_eq!(x.to_index(dim, &(vec![0, 0], vec![0, 1])), 2);
        assert_eq!(x.to_index(dim, &(vec![1, 0], vec![0, 1])), 4);
        assert_eq!(x.to_index(dim, &(vec![0, 0], vec![1, 1])), 6);
        assert_eq!(x.to_index(dim, &(vec![1, 0], vec![1, 1])), 8);
        assert_eq!(x.to_index(dim, &(vec![0, 1], vec![1, 1])), 10);
        let mut pos = (Vec::new(), Vec::new());
        for i in 0..6 {
            x.to_pos(dim, i, &mut pos);
            // println!("{} {}", &min[], &max[]);
        }
        x.to_pos(dim, 10, &mut pos);
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
        let x: NeqPair<Of<DimensionN>> = Construct::new();
        let ref dim = conv(vec![2, 2]);
        assert_eq!(x.count(dim), 12usize.into());
        assert_eq!(x.to_index(dim, &conv_pos_of((vec![0, 0], vec![1, 0]))), 0usize.into());
        assert_eq!(x.to_index(dim, &conv_pos_of((vec![0, 0], vec![0, 1]))), 2usize.into());
        assert_eq!(x.to_index(dim, &conv_pos_of((vec![1, 0], vec![0, 1]))), 4usize.into());
        assert_eq!(x.to_index(dim, &conv_pos_of((vec![0, 0], vec![1, 1]))), 6usize.into());
        assert_eq!(x.to_index(dim, &conv_pos_of((vec![1, 0], vec![1, 1]))), 8usize.into());
        assert_eq!(x.to_index(dim, &conv_pos_of((vec![0, 1], vec![1, 1]))), 10usize.into());
        let mut pos = x.zero(dim);
        for i in 0usize..6 {
            x.to_pos(dim, i.into(), &mut pos);
            // println!("{} {}", &min[], &max[]);
            assert_eq!(x.to_index(dim, &pos), i.into());
        }
        x.to_pos(dim, 10usize.into(), &mut pos);
        assert_eq!(pos, conv_pos_of((vec![0, 1], vec![1, 1])));
    }
}
