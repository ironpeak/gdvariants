use std::{
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

use crate::vec::Vec;

impl<T, I> IndexMut<I> for Vec<T>
where
    I: SliceIndex<[T]>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut <Vec<T> as Index<I>>::Output {
        self.base.index_mut(index)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::IndexMut;

    use crate::vec::Vec;

    #[test]
    fn test_index_mut() {
        let mut stdvec = vec![2, 1, 3];
        let mut cratevec = Vec::from(vec![2, 1, 3]);

        assert_eq!(stdvec.index_mut(1), cratevec.index_mut(1));
    }
}
