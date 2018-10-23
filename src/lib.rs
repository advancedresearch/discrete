#![deny(missing_docs)]

//! Combinatorial phantom types for discrete mathematics.
//!
//! All discrete spaces have the following functions:
//!
//! ~~~ignore
//! fn count(dim) -> usize;
//! fn to_index(dim, pos) -> usize;
//! fn to_pos(dim, index, &mut pos);
//! ~~~
//!
//! A discrete space is countable and has a one-to-one map
//! with the natural numbers.
//!
//! For example, a pair of natural numbers is a discrete space.
//! There exists an algorithm that converts each pair of numbers
//! into a number. Likewise there exists an algorithm that
//! takes a number and converts it into a pair.
//!
//! To construct a pair, you write:
//!
//! ~~~ignore
//! let x: Pair<Data> = Construct::new();
//! ~~~
//!
//! The `x` above is a phantom variable, it does not use memory
//! in the compiled program. It represents the discrete space
//! that we have constructed. Now we can call methods on the space
//! to examine its discrete structure.
//!
//! A pair can be visualized as edges between points.
//! If we have 4 points then we can create 6 edges:
//!
//! ```text
//!   o---o
//!   |\ /|
//!   | X |
//!   |/ \|
//!   o---o
//! ```
//!
//! To check this we can write:
//!
//! ~~~ignore
//! let dim = 4; // number of points
//! assert_eq!(x.count(dim), 6); // count edges
//! ~~~
//!
//! Phantom types makes it possible to construct advanced discrete spaces.
//! By using generic program, the algorithms to examine the structure
//! follows from the construction of the space.
//!
//! This makes it possible to solve tasks as:
//!
//! - Compute upper bounds for certain problems
//! - Store data with a non-trivial structure
//! - Convert from and to natural numbers
//! - Iterate through the space
//! - Pick a random object of the space
//!
//! Iterating through the space can be done simply
//! by counting from zero up to the size of the space.
//! For each number, we convert to a position within the space.
//!
//! Pick a random object of the space can be done by generating
//! a random number between 0 and the size of the space.
//!
//! ### Advanced spaces
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
pub use zero::Zero;
pub use power_set::PowerSet;
pub use dimension_n::DimensionN;
pub use dimension::Dimension;
pub use pair::Pair;
pub use eq_pair::EqPair;
pub use neq_pair::NeqPair;
pub use permutation::Permutation;
pub use context::Context;
pub use directed_context::DirectedContext;
pub use either::{Either, Select};

mod construct;
mod count;
mod to_index;
mod to_pos;
mod zero;
mod power_set;
mod dimension_n;
mod dimension;
mod pair;
mod eq_pair;
mod neq_pair;
mod permutation;
mod context;
mod directed_context;
mod subspace;
mod either;

/// Used by the final subspace.
#[derive(Copy, Clone)]
pub struct Data;
/// Used to combine the dimensional and position types.
pub struct Of<T>(PhantomData<T>);

#[cfg(test)]
pub fn does_count<T, U>()
    where
        T: Count<U>
{}

#[cfg(test)]
pub fn does_zero<T, U, V>()
    where
        T: Zero<U, V>
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

// T - discrete space
// U - dimension
// V - Position
#[cfg(test)]
pub fn is_complete<T, U, V>()
    where
        T: Count<U> + Zero<U, V> + ToIndex<U, V> + ToPos<U, V>
{
    does_count::<T, U>();
    does_zero::<T, U, V>();
    does_to_index::<T, U, V>();
    does_to_pos::<T, U, V>();
}
