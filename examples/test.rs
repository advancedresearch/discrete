extern crate discrete;

use discrete::*;

fn main() {
    let s: Pair<Of<PowerSet<Of<Permutation>>>> = Construct::new();
    let dim: BigUint = 2usize.into();

    let dim = dim.pow(1);
    let count = s.count(&dim);

    let mut i: BigUint = count.clone() - 1usize;
    let mut pos = s.zero(&dim);
    let mut j = 0;
    let z: BigUint = 0usize.into();
    while i >= z {
        if j > 3 {break};

        s.to_pos(&dim, i.clone(), &mut pos);
        println!("{}: {:?}", i, pos);
        
        i -= 1usize;
        j += 1;
    }

    println!("{}", count);
}
