use crate::vec::Vec;
use std::fmt::{Debug, Error, Formatter};

impl<T> Debug for Vec<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.base.fmt(f)
    }
}
