
use std::marker::PhantomData;
use std::default::Default;

use Construct;
use Count;
use Data;
use Of;
use ToPos;
use ToIndex;
use Zero;

/// Dimension is natural number, position is a list of numbers.
pub struct Permutation<T = Data>(PhantomData<T>);

impl<T> Construct for Permutation<T> {
    fn new() -> Permutation<T> {
        Permutation(PhantomData)
    }
}

impl Count<usize> for Permutation<Data> {
    fn count(&self, dim: usize) -> usize {
        let mut res = 1;
        for x in 1..dim + 1 {
            res *= x;
        }
        res
    }
}

impl<T, U> Count<U> for Permutation<Of<T>>
    where
        T: Construct + Count<U>
{
    fn count(&self, dim: U) -> usize {
        let of: T = Construct::new();
        let mut res = 1;
        for x in 1..of.count(dim) + 1 {
            res *= x;
        }
        res
    }
}

impl Zero<usize, Vec<usize>> for Permutation<Data> {
    fn zero(&self, dim: usize) -> Vec<usize> {
        vec![0, dim]
    }
}

impl<T, U, V> Zero<U, Vec<V>> for Permutation<Of<T>>
    where
        T: Construct + Count<U> + Zero<U, V>,
        U: Copy,
        V: Default + Clone
{
    fn zero(&self, dim: U) -> Vec<V> {
        let of: T = Construct::new();
        vec![of.zero(dim); of.count(dim)]
    }
}

impl ToIndex<usize, Vec<usize>> for Permutation<Data> {
    fn to_index(&self, dim: usize, pos: &Vec<usize>) -> usize {
        let mut index = 0;
        let mut count = 1;
        for (i, &x) in pos.iter().enumerate().rev() {
            let lower = pos[..i].iter().filter(|&&y| y < x).count();
            index += count * (x - lower);
            count *= dim - i;
        }
        index
    }
}

impl<T, U: Copy, V: Copy> ToIndex<U, Vec<V>> for Permutation<Of<T>>
    where
        T: Construct + ToIndex<U, V> + Count<U>
{
    fn to_index(&self, dim: U, pos: &Vec<V>) -> usize {
        let of: T = Construct::new();
        let mut index = 0;
        let dim_count = of.count(dim);
        let mut count = 1;
        for (i, x) in pos.iter()
            .map(|x| of.to_index(dim, x))
            .enumerate().rev() {
            let lower = pos[..i].iter()
                .map(|y| of.to_index(dim, y))
                .filter(|&y| y < x).count();
            index += count * (x - lower);
            count *= dim_count - i;
        }
        index
    }
}

impl ToPos<usize, Vec<usize>> for Permutation<Data> {
    fn to_pos(&self, dim: usize, mut index: usize, pos: &mut Vec<usize>) {
        unsafe { pos.set_len(0); }

        let mut count = 1;
        for (j, x) in (1..dim + 1).enumerate() {
            count *= x;
            pos.push(j);
        }

        for i in 0..dim {
            let block = count / (dim - i);
            let ind = index / block;
            let item = pos.remove(ind);
            pos.push(item);
            count /= dim - i;
            index -= ind * block;
        }
    }
}

impl<T, U, V> ToPos<U, Vec<V>> for Permutation<Of<T>>
    where
        T: Construct + Count<U> + ToPos<U, V>,
        U: Copy,
        V: Default
{
    fn to_pos(&self, dim: U, mut index: usize, pos: &mut Vec<V>) {
        let of: T = Construct::new();
        let of_count = of.count(dim);
        pos.clear();

        let mut count = 1;
        for (j, x) in (1..of_count + 1).enumerate() {
            count *= x;
            let mut new_pos: V = Default::default();
            of.to_pos(dim, j, &mut new_pos);
            pos.push(new_pos);
        }

        for i in 0..of_count {
            let block = count / (of_count - i);
            let ind = index / block;
            let item = pos.remove(ind);
            pos.push(item);
            count /= of_count - i;
            index -= ind * block;
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::*;

    #[test]
    fn features() {
        is_complete::<Permutation, usize, Vec<usize>>();
        is_complete::<Permutation<Of<Pair>>, usize, Vec<(usize, usize)>>();
    }

    #[test]
    fn data() {
        let permutation: Permutation = Construct::new();
        assert_eq!(permutation.count(1), 1);
        assert_eq!(permutation.count(2), 2);
        assert_eq!(permutation.count(3), 6);
        assert_eq!(permutation.count(4), 24);

        let mut pos = Vec::new();
        let dim = 4;
        let count = permutation.count(dim);
        for i in 0..count {
            permutation.to_pos(dim, i, &mut pos);
            let index = permutation.to_index(dim, &pos);
            assert_eq!(index, i);
        }
    }

    #[test]
    fn of() {
        let space: Permutation<Of<Pair>> = Construct::new();
        let dim = 3;
        let count = space.count(dim);
        let mut pos = Vec::new();
        for i in 0..count {
            space.to_pos(dim, i, &mut pos);
            let index = space.to_index(dim, &pos);
            assert_eq!(index, i);
        }
    }
}
