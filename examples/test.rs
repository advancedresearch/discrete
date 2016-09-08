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

*/

extern crate discrete;

use discrete::*;

fn main() {
    let x: NeqPair = Construct::new();
    let n = 4;
    let count = x.count(n);
    println!("count {:?}", count);
    let mut pos = x.zero(n);
    for i in 0..count {
        x.to_pos(n, i, &mut pos);
        println!("{:?}", pos);
    }
}
