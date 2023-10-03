
use std::marker::PhantomData;
use std::ops::{
    Add,
    Mul,
    MulAssign,
    Sub,
    Div,
    Rem,
    AddAssign,
    DivAssign,
    SubAssign,
};

use num::BigUint;

use Construct;
use Data;
use Of;
use NeqPair;
use Pair;
use space::Space;

/// Same as `Context`, but for directed edges.
pub struct DirectedContext<T = Data>(PhantomData<T>);

impl<T> Construct for DirectedContext<T> {
    fn new() -> Self { DirectedContext(PhantomData) }
}

impl Space<usize> for DirectedContext<Data> {
    type Dim = Vec<usize>;
    type Pos = (Vec<usize>, usize, usize);
    fn count(&self, dim: &Vec<usize>) -> usize {
        let pair: NeqPair<Data> = Construct::new();
        let mut sum: usize = pair.count(&dim[0]);
        let mut prod = dim[0];
        for d in &dim[1..] {
            let count: usize = pair.count(d);
            sum = d * sum + count * prod;
            prod *= *d;
        }
        sum
    }
    fn zero(&self, dim: &Vec<usize>) -> (Vec<usize>, usize, usize) {
        (vec![0; dim.len()], 0, 0)
    }
    fn to_index(
        &self, dim: &Vec<usize>,
        &(ref p, ind, b): &(Vec<usize>, usize, usize)
    ) -> usize {
        use Context;

        let context: Context<Data> = Construct::new();
        let index: usize = context.to_index(dim, &(p.clone(), ind, b));
        if p[ind] > b {
            2 * index + 1
        } else {
            2 * index
        }
    }
    fn to_pos(
        &self,
        dim: &Vec<usize>,
        index: usize,
        pos: &mut (Vec<usize>, usize, usize)
    ) {
        use Context;

        let context: Context<Data> = Construct::new();
        if index % 2 == 0 {
            context.to_pos(dim, index / 2, pos);
        } else {
            context.to_pos(dim, (index - 1) / 2, pos);
            let tmp = pos.0[pos.1];
            pos.0[pos.1] = pos.2;
            pos.2 = tmp;
        }
    }
}

impl Space<BigUint> for DirectedContext<Data> {
    type Dim = Vec<BigUint>;
    type Pos = (Vec<BigUint>, usize, BigUint);
    fn count(&self, dim: &Vec<BigUint>) -> BigUint {
        let pair: NeqPair<Data> = Construct::new();
        let mut sum: BigUint = pair.count(&dim[0]);
        let mut prod = dim[0].clone();
        for d in &dim[1..] {
            let count: BigUint = pair.count(d);
            sum = d * sum + count * &prod;
            prod *= d;
        }
        sum
    }
    fn zero(&self, dim: &Vec<BigUint>) -> (Vec<BigUint>, usize, BigUint) {
        (vec![0usize.into(); dim.len()], 0, 0usize.into())
    }
    fn to_index(
        &self, dim: &Self::Dim,
        (p, ind, b): &Self::Pos,
    ) -> BigUint {
        use Context;

        let context: Context<Data> = Construct::new();
        let index: BigUint = context.to_index(dim, &(p.clone(), *ind, b.clone()));
        if &p[*ind] > b {
            2usize * index + 1usize
        } else {
            2usize * index
        }
    }
    fn to_pos(
        &self,
        dim: &Self::Dim,
        index: BigUint,
        pos: &mut Self::Pos,
    ) {
        use Context;

        let context: Context<Data> = Construct::new();
        if &index % 2usize == 0usize.into() {
            context.to_pos(dim, index / 2usize, pos);
        } else {
            context.to_pos(dim, (index - 1usize) / 2usize, pos);
            std::mem::swap(&mut pos.0[pos.1], &mut pos.2);
        }
    }
}

