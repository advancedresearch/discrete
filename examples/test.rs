/*

    Zero

    - [ ] Dimension
        - [x] Data
        - [x] Subspace<T>
        - [x] Of<T>
    - [ ] Pair
        - [x] Data
        - [x] Subspace<T>
        - [x] Of<T>
    - [ ] EqPair
        - [x] Data
        - [x] Subspace<T>
        - [x] Of<T>
    - [ ] NeqPair
        - [x] Data
        - [x] Subspace<T>
        - [x] Of<T>
    - [ ] DimensionN
    - [ ] Context
        - [x] Data
        - [x] Subspace<T>
        - [x] Of<T>
    - [ ] DirectedContext
        - [x] Data
        - [ ] Subspace<T>
        - [ ] Of<T>
    - [ ] Permutation
        - [x] Data
        - [x] Subspace<T>
        - [x] Of<T>
    - [ ] PowerSet
        - [x] Data
        - [x] Subspace<T>
        - [x] Of<T>

*/

extern crate discrete;

use discrete::*;

fn main() {
    // Kind of triangles.
    let triangles = &[
        [1, 0, 0],
        [2, 0, 0],
        [1, 1, 0],
        [2, 1, 0],
        [1, 2, 0],
        [2, 2, 0],
        [1, 1, 1],
        [2, 1, 1],
        [2, 2, 1],
        [2, 2, 2],
    ];
    let glue_pair = &[
        (0, 1),
        (0, 2),
        (1, 2),
    ];
    let edges = &[
        (0, 0),
        (0, 1),
        (1, 1),
        (0, 2),
        (1, 2),
        (2, 2),
    ];

    let x: DimensionN = Construct::new();
    let dim = &[glue_pair.len(), edges.len()];
    let count = x.count(dim);
    println!("count {}", count);
    let mut pos = vec![0, 0];
    for i in 0..count {
        x.to_pos(dim, i, &mut pos);
        println!("{:?} - {:?} {:?}", pos, glue_pair[pos[0]], edges[pos[1]]);
    }
    println!("count {}", count);

    // print_edges();
    let triangle_kinds: DimensionN = Construct::new();
    let dim = &[triangles.len(); 3];
    println!("{}", triangle_kinds.count(dim));
}

fn print_edges() {
    let x: EqPair = Construct::new();
    let dim = 3;
    let count = x.count(dim);
    let mut pos = (0, 0);
    for i in 0..count {
        x.to_pos(dim, i, &mut pos);
        println!("{:?},", pos);
    }
}

/*
It would be useful to have a function for reduction by rotation symmetry.
This problem appears when you have objects that you can rotate,
and you want to know how many kinds of objects there are.

For example, you pick 3 numbers, so all sequences that are not the same
number get divided by 3.

    3 + (N - 3) / 3

    3 + (27 - 3) / 3
    3 + 24 / 3
    3 + 8
    11

    f(base, dim)
    f(3, 3) = 11

    3 + (3^2 - 3) / 2
    3 + (9 - 3) / 2
    3 + 6 / 2
    3 + 3
    f(3, 2) = 6

    f(3, 1) = 3

For 4 dimensions there are 3 pair of numbers that are rotation
symmetric after 2 rotations.

    0 1 0 1 -> 1 0 1 0
    0 2 0 2 -> 2 0 2 0
    1 2 1 2 -> 2 1 2 1

These must be subtracted before dividing by 4,

Could iterate through all N-dimensional number,
and count those who do not have a smaller rotational symmetric number.

These are called "necklaces".

A fast way to generate a necklace could be to analyze the
number itself. It could have properties that determine
some information that can be used to speed it up.

*/
