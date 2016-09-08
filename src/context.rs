
use std::marker::PhantomData;

use Construct;
use Data;
use Count;
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
        sum += pair.count(v[i]) * prod;
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
        let add = pair.count(v[i]) * prod;
        if sum + add > index { return (i, sum); }
        sum += add;
    }
    (v.len(), sum)
}

impl<T> Construct for Context<T> {
    fn new() -> Context<T> { Context(PhantomData) }
}

impl<'a> Count<&'a [usize]> for Context<Data> {
    fn count(&self, dim: &'a [usize]) -> usize {
        use Pair;

        let pair: Pair<Data> = Construct::new();
        let mut sum = pair.count(dim[0]);
        let mut prod = dim[0];
        for &d in &dim[1..] {
            sum = d * sum + pair.count(d) * prod;
            prod *= d;
        }
        sum
    }
}

impl<'a> Zero<&'a [usize], (Vec<usize>, usize, usize)> for Context<Data> {
    fn zero(&self, dim: &'a [usize]) -> (Vec<usize>, usize, usize) {
        (vec![0; dim.len()], 0, 0)
    }
}

impl<'a> ToIndex<&'a [usize], (&'a [usize], usize, usize)> for Context<Data> {
    fn to_index(&self, dim: &'a [usize], (p, ind, b): (&'a [usize], usize, usize)) -> usize {
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
        let single = pair.to_index(0, (min(p[ind], b), max(p[ind], b)));
        let pos_offset = single * prod;
        let mut dim_index = 0;
        for i in (0..p.len()).rev() {
            if ind == i { continue; }
            dim_index = dim_index * dim[i] + p[i];
        }
        offset + pos_offset + dim_index
    }
}

impl<'a> ToPos<&'a [usize], (Vec<usize>, usize, usize)> for Context<Data> {
    fn to_pos(
        &self,
        dim: &'a [usize],
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
        pair_space.to_pos(0, single, &mut pair);
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

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn features() {
        does_count::<Context, &[usize]>();
    }

    #[test]
    fn data() {
        let x: Context = Construct::new();
        let dim = &[2, 2, 2];
        // 12 edges on a cube
        assert_eq!(x.count(dim), 12);
        assert_eq!(x.to_index(dim, (&[0, 0, 0], 0, 1)), 0);
        for i in 0..x.count(dim) {
            let mut pos = (vec![], 0, 0);
            x.to_pos(dim, i, &mut pos);
            assert_eq!(x.to_index(dim, (&pos.0, pos.1, pos.2)), i);
        }
    }
}
