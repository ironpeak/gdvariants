use std::{
    borrow::Cow,
    collections::{BinaryHeap, VecDeque},
    ffi::CString,
    num::NonZeroU8,
    rc::Rc,
    sync::Arc,
};

use crate::vec::Vec;

impl<T> From<&[T]> for Vec<T>
where
    T: Clone,
{
    /// Allocate a `Vec<T>` and fill it by cloning `s`'s items.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(Vec::from(&[1, 2, 3][..]), vec![1, 2, 3]);
    /// ```
    fn from(s: &[T]) -> Vec<T> {
        Vec { base: s.to_vec() }
    }
}

impl<T> From<&mut [T]> for Vec<T>
where
    T: Clone,
{
    /// Allocate a `Vec<T>` and fill it by cloning `s`'s items.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(Vec::from(&mut [1, 2, 3][..]), vec![1, 2, 3]);
    /// ```
    fn from(s: &mut [T]) -> Vec<T> {
        Vec { base: s.to_vec() }
    }
}

impl<T, const N: usize> From<[T; N]> for Vec<T> {
    fn from(s: [T; N]) -> Vec<T> {
        Vec {
            base: <[T]>::into_vec(Box::new(s)),
        }
    }
}

impl<'a, T> From<Cow<'a, [T]>> for Vec<T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    /// Convert a clone-on-write slice into a vector.
    ///
    /// If `s` already owns a `Vec<T>`, it will be returned directly.
    /// If `s` is borrowing a slice, a new `Vec<T>` will be allocated and
    /// filled by cloning `s`'s items into it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::borrow::Cow;
    /// let o: Cow<[i32]> = Cow::Owned(vec![1, 2, 3]);
    /// let b: Cow<[i32]> = Cow::Borrowed(&[1, 2, 3]);
    /// assert_eq!(Vec::from(o), Vec::from(b));
    /// ```
    fn from(s: Cow<'a, [T]>) -> Vec<T> {
        s.into_owned()
    }
}

impl<T> From<Box<[T]>> for Vec<T> {
    /// Convert a boxed slice into a vector by transferring ownership of
    /// the existing heap allocation.
    ///
    /// # Examples
    ///
    /// ```
    /// let b: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
    /// assert_eq!(Vec::from(b), vec![1, 2, 3]);
    /// ```
    fn from(s: Box<[T]>) -> Vec<T> {
        Vec { base: s.into_vec() }
    }
}

impl<T> From<Vec<T>> for Box<[T]> {
    /// Convert a vector into a boxed slice.
    ///
    /// If `v` has excess capacity, its items will be moved into a
    /// newly-allocated buffer with exactly the right capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(Box::from(vec![1, 2, 3]), vec![1, 2, 3].into_boxed_slice());
    /// ```
    fn from(v: Vec<T>) -> Self {
        v.into_boxed_slice()
    }
}

impl From<&str> for Vec<u8> {
    /// Allocate a `Vec<u8>` and fill it with a UTF-8 string.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(Vec::from("123"), vec![b'1', b'2', b'3']);
    /// ```
    fn from(s: &str) -> Vec<u8> {
        From::from(s.as_bytes())
    }
}

impl<'a, T> From<&'a Vec<T>> for Cow<'a, [T]>
where
    T: Clone,
{
    fn from(v: &'a Vec<T>) -> Cow<'a, [T]> {
        Cow::Borrowed(v.as_slice())
    }
}

impl<T> From<BinaryHeap<T>> for Vec<T> {
    fn from(heap: BinaryHeap<T>) -> Vec<T> {
        Vec {
            base: std::vec::Vec::from(heap),
        }
    }
}

impl From<CString> for Vec<u8> {
    fn from(s: CString) -> Vec<u8> {
        Vec {
            base: s.into_bytes(),
        }
    }
}

impl From<String> for Vec<u8> {
    fn from(string: String) -> Vec<u8> {
        Vec {
            base: string.into_bytes(),
        }
    }
}

impl From<Vec<NonZeroU8>> for CString {
    fn from(v: Vec<NonZeroU8>) -> CString {
        CString::from(v.base)
    }
}

impl<T> From<Vec<T>> for VecDeque<T> {
    fn from(other: Vec<T>) -> VecDeque<T> {
        VecDeque::from(other.base)
    }
}

impl<'a, T> From<Vec<T>> for Cow<'a, [T]>
where
    T: Clone,
{
    fn from(v: Vec<T>) -> Cow<'a, [T]> {
        Cow::Owned(v.base)
    }
}

impl<T> From<Vec<T>> for Arc<[T]> {
    fn from(v: Vec<T>) -> Arc<[T]> {
        Arc::from(v.base)
    }
}

impl<T> From<Vec<T>> for Rc<[T]> {
    fn from(v: Vec<T>) -> Rc<[T]> {
        Rc::from(v.base)
    }
}

impl<T> From<Vec<T>> for BinaryHeap<T>
where
    T: Ord,
{
    fn from(vec: Vec<T>) -> BinaryHeap<T> {
        BinaryHeap::from(vec.base)
    }
}

impl<T> From<VecDeque<T>> for Vec<T> {
    fn from(other: VecDeque<T>) -> Vec<T> {
        Vec {
            base: std::vec::Vec::from(other),
        }
    }
}
