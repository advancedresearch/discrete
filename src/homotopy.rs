use std::marker::PhantomData;

use num::BigUint;

use Construct;
use Data;
use Count;
use Of;
use ToIndex;
use ToPos;
use Zero;
use EqPair;

/// Stores a higher order point for homotopy spaces.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HPoint<T = usize> {
    /// A point.
    Point(T),
    /// A path between two higher order points.
    Path(Box<(HPoint<T>, HPoint<T>)>),
}

impl<T> HPoint<T> {
    /// Gets the homotopy level of a higher order point for homotopy spaces.
    pub fn level(&self) -> usize {
        use HPoint::*;

        match self {
            Point(_) => 0,
            Path(ab) => ab.0.level().max(ab.1.level()) + 1,
        }
    }
}

/// A discrete space that models undirected homotopy paths
/// at a specified homotopy level.
///
/// A homotopy level is a space that connects pieces of
/// lower homotopy levels.
/// Homotopy level 0 consists of the space of the pieces themselves.
///
/// Dimension is `(<homotopy level>, <number of pieces>)`,
/// position is `HPoint`.
///
/// Uses `EqPair` internally to include loop spaces.
pub struct Homotopy<T = Data>(PhantomData<T>);

impl<T> Construct for Homotopy<T> {
    fn new() -> Homotopy<T> {Homotopy(PhantomData)}
}

impl Count<(usize, usize)> for Homotopy<Data> {
    type N = usize;
    fn count(&self, &(level, n): &(usize, usize)) -> usize {
        let s: EqPair = Construct::new();
        let mut count = n;
        for _ in 0..level {
            count = s.count(&count);
        }
        count
    }
}

impl Count<(usize, BigUint)> for Homotopy<Data> {
    type N = BigUint;
    fn count(&self, (level, n): &(usize, BigUint)) -> BigUint {
        let s: EqPair = Construct::new();
        let mut count = n.clone();
        for _ in 0..*level {
            count = s.count(&count);
        }
        count
    }
}

impl<T, U> Count<(usize, U)> for Homotopy<Of<T>>
    where
        T: Construct + Count<U>,
        Homotopy: Count<(usize, <T as Count<U>>::N), N = <T as Count<U>>::N>
{
    type N = <T as Count<U>>::N;
    fn count(&self, (level, dim): &(usize, U)) -> Self::N {
        let of: T = Construct::new();
        let data: Homotopy<Data> = Construct::new();
        data.count(&(*level, of.count(dim)))
    }
}

impl Zero<(usize, usize), HPoint> for Homotopy<Data> {
    fn zero(&self, &(level, n): &(usize, usize)) -> HPoint {
        use HPoint::*;

        match level {
            0 => Point(0),
            _ => Path(Box::new((self.zero(&(level-1, n)), self.zero(&(level-1, n))))),
        }
    }
}

impl Zero<(usize, BigUint), HPoint<BigUint>> for Homotopy<Data> {
    fn zero(&self, (level, n): &(usize, BigUint)) -> HPoint<BigUint> {
        use HPoint::*;

        match *level {
            0 => Point(0usize.into()),
            _ => Path(Box::new((self.zero(&(level-1, n.clone())), self.zero(&(level-1, n.clone()))))),
        }
    }
}

impl<T, U, V>
Zero<(usize, U), HPoint<V>> for Homotopy<Of<T>>
    where T: Construct + Zero<U, V>, U: Clone
{
    fn zero(&self, &(level, ref dim): &(usize, U)) -> HPoint<V> {
        use HPoint::*;

        match level {
            0 => {
                let of: T = Construct::new();
                Point(of.zero(&dim))
            }
            _ => Path(Box::new((self.zero(&(level-1, dim.clone())),
                                self.zero(&(level-1, dim.clone()))))),
        }
    }
}

