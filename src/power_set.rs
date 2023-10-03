use std::marker::PhantomData;
use std::convert::TryInto;
use std::fmt::Debug;

use std::ops::{BitOrAssign, Rem, Shr};

use num::BigUint;
use num::pow::Pow;

use Construct;
use Of;
use Data;
use space::Space;

/// Dimension is natural number, position is a list of numbers.
pub struct PowerSet<T = Data>(PhantomData<T>);

impl<T> Construct for PowerSet<T> {
    fn new() -> Self { PowerSet(PhantomData) }
}

impl Space<usize> for PowerSet<Data> {
    type Dim = usize;
    type Pos = Vec<usize>;
    fn count(&self, dim: &usize) -> usize {
        1 << *dim
    }
    fn zero(&self, _dim: &usize) -> Vec<usize> {
        vec![]
    }
    fn to_index(
        &self,
        _dim: &usize,
        pos: &Vec<usize>
    ) -> usize {
        let mut index = 0;
        for &i in pos.iter() {
            index |= 1 << i;
        }
        index
    }
    fn to_pos(
        &self,
        dim: &usize,
        index: usize,
        pos: &mut Vec<usize>
    ) {
        unsafe { pos.set_len(0); }
        for i in 0..*dim {
            if ((index >> i) & 1) == 1 {
                pos.push(i);
            }
        }
    }
}

impl Space<BigUint> for PowerSet<Data> {
    type Dim = BigUint;
    type Pos = Vec<BigUint>;
    fn count(&self, dim: &BigUint) -> BigUint {
        let _two: BigUint = 2usize.into();
        let dim: u32 = dim.try_into().unwrap();
        _two.pow(dim)
    }
    fn zero(&self, _dim: &BigUint) -> Vec<BigUint> {
        vec![]
    }
    fn to_index(
        &self,
        _dim: &Self::Dim,
        pos: &Self::Pos,
    ) -> BigUint {
        let mut index: BigUint = 0usize.into();
        let ref _2: BigUint = 2usize.into();
        for i in pos {
            index |= _2.pow(i.try_into().unwrap());
        }
        index
    }
    fn to_pos(
        &self,
        dim: &Self::Dim,
        index: BigUint,
        pos: &mut Self::Pos,
    ) {
        pos.clear();
        let dim: u32 = dim.try_into().unwrap();
        let ref _2: BigUint = 2u32.into();
        let ref _1: BigUint = 1u32.into();
        for i in 0u32..dim {
            if &((&index >> i) % _2) == _1 {
                pos.push(i.into());
            }
        }
    }
}

