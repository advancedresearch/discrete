use std::marker::PhantomData;

use Construct;
use Data;
use Count;
use Of;
use ToIndex;
use ToPos;
use Zero;

/// Dimension is a list of numbers, position is a list of numbers.
pub struct DimensionN<T = Data>(PhantomData<T>);

impl<T> Construct for DimensionN<T> {
    fn new() -> DimensionN<T> { DimensionN(PhantomData) }
}

impl Count<Vec<usize>> for DimensionN<Data> {
    type N = usize;
    fn count(&self, dim: &Vec<usize>) -> usize {
        let mut prod = 1;
        for i in 0..dim.len() {
            prod *= dim[i];
        }
        prod
    }
}

impl<T, U>
Count<Vec<U>> for DimensionN<Of<T>>
    where
        T: Construct + Count<U, N = usize>
{
    type N = usize;
    fn count(&self, dim: &Vec<U>) -> usize {
        let of: T = Construct::new();
        let mut prod = 1;
        for i in 0..dim.len() {
            prod *= of.count(&dim[i]);
        }
        prod
    }
}

impl Zero<Vec<usize>, Vec<usize>> for DimensionN<Data> {
    fn zero(&self, dim: &Vec<usize>) -> Vec<usize> {
        vec![0, dim.len()]
    }
}

impl<T, U, V>
Zero<Vec<U>, Vec<V>>
for DimensionN<Of<T>>
    where T: Construct + Count<U> + ToPos<U, V> + Zero<U, V>
{
    fn zero(&self, dim: &Vec<U>) -> Vec<V> {
        let of: T = Construct::new();
        let mut v = Vec::with_capacity(dim.len());
        for i in 0..dim.len() {
            v.push(of.zero(&dim[i]));
        }
        v
    }
}

impl ToIndex<Vec<usize>, Vec<usize>> for DimensionN<Data> {
    fn to_index(&self, dim: &Vec<usize>, pos: &Vec<usize>) -> usize {
        let mut dim_index = 0;
        for i in (0..dim.len()).rev() {
            dim_index = dim_index * dim[i] + pos[i];
        }
        dim_index
    }
}

impl<T, U, V>
ToIndex<Vec<U>, Vec<V>> for DimensionN<Of<T>>
    where T: Construct + Count<U, N = usize> + ToIndex<U, V>
{
    fn to_index(
        &self,
        dim: &Vec<U>,
        pos: &Vec<V>
    ) -> usize {
        let of: T = Construct::new();
        let mut dim_index = 0;
        for i in (0..dim.len()).rev() {
            dim_index = dim_index * of.count(&dim[i])
                      + of.to_index(&dim[i], &pos[i]);
        }
        dim_index
    }
}

impl ToPos<Vec<usize>, Vec<usize>> for DimensionN<Data> {
    fn to_pos(&self, dim: &Vec<usize>, index: usize, pos: &mut Vec<usize>) {
        unsafe { pos.set_len(0); }
        let mut prod = self.count(dim);
        for _ in 0..dim.len() {
            pos.push(0);
        }
        let mut dim_index = index;
        for i in (0..dim.len()).rev() {
            prod /= dim[i];
            let p_i = dim_index / prod;
            *pos.get_mut(i).unwrap() = p_i;
            dim_index -= p_i * prod;
        }
    }
}

impl<T, U, V>
ToPos<Vec<U>, Vec<V>>
for DimensionN<Of<T>>
    where T: Construct + Count<U, N = usize> + ToPos<U, V>
{
    fn to_pos(
        &self,
        dim: &Vec<U>,
        index: usize,
        pos: &mut Vec<V>
    ) {
        let of: T = Construct::new();
        let mut prod = self.count(dim);
        let mut dim_index = index;
        for (i, p) in pos.iter_mut().enumerate().rev() {
            prod /= of.count(&dim[i]);
            let p_i = dim_index / prod;
            of.to_pos(&dim[i], p_i, p);
            dim_index -= p_i * prod;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn features() {
        is_complete::<DimensionN, Vec<usize>, Vec<usize>>();
        is_complete::<DimensionN<Of<Pair>>, Vec<usize>,
            Vec<(usize, usize)>>();
    }

    #[test]
    fn data() {
        let x: DimensionN = Construct::new();
        let ref dim = vec![3, 3];
        assert_eq!(x.count(dim), 9);
        assert_eq!(x.to_index(dim, &vec![0, 0]), 0);
        assert_eq!(x.to_index(dim, &vec![1, 0]), 1);
        assert_eq!(x.to_index(dim, &vec![0, 1]), 3);
        let mut new_pos = vec![0, 0];
        x.to_pos(dim, 3, &mut new_pos);
        assert_eq!(&new_pos, &[0, 1]);
    }

    #[test]
    fn of() {
        let x: DimensionN<Of<Pair>> = Construct::new();
        let ref dim = vec![3, 4];
        assert_eq!(x.count(dim), 18);
        assert_eq!(x.to_index(dim, &vec![(0, 1), (0, 1)]), 0);
        assert_eq!(x.to_index(dim, &vec![(0, 2), (0, 1)]), 1);
        assert_eq!(x.to_index(dim, &vec![(1, 2), (0, 1)]), 2);
        assert_eq!(x.to_index(dim, &vec![(0, 1), (0, 2)]), 3);
        let mut pos = vec![(0, 0), (0, 0)];
        x.to_pos(dim, 3, &mut pos);
        assert_eq!(pos[0], (0, 1));
        assert_eq!(pos[1], (0, 2));
    }
}
