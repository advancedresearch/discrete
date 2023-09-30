//! Implements traits for tuples such that subspaces can be constructed.

use std::ops::{Mul, Div, Sub, Add};

use Construct;
use space::Space;

impl<T, U> Construct for (T, U)
    where T: Construct, U: Construct,
{
    fn new() -> Self { (Construct::new(), Construct::new()) }
}

impl<N, T, U> Space<N> for (T, U)
    where T: Space<N>,
          U: Space<N>,
          N: Add<N, Output = N>,
          for<'a> &'a N: Mul<&'a N, Output = N> +
                         Div<&'a N, Output = N> +
                         Sub<&'a N, Output = N>,
{
    type Dim = (T::Dim, U::Dim);
    type Pos = (T::Pos, U::Pos);
    fn count(&self, &(ref dim_t, ref dim_u): &Self::Dim) -> N {
        let t: T = Construct::new();
        let u: U = Construct::new();
        &t.count(dim_t) * &u.count(dim_u)
    }
    fn zero(&self, &(ref dim_t, ref dim_u): &Self::Dim) -> Self::Pos {
        let t: T = Construct::new();
        let u: U = Construct::new();
        (t.zero(dim_t), u.zero(dim_u))
    }
    fn to_index(
        &self,
        &(ref dim_t, ref dim_u): &Self::Dim,
        &(ref pt, ref pu): &Self::Pos,
    ) -> N {
        let t: T = Construct::new();
        let u: U = Construct::new();
        let count = u.count(dim_u);
        &t.to_index(dim_t, pt) * &count + u.to_index(dim_u, pu)
    }
    fn to_pos(
        &self,
        &(ref dim_t, ref dim_u): &Self::Dim,
        ind: N,
        &mut (ref mut pt, ref mut pu): &mut Self::Pos,
    ) {
        let t: T = Construct::new();
        let u: U = Construct::new();
        let count = u.count(dim_u);
        let x = &ind / &count;
        u.to_pos(dim_u, &ind - &(&x * &count), pu);
        t.to_pos(dim_t, x, pt);
    }
}
