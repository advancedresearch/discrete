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

use std::num::Float;

/// Used by the final subspace.
#[derive(Copy)]
pub struct Data;
/// Used to nest a subspace.
pub struct Subspace<T>;
/// Used to combine the dimensional and position types.
pub struct Of<T>;

/// Dimension is natural number, position is the same as index.
pub struct Dimension<T>;
/// Dimension is natural number, position is (min, max).
pub struct Pair<T>;
/// Dimension is a list of numbers, position is a list of numbers.
pub struct DimensionN<T>;
/// Dimension is natural number, position is a list of numbers.
pub struct PowerSet<T>;

pub trait Construct {
    fn new() -> Self;
}
impl<T> Construct for Dimension<T> {
    fn new() -> Dimension<T> { Dimension }
}
impl<T> Construct for Pair<T> {
    fn new() -> Pair<T> { Pair }
}
impl<T> Construct for DimensionN<T> {
    fn new() -> DimensionN<T> { DimensionN }
}
impl<T> Construct for PowerSet<T> {
    fn new() -> PowerSet<T> { PowerSet }
}

pub trait Count<T> {
    fn count(&self, dim: T) -> usize;
}
impl Count<usize> for Dimension<Data> {
    fn count(&self, dim: usize) -> usize { dim }
}
impl<T: Construct + Count<U>, U> Count<(usize, U)> for Dimension<Subspace<T>> {
    fn count(&self, (a, b): (usize, U)) -> usize {
        let subspace: T = Construct::new();
        a * subspace.count(b)
    }
}
impl Count<usize> for Pair<Data> {
    fn count(&self, dim: usize) -> usize { dim * (dim - 1) / 2 }
}
impl<T: Construct + Count<U>, U> Count<(usize, U)> for Pair<Subspace<T>> {
    fn count(&self, (a, b): (usize, U)) -> usize {
        let subspace: T = Construct::new();
        let data: Pair<Data> = Construct::new();
        data.count(a) * subspace.count(b)
    }
}
impl<T: Construct + Count<U>, U> Count<U> for Pair<Of<T>> {
    fn count(&self, dim: U) -> usize {
        let of: T = Construct::new();
        let data: Pair<Data> = Construct::new();
        data.count(of.count(dim))
    }
}
impl<'a> Count<&'a [usize]> for DimensionN<Data> {
    fn count(&self, dim: &'a [usize]) -> usize {
        let mut prod = 1;
        for i in 0..dim.len() {
            prod *= dim[i];
        }
        prod
    }
}
impl<'a, T: Construct + Count<U>, U>
Count<(&'a [usize], U)> for DimensionN<Subspace<T>> {
    fn count(&self, (a, b): (&'a [usize], U)) -> usize {
        let subspace: T = Construct::new();
        let data: DimensionN<Data> = Construct::new();
        data.count(a) * subspace.count(b)
    }
}
impl<'a, T: Construct + Count<U>, U: Copy>
Count<&'a [U]> for DimensionN<Of<T>> {
    fn count(&self, dim: &'a [U]) -> usize {
        let of: T = Construct::new();
        let mut prod = 1;
        for i in (0..dim.len()) {
            prod *= of.count(dim[i]);
        }
        prod
    }
}
impl Count<usize> for PowerSet<Data> {
    fn count(&self, dim: usize) -> usize {
        1 << dim
    }
}
impl<T: Construct + Count<U>, U> Count<U> for PowerSet<Of<T>> {
    fn count(&self, dim: U) -> usize {
        let of: T = Construct::new();
        1 << of.count(dim)
    }
}

