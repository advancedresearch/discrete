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
//! The `x` above is a phantom variable, which means it does not use memory
//! in the compiled program. The phantom variable represents the discrete space
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
//! By using a generic program, the algorithms to examine the structure
//! follows from the construction of the space.
//!
//! This makes it possible to solve tasks such as:
//!
//! - Compute upper bounds for many problems
//! - Store data with a non-trivial structure
//! - Convert from and to natural numbers
//! - Iterate through all elements of a space
//! - Pick a random object of the space
//!
//! Iterating through all elements of a space can be done simply
//! by counting from zero up to the size of the space.
//! For each number, we convert to a position within the space.
//!
//! Picking a random object of the space can be done by generating
//! a random number between 0 and the size of the space.
//!
//! ### Advanced spaces
//!
//! Phantom types are used because they represent the general spaces.
//! For example, we can represent a general two-dimensional space,
//! instead of binding the type to the size.
//!
//! For any constructed space, there is a dimension and position type.
//! The dimension and position types are compositions,
//! given by the type of the constructed space.

extern crate num;

use std::marker::PhantomData;

pub use construct::Construct;
pub use count::Count;
pub use zero::Zero;
pub use to_index::ToIndex;
pub use to_pos::ToPos;

pub use power_set::PowerSet;
pub use dimension_n::DimensionN;
pub use dimension::Dimension;
pub use pair::Pair;
pub use eq_pair::EqPair;
pub use neq_pair::NeqPair;
pub use sq_pair::SqPair;
pub use permutation::Permutation;
pub use context::Context;
pub use directed_context::DirectedContext;
pub use either::{Either, Select};
pub use homotopy::{Homotopy, HPoint};
pub use num::BigUint;

pub mod space;

mod construct;
mod count;
mod zero;
mod to_index;
mod to_pos;

mod power_set;
mod dimension_n;
mod dimension;
mod pair;
mod eq_pair;
mod neq_pair;
mod sq_pair;
mod permutation;
mod context;
mod directed_context;
mod subspace;
mod either;
mod homotopy;

/// Used by the final subspace.
#[derive(Copy, Clone)]
pub struct Data;
/// Used to combine the dimensional and position types.
pub struct Of<T>(PhantomData<T>);

// N - numeric type
// T - discrete space
#[cfg(test)]
pub fn is_complete<N, T>()
    where
        T: space::Space<N>
{}
