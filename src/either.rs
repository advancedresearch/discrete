use std::marker::PhantomData;

use std::ops::Add;

use Construct;
use Count;
use ToIndex;
use ToPos;
use Zero;

/// Selects between two spaces.
pub struct Either<T, U>(PhantomData<T>, PhantomData<U>);

/// Selects between spaces.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Select<T, U> {
    /// The first space.
    Fst(T),
    /// The second space.
    Snd(U),
}

impl<T, U> Construct for Either<T, U>
    where T: Construct,
          U: Construct
{
    fn new() -> Either<T, U> {
        Either(PhantomData, PhantomData)
    }
}

impl<T, U, V, W> Count<(V, W)> for Either<T, U>
    where T: Construct + Count<V>,
          U: Construct + Count<W, N = <T as Count<V>>::N>,
          <T as Count<V>>::N: Add<Output = <T as Count<V>>::N>
{
    type N = <T as Count<V>>::N;
    fn count(&self, &(ref dim_t, ref dim_u): &(V, W)) -> Self::N {
        let t: T = Construct::new();
        let u: U = Construct::new();
        t.count(dim_t) + u.count(dim_u)
    }
}

impl<T, U, V, W, X, Y> Zero<(V, W), Select<X, Y>> for Either<T, U>
    where T: Construct + Zero<V, X>
{
    fn zero(&self, &(ref dim_t, _): &(V, W)) -> Select<X, Y> {
        let t: T = Construct::new();
        Select::Fst(t.zero(dim_t))
    }
}

impl<T, U, V, W, X, Y> ToIndex<(V, W), Select<X, Y>> for Either<T, U>
    where T: Construct + Count<V, N = usize> + ToIndex<V, X>,
          U: Construct + ToIndex<W, Y>
{
    fn to_index(
        &self,
        &(ref dim_t, ref dim_u): &(V, W),
        s: &Select<X, Y>
    ) -> usize {
        let t: T = Construct::new();
        let u: U = Construct::new();
        match *s {
            Select::Fst(ref pt) => {
                t.to_index(dim_t, pt)
            }
            Select::Snd(ref pu) => {
                let count = t.count(dim_t);
                count + u.to_index(dim_u, pu)
            }
        }
    }
}

impl<T, U, V, W, X, Y> ToPos<(V, W), Select<X, Y>> for Either<T, U>
    where T: Construct + Count<V, N = usize> + ToPos<V, X> + Zero<V, X>,
          U: Construct + ToPos<W, Y> + Zero<W, Y>
{
    fn to_pos(
        &self,
        &(ref dim_t, ref dim_u): &(V, W),
        ind: usize,
        pos: &mut Select<X, Y>
    ) {
        let t: T = Construct::new();
        let u: U = Construct::new();
        let count = t.count(dim_t);
        if ind < count {
            let mut zero = t.zero(dim_t);
            t.to_pos(dim_t, ind, &mut zero);
            *pos = Select::Fst(zero)
        } else {
            let mut zero = u.zero(dim_u);
            u.to_pos(dim_u, ind - count, &mut zero);
            *pos = Select::Snd(zero)
        }
    }
}
