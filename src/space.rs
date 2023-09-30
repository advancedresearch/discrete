//! Helper trait for implementing discrete spaces.

use Construct;
use Count;
use Zero;
use ToIndex;
use ToPos;

use BigUint;

/// Implemented by discrete spaces.
pub trait Space<N>: Construct + Sized {
    /// The dimension type of the space.
    type Dim;
    /// The position type of the space.
    type Pos;

    /// Counts the size of space given the dimensions.
    fn count(&self, dim: &Self::Dim) -> N;
    /// Creates a default element.
    fn zero(&self, dim: &Self::Dim) -> Self::Pos;
    /// Converts position to index.
    fn to_index(&self, dim: &Self::Dim, pos: &Self::Pos) -> N;
    /// Converts index to position.
    fn to_pos(&self, dim: &Self::Dim, index: N, pos: &mut Self::Pos);
}

impl<D, T: Space<usize, Dim = D>> Count<D, usize> for T {
    fn count(&self, dim: &D) -> usize {Space::<usize>::count(self, dim)}
}

impl<D, T: Space<BigUint, Dim = D>> Count<D, BigUint> for T {
    fn count(&self, dim: &D) -> BigUint {Space::<BigUint>::count(self, dim)}
}

impl<D, P, T: Space<usize, Dim = D, Pos = P>> Zero<D, P, usize> for T {
    fn zero(&self, dim: &D) -> P {Space::<usize>::zero(self, dim)}
}

impl<D, P, T: Space<BigUint, Dim = D, Pos = P>> Zero<D, P, BigUint> for T {
    fn zero(&self, dim: &D) -> P {Space::<BigUint>::zero(self, dim)}
}

impl<D, P, T: Space<usize, Dim = D, Pos = P>> ToIndex<D, P, usize> for T {
    fn to_index(&self, dim: &D, pos: &P) -> usize {Space::<usize>::to_index(self, dim, pos)}
}

impl<D, P, T: Space<BigUint, Dim = D, Pos = P>> ToIndex<D, P, BigUint> for T {
    fn to_index(&self, dim: &D, pos: &P) -> BigUint {Space::<BigUint>::to_index(self, dim, pos)}
}

impl<D, P, T: Space<usize, Dim = D, Pos = P>> ToPos<D, P, usize> for T {
    fn to_pos(&self, dim: &D, ind: usize, pos: &mut P) {Space::<usize>::to_pos(self, dim, ind, pos)}
}

impl<D, P, T: Space<BigUint, Dim = D, Pos = P>> ToPos<D, P, BigUint> for T {
    fn to_pos(&self, dim: &D, ind: BigUint, pos: &mut P) {Space::<BigUint>::to_pos(self, dim, ind, pos)}
}
