use std::marker::PhantomData;

use num::BigUint;

use Construct;
use Data;
use Of;
use EqPair;
use space::Space;

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
    fn new() -> Self {Homotopy(PhantomData)}
}

impl Space<usize> for Homotopy<Data> {
    type Dim = (usize, usize);
    type Pos = HPoint;
    fn count(&self, &(level, n): &(usize, usize)) -> usize {
        let s: EqPair = Construct::new();
        let mut count = n;
        for _ in 0..level {
            count = s.count(&count);
        }
        count
    }
    fn zero(&self, &(level, n): &(usize, usize)) -> HPoint {
        use HPoint::*;

        match level {
            0 => Point(0),
            _ => Path(Box::new((
                Space::<usize>::zero(self, &(level-1, n)),
                Space::<usize>::zero(self, &(level-1, n))))),
        }
    }
    fn to_index(&self, &(level, n): &(usize, usize), pos: &HPoint) -> usize {
        use HPoint::*;

        match pos {
            Point(x) => *x,
            Path(ab) => {
                let count = self.count(&(level, n));
                let a: usize = self.to_index(&(level-1, n), &ab.0);
                let b: usize = self.to_index(&(level-1, n), &ab.1);
                let min = a.min(b);
                let max = a.max(b);
                let s: EqPair = Construct::new();
                s.to_index(&count, &(min, max))
            }
        }
    }
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

impl Space<BigUint> for Homotopy<Data> {
    type Dim = (usize, BigUint);
    type Pos = HPoint<BigUint>;
    fn count(&self, (level, n): &Self::Dim) -> BigUint {
        let s: EqPair = Construct::new();
        let mut count = n.clone();
        for _ in 0..*level {
            count = s.count(&count);
        }
        count
    }
    fn zero(&self, (level, n): &Self::Dim) -> Self::Pos {
        use HPoint::*;

        match *level {
            0 => Point(0usize.into()),
            _ => Path(Box::new((
                Space::<BigUint>::zero(self, &(level-1, n.clone())),
                Space::<BigUint>::zero(self, &(level-1, n.clone()))
            ))),
        }
    }
    fn to_index(&self, (level, n): &Self::Dim, pos: &Self::Pos) -> BigUint {
        use HPoint::*;

        let level = *level;
        match pos {
            Point(x) => x.clone(),
            Path(ab) => {
                let count = self.count(&(level, n.clone()));
                let a: BigUint = self.to_index(&(level-1, n.clone()), &ab.0);
                let b: BigUint = self.to_index(&(level-1, n.clone()), &ab.1);
                let min = a.clone().min(b.clone());
                let max = a.max(b);
                let s: EqPair = Construct::new();
                s.to_index(&count, &(min, max))
            }
        }
    }
    fn to_pos(&self, (level, n): &Self::Dim, index: BigUint, pos: &mut Self::Pos) {
        use HPoint::*;

        let level = *level;
        match level {
            0 => {
                *pos = Point(index);
                return;
            }
            1 => {
                let s: EqPair = Construct::new();
                let mut ab = (0usize.into(), 0usize.into());
                s.to_pos(n, index, &mut ab);
                *pos = Path(Box::new((Point(ab.0), Point(ab.1))));
                return;
            }
            _ => {
                let count: BigUint = self.count(&(level, n.clone()));
                let s: EqPair = Construct::new();
                let mut ab = (0usize.into(), 0usize.into());
                s.to_pos(&count, index, &mut ab);
                let mut a: HPoint<BigUint> = Point(0usize.into());
                let mut b: HPoint<BigUint> = Point(0usize.into());
                self.to_pos(&(level - 1, n.clone()), ab.0, &mut a);
                self.to_pos(&(level - 1, n.clone()), ab.1, &mut b);
                *pos = Path(Box::new((a, b)));
                return;
            }
        }
    }
}

impl<N, T> Space<N> for Homotopy<Of<T>>
    where T: Space<N>,
          T::Dim: Clone,
          EqPair<Data>: Space<N, Dim = N, Pos = (N, N)>,
          EqPair<Of<T>>: Space<N, Dim = T::Dim, Pos = (T::Pos, T::Pos)>,
          N: Clone +
             Ord +
             From<usize>,
          Homotopy<Data>: Space<N, Dim = (usize, N), Pos = HPoint<N>>,
{
    type Dim = (usize, T::Dim);
    type Pos = HPoint<T::Pos>;
    fn count(&self, (level, dim): &Self::Dim) -> N {
        let of: T = Construct::new();
        let data: Homotopy<Data> = Construct::new();
        data.count(&(*level, of.count(dim)))
    }
    fn zero(&self, (level, dim): &Self::Dim) -> Self::Pos {
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
    fn to_index(
        &self,
        dim: &Self::Dim,
        pos: &Self::Pos,
    ) -> N {
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
                let min = a.clone().min(b.clone());
                let max = a.max(b);
                let s: EqPair = Construct::new();
                s.to_index(&count, &(min, max))
            }
        }
    }
    fn to_pos(
        &self,
        &(level, ref dim): &Self::Dim,
        index: N,
        pos: &mut Self::Pos,
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
                let mut ab = (0usize.into(), 0usize.into());
                s.to_pos(&count, index, &mut ab);
                let mut a: HPoint<T::Pos> = Point(of.zero(dim));
                let mut b: HPoint<T::Pos> = Point(of.zero(dim));
                self.to_pos(&(level - 1, dim.clone()), ab.0, &mut a);
                self.to_pos(&(level - 1, dim.clone()), ab.1, &mut b);
                *pos = Path(Box::new((a, b)));
                return;
            }
        }
    }
}
