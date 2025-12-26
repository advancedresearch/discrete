
use std::marker::PhantomData;
use std::ops::{
    AddAssign,
    MulAssign,
    Sub,
    Mul,
    SubAssign,
    DivAssign,
    Div,
};
use std::convert::TryInto;
use std::fmt::Debug;

use crate::{
    BigUint,
    Construct,
    Data,
    Of,
    space::Space,
};

/// Dimension is natural number, position is a list of numbers.
pub struct Permutation<T = Data>(PhantomData<T>);

impl<T> Construct for Permutation<T> {
    fn new() -> Self { Permutation(PhantomData) }
}

impl Space<usize> for Permutation<Data> {
    type Dim = usize;
    type Pos = Vec<usize>;
    fn count(&self, dim: &usize) -> usize {
        let mut res = 1;
        for x in 1..dim + 1 {
            res *= x;
        }
        res
    }
    fn zero(&self, dim: &usize) -> Vec<usize> {
        vec![0; *dim]
    }
    fn to_index(&self, dim: &usize, pos: &Vec<usize>) -> usize {
        let mut index = 0;
        let mut count = 1;
        for (i, &x) in pos.iter().enumerate().rev() {
            let lower = pos[..i].iter().filter(|&&y| y < x).count();
            index += count * (x - lower);
            count *= dim - i;
        }
        index
    }
    fn to_pos(&self, dim: &usize, mut index: usize, pos: &mut Vec<usize>) {
        unsafe { pos.set_len(0); }

        let mut count = 1;
        for (j, x) in (1..dim + 1).enumerate() {
            count *= x;
            pos.push(j);
        }

        for i in 0..*dim {
            let block = count / (dim - i);
            let ind = index / block;
            let item = pos.remove(ind);
            pos.push(item);
            count /= dim - i;
            index -= ind * block;
        }
    }
}

impl Space<BigUint> for Permutation<Data> {
    type Dim = BigUint;
    type Pos = Vec<BigUint>;
    fn count(&self, dim: &BigUint) -> BigUint {
        let _1: BigUint = 1usize.into();
        let mut res: BigUint = _1.clone();
        let mut x = _1.clone();
        loop {
            if &x > dim {break}
            res *= &x;
            x += &_1;
        }
        res
    }
    fn zero(&self, dim: &BigUint) -> Vec<BigUint> {
        let dim: usize = dim.try_into().unwrap();
        vec![0usize.into(); dim]
    }
    fn to_index(&self, dim: &Self::Dim, pos: &Self::Pos) -> BigUint {
        let mut index: BigUint = 0usize.into();
        let mut count: BigUint = 1usize.into();
        for (i, x) in pos.iter().enumerate().rev() {
            let lower = pos[..i].iter().filter(|&y| y < x).count();
            index += &count * (x - lower);
            count *= dim - i;
        }
        index
    }
    fn to_pos(&self, dim: &Self::Dim, mut index: BigUint, pos: &mut Self::Pos) {
        pos.clear();

        let mut count: BigUint = 1usize.into();
        let dim: usize = dim.try_into().unwrap();
        for (j, x) in (1usize..dim + 1).enumerate() {
            count *= x;
            pos.push(j.into());
        }

        let dim: usize = dim.try_into().unwrap();
        for i in 0..dim {
            let block = &count / (dim - i);
            let ind: BigUint = &index / &block;
            let item = pos.remove((&ind).try_into().unwrap());
            pos.push(item);
            count /= dim - i;
            index -= &ind * block;
        }
    }
}

