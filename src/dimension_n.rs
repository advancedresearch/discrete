use std::marker::PhantomData;
use std::ops::{
    AddAssign,
    Div,
    Mul,
    MulAssign,
    SubAssign,
    DivAssign,
};

use num::BigUint;

use Construct;
use Data;
use Of;
use space::Space;

/// Dimension is a list of numbers, position is a list of numbers.
pub struct DimensionN<T = Data>(PhantomData<T>);

impl<T> Construct for DimensionN<T> {
    fn new() -> Self { DimensionN(PhantomData) }
}

impl Space<usize> for DimensionN<Data> {
    type Dim = Vec<usize>;
    type Pos = Vec<usize>;
    fn count(&self, dim: &Vec<usize>) -> usize {
        let mut prod = 1;
        for i in 0..dim.len() {
            prod *= dim[i];
        }
        prod
    }
    fn zero(&self, dim: &Vec<usize>) -> Vec<usize> {
        vec![0; dim.len()]
    }
    fn to_index(&self, dim: &Vec<usize>, pos: &Vec<usize>) -> usize {
        let mut dim_index = 0;
        for i in (0..dim.len()).rev() {
            dim_index = dim_index * dim[i] + pos[i];
        }
        dim_index
    }
    fn to_pos(&self, dim: &Vec<usize>, index: usize, pos: &mut Vec<usize>) {
        unsafe { pos.set_len(0); }
        let mut prod: usize = self.count(dim);
        for _ in 0..dim.len() {
            pos.push(0);
        }
        let mut dim_index = index;
        for i in (0..dim.len()).rev() {
            prod /= dim[i];
            let p_i = dim_index / prod;
            *pos.get_mut(i).unwrap() = p_i;
            dim_index -= p_i * prod;
        }
    }
}

impl Space<BigUint> for DimensionN<Data> {
    type Dim = Vec<BigUint>;
    type Pos = Vec<BigUint>;
    fn count(&self, dim: &Self::Dim) -> BigUint {
        let mut prod: BigUint = 1usize.into();
        for i in 0..dim.len() {
            prod *= &dim[i];
        }
        prod
    }
    fn zero(&self, dim: &Self::Dim) -> Self::Pos {
        vec![0usize.into(); dim.len()]
    }
    fn to_index(&self, dim: &Self::Dim, pos: &Self::Pos) -> BigUint {
        let mut dim_index: BigUint = 0usize.into();
        for i in (0..dim.len()).rev() {
            dim_index = dim_index * &dim[i] + &pos[i];
        }
        dim_index
    }
    fn to_pos(&self, dim: &Self::Dim, index: BigUint, pos: &mut Self::Pos) {
        pos.clear();
        let mut prod: BigUint = self.count(dim);
        for _ in 0..dim.len() {
            pos.push(0usize.into());
        }
        let mut dim_index = index;
        for i in (0..dim.len()).rev() {
            prod /= &dim[i];
            let p_i = &dim_index / &prod;
            dim_index -= &p_i * &prod;
            *pos.get_mut(i).unwrap() = p_i;
        }
    }
}

