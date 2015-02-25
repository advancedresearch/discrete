/*

PAIR OF POWERSETS

In this example we will try to solve the following problem:

    Alice is fishing, and she knows of 10 fish species in the ocean.
    After 2 hours, Alice catched 4 different species.

    As she watches the sunset, she wonders how many possible worlds
    she could catch exactly 4 species out of the unknown number of
    species that lives in this particular area.

A powerset is a way to describe how many ways you can pick things.
Because Alice is fishing, she is picking fish species from the ocean.
We can also think of the species living in this area as being "picked"
from the species that lives in the ocean in general.

A pair can be described with a tuple `(a, b)` where `b` is greater than `a`.
Because Alice can't catch other species than those who lives in the area,
we can think of the species living nearby as a powerset, paired with
a powerset that Alice catches.

Alice might catch all the species living nearby, while a pair
excludes tuples of the kind `(a, a)`. Therefore, instead of `Pair` we'll use
`EqPair`.

A pair of power sets can contain sets that are not subsets of another.
We have to eliminate those by checking for subsets, such that Alice
doesn't catch fishes that doesn't live in the nearby area.

When we print out the number of pairs of power sets of N where one is subset of
another, we get a familiar sequence:

    N = 0: 1
    N = 1: 3
    N = 2: 9
    N = 3: 27
    N = 4: 81

If the number of fishes in the ocean are `N`, then the number of worlds
Alice can catch fishes nearby is `3^N`. Among the 10 species in the ocean,
there are 59049 kinds of worlds.

Last, we need to filter out the worlds where Alice doesn't catch 4 species.

There are exactly 13440 such worlds, and we can list all of them.

*/

extern crate discrete;

use discrete::*;

fn main() {
    let pair_of_powersets: EqPair<Of<PowerSet<Data>>> = Construct::new();
    let n = 10;
    let count = pair_of_powersets.count(n);
    println!("count {:?}", count);
    let mut pos: (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    let mut res_count = 0;
    for x in 0..count {
        pair_of_powersets.to_pos(n, x, &mut pos);
        // Ignore worlds where Alice doesn't catch 4 species.
        if (pos.0).len() != 4 { continue; }
        let mut subset = true;
        for a in (pos.0).iter() {
            let mut found = false;
            for b in (pos.1).iter() {
                if a == b { found = true; break; }
            }
            if !found { subset = false; break; }
        }
        if subset {
            println!("{:?}", pos);
            res_count += 1;
        }
    }
    println!("{:?}", res_count);
}
