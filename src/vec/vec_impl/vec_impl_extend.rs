use crate::vec::Vec;

impl<T> Extend<T> for Vec<T> {
    #[inline]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.base.extend(iter)
    }
}

/// Extend implementation that copies elements out of references before pushing them onto the Vec.
///
/// This implementation is specialized for slice iterators, where it uses [`copy_from_slice`] to
/// append the entire slice at once.
///
/// [`copy_from_slice`]: slice::copy_from_slice
impl<'a, T> Extend<&'a T> for Vec<T>
where
    T: 'a + Copy,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        self.base.extend(iter)
    }
}

#[cfg(test)]
mod tests {
    use crate::vec::Vec;

    #[test]
    fn test_extend() {
        let mut stdvec = vec![2, 1, 3];
        let mut cratevec = Vec::from(vec![2, 1, 3]);

        stdvec.extend([1, 2, 3]);
        cratevec.extend([1, 2, 3]);

        assert_eq!(stdvec, cratevec);
    }
}
