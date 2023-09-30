use std::marker::PhantomData;

use std::ops::{Add, Sub};

use Construct;
use space::Space;

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

impl<T, U> Construct for Either<T, U> {
    fn new() -> Self { Either(PhantomData, PhantomData) }
}

impl<T, U, N> Space<N> for Either<T, U>
    where T: Space<N>,
          U: Space<N>,
          N: Add<Output = N> +
             Sub<Output = N> +
             PartialOrd,
{
    type Dim = (T::Dim, U::Dim);
    type Pos = Select<T::Pos, U::Pos>;
    fn count(&self, &(ref dim_t, ref dim_u): &Self::Dim) -> N {
        let t: T = Construct::new();
        let u: U = Construct::new();
        t.count(dim_t) + u.count(dim_u)
    }
    fn zero(&self, &(ref dim_t, _): &Self::Dim) -> Self::Pos {
        let t: T = Construct::new();
        Select::Fst(t.zero(dim_t))
    }
    fn to_index(
        &self,
        &(ref dim_t, ref dim_u): &Self::Dim,
        s: &Self::Pos
    ) -> N {
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
    fn to_pos(
        &self,
        &(ref dim_t, ref dim_u): &Self::Dim,
        ind: N,
        pos: &mut Self::Pos,
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
