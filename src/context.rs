
use std::marker::PhantomData;
use std::ops::{
    Add,
    Div,
    Mul,
    MulAssign,
    SubAssign,
    DivAssign,
    Sub,
    AddAssign,
};

use crate::BigUint;

use Construct;
use Data;
use Of;
use Pair;
use space::Space;

/// A discrete space that can model spatial operations over arbitrary states,
/// therefore useful for context analysis.
///
/// It can be constructed by taking a N-dimensional space,
/// for each dimension create a pair subspace and sum over the dimensions.
/// It can also be thought of as the edges in an undirected graph,
/// where each node is described by a N-dimensional coordinate,
/// and all nodes are connected which differ by one axis.
///
/// Dimensions of size 2 gives the edges on a hypercube of the number of dimensions.
/// For example, `[2, 2, 2]` gives edges on a cube in 3 dimensions.
///
/// The position is a tuple `(Vec<usize>, usize, usize)`,
/// where the first component describes the node coordinates,
/// the second component describes the index of the coordinate that changes,
/// and the third component describes the new value.
pub struct Context<T = Data>(PhantomData<T>);

/// Computes subspace offset from which index that changes.
/// The space is divided into N subspaces,
/// because only one axis can change at a time.
///
/// ```ignore
/// [(a, x), b, c]
/// [a, (b, x), c]
/// [a, b, (c, x)]
/// ```
fn subspace_offset(v: &[usize], ind: usize) -> usize {
    let pair: Pair<Data> = Construct::new();
    let mut sum = 0;
    for i in 0..ind {
        let mut prod = 1;
        for j in 0..v.len() {
            if i == j { continue; }
            prod *= v[j];
        }
        sum += <Pair as Space<usize>>::count(&pair, &v[i]) * prod;
    }
    sum
}

/// Computes subspace offset from which index that changes.
/// The space is divided into N subspaces,
/// because only one axis can change at a time.
///
/// ```ignore
/// [(a, x), b, c]
/// [a, (b, x), c]
/// [a, b, (c, x)]
/// ```
fn biguint_subspace_offset(v: &[BigUint], ind: usize) -> BigUint {
    let pair: Pair<Data> = Construct::new();
    let mut sum: BigUint = 0usize.into();
    for i in 0..ind {
        let mut prod: BigUint = 1usize.into();
        for j in 0..v.len() {
            if i == j { continue; }
            prod *= &v[j];
        }
        sum += <Pair as Space<BigUint>>::count(&pair, &v[i]) * prod;
    }
    sum
}

/// Computes the index of the axis that changes from index position.
/// This works because the layout are separated by which
/// axis that changes, and the subspace offset can be computed.
/// Returns `(ind, offset)`
fn ind_from_index(v: &[usize], index: usize) -> (usize, usize) {
    let pair: Pair<Data> = Construct::new();
    let mut sum = 0;
    for i in 0..v.len() {
        let mut prod = 1;
        for j in 0..v.len() {
            if i == j { continue; }
            prod *= v[j];
        }
        let add = <Pair as Space<usize>>::count(&pair, &v[i]) * prod;
        if sum + add > index { return (i, sum); }
        sum += add;
    }
    (v.len(), sum)
}

/// Computes the index of the axis that changes from index position.
/// This works because the layout are separated by which
/// axis that changes, and the subspace offset can be computed.
/// Returns `(ind, offset)`
fn biguint_ind_from_index(v: &[BigUint], index: &BigUint) -> (usize, BigUint) {
    let pair: Pair<Data> = Construct::new();
    let mut sum: BigUint = 0usize.into();
    for i in 0..v.len() {
        let mut prod: BigUint = 1usize.into();
        for j in 0..v.len() {
            if i == j { continue; }
            prod *= &v[j];
        }
        let add = <Pair as Space<BigUint>>::count(&pair, &v[i]) * prod;
        if &(&sum + &add) > index { return (i, sum); }
        sum += add;
    }
    (v.len(), sum)
}


impl<T> Construct for Context<T> {
    fn new() -> Context<T> { Context(PhantomData) }
}

