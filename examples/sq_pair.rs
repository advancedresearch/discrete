extern crate discrete;

use discrete::*;

fn main() {
    println!("One way to create a square of pairs is using `(Dimension, Dimension):`");
    let s: (Dimension, Dimension) = Construct::new();
    println!("{}", s.count(&(2, 2)));

    println!("Another way is to use `DimensionN`:");
    let s: DimensionN = Construct::new();
    println!("{}", s.count(&vec![2, 2]));

    println!("Or, to avoid needing specifying the dimension twice, one can use `SqPair`:");
    let s: SqPair = Construct::new();
    println!("{}", s.count(&2));

    let mut p = s.zero(&2);
    for i in 0..4 {
        s.to_pos(&2, i, &mut p);
        println!("{:?}", p);
    }

    println!("One can also create a square of another discrete space, e.g. `SqPair<Of<Pair>>`:");
    let s: SqPair<Of<Pair>> = Construct::new();
    let n = 3;
    let count = s.count(&n);
    println!("{}", count);

    let mut p = s.zero(&n);
    for i in 0..count {
        s.to_pos(&n, i, &mut p);
        println!("{:?}", p);
    }
}
