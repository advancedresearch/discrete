/*

A Directed Acyclic Graph (DAG) of `n` nodes is isomorphic to
the upper or lower strictly triangular binary square `n ⨯ n` matrix.

The discrete space of pairs can be thought of as:

    -       (a, b)  (a, c)  (a, d)
    -       -       (b, c)  (b, d)
    -       -       -       (c, d)
    -       -       -       -

In a strictly triangular binary square matrix,
these pairs are filled with `0` or `1`.

Therefore, by taking the powerset of pair,
one can construct a discrete space that is isomorphic to DAGs.

    n       count
    1       1
    2       2
    3       8
    4       64
    5       1024
    6       32768
    7       2097152
    8       268435456
    9       68719476736
    10      35184372088832
    11      36028797018963968

When enumerating this space, the position is a list of pairs.

For example: `[(0, 2), (1, 2)]`.

This means that `2` depends on `0` and `1`.
The DAG can be constructed directly from this data.

*/

extern crate discrete;

use discrete::{ Construct, Count, ToPos, PowerSet, Of, Pair };

fn main() {
    let n = 3;
    let dag: PowerSet<Of<Pair>> = Construct::new();
    let count = dag.count(&n);
    println!("{}", count);

    let mut pos = vec![];
    for i in 0..count {
        dag.to_pos(&n, i, &mut pos);
        println!("{:?}", pos);
    }
}

