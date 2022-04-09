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