impl ToIndex<(usize, usize), HPoint>
for Homotopy<Data> {
    type N = usize;
    fn to_index(&self, &(level, n): &(usize, usize), pos: &HPoint) -> usize {
        use HPoint::*;

        match pos {
            Point(x) => *x,
            Path(ab) => {
                let count = self.count(&(level, n));
                let a = self.to_index(&(level-1, n), &ab.0);
                let b = self.to_index(&(level-1, n), &ab.1);
                let min = a.min(b);
                let max = a.max(b);
                let s: EqPair = Construct::new();
                s.to_index(&count, &(min, max))
            }
        }
    }
}

impl<T, U, V>
ToIndex<(usize, U), HPoint<V>> for Homotopy<Of<T>>
    where T: Construct + ToIndex<U, V, N = usize> + Count<U, N = usize>, U: Clone
{
    type N = usize;
    fn to_index(
        &self,
        dim: &(usize, U),
        pos: &HPoint<V>
    ) -> usize {
        use HPoint::*;

        match pos {
            Point(x) => {
                let of: T = Construct::new();
                of.to_index(&dim.1, x)
            }
            Path(ab) => {
                let count = self.count(dim);
                let dim_n = (dim.0-1, dim.1.clone());
                let a = self.to_index(&dim_n, &ab.0);
                let b = self.to_index(&dim_n, &ab.1);
                let min = a.min(b);
                let max = a.max(b);
                let s: EqPair = Construct::new();
                s.to_index(&count, &(min, max))
            }
        }
    }
}

impl ToPos<(usize, usize), HPoint> for Homotopy<Data> {
    type N = usize;
    fn to_pos(&self, &(level, n): &(usize, usize), index: usize, pos: &mut HPoint) {
        use HPoint::*;

        match level {
            0 => {
                *pos = Point(index);
                return;
            }
            1 => {
                let s: EqPair = Construct::new();
                let mut ab = (0, 0);
                s.to_pos(&n, index, &mut ab);
                *pos = Path(Box::new((Point(ab.0), Point(ab.1))));
                return;
            }
            _ => {
                let count = self.count(&(level, n));
                let s: EqPair = Construct::new();
                let mut ab = (0, 0);
                s.to_pos(&count, index, &mut ab);
                let mut a = Point(0);
                let mut b = Point(0);
                self.to_pos(&(level - 1, n), ab.0, &mut a);
                self.to_pos(&(level - 1, n), ab.1, &mut b);
                *pos = Path(Box::new((a, b)));
                return;
            }
        }
    }
}

impl<T, U, V>
ToPos<(usize, U), HPoint<V>> for Homotopy<Of<T>>
    where T: Construct + Count<U, N = usize> + ToPos<U, V, N = usize> + Zero<U, V>, U: Clone
{
    type N = usize;
    fn to_pos(
        &self,
        &(level, ref dim): &(usize, U),
        index: usize,
        pos: &mut HPoint<V>
    ) {
        use HPoint::*;

        match level {
            0 => {
                let of: T = Construct::new();
                let mut of_pos = of.zero(dim);
                of.to_pos(dim, index, &mut of_pos);
                *pos = Point(of_pos);
                return;
            }
            1 => {
                let of: T = Construct::new();
                let s: EqPair<Of<T>> = Construct::new();
                let mut ab = (of.zero(dim), of.zero(dim));
                s.to_pos(&dim, index, &mut ab);
                *pos = Path(Box::new((Point(ab.0), Point(ab.1))));
                return;
            }
            _ => {
                let of: T = Construct::new();
                let count = self.count(&(level, dim.clone()));
                let s: EqPair = Construct::new();
                let mut ab = (0, 0);
                s.to_pos(&count, index, &mut ab);
                let mut a = Point(of.zero(dim));
                let mut b = Point(of.zero(dim));
                self.to_pos(&(level - 1, dim.clone()), ab.0, &mut a);
                self.to_pos(&(level - 1, dim.clone()), ab.1, &mut b);
                *pos = Path(Box::new((a, b)));
                return;
            }
        }
    }
}
