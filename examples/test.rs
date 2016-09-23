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
        - [x] Data
        - [x] Subspace<T>
        - [x] Of<T>
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
    let dir: Context<Of<Context>> = Construct::new();
    let ref dim = vec![vec![2, 2], vec![2, 2]];
    let count = dir.count(dim);
    let mut pos = dir.zero(dim);
    for i in 0..count {
        dir.to_pos(dim, i, &mut pos);
        print!("{:?}", pos);
        println!(" => {}", dir.to_index(dim, &pos));
    }
    println!("count {}", count);
}
