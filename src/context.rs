
use std::marker::PhantomData;

use Construct;
use Data;
use Count;
use Of;
use ToIndex;
use ToPos;
use Zero;

/// A discrete space that can model spatial operations over arbitrary states,
/// therefore useful for context analysis.
///
/// It can be constructed by taking a N-dimensional space,
/// for each dimension create a pair subspace and sum over the dimensions.
/// It can also be thought of as the edges in an undirected graph,
/// where each node is described by a N-dimensional coordinate,
/// and all nodes are connected which differ by one axis.
///
/// Dimensions of size 2 gives the edges on a hypercube of the number of dimensions.
/// For example, `[2, 2, 2]` gives edges on a cube in 3 dimensions.
///
/// The position is a tuple `(Vec<usize>, usize, usize)`,
/// where the first component describes the node coordinates,
/// the second component describes the index of the coordinate that changes,
/// and the third component describes the new value.
pub struct Context<T = Data>(PhantomData<T>);

/// Computes subspace offset from which index that changes.
/// The space is divided into N subspaces,
/// because only one axis can change at a time.
///
/// ```ignore
/// [(a, x), b, c]
/// [a, (b, x), c]
/// [a, b, (c, x)]
/// ```
fn subspace_offset(v: &[usize], ind: usize) -> usize {
    use Pair;

    let pair: Pair<Data> = Construct::new();
    let mut sum = 0;
    for i in 0..ind {
        let mut prod = 1;
        for j in 0..v.len() {
            if i == j { continue; }
            prod *= v[j];
        }
        sum += pair.count(&v[i]) * prod;
    }
    sum
}

/// Computes the index of the axis that changes from index position.
/// This works because the layout are separated by which
/// axis that changes, and the subspace offset can be computed.
/// Returns `(ind, offset)`
fn ind_from_index(v: &[usize], index: usize) -> (usize, usize) {
    use Pair;

    let pair: Pair<Data> = Construct::new();
    let mut sum = 0;
    for i in 0..v.len() {
        let mut prod = 1;
        for j in 0..v.len() {
            if i == j { continue; }
            prod *= v[j];
        }
        let add = pair.count(&v[i]) * prod;
        if sum + add > index { return (i, sum); }
        sum += add;
    }
    (v.len(), sum)
}

impl<T> Construct for Context<T> {
    fn new() -> Context<T> { Context(PhantomData) }
}

impl Count<Vec<usize>> for Context<Data> {
    fn count(&self, dim: &Vec<usize>) -> usize {
        use Pair;

        let pair: Pair<Data> = Construct::new();
        let mut sum = pair.count(&dim[0]);
        let mut prod = dim[0];
        for d in &dim[1..] {
            sum = d * sum + pair.count(d) * prod;
            prod *= *d;
        }
        sum
    }
}

impl<T, U> Count<Vec<U>> for Context<Of<T>>
    where
        T: Construct + Count<U>
{
    fn count(&self, dim: &Vec<U>) -> usize {
        use Pair;

        let of: T = Construct::new();
        let pair: Pair<Data> = Construct::new();
        let mut sum = pair.count(&of.count(&dim[0]));
        let mut prod = of.count(&dim[0]);
        for d in &dim[1..] {
            let d = of.count(d);
            sum = d * sum + pair.count(&d) * prod;
            prod *= d;
        }
        sum
    }
}

impl Zero<Vec<usize>, (Vec<usize>, usize, usize)> for Context<Data> {
    fn zero(&self, dim: &Vec<usize>) -> (Vec<usize>, usize, usize) {
        (vec![0; dim.len()], 0, 0)
    }
}

impl<T, U, V>
Zero<Vec<U>, (Vec<V>, usize, V)>
for Context<Of<T>>
    where
        T: Construct + Count<U> + ToPos<U, V> + Zero<U, V>
{
    fn zero(&self, dim: &Vec<U>) -> (Vec<V>, usize, V) {
        let of: T = Construct::new();
        let mut v = Vec::with_capacity(dim.len());
        for i in 0..dim.len() {
            v.push(of.zero(&dim[i]));
        }
        (v, 0, of.zero(&dim[0]))
    }
}

impl ToIndex<Vec<usize>, (Vec<usize>, usize, usize)> for Context<Data> {
    fn to_index(
        &self,
        dim: &Vec<usize>,
        &(ref p, ind, b): &(Vec<usize>, usize, usize)
    ) -> usize {
        use std::cmp::{ min, max };
        use Pair;

        let offset = subspace_offset(dim, ind);
        let pair: Pair<Data> = Construct::new();
        let mut prod = 1;
        for j in 0..dim.len() {
            if ind == j { continue; }
            prod *= dim[j];
        }
        // Pair doesn't care about dimension.
        let single = pair.to_index(&0, &(min(p[ind], b), max(p[ind], b)));
        let pos_offset = single * prod;
        let mut dim_index = 0;
        for i in (0..p.len()).rev() {
            if ind == i { continue; }
            dim_index = dim_index * dim[i] + p[i];
        }
        offset + pos_offset + dim_index
    }
}

