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

#[cfg(test)]
mod tests {
    use std::{
        collections::{BinaryHeap, VecDeque},
        ffi::CString,
        num::NonZeroU8,
        rc::Rc,
        sync::Arc,
    };

    use crate::vec::Vec;

    #[test]
    fn test_from_slice() {
        let source = [2, 1, 3];

        let stdvec = std::vec::Vec::from(&source[..]);
        let cratevec = Vec::from(&source[..]);

        assert_eq!(stdvec, cratevec);
    }

    #[test]
    fn test_from_mut_slice() {
        let mut source = [2, 1, 3];

        let stdvec = std::vec::Vec::from(&mut source[..]);
        let cratevec = Vec::from(&mut source[..]);

        assert_eq!(stdvec, cratevec);
    }

    #[test]
    fn test_from_array() {
        let source: [i32; 3] = [2, 1, 3];

        let stdvec = std::vec::Vec::from(source);
        let cratevec = Vec::from(source);

        assert_eq!(stdvec, cratevec);
    }

    #[test]
    fn test_from_box_slice() {
        let source: Box<[i32]> = Box::new([2, 1, 3]);

        let stdvec = std::vec::Vec::from(source.clone());
        let cratevec = Vec::from(source.clone());

        assert_eq!(stdvec, cratevec);
    }

    #[test]
    fn test_from_vec_to_box() {
        let stdvec = std::vec::Vec::from([2, 1, 3]);
        let cratevec = Vec::from([2, 1, 3]);

        let stdres: Box<[i32]> = Box::from(stdvec);
        let crateres: Box<[i32]> = Box::from(cratevec);

        assert_eq!(stdres, crateres);
    }

    #[test]
    fn test_from_str() {
        let stdvec = std::vec::Vec::from("Hello World!");
        let cratevec = Vec::from("Hello World!");

        assert_eq!(stdvec, cratevec);
    }

    #[test]
    fn test_from_binary_heap() {
        let stdvec = std::vec::Vec::from(BinaryHeap::from([2, 1, 3]));
        let cratevec = Vec::from(BinaryHeap::from([2, 1, 3]));

        assert_eq!(stdvec, cratevec);
    }

    #[test]
    fn test_from_c_string() {
        let stdvec = std::vec::Vec::from(CString::new("Hello, world!").unwrap());
        let cratevec = Vec::from(CString::new("Hello, world!").unwrap());

        assert_eq!(stdvec, cratevec);
    }

    #[test]
    fn test_from_string() {
        let stdvec = std::vec::Vec::from("Hello World!".to_string());
        let cratevec = Vec::from("Hello World!".to_string());

        assert_eq!(stdvec, cratevec);
    }

    #[test]
    fn test_from_vec_non_zero_u8_to_c_string() {
        let stdsrc = std::vec::Vec::from(vec![
            NonZeroU8::new(2).unwrap(),
            NonZeroU8::new(1).unwrap(),
            NonZeroU8::new(3).unwrap(),
        ]);
        let cratesrc = Vec::from(vec![
            NonZeroU8::new(2).unwrap(),
            NonZeroU8::new(1).unwrap(),
            NonZeroU8::new(3).unwrap(),
        ]);

        let stdres = CString::from(stdsrc);
        let crateres = CString::from(cratesrc);

        assert_eq!(stdres, crateres);
    }

    #[test]
    fn test_from_vec_to_vec_deque() {
        let stdres = VecDeque::from(std::vec::Vec::from([2, 1, 3]));
        let crateres = VecDeque::from(Vec::from([2, 1, 3]));

        assert_eq!(stdres, crateres);
    }

    #[test]
    fn test_from_vec_to_arc() {
        let stdres: Arc<[i32]> = Arc::from(std::vec::Vec::from([2, 1, 3]));
        let crateres: Arc<[i32]> = Arc::from(Vec::from([2, 1, 3]));

        assert_eq!(stdres, crateres);
    }

    #[test]
    fn test_from_vec_to_rc() {
        let stdres: Rc<[i32]> = Rc::from(std::vec::Vec::from([2, 1, 3]));
        let crateres: Rc<[i32]> = Rc::from(Vec::from([2, 1, 3]));

        assert_eq!(stdres, crateres);
    }

    #[test]
    fn test_from_vec_to_binary_heap() {
        let stdres: Vec<i32> = BinaryHeap::from(std::vec::Vec::from([2, 1, 3]))
            .into_iter()
            .collect();
        let crateres: Vec<i32> = BinaryHeap::from(Vec::from([2, 1, 3])).into_iter().collect();

        assert_eq!(stdres, crateres);
    }

    #[test]
    fn test_from_vec_deque_to_vec() {
        let stdvec: std::vec::Vec<i32> = std::vec::Vec::from(VecDeque::from([2, 1, 3]));
        let cratevec: Vec<i32> = Vec::from(VecDeque::from([2, 1, 3]));

        assert_eq!(stdvec, cratevec);
    }
}
