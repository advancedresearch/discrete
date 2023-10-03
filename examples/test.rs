extern crate discrete;

use discrete::*;

fn main() {
    let s: Context = Construct::new();
    let dim: Vec<BigUint> = vec![3usize.into(), 2usize.into()];
    let mut pos = s.zero(&dim);
    let c: Vec<u64> = s.count(&dim).iter_u64_digits().collect();
    let count: usize = c[0] as usize;
    for i in 0..count {
        let i: BigUint = i.into();
        s.to_pos(&dim, i, &mut pos);
        println!("{:?}", pos);
    }
    println!("===");

    let s: Context = Construct::new();
    let dim = vec![3, 2];
    let mut pos = s.zero(&dim);
    let count = s.count(&dim);
    for i in 0..count {
        s.to_pos(&dim, i, &mut pos);
        println!("{:?}", pos);
    }
}
