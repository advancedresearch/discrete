use std::marker::PhantomData;

use num::BigUint;

use Construct;
use Data;
use Of;
use space::Space;

/// Dimension is natural number, position is the same as index.
pub struct Dimension<T = Data>(PhantomData<T>);

impl<T> Construct for Dimension<T> {
    fn new() -> Self { Dimension(PhantomData) }
}

impl Space<usize> for Dimension<Data> {
    type Dim = usize;
    type Pos = usize;

    fn count(&self, dim: &usize) -> usize { *dim }
    fn zero(&self, _dim: &usize) -> usize { 0 }
    fn to_index(&self, _dim: &usize, pos: &usize) -> usize { *pos }
    fn to_pos(&self, _dim: &usize, index: usize, pos: &mut usize) {
        *pos = index;
    }
}

impl Space<BigUint> for Dimension<Data> {
    type Dim = BigUint;
    type Pos = BigUint;

    fn count(&self, dim: &Self::Dim) -> BigUint { (*dim).clone() }
    fn zero(&self, _dim: &Self::Dim) -> BigUint { 0usize.into() }
    fn to_index(&self, _dim: &Self::Dim, pos: &Self::Pos) -> BigUint { (*pos).clone() }
    fn to_pos(&self, _dim: &Self::Dim, index: BigUint, pos: &mut Self::Pos) {
        *pos = index;
    }
}

impl<N, T: Space<N>> Space<N> for Dimension<Of<T>> {
    type Dim = T::Dim;
    type Pos = T::Pos;
    fn count(&self, dim: &Self::Dim) -> N {
        let of: T = Construct::new();
        of.count(dim)
    }
    fn zero(&self, dim: &Self::Dim) -> Self::Pos {
        let of: T = Construct::new();
        of.zero(dim)
    }
    fn to_index(&self, dim: &Self::Dim, pos: &Self::Pos) -> N {
        let of: T = Construct::new();
        of.to_index(dim, pos)
    }
    fn to_pos(&self, dim: &Self::Dim, index: N, pos: &mut Self::Pos) {
        let of: T = Construct::new();
        of.to_pos(dim, index, pos);
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn features() {
        is_complete::<usize, Dimension>();
        is_complete::<usize, Dimension<Of<Pair>>>();
    }
}
