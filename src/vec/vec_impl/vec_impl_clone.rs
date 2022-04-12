use crate::vec::Vec;

impl<T> Clone for Vec<T>
where
    T: Clone,
{
    fn clone(&self) -> Vec<T> {
        Vec {
            base: self.base.clone(),
        }
    }

    fn clone_from(&mut self, other: &Vec<T>) {
        self.base.clone_from(&other.base)
    }
}
