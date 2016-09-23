/*

PAIR OF PAIRS

In this example we will try to solve the following problem:

   You are a secret MIB agent trying to detect activity of 4 criminal aliens.
   The aliens are of species that live in pairs and die if separated by 10m.
   You know that the 4 criminal aliens met each other at some point,
   but not when or where.

   A super computer is monitoring all the people in a city.
   Each person has a unique identity.
   Unfortunately, the computer's AI refuses to collaborate of ethical reasons,
   and the super computer can therefore only tell you yes/no questions.

   If the city contains N people, find all possible meetups of pairs.

   Notice that a person can not meet itself, and even if the alien species
   can time travel they always occur in the same pair.

Using this library, we can construct a discrete space that contains all
combinations of pairs. Not only can we get all pairs of up to N objects,
but we can also construct a space containing pair of pairs.

We simply create a variable of type `Pair<Of<Pair<Data>>>` and call
`Construct::new()` to create it.

The Rust compiler can now assist you solving rest of the task.

For every space, there is a dimension type and a position type.
The dimension type is needed to control the size of the space.
In our case the dimension type is a number N.
The position type is a structure that describes the data within the space.

Discrete spaces have the property that each number from 0 up to the size
corresponds to position within the space. It is the information inside
the position we are interested in, because plain numbers are meaningless.
The `ToPos` trait converts from a number to the position structure.

By computing the size of the space and iterating through every number,
we can convert it to a position and then do filtering based on some criteria.
In our case we can filter out any meetups between pairs where one person
is in both pairs.

For example, for 150 people there are 61 332 125 possible meetups.

*/

extern crate discrete;

use discrete::{ Count, Construct, Pair, Of, Data, ToPos };

fn main() {
    let pair_of_pairs: Pair<Of<Pair<Data>>> = Construct::new();
    let ref n = 150;
    let count = pair_of_pairs.count(n);
    println!("count {:?}", count);
    let mut pos: ((usize, usize), (usize, usize)) = ((0, 0), (0, 0));
    let mut res_count = 0;
    for x in 0..count {
        pair_of_pairs.to_pos(n, x, &mut pos);
        if (pos.0).0 == (pos.1).0
        || (pos.0).0 == (pos.1).1
        || (pos.0).1 == (pos.1).0 {
            // println!("anomaly {:?}", pos)
        } else {
            // println!("{:?}", pos);
            res_count += 1;
        }
    }
    println!("{:?} results", res_count);
}