impl Space<usize> for Context<Data> {
    type Dim = Vec<usize>;
    type Pos = (Vec<usize>, usize, usize);
    fn count(&self, dim: &Vec<usize>) -> usize {
        let pair: Pair<Data> = Construct::new();
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
        &self,
        dim: &Vec<usize>,
        &(ref p, ind, b): &(Vec<usize>, usize, usize)
    ) -> usize {
        use std::cmp::{ min, max };

        let offset = subspace_offset(dim, ind);
        let pair: Pair<Data> = Construct::new();
        let mut prod = 1;
        for j in 0..dim.len() {
            if ind == j { continue; }
            prod *= dim[j];
        }
        // Pair doesn't care about dimension.
        let single: usize = pair.to_index(&0, &(min(p[ind], b), max(p[ind], b)));
        let pos_offset: usize = single * prod;
        let mut dim_index = 0;
        for i in (0..p.len()).rev() {
            if ind == i { continue; }
            dim_index = dim_index * dim[i] + p[i];
        }
        offset + pos_offset + dim_index
    }
    fn to_pos(
        &self,
        dim: &Vec<usize>,
        index: usize,
        &mut (ref mut p, ref mut ind, ref mut b): &mut (Vec<usize>, usize, usize)
    ) {
        p.clear();
        let pair_space: Pair<Data> = Construct::new();
        let (ind_val, offset) = ind_from_index(dim, index);
        // Get rid of offset.
        // The rest equals: single * prod + dim_index
        let index = index - offset;
        let mut prod = 1;
        for j in 0..dim.len() {
            p.push(0); // zero position
            if ind_val == j { continue; }
            prod *= dim[j];
        }
        let single = index / prod;

        let mut pair = (0, 0);
        // Pair doesn't care about dimension.
        pair_space.to_pos(&0, single, &mut pair);
        let (min, max) = pair;

        // Resolve other dimension components.
        let mut dim_index = index - single * prod;
        for i in (0..p.len()).rev() {
            if ind_val == i { continue; }
            prod /= dim[i];
            let p_i = dim_index / prod;
            p[i] = p_i;
            dim_index -= p_i * prod;
        }
        p[ind_val] = min;
        *b = max;
        *ind = ind_val;
    }
}

impl Space<BigUint> for Context<Data> {
    type Dim = Vec<BigUint>;
    type Pos = (Vec<BigUint>, usize, BigUint);
    fn count(&self, dim: &Vec<BigUint>) -> BigUint {
        let pair: Pair<Data> = Construct::new();
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
        &self,
        dim: &Self::Dim,
        (p, ind, b): &Self::Pos,
    ) -> BigUint {
        use std::cmp::{ min, max };

        let ind = *ind;
        let offset = biguint_subspace_offset(dim, ind);
        let pair: Pair<Data> = Construct::new();
        let mut prod: BigUint = 1usize.into();
        for j in 0..dim.len() {
            if ind == j { continue; }
            prod *= &dim[j];
        }
        // Pair doesn't care about dimension.
        let single: BigUint = pair.to_index(&0usize.into(), &(min(p[ind].clone(), b.clone()), max(p[ind].clone(), b.clone())));
        let pos_offset: BigUint = single * prod;
        let mut dim_index: BigUint = 0usize.into();
        for i in (0..p.len()).rev() {
            if ind == i { continue; }
            dim_index = dim_index * &dim[i] + &p[i];
        }
        offset + pos_offset + dim_index
    }
    fn to_pos(
        &self,
        dim: &Self::Dim,
        index: BigUint,
        &mut (ref mut p, ref mut ind, ref mut b): &mut (Vec<BigUint>, usize, BigUint)
    ) {
        p.clear();
        let pair_space: Pair<Data> = Construct::new();
        let (ind_val, offset) = biguint_ind_from_index(dim, &index);
        // Get rid of offset.
        // The rest equals: single * prod + dim_index
        let index = index - &offset;
        let mut prod: BigUint = 1usize.into();
        for j in 0..dim.len() {
            p.push(0usize.into()); // zero position
            if ind_val == j { continue; }
            prod *= &dim[j];
        }
        let single = &index / &prod;

        let mut pair: (BigUint, BigUint) = (0usize.into(), 0usize.into());
        // Pair doesn't care about dimension.
        let z: BigUint = 0usize.into();
        pair_space.to_pos(&z, single.clone(), &mut pair);
        let (min, max) = pair;

        // Resolve other dimension components.
        let mut dim_index = index - &single * &prod;
        for i in (0..p.len()).rev() {
            if ind_val == i { continue; }
            prod /= &dim[i];
            let p_i = &dim_index / &prod;
            dim_index -= &p_i * &prod;
            p[i] = p_i;
        }
        p[ind_val] = min;
        *b = max;
        *ind = ind_val;
    }
}

