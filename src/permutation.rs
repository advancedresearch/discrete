
use std::marker::PhantomData;

use Construct;
use Count;
use Data;
use ToPos;
use ToIndex;
use Of;

/// Dimension is natural number, position is a list of numbers.
pub struct Permutation<T>(PhantomData<T>);

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

impl<'a> ToIndex<usize, &'a [usize]> for Permutation<Data> {
    fn to_index(&self, dim: usize, pos: &'a [usize]) -> usize {
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

impl<'a, T, U: Copy, V: Copy> ToIndex<U, &'a [V]> for Permutation<Of<T>>
    where
        T: Construct + ToIndex<U, V> + Count<U>
{
    fn to_index(&self, dim: U, pos: &'a [V]) -> usize {
        let of: T = Construct::new();
        let mut index = 0;
        let dim_count = of.count(dim);
        let mut count = 1;
        for (i, x) in pos.iter()
            .map(|&x| of.to_index(dim, x))
            .enumerate().rev() {
            let lower = pos[..i].iter()
                .map(|&y| of.to_index(dim, y))
                .filter(|&y| y < x).count();
            index += count * (x - lower);
            count *= dim_count - i;
        }
        index
    }
}

impl<'a> ToPos<usize, &'a mut Vec<usize>> for Permutation<Data> {
    fn to_pos(&self, dim: usize, mut index: usize, pos: &'a mut Vec<usize>) {
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

#[cfg(test)]
mod test {
    use super::super::*;

    #[test]
    fn data() {
        let permutation: Permutation<Data> = Construct::new();
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
        let space: Permutation<Of<Pair<Data>>> = Construct::new();
        let dim = 3;
        let count = space.count(dim);
        println!("{:?}", count);
        println!("{:?}", space.to_index(dim, &[(0, 1), (0, 2), (1, 2)]));
        println!("{:?}", space.to_index(dim, &[(0, 1), (1, 2), (0, 2)]));
        println!("{:?}", space.to_index(dim, &[(0, 2), (0, 1), (1, 2)]));
        println!("{:?}", space.to_index(dim, &[(0, 2), (1, 2), (0, 1)]));
        assert!(false);
    }
}
