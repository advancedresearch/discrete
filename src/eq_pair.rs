use std::marker::PhantomData;

use crate::{
    BigUint,
    Construct,
    Data,
    Of,
    space::Space,
};

/// Dimension is natural number, position is (min, max).
pub struct EqPair<T = Data>(PhantomData<T>);

impl<T> Construct for EqPair<T> {
    fn new() -> Self { EqPair(PhantomData) }
}

impl Space<usize> for EqPair<Data> {
    type Dim = usize;
    type Pos = (usize, usize);
    fn count(&self, dim: &usize) -> usize { dim * (dim + 1) / 2 }
    fn zero(&self, _dim: &usize) -> (usize, usize) { (0, 0) }
    fn to_index(&self, _dim: &usize, &(min, max): &(usize, usize)) -> usize {
        min + max * (max + 1) / 2
    }
    fn to_pos(&self, _dim: &usize, index: usize, pos: &mut (usize, usize)) {
        let max = ((-1f64 + (8f64 * index as f64 + 1f64).sqrt()) / 2f64) as usize;
        let min = index - max * (max + 1) / 2;
        *pos = (min, max)
    }
}

impl Space<BigUint> for EqPair<Data> {
    type Dim = BigUint;
    type Pos = (BigUint, BigUint);
    fn count(&self, dim: &Self::Dim) -> BigUint { dim * (dim + 1usize) / 2usize }
    fn zero(&self, _dim: &Self::Dim) -> Self::Pos { (0usize.into(), 0usize.into()) }
    fn to_index(&self, _dim: &Self::Dim, (min, max): &Self::Pos) -> BigUint {
        min + max * (max + 1usize) / 2usize
    }
    fn to_pos(&self, _dim: &Self::Dim, index: BigUint, pos: &mut Self::Pos) {
        let max: BigUint = ((8usize * &index + 1usize).sqrt() - 1usize) / 2usize;
        let min: BigUint = index - &max * (&max + 1usize) / 2usize;
        *pos = (min, max)
    }
}

impl<N, T> Space<N> for EqPair<Of<T>>
    where T: Space<N>,
          EqPair<Data>: Space<N, Dim = N, Pos = (N, N)>,
{
    type Dim = T::Dim;
    type Pos = (T::Pos, T::Pos);
    fn count(&self, dim: &Self::Dim) -> N {
        let of: T = Construct::new();
        let data: EqPair<Data> = Construct::new();
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
        let data: EqPair<Data> = Construct::new();
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
        let data: EqPair<Data> = Construct::new();
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
        is_complete::<usize, EqPair>();
        is_complete::<usize, EqPair<Of<EqPair>>>();
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

    #[test]
    fn test_eq_pair_big() {
        use std::convert::TryInto;

        // 1 0 0 0
        // 2 3 0 0
        // 4 5 6 0
        // 7 8 9 10

        let eq_pair: EqPair = Construct::new();
        let ref n: BigUint = 4usize.into();
        let count = eq_pair.count(n);
        assert_eq!(count, 10usize.into());

        let mut pos = eq_pair.zero(n);
        let count: usize = count.try_into().unwrap();
        for x in 0..count {
            eq_pair.to_pos(n, x.into(), &mut pos);
            println!("{:?}", pos);
            assert_eq!(eq_pair.to_index(n, &pos), x.into());
        }
    }
}
