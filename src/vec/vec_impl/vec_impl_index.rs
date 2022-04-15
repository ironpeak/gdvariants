use std::{ops::Index, slice::SliceIndex};

use crate::vec::Vec;

impl<T, I> Index<I> for Vec<T>
where
    I: SliceIndex<[T]>,
{
    type Output = <I as SliceIndex<[T]>>::Output;

    fn index(&self, index: I) -> &<Vec<T> as Index<I>>::Output {
        self.base.index(index)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Index;

    use crate::vec::Vec;

    #[test]
    fn test_index() {
        let stdvec = vec![2, 1, 3];
        let cratevec = Vec::from(vec![2, 1, 3]);

        assert_eq!(stdvec.index(1), cratevec.index(1));
    }
}
