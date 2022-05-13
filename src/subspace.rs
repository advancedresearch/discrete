//! Implements traits for tuples such that subspaces can be constructed.

use std::ops::Mul;

use Construct;
use Count;
use ToIndex;
use ToPos;
use Zero;

impl<T, U> Construct for (T, U)
    where T: Construct,
          U: Construct
{
    fn new() -> (T, U) {
        (Construct::new(), Construct::new())
    }
}

impl<T, U, V, W> Count<(V, W)> for (T, U)
    where T: Construct + Count<V>,
          U: Construct + Count<W, N = <T as Count<V>>::N>,
          <T as Count<V>>::N: Mul<Output = <T as Count<V>>::N>
{
    type N = <T as Count<V>>::N;
    fn count(&self, &(ref dim_t, ref dim_u): &(V, W)) -> Self::N {
        let t: T = Construct::new();
        let u: U = Construct::new();
        t.count(dim_t) * u.count(dim_u)
    }
}

impl<T, U, V, W, X, Y> Zero<(V, W), (X, Y)> for (T, U)
    where T: Construct + Zero<V, X>,
          U: Construct + Zero<W, Y>
{
    fn zero(&self, &(ref dim_t, ref dim_u): &(V, W)) -> (X, Y) {
        let t: T = Construct::new();
        let u: U = Construct::new();
        (t.zero(dim_t), u.zero(dim_u))
    }
}

impl<T, U, V, W, X, Y> ToIndex<(V, W), (X, Y)> for (T, U)
    where T: Construct + ToIndex<V, X>,
          U: Construct + Count<W, N = usize> + ToIndex<W, Y>
{
    fn to_index(
        &self,
        &(ref dim_t, ref dim_u): &(V, W),
        &(ref pt, ref pu): &(X, Y)
    ) -> usize {
        let t: T = Construct::new();
        let u: U = Construct::new();
        let count = u.count(dim_u);
        t.to_index(dim_t, pt) * count + u.to_index(dim_u, pu)
    }
}

impl<T, U, V, W, X, Y> ToPos<(V, W), (X, Y)> for (T, U)
    where T: Construct + ToPos<V, X>,
          U: Construct + Count<W, N = usize> + ToPos<W, Y>
{
    fn to_pos(
        &self,
        &(ref dim_t, ref dim_u): &(V, W),
        ind: usize,
        &mut (ref mut pt, ref mut pu): &mut (X, Y)
    ) {
        let t: T = Construct::new();
        let u: U = Construct::new();
        let count = u.count(dim_u);
        let x = ind / count;
        t.to_pos(dim_t, x, pt);
        u.to_pos(dim_u, ind - x * count, pu);
    }
}
