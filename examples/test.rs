extern crate discrete;

use discrete::*;

fn main() {
    let s: DimensionN = Construct::new();
    let dim = vec![2, 2, 2];
    println!("{:?}", s.zero(&dim));
}