impl<N, T> Space<N> for DimensionN<Of<T>>
    where N: Clone +
             From<usize> +
             AddAssign<N> +
             SubAssign<N> +
             MulAssign<N> +
             Div<Output = N> +
             DivAssign<N>,
          for<'a> &'a N: Div<&'a N, Output = N> + Mul<&'a N, Output = N>,
          T: Space<N>,
{
    type Dim = Vec<T::Dim>;
    type Pos = Vec<T::Pos>;
    fn count(&self, dim: &Self::Dim) -> N {
        let of: T = Construct::new();
        let mut prod: N = 1usize.into();
        for i in 0..dim.len() {
            prod *= of.count(&dim[i]);
        }
        prod
    }
    fn zero(&self, dim: &Self::Dim) -> Self::Pos {
        let of: T = Construct::new();
        let mut v = Vec::with_capacity(dim.len());
        for i in 0..dim.len() {
            v.push(of.zero(&dim[i]));
        }
        v
    }
    fn to_index(
        &self,
        dim: &Self::Dim,
        pos: &Self::Pos,
    ) -> N {
        let of: T = Construct::new();
        let mut dim_index: N = 0usize.into();
        for i in (0..dim.len()).rev() {
            dim_index *= of.count(&dim[i]);
            dim_index += of.to_index(&dim[i], &pos[i]);
        }
        dim_index
    }
    fn to_pos(
        &self,
        dim: &Self::Dim,
        index: N,
        pos: &mut Self::Pos,
    ) {
        let of: T = Construct::new();
        let mut prod = self.count(dim);
        let mut dim_index = index.clone();
        for (i, p) in pos.iter_mut().enumerate().rev() {
            prod /= of.count(&dim[i]);
            let p_i = &dim_index / &prod;
            dim_index -= &p_i * &prod;
            of.to_pos(&dim[i], p_i, p);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn features() {
        is_complete::<usize, DimensionN>();
        is_complete::<usize, DimensionN<Of<Pair>>>();
    }

    #[test]
    fn data() {
        let x: DimensionN = Construct::new();
        let ref dim = vec![3, 3];
        assert_eq!(x.count(dim), 9);
        assert_eq!(x.to_index(dim, &vec![0, 0]), 0);
        assert_eq!(x.to_index(dim, &vec![1, 0]), 1);
        assert_eq!(x.to_index(dim, &vec![0, 1]), 3);
        let mut new_pos = vec![0, 0];
        x.to_pos(dim, 3, &mut new_pos);
        assert_eq!(&new_pos, &[0, 1]);
    }

    fn conv(v: Vec<usize>) -> Vec<BigUint> {
        v.into_iter().map(|n| n.into()).collect()
    }

    #[test]
    fn data_big() {
        let x: DimensionN = Construct::new();
        let ref dim: Vec<BigUint> = conv(vec![3, 3]);
        assert_eq!(x.count(dim), 9usize.into());
        assert_eq!(x.to_index(dim, &conv(vec![0, 0])), 0usize.into());
        assert_eq!(x.to_index(dim, &conv(vec![1, 0])), 1usize.into());
        assert_eq!(x.to_index(dim, &conv(vec![0, 1])), 3usize.into());
        let mut new_pos = x.zero(dim);
        x.to_pos(dim, 3usize.into(), &mut new_pos);
        assert_eq!(new_pos, conv(vec![0, 1]));
    }

    #[test]
    fn of() {
        let x: DimensionN<Of<Pair>> = Construct::new();
        let ref dim = vec![3, 4];
        assert_eq!(x.count(dim), 18);
        assert_eq!(x.to_index(dim, &vec![(0, 1), (0, 1)]), 0);
        assert_eq!(x.to_index(dim, &vec![(0, 2), (0, 1)]), 1);
        assert_eq!(x.to_index(dim, &vec![(1, 2), (0, 1)]), 2);
        assert_eq!(x.to_index(dim, &vec![(0, 1), (0, 2)]), 3);
        let mut pos = vec![(0, 0), (0, 0)];
        x.to_pos(dim, 3, &mut pos);
        assert_eq!(pos[0], (0, 1));
        assert_eq!(pos[1], (0, 2));
    }

    fn conv_pair(v: Vec<(usize, usize)>) -> Vec<(BigUint, BigUint)> {
        v.into_iter().map(|(a, b)| (a.into(), b.into())).collect()
    }

    #[test]
    fn of_big() {
        let x: DimensionN<Of<Pair>> = Construct::new();
        let ref dim = conv(vec![3, 4]);
        assert_eq!(x.count(dim), 18usize.into());
        assert_eq!(x.to_index(dim, &conv_pair(vec![(0, 1), (0, 1)])), 0usize.into());
        assert_eq!(x.to_index(dim, &conv_pair(vec![(0, 2), (0, 1)])), 1usize.into());
        assert_eq!(x.to_index(dim, &conv_pair(vec![(1, 2), (0, 1)])), 2usize.into());
        assert_eq!(x.to_index(dim, &conv_pair(vec![(0, 1), (0, 2)])), 3usize.into());
        let mut pos = x.zero(dim);
        x.to_pos(dim, 3usize.into(), &mut pos);
        assert_eq!(pos, conv_pair(vec![(0, 1), (0, 2)]));
    }

    #[test]
    fn zero() {
        let x: DimensionN = Construct::new();
        let ref dim = vec![2; 3];
        assert_eq!(x.zero(dim), vec![0; 3]);
    }
}
