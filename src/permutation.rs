
use std::marker::PhantomData;

use Construct;
use Count;
use Data;
use ToPos;
use ToIndex;

/// Dimension is natural number, position is a list of numbers.
pub struct Permutation<T>(PhantomData<T>);

impl<T> Construct for Permutation<T> {
    fn new() -> Permutation<T> {
        Permutation(PhantomData)
    }
}

impl Count<usize> for Permutation<Data> {
    fn count(&self, dim: usize) -> usize {
        let mut res = 1;
        for x in 1..dim + 1 {
            res *= x;
        }
        res
    }
}

/*
impl<'a> ToIndex<usize, &'a [usize]> for Permutation<Data> {
    fn to_index(&self, dim: usize, pos: &'a [usize]) -> usize {

    }
}
*/

impl<'a> ToPos<usize, &'a mut Vec<usize>> for Permutation<Data> {
    fn to_pos(&self, dim: usize, mut index: usize, pos: &'a mut Vec<usize>) {
        unsafe { pos.set_len(0); }

        let mut count = 1;
        for (j, x) in (1..dim + 1).enumerate() {
            count *= x;
            pos.push(j);
        }

        for i in 0..dim {
            let block = count / (dim - i);
            let ind = index / block;
            let item = pos.remove(ind);
            pos.push(item);
            count /= dim - i;
            index -= ind * block;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Construct;
    use Data;
    use Count;
    use ToPos;

    #[test]
    fn test() {
        let permutation: Permutation<Data> = Construct::new();
        assert_eq!(permutation.count(1), 1);
        assert_eq!(permutation.count(2), 2);
        assert_eq!(permutation.count(3), 6);
        assert_eq!(permutation.count(4), 24);

        let mut pos = Vec::new();
        let dim = 4;
        permutation.to_pos(dim, 0, &mut pos);
        assert_eq!(&pos, &[0, 1, 2, 3]);
        permutation.to_pos(dim, 1, &mut pos);
        assert_eq!(&pos, &[0, 1, 3, 2]);
        permutation.to_pos(dim, 2, &mut pos);
        assert_eq!(&pos, &[0, 2, 1, 3]);
        permutation.to_pos(dim, 3, &mut pos);
        assert_eq!(&pos, &[0, 2, 3, 1]);
        permutation.to_pos(dim, 4, &mut pos);
        assert_eq!(&pos, &[0, 3, 1, 2]);
        permutation.to_pos(dim, 5, &mut pos);
        assert_eq!(&pos, &[0, 3, 2, 1]);
        permutation.to_pos(dim, 6, &mut pos);
        assert_eq!(&pos, &[1, 0, 2, 3]);
    }
}
