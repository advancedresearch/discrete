/*

    Zero - implement unitialized element for all spaces.

    First, implement all lacking combinations.

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
        - [x] Subspace<T>
        - [x] Of<T>
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
    let dir: DirectedContext<Of<Pair>> = Construct::new();
    let dim = &[3, 2];
    let count = dir.count(dim);
    println!("count {}", count);
    let mut pos = dir.zero(dim);
    for i in 0..count {
        dir.to_pos(dim, i, &mut pos);
        println!("{:?}", pos);
    }
}
