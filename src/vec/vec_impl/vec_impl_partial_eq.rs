use crate::vec::Vec;

macro_rules! __impl_slice_eq {
    ([$($vars:tt)*] $lhs:ty, $rhs:ty $(where $ty:ty: $bound:ident)?) => {
        impl<T, U, $($vars)*> PartialEq<$rhs> for $lhs
        where
            T: PartialEq<U>,
            $($ty: $bound)?
        {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool { self[..] == other[..] }
        }
    }
}

__impl_slice_eq! { [] Vec<T>, Vec<U> }
__impl_slice_eq! { [] Vec<T>, &[U] }
__impl_slice_eq! { [] Vec<T>, &mut [U] }
__impl_slice_eq! { [] Vec<T>, std::vec::Vec<U> }
__impl_slice_eq! { [] &[T], Vec<U> }
__impl_slice_eq! { [] &mut [T], Vec<U> }
__impl_slice_eq! { [] Vec<T>, [U] }
__impl_slice_eq! { [] [T], Vec<U> }
__impl_slice_eq! { [] std::vec::Vec<T>, Vec<U> }
__impl_slice_eq! { [const N: usize] Vec<T>, [U; N] }
__impl_slice_eq! { [const N: usize] Vec<T>, &[U; N] }
