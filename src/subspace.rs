use std::marker::PhantomData;

use Construct;
use Count;
use ToIndex;
use ToPos;
use Zero;

/// Used to nest a subspace.
pub struct Subspace<T>(PhantomData<T>);

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
          U: Construct + Count<W>
{
    fn count(&self, (dim_t, dim_u): (V, W)) -> usize {
        let t: T = Construct::new();
        let u: U = Construct::new();
        t.count(dim_t) * u.count(dim_u)
    }
}

impl<T, U, V, W, X, Y> Zero<(V, W), (X, Y)> for (T, U)
    where T: Construct + Zero<V, X>,
          U: Construct + Zero<W, Y>
{
    fn zero(&self, (dim_t, dim_u): (V, W)) -> (X, Y) {
        let t: T = Construct::new();
        let u: U = Construct::new();
        (t.zero(dim_t), u.zero(dim_u))
    }
}

impl<T, U, V, W, X, Y> ToIndex<(V, W), (X, Y)> for (T, U)
    where T: Construct + ToIndex<V, X>,
          U: Construct + Count<W> + ToIndex<W, Y>,
          W: Copy
{
    fn to_index(
        &self,
        (dim_t, dim_u): (V, W),
        (pt, pu): (X, Y)
    ) -> usize {
        let t: T = Construct::new();
        let u: U = Construct::new();
        let count = u.count(dim_u);
        t.to_index(dim_t, pt) * count + u.to_index(dim_u, pu)
    }
}

impl<T, U, V, W, X, Y> ToPos<(V, W), (X, Y)> for (T, U)
    where T: Construct + ToPos<V, X>,
          U: Construct + Count<W> + ToPos<W, Y>,
          W: Copy
{
    fn to_pos(
        &self,
        (dim_t, dim_u): (V, W),
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
