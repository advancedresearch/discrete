use std::marker::PhantomData;

use Construct;
use Of;
use ToPos;
use Count;
use Data;
use ToIndex;
use Zero;

/// Dimension is natural number, position is a list of numbers.
pub struct PowerSet<T = Data>(PhantomData<T>);

impl<T> Construct for PowerSet<T> {
    fn new() -> PowerSet<T> { PowerSet(PhantomData) }
}

impl Count<usize> for PowerSet<Data> {
    fn count(&self, dim: usize) -> usize {
        1 << dim
    }
}

impl<T, U> Count<U> for PowerSet<Of<T>>
    where
        T: Construct + Count<U>
{
    fn count(&self, dim: U) -> usize {
        let of: T = Construct::new();
        1 << of.count(dim)
    }
}

impl Zero<usize, Vec<usize>> for PowerSet<Data> {
    fn zero(&self, _dim: usize) -> Vec<usize> {
        vec![]
    }
}

impl<T, U, V> Zero<U, Vec<V>> for PowerSet<Of<T>>
    where
        T: Construct + Count<U> + Zero<U, V>,
        U: Copy
{
    fn zero(&self, _dim: U) -> Vec<V> {
        vec![]
    }
}

impl ToIndex<usize, Vec<usize>> for PowerSet<Data> {
    fn to_index(
        &self,
        _dim: usize,
        pos: &Vec<usize>
    ) -> usize {
        let mut index = 0;
        for &i in pos.iter() {
            index |= 1 << i;
        }
        index
    }
}

impl<T, U: Copy, V: Copy>
ToIndex<U, Vec<V>> for PowerSet<Of<T>>
    where
        T: Construct + ToIndex<U, V>
{
    fn to_index(
        &self,
        dim: U,
        pos: &Vec<V>
    ) -> usize {
        let of: T = Construct::new();
        let mut index = 0;
        for i in pos.iter() {
            index |= 1 << of.to_index(dim, i);
        }
        index
    }
}

impl ToPos<usize, Vec<usize>> for PowerSet<Data> {
    fn to_pos(
        &self,
        dim: usize,
        index: usize,
        pos: &mut Vec<usize>
    ) {
        unsafe { pos.set_len(0); }
        for i in 0..dim {
            if ((index >> i) & 1) == 1 {
                pos.push(i);
            }
        }
    }
}

impl<T, U, V>
ToPos<U, Vec<V>>
for PowerSet<Of<T>>
    where
        T: Construct + Count<U> + ToPos<U, V> + Zero<U, V>,
        U: Copy
{
    fn to_pos(
        &self,
        dim: U,
        index: usize,
        pos: &mut Vec<V>
    ) {
        let of: T = Construct::new();
        let count = of.count(dim);
        pos.clear();
        pos.reserve_exact(count);
        for j in 0..count {
            if ((index >> j) & 1) == 1 {
                let mut p = of.zero(dim);
                of.to_pos(dim, j, &mut p);
                pos.push(p);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn features() {
        is_complete::<PowerSet, usize, Vec<usize>>();
        does_zero::<PowerSet, usize, Vec<usize>>();
        does_zero::<PowerSet<Of<Pair>>, usize, Vec<(usize, usize)>>();
    }

    #[test]
    fn data() {
        let x: PowerSet = Construct::new();
        let dim = 6;
        assert_eq!(x.count(dim), 64);
        assert_eq!(x.to_index(dim, &vec![]), 0);
        assert_eq!(x.to_index(dim, &vec![0]), 1);
        assert_eq!(x.to_index(dim, &vec![1]), 2);
        assert_eq!(x.to_index(dim, &vec![0, 1]), 3);
        let mut a = vec![];
        x.to_pos(dim, 9, &mut a);
        assert_eq!(&a, &[0, 3]);
    }

    #[test]
    fn of() {
        let x: PowerSet<Of<Pair>> = Construct::new();
        let dim = 4;
        assert_eq!(x.count(dim), 64);
        assert_eq!(x.to_index(dim, &vec![]), 0);
        assert_eq!(x.to_index(dim, &vec![(0, 1)]), 1);
        assert_eq!(x.to_index(dim, &vec![(0, 2)]), 2);
        assert_eq!(x.to_index(dim, &vec![(0, 1), (0, 2)]), 3);
        assert_eq!(x.to_index(dim, &vec![(1, 2)]), 4);
        assert_eq!(x.to_index(dim, &vec![(0, 1), (1, 2)]), 5);
        assert_eq!(x.to_index(dim, &vec![(0, 2), (1, 2)]), 6);
        assert_eq!(x.to_index(dim, &vec![(0, 1), (0, 2), (1, 2)]), 7);
        let mut a = vec![(0, 0); 64];
        x.to_pos(dim, 7, &mut a);
        assert_eq!(a[0], (0, 1));
    }
}
