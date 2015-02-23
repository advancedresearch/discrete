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
//! - Iterator through the space
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
pub use permutation::Permutation;

mod construct;
mod count;
mod to_index;
mod to_pos;
mod power_set;
mod dimension_n;
mod dimension;
mod pair;
mod eq_pair;
mod permutation;

/// Used by the final subspace.
#[derive(Copy)]
pub struct Data;
/// Used to nest a subspace.
pub struct Subspace<T>(PhantomData<T>);
/// Used to combine the dimensional and position types.
pub struct Of<T>(PhantomData<T>);

#[test]
fn test() {
    type D2 = Dimension<Subspace<Dimension<Data>>>;
    type D3 = Dimension<Subspace<D2>>;

    let x: D3 = Construct::new();
    let dim = (3, (3, 3));
    assert_eq!(x.count(dim), 27);
    let pos = (1, (0, 2));
    let index = x.to_index(dim, pos);
    assert_eq!(index, 11);
    let mut new_pos = pos;
    x.to_pos(dim, index, &mut new_pos);
    assert_eq!(pos, new_pos);

    let x: Pair<Data> = Construct::new();
    let dim = 4;
    assert_eq!(x.count(dim), 6);
    assert_eq!(x.to_index(dim, (0, 1)), 0);
    assert_eq!(x.to_index(dim, (0, 2)), 1);
    assert_eq!(x.to_index(dim, (1, 2)), 2);
    assert_eq!(x.to_index(dim, (0, 3)), 3);
    let mut new_pos = (0, 0);
    x.to_pos(dim, 3, &mut new_pos);
    assert_eq!(new_pos, (0, 3));

    let x: Pair<Subspace<Dimension<Data>>> = Construct::new();
    let dim = (4, 3);
    assert_eq!(x.count(dim), 18);
    assert_eq!(x.to_index(dim, ((0, 1), 0)), 0);
    assert_eq!(x.to_index(dim, ((0, 1), 1)), 1);
    assert_eq!(x.to_index(dim, ((0, 2), 0)), 3);
    let mut new_pos = ((0, 0), 0);
    x.to_pos(dim, 3, &mut new_pos);
    assert_eq!(new_pos, ((0, 2), 0));

    let x: DimensionN<Data> = Construct::new();
    let dim = &[3, 3];
    assert_eq!(x.count(dim), 9);
    assert_eq!(x.to_index(dim, &[0, 0]), 0);
    assert_eq!(x.to_index(dim, &[1, 0]), 1);
    assert_eq!(x.to_index(dim, &[0, 1]), 3);
    let mut new_pos = [0, 0];
    x.to_pos(dim, 3, &mut new_pos);
    assert_eq!(&new_pos, &[0, 1]);

    let x: Pair<Of<DimensionN<Data>>> = Construct::new();
    let dim = [2, 2];
    assert_eq!(x.count(&dim), 6);
    assert_eq!(x.to_index(&dim, (&[0, 0], &[1, 0])), 0);
    assert_eq!(x.to_index(&dim, (&[0, 0], &[0, 1])), 1);
    assert_eq!(x.to_index(&dim, (&[1, 0], &[0, 1])), 2);
    assert_eq!(x.to_index(&dim, (&[0, 0], &[1, 1])), 3);
    assert_eq!(x.to_index(&dim, (&[1, 0], &[1, 1])), 4);
    assert_eq!(x.to_index(&dim, (&[0, 1], &[1, 1])), 5);
    let mut min = [0, 0];
    let mut max = [0, 0];
    for i in 0..6 {
        x.to_pos(&dim, i, (&mut min, &mut max));
        // println!("{} {}", &min[], &max[]);
    }
    x.to_pos(&dim, 5, (&mut min, &mut max));
    assert_eq!(&min, &[0, 1]);
    assert_eq!(&max, &[1, 1]);

    let x: DimensionN<Of<Pair<Data>>> = Construct::new();
    let dim = [3, 4];
    assert_eq!(x.count(&dim), 18);
    assert_eq!(x.to_index(&dim, &[(0, 1), (0, 1)]), 0);
    assert_eq!(x.to_index(&dim, &[(0, 2), (0, 1)]), 1);
    assert_eq!(x.to_index(&dim, &[(1, 2), (0, 1)]), 2);
    assert_eq!(x.to_index(&dim, &[(0, 1), (0, 2)]), 3);
    let ref mut a = (0, 0);
    let ref mut b = (0, 0);
    x.to_pos(&dim, 3, &mut [a, b]);
    assert_eq!(*a, (0, 1));
    assert_eq!(*b, (0, 2));

    let x: PowerSet<Data> = Construct::new();
    let dim = 6;
    assert_eq!(x.count(dim), 64);
    assert_eq!(x.to_index(dim, &[]), 0);
    assert_eq!(x.to_index(dim, &[0]), 1);
    assert_eq!(x.to_index(dim, &[1]), 2);
    assert_eq!(x.to_index(dim, &[0, 1]), 3);
    let mut a = vec![];
    x.to_pos(dim, 9, &mut a);
    assert_eq!(&a, &[0, 3]);

    let x: PowerSet<Of<Pair<Data>>> = Construct::new();
    let dim = 4;
    assert_eq!(x.count(dim), 64);
    assert_eq!(x.to_index(dim, &[]), 0);
    assert_eq!(x.to_index(dim, &[(0, 1)]), 1);
    assert_eq!(x.to_index(dim, &[(0, 2)]), 2);
    assert_eq!(x.to_index(dim, &[(0, 1), (0, 2)]), 3);
    assert_eq!(x.to_index(dim, &[(1, 2)]), 4);
    assert_eq!(x.to_index(dim, &[(0, 1), (1, 2)]), 5);
    assert_eq!(x.to_index(dim, &[(0, 2), (1, 2)]), 6);
    assert_eq!(x.to_index(dim, &[(0, 1), (0, 2), (1, 2)]), 7);
    let mut a = [(0, 0); 64];
    {
        let mut b = a.iter_mut().collect();
        x.to_pos(dim, 7, &mut b);
    }
    assert_eq!(a[0], (0, 1));

}
