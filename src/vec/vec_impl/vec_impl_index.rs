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
