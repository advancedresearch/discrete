use std::marker::PhantomData;

use Construct;
use Of;
use ToPos;
use Count;
use Data;
use ToIndex;

/// Dimension is natural number, position is a list of numbers.
pub struct PowerSet<T>(PhantomData<T>);

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

impl<'a, T, U: Copy, V: Copy>
ToIndex<U, &'a[V]> for PowerSet<Of<T>>
    where
        T: Construct + ToIndex<U, V>
{
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

impl<'a, T, U, V>
ToPos<U, &'a mut Vec<&'a mut V>>
for PowerSet<Of<T>>
    where
        T: Construct + Count<U> + ToPos<U, &'a mut V>,
        U: Copy
{
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

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn data() {
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
    }

    #[test]
    fn of() {
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
}
