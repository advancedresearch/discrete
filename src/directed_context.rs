
use std::marker::PhantomData;

use Construct;
use Data;
use Count;
use ToIndex;
use ToPos;

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

impl<'a> ToIndex<&'a [usize], (&'a [usize], usize, usize)> for DirectedContext<Data> {
    fn to_index(&self, dim: &'a [usize], (p, ind, b): (&'a [usize], usize, usize)) -> usize {
        use Context;

        let context: Context<Data> = Construct::new();
        let index = context.to_index(dim, (p, ind, b));
        if p[ind] > b {
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

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn data() {
        let x: DirectedContext = Construct::new();
        let dim = &[2, 2, 2];
        // 12 edges on a cube * 2 = 24 directed edges
        assert_eq!(x.count(dim), 24);
        assert_eq!(x.to_index(dim, (&[0, 0, 0], 0, 1)), 0);
        assert_eq!(x.to_index(dim, (&[1, 0, 0], 0, 0)), 1);
        for i in 0..x.count(dim) {
            let mut pos = (vec![], 0, 0);
            x.to_pos(dim, i, &mut pos);
            println!("{:?}", pos);
            assert_eq!(x.to_index(dim, (&pos.0, pos.1, pos.2)), i);
        }
        // assert!(false);
    }
}