pub trait ToIndex<T, U> {
    fn to_index(&self, dim: T, pos: U) -> usize;
}
impl ToIndex<usize, usize> for Dimension<Data> {
    fn to_index(&self, _dim: usize, pos: usize) -> usize { pos }
}
impl<T: Construct + Count<U> + ToIndex<U, V>, U: Copy, V>
ToIndex<(usize, U), (usize, V)> for Dimension<Subspace<T>> {
    fn to_index(&self, (_a, b): (usize, U), (pa, pb): (usize, V)) -> usize {
        let subspace: T = Construct::new();
        let count = subspace.count(b);
        pa * count + subspace.to_index(b, pb)
    }
}
impl ToIndex<usize, (usize, usize)> for Pair<Data> {
    fn to_index(&self, _dim: usize, (min, max): (usize, usize)) -> usize {
        min + max * (max - 1) / 2
    }
}
impl<T: Construct + Count<U> + ToIndex<U, V>, U: Copy, V>
ToIndex<(usize, U), ((usize, usize), V)> for Pair<Subspace<T>> {
    fn to_index(
        &self,
        (a, b): (usize, U),
        (pa, pb): ((usize, usize), V)
    ) -> usize {
        let subspace: T = Construct::new();
        let count = subspace.count(b);
        let data: Pair<Data> = Construct::new();
        data.to_index(a, pa) * count + subspace.to_index(b, pb)
    }
}
impl<T: Construct + ToIndex<U, V> + Count<U>, U: Copy, V>
ToIndex<U, (V, V)> for Pair<Of<T>> {
    fn to_index(
        &self,
        dim: U,
        (min, max): (V, V)
    ) -> usize {
        let of: T = Construct::new();
        let data: Pair<Data> = Construct::new();
        let min = of.to_index(dim, min);
        let max = of.to_index(dim, max);
        data.to_index(self.count(dim), (min, max))
    }
}
impl<'a> ToIndex<&'a [usize], &'a [usize]> for DimensionN<Data> {
    fn to_index(&self, dim: &'a [usize], pos: &'a [usize]) -> usize {
        let mut dim_index = 0;
        for i in (0..dim.len()).rev() {
            dim_index = dim_index * dim[i] + pos[i];
        }
        dim_index
    }
}
impl<'a, T: Construct + Count<U> + ToIndex<U, V>, U: Copy, V>
ToIndex<(&'a [usize], U), (&'a [usize], V)> for DimensionN<Subspace<T>> {
    fn to_index(
        &self,
        (a, b): (&'a [usize], U),
        (pa, pb): (&'a [usize], V)
    ) -> usize {
        let subspace: T = Construct::new();
        let count = subspace.count(b);
        let data: DimensionN<Data> = Construct::new();
        data.to_index(a, pa) * count + subspace.to_index(b, pb)
    }
}
impl<'a, T: Construct + Count<U> + ToIndex<U, V>, U: Copy, V: Copy>
ToIndex<&'a [U], &'a [V]> for DimensionN<Of<T>> {
    fn to_index(
        &self,
        dim: &'a [U],
        pos: &'a [V]
    ) -> usize {
        let of: T = Construct::new();
        let mut dim_index = 0;
        for i in (0..dim.len()).rev() {
            dim_index = dim_index * of.count(dim[i])
                      + of.to_index(dim[i], pos[i]);
        }
        dim_index
    }
}
impl<'a> ToIndex<usize, &'a [usize]> for PowerSet<Data> {
    fn to_index(
        &self,
        _dim: usize,
        pos: &'a [usize]
    ) -> usize {
        let mut index = 0;
        for &i in pos.iter() {
            index |= 1 << i;
        }
        index
    }
}
impl<'a, T: Construct + ToIndex<U, V>, U: Copy, V: Copy>
ToIndex<U, &'a[V]> for PowerSet<Of<T>> {
    fn to_index(
        &self,
        dim: U,
        pos: &'a [V]
    ) -> usize {
        let of: T = Construct::new();
        let mut index = 0;
        for &i in pos.iter() {
            index |= 1 << of.to_index(dim, i);
        }
        index
    }
}

pub trait ToPos<T, U> {
    fn to_pos(&self, dim: T, index: usize, pos: U);
}
impl<'a> ToPos<usize, &'a mut usize> for Dimension<Data> {
    fn to_pos(&self, _dim: usize, index: usize, pos: &'a mut usize) {
        *pos = index;
    }
}
impl<'a, T: Construct + Count<U> + ToPos<U, &'a mut V>, U: Copy, V>
ToPos<(usize, U), &'a mut (usize, V)> for Dimension<Subspace<T>> {
    fn to_pos(
        &self,
        (_a, b): (usize, U),
        index: usize,
        &mut (ref mut head, ref mut tail): &'a mut (usize, V)
    ) {
        let subspace: T = Construct::new();
        let count = subspace.count(b);
        let x = index / count;
        *head = x;
        subspace.to_pos(b, index - x * count, tail)
    }
}
impl<'a> ToPos<usize, &'a mut (usize, usize)> for Pair<Data> {
    fn to_pos(&self, _dim: usize, index: usize, pos: &'a mut (usize, usize)) {
        let max = ((-1f64 + (8f64 * index as f64 + 1f64).sqrt()) / 2f64) as usize + 1;
        let min = index - max * (max + 1) / 2 + max;
        *pos = (min, max)
    }
}
impl<'a, T: Construct + Count<U> + ToPos<U, &'a mut V>, U: Copy, V>
ToPos<(usize, U), &'a mut ((usize, usize), V)> for Pair<Subspace<T>> {
    fn to_pos(
        &self,
        (a, b): (usize, U),
        index: usize,
        &mut (ref mut head, ref mut tail): &'a mut ((usize, usize), V)
    ) {
        let subspace: T = Construct::new();
        let count = subspace.count(b);
        let data: Pair<Data> = Construct::new();
        let x = index / count;
        data.to_pos(a, x, head);
        subspace.to_pos(b, index - x * count, tail)
    }
}
impl<'a, T: Construct + Count<U> + ToPos<U, V>, U: Copy, V>
ToPos<U, (V, V)> for Pair<Of<T>> {
    fn to_pos(
        &self,
        dim: U,
        index: usize,
        (min, max): (V, V)
    ) {
        let of: T = Construct::new();
        let data: Pair<Data> = Construct::new();
        let count = self.count(dim);
        let mut pair = (0, 0);
        data.to_pos(count, index, &mut pair);
        let (pair_min, pair_max) = pair;
        of.to_pos(dim, pair_min, min);
        of.to_pos(dim, pair_max, max);
    }
}
impl<'a> ToPos<&'a [usize], &'a mut [usize]> for DimensionN<Data> {
    fn to_pos(&self, dim: &'a [usize], index: usize, pos: &'a mut [usize]) {
        let mut prod = self.count(dim);
        let mut dim_index = index;
        for i in (0..dim.len()).rev() {
            prod /= dim[i];
            let p_i = dim_index / prod;
            *pos.get_mut(i).unwrap() = p_i;
            dim_index -= p_i * prod;
        }
    }
}
impl<'a, T: Construct + Count<U> + ToPos<U, &'a mut V>, U: Copy, V>
ToPos<(&'a [usize], U), &'a mut (&'a mut [usize], V)> for DimensionN<Subspace<T>> {
    fn to_pos(
        &self,
        (a, b): (&'a [usize], U),
        index: usize,
        &mut (ref mut head, ref mut tail): &'a mut (&'a mut [usize], V)
    ) {
        let subspace: T = Construct::new();
        let count = subspace.count(b);
        let data: DimensionN<Data> = Construct::new();
        let x = index / count;
        data.to_pos(a, index / count, *head);
        subspace.to_pos(b, index - x * count, tail)
    }
}
impl<'a, T: Construct + Count<U> + ToPos<U, &'a mut V>, U: Copy, V>
ToPos<&'a [U], &'a mut [&'a mut V]> for DimensionN<Of<T>> {
    fn to_pos(
        &self,
        dim: &'a [U],
        index: usize,
        pos: &'a mut [&'a mut V]
    ) {
        let of: T = Construct::new();
        let mut prod = self.count(dim);
        let mut dim_index = index;
        for (i, p) in pos.iter_mut().enumerate().rev() {
            prod /= of.count(dim[i]);
            let p_i = dim_index / prod;
            of.to_pos(dim[i], p_i, *p);
            dim_index -= p_i * prod;
        }
    }
}
impl<'a> ToPos<usize, &'a mut Vec<usize>> for PowerSet<Data> {
    fn to_pos(
        &self,
        dim: usize,
        index: usize,
        pos: &'a mut Vec<usize>
    ) {
        unsafe { pos.set_len(0); }
        for i in 0..dim {
            if ((index >> i) & 1) == 1 {
                pos.push(i);
            }
        }
    }
}
impl<
    'a,
    T: Construct + Count<U> + ToPos<U, &'a mut V>,
    U: Copy,
    V