impl<N, T> Space<N> for Permutation<Of<T>>
    where T: Space<N>,
          T::Pos: Clone,
          N: Clone +
             From<usize> +
             TryInto<usize> +
             for<'a> AddAssign<&'a N> +
             for<'a> MulAssign<&'a N> +
             Sub<usize, Output = N> +
             SubAssign +
             DivAssign<usize> +
             MulAssign<usize> +
             PartialOrd,
          <N as TryInto<usize>>::Error: Debug,
          for<'a> &'a N: Sub<usize, Output = N> +
                         Mul<&'a N, Output = N> +
                         Div<usize, Output = N> +
                         Div<&'a N, Output = N> +,
          <N as TryInto<usize>>::Error: Debug,
{
    type Dim = T::Dim;
    type Pos = Vec<T::Pos>;
    fn count(&self, dim: &Self::Dim) -> N {
        let of: T = Construct::new();
        let _1: N = 1usize.into();
        let mut x = _1.clone();
        let mut res = _1.clone();
        let of_count = of.count(dim);
        loop {
            if &x > &of_count {break}
            res *= &x;
            x += &_1;
        }
        res
    }
    fn zero(&self, dim: &Self::Dim) -> Self::Pos {
        let of: T = Construct::new();
        let count = match of.count(dim).try_into() {
            Ok(x) => x,
            Err(_) => panic!("Out of range"),
        };
        vec![of.zero(dim); count]
    }
    fn to_index(&self, dim: &Self::Dim, pos: &Self::Pos) -> N {
        let of: T = Construct::new();
        let mut index: N = 0usize.into();
        let dim_count = of.count(dim);
        let mut count: N = 1usize.into();
        for (i, x) in pos.iter()
            .map(|x| of.to_index(dim, x))
            .enumerate().rev() {
            let lower = pos[..i].iter()
                .map(|y| of.to_index(dim, y))
                .filter(|y| y < &x).count();
            index += &(&count * &(x - lower));
            count *= &(&dim_count - i);
        }
        index
    }
    fn to_pos(&self, dim: &Self::Dim, mut index: N, pos: &mut Self::Pos) {
        let of: T = Construct::new();
        let of_count: usize = of.count(dim).try_into().unwrap();
        pos.clear();

        let mut count: N = 1usize.into();
        for (j, x) in (1..of_count + 1).enumerate() {
            count *= x;
            let mut new_pos: T::Pos = of.zero(&dim);
            of.to_pos(dim, j.into(), &mut new_pos);
            pos.push(new_pos);
        }

        for i in 0..of_count {
            let diff = of_count - i;
            let block = &count / diff;
            let ind = &index / &block;
            index -= &ind * &block;
            let item = pos.remove(ind.try_into().unwrap());
            pos.push(item);
            count /= diff;
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::*;

    #[test]
    fn features() {
        is_complete::<usize, Permutation>();
        is_complete::<usize, Permutation<Of<Pair>>>();
    }

    #[test]
    fn data() {
        let permutation: Permutation = Construct::new();
        assert_eq!(permutation.count(&1), 1);
        assert_eq!(permutation.count(&2), 2);
        assert_eq!(permutation.count(&3), 6);
        assert_eq!(permutation.count(&4), 24);

        let mut pos = Vec::new();
        let ref dim = 4;
        let count = permutation.count(dim);
        for i in 0..count {
            permutation.to_pos(dim, i, &mut pos);
            let index = permutation.to_index(dim, &pos);
            assert_eq!(index, i);
        }
    }

    #[test]
    fn data_big() {
        use std::convert::TryInto;

        let permutation: Permutation = Construct::new();
        let ins: Vec<BigUint> = vec![
            1usize.into(),
            2usize.into(),
            3usize.into(),
            4usize.into(),
        ];
        let outs: Vec<BigUint> = vec![
            1usize.into(),
            2usize.into(),
            6usize.into(),
            24usize.into(),
        ];
        for i in 0..ins.len() {
            assert_eq!(permutation.count(&ins[i]), outs[i]);
        }

        let mut pos: Vec<BigUint> = Vec::new();
        let ref dim: BigUint = 4usize.into();
        let count: usize = permutation.count(dim).try_into().unwrap();
        for i in 0usize..count {
            permutation.to_pos(dim, i.into(), &mut pos);
            let index = permutation.to_index(dim, &pos);
            assert_eq!(index, i.into());
        }
    }

    #[test]
    fn of() {
        let space: Permutation<Of<Pair>> = Construct::new();
        let ref dim = 3;
        let count = space.count(dim);
        let mut pos = space.zero(dim);
        for i in 0..count {
            space.to_pos(dim, i, &mut pos);
            let index = space.to_index(dim, &pos);
            assert_eq!(index, i);
        }
    }

    #[test]
    fn of_big() {
        use std::convert::TryInto;

        let space: Permutation<Of<Pair>> = Construct::new();
        let ref dim: BigUint = 3usize.into();
        let count: usize = space.count(dim).try_into().unwrap();
        let mut pos = space.zero(dim);
        for i in 0..count {
            space.to_pos(dim, i.into(), &mut pos);
            let index = space.to_index(dim, &pos);
            assert_eq!(index, i.into());
        }
    }
}