impl<T, U, V> ToIndex<Vec<U>, (Vec<V>, usize, V)> for Context<Of<T>>
    where
        T: Construct + Count<U> + ToIndex<U, V>
{
    fn to_index(
        &self,
        dim: &Vec<U>,
        &(ref p, ind, ref b): &(Vec<V>, usize, V)
    ) -> usize {
        fn subspace_offset<T, U>(v: &[U], ind: usize) -> usize
            where T: Construct + Count<U>
        {
            use Pair;

            let of: T = Construct::new();
            let pair: Pair<Data> = Construct::new();
            let mut sum = 0;
            for i in 0..ind {
                let mut prod = 1;
                for j in 0..v.len() {
                    if i == j { continue; }
                    prod *= of.count(&v[j]);
                }
                sum += pair.count(&of.count(&v[i])) * prod;
            }
            sum
        }

        use std::cmp::{ min, max };
        use Pair;

        let of: T = Construct::new();
        let offset = subspace_offset::<T, U>(dim, ind);
        let pair: Pair<Data> = Construct::new();
        let mut prod = 1;
        for j in 0..dim.len() {
            if ind == j { continue; }
            prod *= of.count(&dim[j]);
        }
        // Pair doesn't care about dimension.
        let single = pair.to_index(&0,
            &(min(of.to_index(&dim[ind], &p[ind]), of.to_index(&dim[ind], b)),
             max(of.to_index(&dim[ind], &p[ind]), of.to_index(&dim[ind], b))));
        let pos_offset = single * prod;
        let mut dim_index = 0;
        for i in (0..p.len()).rev() {
            if ind == i { continue; }
            dim_index = dim_index * of.count(&dim[i]) + of.to_index(&dim[i], &p[i]);
        }
        offset + pos_offset + dim_index
    }
}

impl ToPos<Vec<usize>, (Vec<usize>, usize, usize)> for Context<Data> {
    fn to_pos(
        &self,
        dim: &Vec<usize>,
        index: usize,
        &mut (ref mut p, ref mut ind, ref mut b): &mut (Vec<usize>, usize, usize)
    ) {
        use Pair;

        p.clear();
        let pair_space: Pair<Data> = Construct::new();
        let (ind_val, offset) = ind_from_index(dim, index);
        // Get rid of offset.
        // The rest equals: single * prod + dim_index
        let index = index - offset;
        let mut prod = 1;
        for j in 0..dim.len() {
            p.push(0); // zero position
            if ind_val == j { continue; }
            prod *= dim[j];
        }
        let single = index / prod;

        let mut pair = (0, 0);
        // Pair doesn't care about dimension.
        pair_space.to_pos(&0, single, &mut pair);
        let (min, max) = pair;

        // Resolve other dimension components.
        let mut dim_index = index - single * prod;
        for i in (0..p.len()).rev() {
            if ind_val == i { continue; }
            prod /= dim[i];
            let p_i = dim_index / prod;
            p[i] = p_i;
            dim_index -= p_i * prod;
        }
        p[ind_val] = min;
        *b = max;
        *ind = ind_val;
    }
}