impl<N, T> Space<N> for Context<Of<T>>
    where T: Space<N>,
          Pair<Data>: Space<N, Dim = N, Pos = (N, N)>,
          for<'a> N: Clone +
                     From<usize> +
                     Ord +
                     MulAssign<&'a N> +
                     SubAssign<&'a N> +
                     DivAssign<&'a N> +
                     AddAssign<&'a N>,
          for<'a> &'a N: Mul<&'a N, Output = N> +
                         Div<&'a N, Output = N> +
                         Add<&'a N, Output = N> +
                         Sub<&'a N, Output = N>,
{
    type Dim = Vec<T::Dim>;
    type Pos = (Vec<T::Pos>, usize, T::Pos);
    fn count(&self, dim: &Self::Dim) -> N {
        let of: T = Construct::new();
        let pair: Pair<Data> = Construct::new();
        let mut sum: N = pair.count(&of.count(&dim[0]));
        let mut prod = of.count(&dim[0]);
        for d in &dim[1..] {
            let d: N = of.count(d);
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
        &(ref p, ind, ref b): &Self::Pos
    ) -> N {
        fn subspace_offset<N, T>(v: &Vec<T::Dim>, ind: usize) -> N
            where T: Space<N>,
                  Pair<Data>: Space<N, Dim = N>,
                  for<'a> N: From<usize> +
                             AddAssign<&'a N> +
                             MulAssign<&'a N>,
                  for<'a> &'a N: Mul<&'a N, Output = N>,
        {
            let of: T = Construct::new();
            let pair: Pair<Data> = Construct::new();
            let mut sum: N = 0usize.into();
            for i in 0..ind {
                let mut prod: N = 1usize.into();
                for j in 0..v.len() {
                    if i == j { continue; }
                    prod *= &of.count(&v[j]);
                }
                let c: N = pair.count(&of.count(&v[i]));
                sum += &(&c * &prod);
            }
            sum
        }

        use std::cmp::{ min, max };

        let of: T = Construct::new();
        let offset = subspace_offset::<N, T>(dim, ind);
        let pair: Pair<Data> = Construct::new();
        let mut prod: N = 1usize.into();
        for j in 0..dim.len() {
            if ind == j { continue; }
            prod *= &of.count(&dim[j]);
        }
        // Pair doesn't care about dimension.
        let single = pair.to_index(&0usize.into(),
            &(min(of.to_index(&dim[ind], &p[ind]), of.to_index(&dim[ind], b)),
             max(of.to_index(&dim[ind], &p[ind]), of.to_index(&dim[ind], b))));
        let pos_offset = &(&single * &prod);
        let mut dim_index: N = 0usize.into();
        for i in (0..p.len()).rev() {
            if ind == i { continue; }
            dim_index = &(&dim_index * &of.count(&dim[i])) + &of.to_index(&dim[i], &p[i]);
        }
        &(&offset + &pos_offset) + &dim_index
    }
    fn to_pos(
        &self,
        dim: &Self::Dim,
        index: N,
        &mut (ref mut p, ref mut ind, ref mut b): &mut Self::Pos
    ) {
        fn ind_from_index<N, T>(v: &Vec<T::Dim>, index: &N) -> (usize, N)
            where T: Space<N>,
                  Pair: Space<N, Dim = N>,
                  for<'a> N: From<usize> +
                             PartialOrd +
                             AddAssign<&'a N> +
                             MulAssign<&'a N>,
                  for<'a> &'a N: Add<&'a N, Output = N> +
                                 Mul<&'a N, Output = N>,
        {
            let of: T = Construct::new();
            let pair: Pair<Data> = Construct::new();
            let mut sum: N = 0usize.into();
            for i in 0..v.len() {
                let mut prod: N = 1usize.into();
                for j in 0..v.len() {
                    if i == j { continue; }
                    prod *= &of.count(&v[j]);
                }
                let c: N = pair.count(&of.count(&v[i]));
                let add: N = &c * &prod;
                if &(&sum + &add) > index { return (i, sum); }
                sum += &add;
            }
            (v.len(), sum)
        }

        let of: T = Construct::new();
        p.clear();
        let pair_space: Pair<Data> = Construct::new();
        let (ind_val, offset) = ind_from_index::<N, T>(dim, &index);
        // Get rid of offset.
        // The rest equals: single * prod + dim_index
        let index: N = &index - &offset;
        let mut prod: N = 1usize.into();
        for j in 0..dim.len() {
            p.push(of.zero(&dim[j])); // zero position
            if ind_val == j { continue; }
            prod *= &of.count(&dim[j]);
        }
        let single: N = &index / &prod;

        let mut pair = (0usize.into(), 0usize.into());
        // Pair doesn't care about dimension.
        pair_space.to_pos(&0usize.into(), single.clone(), &mut pair);
        let (min, max) = pair;

        // Resolve other dimension components.
        let mut dim_index: N = &index - &(&single * &prod);
        for i in (0..p.len()).rev() {
            if ind_val == i { continue; }
            prod /= &of.count(&dim[i]);
            let p_i = &dim_index / &prod;
            dim_index -= &(&p_i * &prod);
            of.to_pos(&dim[i], p_i, &mut p[i]);
        }
        of.to_pos(&dim[ind_val], min, &mut p[ind_val]);
        of.to_pos(&dim[ind_val], max, b);
        *ind = ind_val;
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn features() {
        is_complete::<usize, Context>();
        is_complete::<usize, Context<Of<Pair>>>();
    }

    #[test]
    fn data() {
        let x: Context = Construct::new();
        let ref dim = vec![2usize, 2, 2];
        // 12 edges on a cube
        assert_eq!(x.count(dim), 12);
        assert_eq!(x.to_index(dim, &(vec![0, 0, 0], 0, 1)), 0);
        for i in 0..x.count(dim) {
            let mut pos = (vec![], 0, 0);
            x.to_pos(dim, i, &mut pos);
            assert_eq!(x.to_index(dim, &pos), i);
        }
    }

    #[test]
    fn data_big() {
        use std::convert::TryInto;

        let x: Context = Construct::new();
        let ref dim: Vec<BigUint> = vec![2usize.into(), 2usize.into(), 2usize.into()];
        // 12 edges on a cube
        assert_eq!(x.count(dim), 12usize.into());
        assert_eq!(x.to_index(dim, &x.zero(dim)), 0usize.into());
        let count = x.count(dim).try_into().unwrap();
        for i in 0usize..count {
            let i: BigUint = i.into();
            let mut pos = x.zero(dim);
            x.to_pos(dim, i.clone(), &mut pos);
            assert_eq!(x.to_index(dim, &pos), i);
        }
    }

    #[test]
    fn of() {
        let x: Context<Of<Pair>> = Construct::new();
        let ref dim = vec![3];
        assert_eq!(x.count(dim), 3);
        assert_eq!(x.to_index(dim, &(vec![(0, 1)], 0, (0, 2))), 0);
        assert_eq!(x.to_index(dim, &(vec![(0, 1)], 0, (1, 2))), 1);
        assert_eq!(x.to_index(dim, &(vec![(0, 2)], 0, (1, 2))), 2);
        let ref dim = vec![3, 3];
        assert_eq!(x.count(dim), 18);
        assert_eq!(x.to_index(dim, &(vec![(0, 1), (0, 1)], 0, (0, 2))), 0);
        assert_eq!(x.to_index(dim, &(vec![(0, 1), (0, 2)], 0, (0, 2))), 1);
        assert_eq!(x.to_index(dim, &(vec![(0, 1), (1, 2)], 0, (0, 2))), 2);
        assert_eq!(x.to_index(dim, &(vec![(0, 1), (0, 1)], 0, (1, 2))), 3);
        assert_eq!(x.to_index(dim, &(vec![(0, 1), (0, 2)], 0, (1, 2))), 4);
        assert_eq!(x.to_index(dim, &(vec![(0, 1), (1, 2)], 0, (1, 2))), 5);
        assert_eq!(x.to_index(dim, &(vec![(0, 2), (0, 1)], 0, (1, 2))), 6);
        assert_eq!(x.to_index(dim, &(vec![(0, 2), (0, 2)], 0, (1, 2))), 7);
        assert_eq!(x.to_index(dim, &(vec![(0, 2), (1, 2)], 0, (1, 2))), 8);
        assert_eq!(x.to_index(dim, &(vec![(0, 1), (0, 1)], 1, (0, 2))), 9);
        assert_eq!(x.to_index(dim, &(vec![(0, 2), (0, 1)], 1, (0, 2))), 10);
        assert_eq!(x.to_index(dim, &(vec![(1, 2), (0, 1)], 1, (0, 2))), 11);
        assert_eq!(x.to_index(dim, &(vec![(0, 1), (0, 1)], 1, (1, 2))), 12);
        assert_eq!(x.to_index(dim, &(vec![(0, 2), (0, 1)], 1, (1, 2))), 13);
        assert_eq!(x.to_index(dim, &(vec![(1, 2), (0, 1)], 1, (1, 2))), 14);
        assert_eq!(x.to_index(dim, &(vec![(0, 1), (0, 2)], 1, (1, 2))), 15);
        assert_eq!(x.to_index(dim, &(vec![(0, 2), (0, 2)], 1, (1, 2))), 16);
        assert_eq!(x.to_index(dim, &(vec![(1, 2), (0, 2)], 1, (1, 2))), 17);

        let mut pos = (vec![(0, 0), (0, 0)], 0, (0, 0));
        x.to_pos(dim, 16, &mut pos);
        assert_eq!(pos, ((vec![(0, 2), (0, 2)], 1, (1, 2))));
    }

    #[test]
    fn of_big() {
        fn conv((x, i, (a, b)): (Vec<(usize, usize)>, usize, (usize, usize))) -> (Vec<(BigUint, BigUint)>, usize, (BigUint, BigUint)) {
            (x.into_iter().map(|(n, m)| (n.into(), m.into())).collect(), i, (a.into(), b.into()))
        }

        let x: Context<Of<Pair>> = Construct::new();
        let ref dim: Vec<BigUint> = vec![3usize.into()];
        assert_eq!(x.count(dim), 3usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(0, 1)], 0, (0, 2)))), 0usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(0, 1)], 0, (1, 2)))), 1usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(0, 2)], 0, (1, 2)))), 2usize.into());
        let ref dim: Vec<BigUint> = vec![3usize.into(), 3usize.into()];
        assert_eq!(x.count(dim), 18usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(0, 1), (0, 1)], 0, (0, 2)))), 0usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(0, 1), (0, 2)], 0, (0, 2)))), 1usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(0, 1), (1, 2)], 0, (0, 2)))), 2usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(0, 1), (0, 1)], 0, (1, 2)))), 3usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(0, 1), (0, 2)], 0, (1, 2)))), 4usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(0, 1), (1, 2)], 0, (1, 2)))), 5usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(0, 2), (0, 1)], 0, (1, 2)))), 6usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(0, 2), (0, 2)], 0, (1, 2)))), 7usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(0, 2), (1, 2)], 0, (1, 2)))), 8usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(0, 1), (0, 1)], 1, (0, 2)))), 9usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(0, 2), (0, 1)], 1, (0, 2)))), 10usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(1, 2), (0, 1)], 1, (0, 2)))), 11usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(0, 1), (0, 1)], 1, (1, 2)))), 12usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(0, 2), (0, 1)], 1, (1, 2)))), 13usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(1, 2), (0, 1)], 1, (1, 2)))), 14usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(0, 1), (0, 2)], 1, (1, 2)))), 15usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(0, 2), (0, 2)], 1, (1, 2)))), 16usize.into());
        assert_eq!(x.to_index(dim, &conv((vec![(1, 2), (0, 2)], 1, (1, 2)))), 17usize.into());

        let mut pos = x.zero(dim);
        x.to_pos(dim, 16usize.into(), &mut pos);
        assert_eq!(pos, conv((vec![(0, 2), (0, 2)], 1, (1, 2))));
    }
}