impl<N, T> Space<N> for PowerSet<Of<T>>
    where T: Space<N>,
          N: Clone +
             From<usize> +
             PartialEq +
             TryInto<u32> +
             BitOrAssign<N> +
             Rem<usize, Output = N> +
             Pow<u32, Output = N> +
             Shr<u32, Output = N>,
          <N as TryInto<u32>>::Error: Debug,
{
    type Dim = T::Dim;
    type Pos = Vec<T::Pos>;
    fn count(&self, dim: &Self::Dim) -> N {
        let _two: N = 2usize.into();
        let of: T = Construct::new();
        let count = of.count(dim);
        _two.pow(count.try_into().unwrap())
    }
    fn zero(&self, _dim: &Self::Dim) -> Self::Pos {
        vec![]
    }
    fn to_index(
        &self,
        dim: &Self::Dim,
        pos: &Self::Pos,
    ) -> N {
        let of: T = Construct::new();
        let mut index: N = 0usize.into();
        let _2: N = 2usize.into();
        for i in pos {
            let i: N = of.to_index(dim, i);
            index |= _2.clone().pow(i.try_into().unwrap());
        }
        index
    }
    fn to_pos(
        &self,
        dim: &Self::Dim,
        index: N,
        pos: &mut Self::Pos,
    ) {
        let of: T = Construct::new();
        let count = of.count(dim);
        pos.clear();
        let count: u32 = count.try_into().unwrap();
        pos.reserve_exact(count as usize);
        let ref _1: N = 1usize.into();
        for j in 0u32..count {
            if &((index.clone() >> j) % 2) == _1 {
                let mut p = of.zero(dim);
                of.to_pos(dim, (j as usize).into(), &mut p);
                pos.push(p);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn features() {
        is_complete::<usize, PowerSet>();
        is_complete::<usize, PowerSet<Of<Pair>>>();
    }

    #[test]
    fn data() {
        let x: PowerSet = Construct::new();
        let ref dim = 6;
        assert_eq!(x.count(dim), 64);
        assert_eq!(x.to_index(dim, &vec![]), 0);
        assert_eq!(x.to_index(dim, &vec![0]), 1);
        assert_eq!(x.to_index(dim, &vec![1]), 2);
        assert_eq!(x.to_index(dim, &vec![0, 1]), 3);
        let mut a = vec![];
        x.to_pos(dim, 9, &mut a);
        assert_eq!(&a, &[0, 3]);
    }

    fn conv(v: Vec<usize>) -> Vec<BigUint> {
        v.into_iter().map(|n| n.into()).collect()
    }

    #[test]
    fn data_big() {
        let x: PowerSet = Construct::new();
        let ref dim: BigUint = 6usize.into();
        assert_eq!(x.count(dim), 64usize.into());
        assert_eq!(x.to_index(dim, &vec![]), 0usize.into());
        assert_eq!(x.to_index(dim, &conv(vec![0])), 1usize.into());
        assert_eq!(x.to_index(dim, &conv(vec![1])), 2usize.into());
        assert_eq!(x.to_index(dim, &conv(vec![0, 1])), 3usize.into());
        let mut a = vec![];
        x.to_pos(dim, 9usize.into(), &mut a);
        assert_eq!(a, conv(vec![0, 3]));
    }

    #[test]
    fn of() {
        let x: PowerSet<Of<Pair>> = Construct::new();
        let ref dim = 4;
        assert_eq!(x.count(dim), 64);
        assert_eq!(x.to_index(dim, &vec![]), 0);
        assert_eq!(x.to_index(dim, &vec![(0, 1)]), 1);
        assert_eq!(x.to_index(dim, &vec![(0, 2)]), 2);
        assert_eq!(x.to_index(dim, &vec![(0, 1), (0, 2)]), 3);
        assert_eq!(x.to_index(dim, &vec![(1, 2)]), 4);
        assert_eq!(x.to_index(dim, &vec![(0, 1), (1, 2)]), 5);
        assert_eq!(x.to_index(dim, &vec![(0, 2), (1, 2)]), 6);
        assert_eq!(x.to_index(dim, &vec![(0, 1), (0, 2), (1, 2)]), 7);
        let mut a = vec![(0, 0); 64];
        x.to_pos(dim, 7, &mut a);
        assert_eq!(a[0], (0, 1));
    }

    fn conv_pos_of(v: Vec<(usize, usize)>) -> Vec<(BigUint, BigUint)> {
        v.into_iter().map(|(a, b)| (a.into(), b.into())).collect()
    }

    #[test]
    fn of_big() {
        let x: PowerSet<Of<Pair>> = Construct::new();
        let ref dim: BigUint = 4usize.into();
        assert_eq!(x.count(dim), 64usize.into());
        assert_eq!(x.to_index(dim, &vec![]), 0usize.into());
        assert_eq!(x.to_index(dim, &conv_pos_of(vec![(0, 1)])), 1usize.into());
        assert_eq!(x.to_index(dim, &conv_pos_of(vec![(0, 2)])), 2usize.into());
        assert_eq!(x.to_index(dim, &conv_pos_of(vec![(0, 1), (0, 2)])), 3usize.into());
        assert_eq!(x.to_index(dim, &conv_pos_of(vec![(1, 2)])), 4usize.into());
        assert_eq!(x.to_index(dim, &conv_pos_of(vec![(0, 1), (1, 2)])), 5usize.into());
        assert_eq!(x.to_index(dim, &conv_pos_of(vec![(0, 2), (1, 2)])), 6usize.into());
        assert_eq!(x.to_index(dim, &conv_pos_of(vec![(0, 1), (0, 2), (1, 2)])), 7usize.into());
        let mut a = conv_pos_of(vec![(0, 0); 64]);
        x.to_pos(dim, 7usize.into(), &mut a);
        assert_eq!(a[0], (0usize.into(), 1usize.into()));
    }
}
