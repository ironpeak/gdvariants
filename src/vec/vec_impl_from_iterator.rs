use crate::vec::Vec;

impl<T> FromIterator<T> for Vec<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vec<T> {
        let mut vec = Vec::new();
        vec.extend(iter);
        vec
    }
}
