
use std::marker::PhantomData;

use Construct;
use Data;
use Count;
use Of;
use ToIndex;
use ToPos;
use Zero;

/// Same as `Context`, but for directed edges.
pub struct DirectedContext<T = Data>(PhantomData<T>);

impl<T> Construct for DirectedContext<T> {
    fn new() -> DirectedContext<T> { DirectedContext(PhantomData) }
}

impl<'a> Count<&'a [usize]> for DirectedContext<Data> {
    fn count(&self, dim: &'a [usize]) -> usize {
        use NeqPair;

        let pair: NeqPair<Data> = Construct::new();
        let mut sum = pair.count(dim[0]);
        let mut prod = dim[0];
        for &d in &dim[1..] {
            sum = d * sum + pair.count(d) * prod;
            prod *= d;
        }
        sum
    }
}

impl<'a, T, U> Count<&'a [U]> for DirectedContext<Of<T>>
    where
        T: Construct + Count<U>,
        U: Copy
{
    fn count(&self, dim: &'a [U]) -> usize {
        use NeqPair;

        let of: T = Construct::new();
        let pair: NeqPair<Data> = Construct::new();
        let mut sum = pair.count(of.count(dim[0]));
        let mut prod = of.count(dim[0]);
        for &d in &dim[1..] {
            let d = of.count(d);
            sum = d * sum + pair.count(d) * prod;
            prod *= d;
        }
        sum
    }
}

impl<'a> Zero<&'a [usize], (Vec<usize>, usize, usize)> for DirectedContext<Data> {
    fn zero(&self, dim: &'a [usize]) -> (Vec<usize>, usize, usize) {
        (vec![0; dim.len()], 0, 0)
    }
}

impl<'a, T, U, V>
Zero<&'a [U], (Vec<V>, usize, V)>
for DirectedContext<Of<T>>
    where
        T: Construct + Count<U> + ToPos<U, V> + Zero<U, V>,
        U: Copy
{
    fn zero(&self, dim: &'a [U]) -> (Vec<V>, usize, V) {
        let of: T = Construct::new();
        let mut v = Vec::with_capacity(dim.len());
        for i in 0..dim.len() {
            v.push(of.zero(dim[i]));
        }
        (v, 0, of.zero(dim[0]))
    }
}

impl<'a> ToIndex<&'a [usize], (Vec<usize>, usize, usize)> for DirectedContext<Data> {
    fn to_index(
        &self, dim: &'a [usize],
        &(ref p, ind, b): &(Vec<usize>, usize, usize)
    ) -> usize {
        use Context;

        let context: Context<Data> = Construct::new();
        let index = context.to_index(dim, &(p.clone(), ind, b));
        if p[ind] > b {
            2 * index + 1
        } else {
            2 * index
        }
    }
}

impl<'a, T, U, V> ToIndex<&'a [U], (Vec<V>, usize, V)>
for DirectedContext<Of<T>>
    where
        T: Construct + Count<U> + ToIndex<U, V>,
        U: Copy,
        V: Clone
{
    fn to_index(
        &self,
        dim: &'a [U],
        &(ref p, ind, ref b): &(Vec<V>, usize, V)
    ) -> usize {
        use Context;

        let of: T = Construct::new();
        let context: Context<Of<T>> = Construct::new();
        let index = context.to_index(dim, &(p.clone(), ind, b.clone()));
        if of.to_index(dim[ind], &p[ind]) > of.to_index(dim[ind], b) {
            2 * index + 1
        } else {
            2 * index
        }
    }
}

impl<'a> ToPos<&'a [usize], (Vec<usize>, usize, usize)> for DirectedContext<Data> {
    fn to_pos(
        &self,
        dim: &'a [usize],
        index: usize,
        pos: &mut (Vec<usize>, usize, usize)
    ) {
        use Context;

        let context: Context<Data> = Construct::new();
        if index % 2 == 0 {
            context.to_pos(dim, index / 2, pos);
        } else {
            context.to_pos(dim, (index - 1) / 2, pos);
            let tmp = pos.0[pos.1];
            pos.0[pos.1] = pos.2;
            pos.2 = tmp;
        }
    }
}

impl<'a, T, U, V>
ToPos<&'a [U], (Vec<V>, usize, V)>
for DirectedContext<Of<T>>
    where
        T: Construct + Count<U> + ToPos<U, V> + Zero<U, V>,
        U: Copy,
        V: Copy
{
    fn to_pos(
        &self,
        dim: &'a [U],
        index: usize,
        pos: &mut (Vec<V>, usize, V)
    ) {
        use Context;

        let context: Context<Of<T>> = Construct::new();
        if index % 2 == 0 {
            context.to_pos(dim, index / 2, pos);
        } else {
            context.to_pos(dim, (index - 1) / 2, pos);
            let tmp = pos.0[pos.1];
            pos.0[pos.1] = pos.2;
            pos.2 = tmp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn features() {
        is_complete::<DirectedContext, &[usize], (Vec<usize>, usize, usize)>();
        is_complete::<DirectedContext<Of<Pair>>, &[usize],
            (Vec<(usize, usize)>, usize, (usize, usize))>();
    }

    #[test]
    fn data() {
        let x: DirectedContext = Construct::new();
        let dim = &[2, 2, 2];
        // 12 edges on a cube * 2 = 24 directed edges
        assert_eq!(x.count(dim), 24);
        assert_eq!(x.to_index(dim, &(vec![0, 0, 0], 0, 1)), 0);
        assert_eq!(x.to_index(dim, &(vec![1, 0, 0], 0, 0)), 1);
        for i in 0..x.count(dim) {
            let mut pos = (vec![], 0, 0);
            x.to_pos(dim, i, &mut pos);
            println!("{:?}", pos);
            assert_eq!(x.to_index(dim, &pos), i);
        }
        // assert!(false);
    }
}
