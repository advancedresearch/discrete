/*

Finds the 16 triangle cycles on a square.

*/

extern crate discrete;

use discrete::*;

fn main() {
    let s: Permutation<Of<Pair>> = Construct::new();
    let dim = 4;
    let mut pos = s.zero(&dim);
    println!("Structure: {:?}\n", pos);
    let count = s.count(&dim);
    let mut triangles = 0;
    // Exploits that the extra 3 pairs are permuted
    // right to left, so one can skip to every 6th solution.
    let scale = 6;
    for i in 0..count/scale {
        let i = i * scale;
        s.to_pos(&dim, i, &mut pos);
        let triangle = connected(pos[0], pos[1]) &&
            connected(pos[1], pos[2]) &&
            connected(pos[2], pos[0]);
        let tri = &pos[0..3];
        let non_rotated = min_pair(tri, |pos| {
            let s: Pair = Construct::new();
            s.to_index(&dim, &pos)
        }) == Some(0);
        if triangle && non_rotated {
            triangles += 1;
            println!("{:?}", tri);
        }
    }
    println!("{:?} => {}", dim, count);
    println!("triangles {}", triangles);
}

pub fn connected(a: (usize, usize), b: (usize, usize)) -> bool {
    a.0 == b.0 ||
    a.1 == b.0 ||
    a.0 == b.1 ||
    a.1 == b.1
}

pub fn min_pair(ps: &[(usize, usize)], to_index: impl Fn((usize, usize)) -> usize) -> Option<usize> {
    ps.iter().map(|&n| to_index(n)).enumerate().min_by_key(|(_, j)| *j).map(|(i, _)| i)
}