impl<T, U, V>
ToPos<Vec<U>, (Vec<V>, usize, V)>
for Context<Of<T>>
    where
        T: Construct + Count<U> + ToPos<U, V> + Zero<U, V>
{
    fn to_pos(
        &self,
        dim: &Vec<U>,
        index: usize,
        &mut (ref mut p, ref mut ind, ref mut b): &mut (Vec<V>, usize, V)
    ) {
        fn ind_from_index<T, U>(v: &[U], index: usize) -> (usize, usize)
            where T: Construct + Count<U>
        {
            use Pair;

            let of: T = Construct::new();
            let pair: Pair<Data> = Construct::new();
            let mut sum = 0;
            for i in 0..v.len() {
                let mut prod = 1;
                for j in 0..v.len() {
                    if i == j { continue; }
                    prod *= of.count(&v[j]);
                }
                let add = pair.count(&of.count(&v[i])) * prod;
                if sum + add > index { return (i, sum); }
                sum += add;
            }
            (v.len(), sum)
        }

        use Pair;

        let of: T = Construct::new();
        p.clear();
        let pair_space: Pair<Data> = Construct::new();
        let (ind_val, offset) = ind_from_index::<T, U>(dim, index);
        // Get rid of offset.
        // The rest equals: single * prod + dim_index
        let index = index - offset;
        let mut prod = 1;
        for j in 0..dim.len() {
            p.push(of.zero(&dim[j])); // zero position
            if ind_val == j { continue; }
            prod *= of.count(&dim[j]);
        }
        let single = index / prod;

        let mut pair = (0, 0);
        // Pair doesn't care about dimension.
        pair_space.to_pos(&0, single, &mut pair);
        let (min, max) = pair;

        // Resolve other dimension components.
        let mut dim_index = index - single * prod;
        for i in (0..p.len()).rev() {
            if ind_val == i { continue; }
            prod /= of.count(&dim[i]);
            let p_i = dim_index / prod;
            of.to_pos(&dim[i], p_i, &mut p[i]);
            dim_index -= p_i * prod;
        }
        of.to_pos(&dim[ind_val], min, &mut p[ind_val]);
        of.to_pos(&dim[ind_val], max, b);
        *ind = ind_val;
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn features() {
        is_complete::<Context, Vec<usize>, (Vec<usize>, usize, usize)>();
        is_complete::<Context<Of<Pair>>, Vec<usize>,
            (Vec<(usize, usize)>, usize, (usize, usize))>();
    }

    #[test]
    fn data() {
        let x: Context = Construct::new();
        let ref dim = vec![2, 2, 2];
        // 12 edges on a cube
        assert_eq!(x.count(dim), 12);
        assert_eq!(x.to_index(dim, &(vec![0, 0, 0], 0, 1)), 0);
        for i in 0..x.count(dim) {
            let mut pos = (vec![], 0, 0);
            x.to_pos(dim, i, &mut pos);
            assert_eq!(x.to_index(dim, &pos), i);
        }
    }

    #[test]
    fn of() {
        let x: Context<Of<Pair>> = Construct::new();
        let ref dim = vec![3];
        assert_eq!(x.count(dim), 3);
        assert_eq!(x.to_index(dim, &(vec![(0, 1)], 0, (0, 2))), 0);
        assert_eq!(x.to_index(dim, &(vec![(0, 1)], 0, (1, 2))), 1);
        assert_eq!(x.to_index(dim, &(vec![(0, 2)], 0, (1, 2))), 2);
        let ref dim = vec![3, 3];
        assert_eq!(x.count(dim), 18);
        assert_eq!(x.to_index(dim, &(vec![(0, 1), (0, 1)], 0, (0, 2))), 0);
        assert_eq!(x.to_index(dim, &(vec![(0, 1), (0, 2)], 0, (0, 2))), 1);
        assert_eq!(x.to_index(dim, &(vec![(0, 1), (1, 2)], 0, (0, 2))), 2);
        assert_eq!(x.to_index(dim, &(vec![(0, 1), (0, 1)], 0, (1, 2))), 3);
        assert_eq!(x.to_index(dim, &(vec![(0, 1), (0, 2)], 0, (1, 2))), 4);
        assert_eq!(x.to_index(dim, &(vec![(0, 1), (1, 2)], 0, (1, 2))), 5);
        assert_eq!(x.to_index(dim, &(vec![(0, 2), (0, 1)], 0, (1, 2))), 6);
        assert_eq!(x.to_index(dim, &(vec![(0, 2), (0, 2)], 0, (1, 2))), 7);
        assert_eq!(x.to_index(dim, &(vec![(0, 2), (1, 2)], 0, (1, 2))), 8);
        assert_eq!(x.to_index(dim, &(vec![(0, 1), (0, 1)], 1, (0, 2))), 9);
        assert_eq!(x.to_index(dim, &(vec![(0, 2), (0, 1)], 1, (0, 2))), 10);
        assert_eq!(x.to_index(dim, &(vec![(1, 2), (0, 1)], 1, (0, 2))), 11);
        assert_eq!(x.to_index(dim, &(vec![(0, 1), (0, 1)], 1, (1, 2))), 12);
        assert_eq!(x.to_index(dim, &(vec![(0, 2), (0, 1)], 1, (1, 2))), 13);
        assert_eq!(x.to_index(dim, &(vec![(1, 2), (0, 1)], 1, (1, 2))), 14);
        assert_eq!(x.to_index(dim, &(vec![(0, 1), (0, 2)], 1, (1, 2))), 15);
        assert_eq!(x.to_index(dim, &(vec![(0, 2), (0, 2)], 1, (1, 2))), 16);
        assert_eq!(x.to_index(dim, &(vec![(1, 2), (0, 2)], 1, (1, 2))), 17);

        let mut pos = (vec![(0, 0), (0, 0)], 0, (0, 0));
        x.to_pos(dim, 16, &mut pos);
        assert_eq!(pos, ((vec![(0, 2), (0, 2)], 1, (1, 2))));
    }
}