impl<N, T> Space<N> for DirectedContext<Of<T>>
    where T: Space<N>,
          T::Pos: Clone,
          NeqPair<Data>: Space<N, Dim = N>,
          Pair<Data>: Space<N, Dim = N, Pos = (N, N)>,
          for<'a> N: Clone +
                     From<usize> +
                     Ord +
                     MulAssign<&'a N> +
                     Sub<usize, Output = N> +
                     Div<usize, Output = N> +
                     AddAssign<&'a N> +
                     DivAssign<&'a N> +
                     SubAssign<&'a N> +
                     Mul<usize, Output = N> +
                     Add<usize, Output = N>,
          for<'a> &'a N: Mul<&'a N, Output = N> +
                         Add<&'a N, Output = N> +
                         Rem<usize, Output = N> +
                         Sub<&'a N, Output = N> +
                         Div<&'a N, Output = N>,
{
    type Dim = Vec<T::Dim>;
    type Pos = (Vec<T::Pos>, usize, T::Pos);
    fn count(&self, dim: &Self::Dim) -> N {
        let of: T = Construct::new();
        let pair: NeqPair<Data> = Construct::new();
        let mut sum: N = pair.count(&of.count(&dim[0]));
        let mut prod: N = of.count(&dim[0]);
        for d in &dim[1..] {
            let d = of.count(d);
            let count: N = pair.count(&d);
            sum = &(&d * &sum) + &(&count * &prod);
            prod *= &d;
        }
        sum
    }
    fn zero(&self, dim: &Self::Dim) -> Self::Pos {
        let of: T = Construct::new();
        let mut v = Vec::with_capacity(dim.len());
        for i in 0..dim.len() {
            v.push(of.zero(&dim[i]));
        }
        (v, 0, of.zero(&dim[0]))
    }
    fn to_index(
        &self,
        dim: &Self::Dim,
        &(ref p, ind, ref b): &Self::Pos,
    ) -> N {
        use Context;

        let of: T = Construct::new();
        let context: Context<Of<T>> = Construct::new();
        let index: N = Space::to_index(&context, dim, &(p.clone(), ind, b.clone()));
        if of.to_index(&dim[ind], &p[ind]) > of.to_index(&dim[ind], b) {
            let x = index * 2usize;
            x + 1usize
        } else {
            index * 2usize
        }
    }
    fn to_pos(
        &self,
        dim: &Self::Dim,
        index: N,
        pos: &mut Self::Pos,
    ) {
        use Context;

        let context: Context<Of<T>> = Construct::new();
        if &index % 2usize == 0usize.into() {
            context.to_pos(dim, index / 2usize, pos);
        } else {
            context.to_pos(dim, (index - 1usize) / 2usize, pos);
            std::mem::swap(&mut pos.0[pos.1], &mut pos.2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn features() {
        is_complete::<usize, DirectedContext>();
        is_complete::<usize, DirectedContext<Of<Pair>>>();
    }

    #[test]
    fn data() {
        let x: DirectedContext = Construct::new();
        let ref dim = vec![2, 2, 2];
        // 12 edges on a cube * 2 = 24 directed edges
        assert_eq!(x.count(dim), 24);
        assert_eq!(x.to_index(dim, &(vec![0, 0, 0], 0, 1)), 0);
        assert_eq!(x.to_index(dim, &(vec![1, 0, 0], 0, 0)), 1);
        for i in 0..x.count(dim) {
            let mut pos = (vec![], 0, 0);
            x.to_pos(dim, i, &mut pos);
            // println!("{:?}", pos);
            assert_eq!(x.to_index(dim, &pos), i);
        }
        // assert!(false);
    }

    fn conv(v: Vec<usize>) -> Vec<BigUint> {
        v.into_iter().map(|n| n.into()).collect()
    }

    fn conv_pos((v, a, b): (Vec<usize>, usize, usize)) -> (Vec<BigUint>, usize, BigUint) {
        (conv(v), a, b.into())
    }

    #[test]
    fn data_big() {
        use std::convert::TryInto;

        let x: DirectedContext = Construct::new();
        let ref dim = conv(vec![2, 2, 2]);
        // 12 edges on a cube * 2 = 24 directed edges
        assert_eq!(x.count(dim), 24usize.into());
        assert_eq!(x.to_index(dim, &conv_pos((vec![0, 0, 0], 0, 1))), 0usize.into());
        assert_eq!(x.to_index(dim, &conv_pos((vec![1, 0, 0], 0, 0))), 1usize.into());
        let count: usize = x.count(dim).try_into().unwrap();
        for i in 0usize..count {
            let mut pos = x.zero(dim);
            x.to_pos(dim, i.into(), &mut pos);
            // println!("{:?}", pos);
            assert_eq!(x.to_index(dim, &pos), i.into());
        }
        // assert!(false);
    }
}
