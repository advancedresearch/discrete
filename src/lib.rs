#![deny(missing_docs)]

//! Combinatorial phantom types for discrete mathematics.
//!
//! All discrete spaces has the following functions:
//!
//! ~~~ignore
//!     fn count(dim) -> uint;
//!     fn to_index(dim, pos) -> uint;
//!     fn to_pos(dim, index, &mut pos);
//! ~~~
//!
//! This makes it possible to allocate to solve tasks as:
//!
//! - Allocate enough memory for the space
//! - Read and write data
//! - Convert from and to natural numbers
//! - Iterate through the space
//! - Pick a random object of the space
//!
//! Iterating through the space can be done simply
//! by counting from zero up to the size of the space.
//!
//! Phantom types are used because they represents the general spaces.
//! For example, we can represent a general two-dimensional space,
//! instead of binding the type to the size.
//!
//! For any constructed space, there is a dimension and position type.
//! The dimension and position types are compositions,
//! given by the type of the constructed space.

use std::marker::PhantomData;

pub use construct::Construct;
pub use count::Count;
pub use to_index::ToIndex;
pub use to_pos::ToPos;
pub use power_set::PowerSet;
pub use dimension_n::DimensionN;
pub use dimension::Dimension;
pub use pair::Pair;
pub use eq_pair::EqPair;
pub use neq_pair::NeqPair;
pub use permutation::Permutation;
pub use context::Context;
pub use directed_context::DirectedContext;

mod construct;
mod count;
mod to_index;
mod to_pos;
mod power_set;
mod dimension_n;
mod dimension;
mod pair;
mod eq_pair;
mod neq_pair;
mod permutation;
mod context;
mod directed_context;

/// Used by the final subspace.
#[derive(Copy, Clone)]
pub struct Data;
/// Used to nest a subspace.
pub struct Subspace<T>(PhantomData<T>);
/// Used to combine the dimensional and position types.
pub struct Of<T>(PhantomData<T>);

#[cfg(test)]
pub fn does_count<T, U>()
    where
        T: Count<U>
{}

#[cfg(test)]
pub fn does_to_index<T, U, V>()
    where
        T: ToIndex<U, V>
{}

#[cfg(test)]
pub fn does_to_pos<T, U, V>()
    where
        T: ToPos<U, V>
{}

#[cfg(test)]
pub fn is_complete<T, U, V, W>()
    where
        T: Count<U> + ToIndex<U, V> + ToPos<U, W>
{
    does_count::<T, U>();
    does_to_index::<T, U, V>();
    does_to_pos::<T, U, W>();
}
