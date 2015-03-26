/*
DIRECTED CONTEXT

In this example we will try to solve the problem:

    A farmer want to cross a river, using a boat with room for two objects.
    With him he has a wolf, a sheep, and some cabbage.
    If not watching, the wolf eats the sheep, and the sheep eats the cabbage.
    Find an upper bound of legal moves below 200.

We have 4 objects: Wolf, sheep, cabbage and the boat.
The wolf, sheep and cabbage can be in 3 locations: Side A, boat, side B.
The boat can be in 2 locations: Side A or side B.
This becomes a N-dimensional configuration space of dimensions `[3, 3, 3, 2]`.

A directed context space models how a configuration space can change by moving
one object into another state at a time.
Each position in the space is equivalent to one move.

When we construct a directed context space with dimension `[3, 3, 3, 2]`,
we get 378 moves. These are all possible ways the objects can move ignoring
all physical rules.

There is a rule for context spaces that information that is symmetric
or invariant for one object can be removed without loosing context.
The boat in this case can always move from one side to the other,
regardless the states of other objects.
Therefore, we can reduce the space to `[3, 3, 3]`.
This reduces to 162 moves.

Each object can only move from side A to side B by using the boat.
We can model a space with dimension `[2, 2, 2, 3]` where the last
dimension tells which item can use the boat.
This reduces to 120 moves.

A side note: The amount of memory needed to store any rules for such a puzzle
is in worst case 120 bits, because each bit can tell us whether a move is legal
or not. This is the same information we get from rules.

*/

extern crate discrete;

use discrete::*;

fn main() {
    let context: DirectedContext<Data> = Construct::new();

    let dim = vec![2, 2, 2, 3];
    let shorter_count = context.count(&dim);

    let object = ["wolf", "sheep", "cabbage"];
    let side = ["side A", "boat", "side B"];
    let dim = vec![3, 3, 3]; /* wolf, sheep, cabbage, boat */
    let count = context.count(&dim);
    for x in 0..count {
        let mut pos = (vec![], 0, 0);
        context.to_pos(&dim, x, &mut pos);

        println!("{:?}", pos);
        print!("the {} ", object[pos.1]);
        print!("went from {} ", side[pos.0[pos.1]]);
        println!("to {}", side[pos.2]);

        for i in 0..3 {
            let pos_state = pos.0[i];
            println!("{} is at {}", object[i], side[pos_state]);
        }
    }
    println!("count {}", count);

    println!("shorter_counter {}", shorter_count);
}
