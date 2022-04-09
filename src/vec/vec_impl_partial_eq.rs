use crate::vec::Vec;

impl<T> PartialEq for Vec<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
    }
}