>
ToPos<U, &'a mut Vec<&'a mut V>> for PowerSet<Of<T>> {
    fn to_pos(
        &self,
        dim: U,
        index: usize,
        pos: &'a mut Vec<&'a mut V>
    ) {
        let of: T = Construct::new();
        let count = of.count(dim);
        let mut i = 0;
        for p in pos.iter_mut() {
            for j in i..count {
                if ((index >> j) & 1) == 1 {
                    of.to_pos(dim, j, *p);
                    i += 1;
                    break;
                }
            }
        }
        // unsafe { pos.set_len(i); }
    }
}

pub type D2 = Dimension<Subspace<Dimension<Data>>>;
pub type D3 = Dimension<Subspace<D2>>;

#[test]
fn test() {
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
    x.to_pos(dim, 3, &mut new_pos[]);
    assert_eq!(&new_pos[], &[0, 1][]);

    let x: Pair<Of<DimensionN<Data>>> = Construct::new();
    let dim = [2, 2];
    assert_eq!(x.count(&dim[]), 6);
    assert_eq!(x.to_index(&dim[], (&[0, 0], &[1, 0])), 0);
    assert_eq!(x.to_index(&dim[], (&[0, 0], &[0, 1])), 1);
    assert_eq!(x.to_index(&dim[], (&[1, 0], &[0, 1])), 2);
    assert_eq!(x.to_index(&dim[], (&[0, 0], &[1, 1])), 3);
    assert_eq!(x.to_index(&dim[], (&[1, 0], &[1, 1])), 4);
    assert_eq!(x.to_index(&dim[], (&[0, 1], &[1, 1])), 5);
    let mut min = [0, 0];
    let mut max = [0, 0];
    for i in range(0, 6) {
        x.to_pos(&dim[], i, (&mut min, &mut max));
        // println!("{} {}", &min[], &max[]);
    }
    x.to_pos(&dim[], 5, (&mut min, &mut max));
    assert_eq!(&min[], &[0, 1][]);
    assert_eq!(&max[], &[1, 1][]);

    let x: DimensionN<Of<Pair<Data>>> = Construct::new();
    let dim = [3, 4];
    assert_eq!(x.count(&dim[]), 18);
    assert_eq!(x.to_index(&dim[], &[(0, 1), (0, 1)]), 0);
    assert_eq!(x.to_index(&dim[], &[(0, 2), (0, 1)]), 1);
    assert_eq!(x.to_index(&dim[], &[(1, 2), (0, 1)]), 2);
    assert_eq!(x.to_index(&dim[], &[(0, 1), (0, 2)]), 3);
    let ref mut a = (0, 0);
    let ref mut b = (0, 0);
    x.to_pos(&dim[], 3, &mut [a, b]);
    assert_eq!(*a, (0, 1));
    assert_eq!(*b, (0, 2));

    let x: PowerSet<Data> = Construct::new();
    let dim = 6;
    assert_eq!(x.count(dim), 64);
    assert_eq!(x.to_index(dim, &[][]), 0);
    assert_eq!(x.to_index(dim, &[0][]), 1);
    assert_eq!(x.to_index(dim, &[1][]), 2);
    assert_eq!(x.to_index(dim, &[0, 1][]), 3);
    let mut a = vec![];
    x.to_pos(dim, 9, &mut a);
    assert_eq!(&a[], &[0, 3][]);

    let x: PowerSet<Of<Pair<Data>>> = Construct::new();
    let dim = 4;
    assert_eq!(x.count(dim), 64);
    assert_eq!(x.to_index(dim, &[][]), 0);
    assert_eq!(x.to_index(dim, &[(0, 1)][]), 1);
    assert_eq!(x.to_index(dim, &[(0, 2)][]), 2);
    assert_eq!(x.to_index(dim, &[(0, 1), (0, 2)][]), 3);
    assert_eq!(x.to_index(dim, &[(1, 2)][]), 4);
    assert_eq!(x.to_index(dim, &[(0, 1), (1, 2)][]), 5);
    assert_eq!(x.to_index(dim, &[(0, 2), (1, 2)][]), 6);
    assert_eq!(x.to_index(dim, &[(0, 1), (0, 2), (1, 2)][]), 7);
    let mut a = [(0, 0); 64];
    {
        let mut b = a.iter_mut().collect();
        x.to_pos(dim, 7, &mut b);
    }
    assert_eq!(a[0], (0, 1));

}
