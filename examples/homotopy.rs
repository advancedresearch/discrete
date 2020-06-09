/*

This example demonstrates the usage of a discrete homotopy space.

In homotopy theory, one constructs a continuous map between functions.
There can be many different kinds of continuous maps, so spaces get very complex.
It even gets worse by having continuous maps between continuous maps.
Since there is a combinatorial explosion of possibilities,
mathematicians developed techniques to classify these spaces.
One of these techniques is called a "homotopy level".

Here is an introduction by Vladimir Voedvodsky: https://www.youtube.com/watch?v=E3steS2Hr1Y

A homotopy level is a way of connecting pieces of a space of a lower homotopy level.
At homotopy level 0, the pieces of the space itself are constructed.

In a space consisting of a single element,
there exists only one way of connecting it to iself,
therefore at homotopy level 1, there is just one element.
This element represents the path from element in homotopy level 0 to itself.
At homotopy level 2, the paths in homotopy level 1 are connected.
Since there is only one path, there is only one path between paths at level 2.

However, when you start with 2 elements at homotopy level 0,
there are 3 ways to connect the 2 elements in homotopy level 1:

    0 ~= 0      0 ~= 1      1 ~= 1

Here is an overview of the first 6 homotopy levels up to `n=4`:

    level   n=0     n=1     n=2     n=3         n=4
    0       0       1       2       3           4
    1       0       1       3       6           10
    2       0       1       6       21          55
    3       0       1       21      231         1540
    4       0       1       231     26796       1186570
    5       0       1       26796   359026206   703974775735

*/

extern crate discrete;

use discrete::*;
use HPoint::*;

fn main() {
    println!("A discrete homotopy space uses `EqPair` internally:");
    let s: EqPair = Construct::new();
    let dim = 4;
    let n = s.count(&dim);
    println!("{}", n);
    let mut pos = (0, 0);
    for x in 0..n {
        s.to_pos(&n, x, &mut pos);
        println!("{:?}", pos);
    }
    println!("================================");

    println!("Using `EqPair` recursively gives the complexity of homotopy levels:");
    let mut dim = 4;
    println!("{}", dim);
    for _ in 0..5 {
        dim = s.count(&dim);
        println!("{}", dim);
    }
    println!("================================");

    println!("Another way to construct a homotopy level 2 is to use `EqPair<Of<EqPair>>`:");
    let s: EqPair<Of<EqPair>> = Construct::new();
    let dim = 2;
    println!("{}", s.count(&dim));

    println!("Similarly, at homotopy level 3 one can use `EqPair<Of<EqPair<Of<EqPair>>>>`:");
    let s: EqPair<Of<EqPair<Of<EqPair>>>> = Construct::new();
    let dim = 2;
    println!("{}", s.count(&dim));

    println!("However, for the more general case is it easier to use the `Homotopy` space:");
    let s: Homotopy = Construct::new();
    let level = 2;
    let pieces = 2;
    let n = s.count(&(level, pieces));
    println!("{}", n);

    let mut pos = s.zero(&(level, pieces));
    for x in 0..n {
        s.to_pos(&(level, pieces), x, &mut pos);
        println!("{:?}", pos);
    }
    println!("================================");

    println!("The `HPoint` enum represents positions in the `Homotopy` space:");
    let a = Path(Box::new((Point(0), Point(0))));
    let b = Path(Box::new((Point(1), Point(1))));
    let pos = Path(Box::new((a, b)));
    let level = pos.level();
    println!("{:?} - level {}", pos, level);
    println!("{}", s.to_index(&(level, 2), &pos));
    println!("================================");

    println!("One can also construct a homotopy of another discrete space, e.g. `Homotopy<Of<Pair>>`:");
    let s: Homotopy<Of<Pair>> = Construct::new();
    let level = 1;
    let pieces = 3;
    let n = s.count(&(level, pieces));
    println!("{}", n);

    let mut pos = s.zero(&(level, pieces));
    for x in 0..n {
        s.to_pos(&(level, pieces), x, &mut pos);
        println!("{:?}", pos);
    }
}
