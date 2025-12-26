use std::marker::PhantomData;

use crate::BigUint;

use Construct;
use Data;
use Of;
use space::Space;

/// A discrete space that models a full square of NxN pairs.
///
/// Dimension is natural number, position is `(x, y)`.
pub struct SqPair<T = Data>(PhantomData<T>);

impl<T> Construct for SqPair<T> {
    fn new() -> Self {SqPair(PhantomData)}
}

impl Space<usize> for SqPair<Data> {
    type Dim = usize;
    type Pos = (usize, usize);
    fn count(&self, dim: &usize) -> usize {dim * dim}
    fn zero(&self, _: &usize) -> (usize, usize) {(0, 0)}
    fn to_index(&self, dim: &usize, &(a, b): &(usize, usize)) -> usize {
        a + b * dim
    }
    fn to_pos(&self, dim: &usize, index: usize, pos: &mut (usize, usize)) {
        pos.0 = index % dim;
        pos.1 = index / dim;
    }
}

impl Space<BigUint> for SqPair<Data> {
    type Dim = BigUint;
    type Pos = (BigUint, BigUint);
    fn count(&self, dim: &Self::Dim) -> BigUint {dim * dim}
    fn zero(&self, _: &Self::Dim) -> Self::Pos {(0usize.into(), 0usize.into())}
    fn to_index(&self, dim: &Self::Dim, (a, b): &Self::Pos) -> BigUint {
        a + b * dim
    }
    fn to_pos(&self, dim: &Self::Dim, index: BigUint, pos: &mut Self::Pos) {
        pos.0 = &index % dim;
        pos.1 = &index / dim;
    }
}

impl<T, N> Space<N> for SqPair<Of<T>>
    where T: Space<N>,
          N: From<usize>,
          SqPair<Data>: Space<N, Dim = N, Pos = (N, N)>,
{
    type Dim = T::Dim;
    type Pos = (T::Pos, T::Pos);
    fn count(&self, dim: &Self::Dim) -> N {
        let of: T = Construct::new();
        let data: SqPair<Data> = Construct::new();
        data.count(&of.count(dim))
    }
    fn zero(&self, dim: &Self::Dim) -> Self::Pos {
        let of: T = Construct::new();
        (of.zero(dim), of.zero(dim))
    }
    fn to_index(
        &self,
        dim: &Self::Dim,
        &(ref a, ref b): &Self::Pos
    ) -> N {
        let of: T = Construct::new();
        let data: SqPair<Data> = Construct::new();
        let a = of.to_index(dim, a);
        let b = of.to_index(dim, b);
        data.to_index(&self.count(dim), &(a, b))
    }
    fn to_pos(
        &self,
        dim: &Self::Dim,
        index: N,
        &mut (ref mut a, ref mut b): &mut Self::Pos
    ) {
        let of: T = Construct::new();
        let data: SqPair<Data> = Construct::new();
        let count = of.count(dim);
        let mut pair = (0usize.into(), 0usize.into());
        data.to_pos(&count, index, &mut pair);
        let (pair_a, pair_b) = pair;
        of.to_pos(dim, pair_a, a);
        of.to_pos(dim, pair_b, b);
    }
}
