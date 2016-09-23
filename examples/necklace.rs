/*
A necklace is a group given by a sequence of numbers that can be rotated
left or right, like beads of different colors on a looped string.

Necklaces are used to describe spaces of objects that can be rotated
around a single axis.
For example, edges between vertices of a 2D triangle are considered the
same no matter how you rotate it.

Instead of constructing objects the way we intrinsically understand
them in the physical world, the way to do it through discrete mathematics
is to describe some property of the object and then go through the
entire space of possibilities with that property.
Then we filter out the groups we get by equivalence proofs until
we have a set of unique objects with the features we are interested in.
In many cases we can only take a peek into this world,
as long there is no efficient algorithm for either storing or iterate
through complex spaces.

The algorithm used here prints out the sequence with lowest index,
which represents the local necklace group:

1. Generate a map of indices for rotations
2. Iterate through all n-dimensional sequences
3. For each sequence, compute rotated sequence
    4. Compute the index of rotated sequence
    5. If rotated sequence has lower index, skip to next sequence
    6. If not, print out sequence

If the space is very large, only every 100 000 necklace is printed out.
*/

extern crate discrete;

use discrete::*;

fn main() {
    let x: DimensionN = Construct::new();
    let n = std::env::args_os().nth(1)
        .and_then(|s| s.into_string().ok())
        .and_then(|n| n.parse().ok())
        .unwrap_or(4);
    let base = std::env::args_os().nth(2)
        .and_then(|s| s.into_string().ok())
        .and_then(|n| n.parse().ok())
        .unwrap_or(3);
    println!("n {}, base {}", n, base);
    let rot = gen_rotation_map(n);
    let ref dim = vec![base; n];
    let count = x.count(dim);
    let mut a = vec![0; n];
    let mut b = vec![0; n];
    let mut counter: u64 = 0;
    'i: for i in 0..count {
        x.to_pos(dim, i, &mut a);

        for k in 0..rot.len() {
            for m in 0..n {
                b[m] = a[rot[k][m]];
            }
            if x.to_index(dim, &b) < i { continue 'i; }
        }

        if count < 500 || counter % 100000 == 0 {
            println!("{:?},", a);
        }
        counter += 1;
    }
    println!("necklaces {}", counter);
}

fn gen_rotation_map(n: usize) -> Vec<Vec<usize>> {
    let mut res = vec![];
    for i in 0..n {
        let mut row = vec![];
        for j in 0..n {
            row.push((j + i) % n);
        }
        res.push(row)
    }
    res
}
