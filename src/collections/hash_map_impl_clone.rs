use crate::collections::HashMap;

impl<K, V, S> Clone for HashMap<K, V, S>
where
    K: Clone,
    V: Clone,
    S: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        HashMap {
            base: self.base.clone(),
        }
    }

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        self.base.clone_from(&other.base)
    }
}
