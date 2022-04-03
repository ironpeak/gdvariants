// Source: https://doc.rust-lang.org/src/alloc/vec/mod.rs.html

use std::{collections::TryReserveError, ops::RangeBounds, vec::Drain};

pub struct Vec<T> {
    base: std::vec::Vec<T>,
}

////////////////////////////////////////////////////////////////////////////////
// Inherent methods
////////////////////////////////////////////////////////////////////////////////

impl<T> Vec<T> {
    /// Constructs a new, empty `Vec<T>`.
    ///
    /// The vector will not allocate until elements are pushed onto it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unused_mut)]
    /// let mut vec: Vec<i32> = Vec::new();
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Vec {
            base: std::vec::Vec::new(),
        }
    }

    /// Constructs a new, empty `Vec<T>` with the specified capacity.
    ///
    /// The vector will be able to hold exactly `capacity` elements without
    /// reallocating. If `capacity` is 0, the vector will not allocate.
    ///
    /// It is important to note that although the returned vector has the
    /// *capacity* specified, the vector will have a zero *length*. For an
    /// explanation of the difference between length and capacity, see
    /// *[Capacity and reallocation]*.
    ///
    /// [Capacity and reallocation]: #capacity-and-reallocation
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = Vec::with_capacity(10);
    ///
    /// // The vector contains no items, even though it has capacity for more
    /// assert_eq!(vec.len(), 0);
    /// assert_eq!(vec.capacity(), 10);
    ///
    /// // These are all done without reallocating...
    /// for i in 0..10 {
    ///     vec.push(i);
    /// }
    /// assert_eq!(vec.len(), 10);
    /// assert_eq!(vec.capacity(), 10);
    ///
    /// // ...but this may make the vector reallocate
    /// vec.push(11);
    /// assert_eq!(vec.len(), 11);
    /// assert!(vec.capacity() >= 11);
    /// ```
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Vec {
            base: std::vec::Vec::with_capacity(capacity),
        }
    }

    /// Creates a `Vec<T>` directly from the raw components of another vector.
    ///
    /// # Safety
    ///
    /// This is highly unsafe, due to the number of invariants that aren't
    /// checked:
    ///
    /// * `ptr` needs to have been previously allocated via [`String`]/`Vec<T>`
    ///   (at least, it's highly likely to be incorrect if it wasn't).
    /// * `T` needs to have the same size and alignment as what `ptr` was allocated with.
    ///   (`T` having a less strict alignment is not sufficient, the alignment really
    ///   needs to be equal to satisfy the [`dealloc`] requirement that memory must be
    ///   allocated and deallocated with the same layout.)
    /// * `length` needs to be less than or equal to `capacity`.
    /// * `capacity` needs to be the capacity that the pointer was allocated with.
    ///
    /// Violating these may cause problems like corrupting the allocator's
    /// internal data structures. For example it is **not** safe
    /// to build a `Vec<u8>` from a pointer to a C `char` array with length `size_t`.
    /// It's also not safe to build one from a `Vec<u16>` and its length, because
    /// the allocator cares about the alignment, and these two types have different
    /// alignments. The buffer was allocated with alignment 2 (for `u16`), but after
    /// turning it into a `Vec<u8>` it'll be deallocated with alignment 1.
    ///
    /// The ownership of `ptr` is effectively transferred to the
    /// `Vec<T>` which may then deallocate, reallocate or change the
    /// contents of memory pointed to by the pointer at will. Ensure
    /// that nothing else uses the pointer after calling this
    /// function.
    ///
    /// [`String`]: crate::string::String
    /// [`dealloc`]: crate::alloc::GlobalAlloc::dealloc
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr;
    /// use std::mem;
    ///
    /// let v = vec![1, 2, 3];
    ///
    // FIXME Update this when vec_into_raw_parts is stabilized
    /// // Prevent running `v`'s destructor so we are in complete control
    /// // of the allocation.
    /// let mut v = mem::ManuallyDrop::new(v);
    ///
    /// // Pull out the various important pieces of information about `v`
    /// let p = v.as_mut_ptr();
    /// let len = v.len();
    /// let cap = v.capacity();
    ///
    /// unsafe {
    ///     // Overwrite memory with 4, 5, 6
    ///     for i in 0..len as isize {
    ///         ptr::write(p.offset(i), 4 + i);
    ///     }
    ///
    ///     // Put everything back together into a Vec
    ///     let rebuilt = Vec::from_raw_parts(p, len, cap);
    ///     assert_eq!(rebuilt, [4, 5, 6]);
    /// }
    /// ```
    #[inline]
    pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
        Vec {
            base: std::vec::Vec::from_raw_parts(ptr, length, capacity),
        }
    }
}

impl<T> Vec<T> {
    /// Returns the number of elements the vector can hold without
    /// reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// let vec: Vec<i32> = Vec::with_capacity(10);
    /// assert_eq!(vec.capacity(), 10);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.base.capacity()
    }

    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the given `Vec<T>`. The collection may reserve more space to avoid
    /// frequent reallocations. After calling `reserve`, capacity will be
    /// greater than or equal to `self.len() + additional`. Does nothing if
    /// capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![1];
    /// vec.reserve(10);
    /// assert!(vec.capacity() >= 11);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.base.reserve(additional)
    }

    /// Reserves the minimum capacity for exactly `additional` more elements to
    /// be inserted in the given `Vec<T>`. After calling `reserve_exact`,
    /// capacity will be greater than or equal to `self.len() + additional`.
    /// Does nothing if the capacity is already sufficient.
    ///
    /// Note that the allocator may give the collection more space than it
    /// requests. Therefore, capacity can not be relied upon to be precisely
    /// minimal. Prefer [`reserve`] if future insertions are expected.
    ///
    /// [`reserve`]: Vec::reserve
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![1];
    /// vec.reserve_exact(10);
    /// assert!(vec.capacity() >= 11);
    /// ```
    pub fn reserve_exact(&mut self, additional: usize) {
        self.base.reserve_exact(additional)
    }

    /// Tries to reserve capacity for at least `additional` more elements to be inserted
    /// in the given `Vec<T>`. The collection may reserve more space to avoid
    /// frequent reallocations. After calling `try_reserve`, capacity will be
    /// greater than or equal to `self.len() + additional`. Does nothing if
    /// capacity is already sufficient.
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an error
    /// is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::TryReserveError;
    ///
    /// fn process_data(data: &[u32]) -> Result<Vec<u32>, TryReserveError> {
    ///     let mut output = Vec::new();
    ///
    ///     // Pre-reserve the memory, exiting if we can't
    ///     output.try_reserve(data.len())?;
    ///
    ///     // Now we know this can't OOM in the middle of our complex work
    ///     output.extend(data.iter().map(|&val| {
    ///         val * 2 + 5 // very complicated
    ///     }));
    ///
    ///     Ok(output)
    /// }
    /// # process_data(&[1, 2, 3]).expect("why is the test harness OOMing on 12 bytes?");
    /// ```
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.base.try_reserve(additional)
    }

    /// Tries to reserve the minimum capacity for exactly `additional`
    /// elements to be inserted in the given `Vec<T>`. After calling
    /// `try_reserve_exact`, capacity will be greater than or equal to
    /// `self.len() + additional` if it returns `Ok(())`.
    /// Does nothing if the capacity is already sufficient.
    ///
    /// Note that the allocator may give the collection more space than it
    /// requests. Therefore, capacity can not be relied upon to be precisely
    /// minimal. Prefer [`try_reserve`] if future insertions are expected.
    ///
    /// [`try_reserve`]: Vec::try_reserve
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an error
    /// is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::TryReserveError;
    ///
    /// fn process_data(data: &[u32]) -> Result<Vec<u32>, TryReserveError> {
    ///     let mut output = Vec::new();
    ///
    ///     // Pre-reserve the memory, exiting if we can't
    ///     output.try_reserve_exact(data.len())?;
    ///
    ///     // Now we know this can't OOM in the middle of our complex work
    ///     output.extend(data.iter().map(|&val| {
    ///         val * 2 + 5 // very complicated
    ///     }));
    ///
    ///     Ok(output)
    /// }
    /// # process_data(&[1, 2, 3]).expect("why is the test harness OOMing on 12 bytes?");
    /// ```
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.base.try_reserve_exact(additional)
    }

    /// Shrinks the capacity of the vector as much as possible.
    ///
    /// It will drop down as close as possible to the length but the allocator
    /// may still inform the vector that there is space for a few more elements.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = Vec::with_capacity(10);
    /// vec.extend([1, 2, 3]);
    /// assert_eq!(vec.capacity(), 10);
    /// vec.shrink_to_fit();
    /// assert!(vec.capacity() >= 3);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.base.shrink_to_fit()
    }

    /// Shrinks the capacity of the vector with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length
    /// and the supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = Vec::with_capacity(10);
    /// vec.extend([1, 2, 3]);
    /// assert_eq!(vec.capacity(), 10);
    /// vec.shrink_to(4);
    /// assert!(vec.capacity() >= 4);
    /// vec.shrink_to(0);
    /// assert!(vec.capacity() >= 3);
    /// ```
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.base.shrink_to(min_capacity)
    }

    /// Converts the vector into [`Box<[T]>`][owned slice].
    ///
    /// Note that this will drop any excess capacity.
    ///
    /// [owned slice]: Box
    ///
    /// # Examples
    ///
    /// ```
    /// let v = vec![1, 2, 3];
    ///
    /// let slice = v.into_boxed_slice();
    /// ```
    ///
    /// Any excess capacity is removed:
    ///
    /// ```
    /// let mut vec = Vec::with_capacity(10);
    /// vec.extend([1, 2, 3]);
    ///
    /// assert_eq!(vec.capacity(), 10);
    /// let slice = vec.into_boxed_slice();
    /// assert_eq!(slice.into_vec().capacity(), 3);
    /// ```
    pub fn into_boxed_slice(self) -> Box<[T]> {
        self.base.into_boxed_slice()
    }

    /// Shortens the vector, keeping the first `len` elements and dropping
    /// the rest.
    ///
    /// If `len` is greater than the vector's current length, this has no
    /// effect.
    ///
    /// The [`drain`] method can emulate `truncate`, but causes the excess
    /// elements to be returned instead of dropped.
    ///
    /// Note that this method has no effect on the allocated capacity
    /// of the vector.
    ///
    /// # Examples
    ///
    /// Truncating a five element vector to two elements:
    ///
    /// ```
    /// let mut vec = vec![1, 2, 3, 4, 5];
    /// vec.truncate(2);
    /// assert_eq!(vec, [1, 2]);
    /// ```
    ///
    /// No truncation occurs when `len` is greater than the vector's current
    /// length:
    ///
    /// ```
    /// let mut vec = vec![1, 2, 3];
    /// vec.truncate(8);
    /// assert_eq!(vec, [1, 2, 3]);
    /// ```
    ///
    /// Truncating when `len == 0` is equivalent to calling the [`clear`]
    /// method.
    ///
    /// ```
    /// let mut vec = vec![1, 2, 3];
    /// vec.truncate(0);
    /// assert_eq!(vec, []);
    /// ```
    ///
    /// [`clear`]: Vec::clear
    /// [`drain`]: Vec::drain
    pub fn truncate(&mut self, len: usize) {
        self.base.truncate(len)
    }

    /// Extracts a slice containing the entire vector.
    ///
    /// Equivalent to `&s[..]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{self, Write};
    /// let buffer = vec![1, 2, 3, 5, 8];
    /// io::sink().write(buffer.as_slice()).unwrap();
    /// ```
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.base.as_slice()
    }

    /// Extracts a mutable slice of the entire vector.
    ///
    /// Equivalent to `&mut s[..]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{self, Read};
    /// let mut buffer = vec![0; 3];
    /// io::repeat(0b101).read_exact(buffer.as_mut_slice()).unwrap();
    /// ```
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.base.as_mut_slice()
    }

    /// Returns a raw pointer to the vector's buffer.
    ///
    /// The caller must ensure that the vector outlives the pointer this
    /// function returns, or else it will end up pointing to garbage.
    /// Modifying the vector may cause its buffer to be reallocated,
    /// which would also make any pointers to it invalid.
    ///
    /// The caller must also ensure that the memory the pointer (non-transitively) points to
    /// is never written to (except inside an `UnsafeCell`) using this pointer or any pointer
    /// derived from it. If you need to mutate the contents of the slice, use [`as_mut_ptr`].
    ///
    /// # Examples
    ///
    /// ```
    /// let x = vec![1, 2, 4];
    /// let x_ptr = x.as_ptr();
    ///
    /// unsafe {
    ///     for i in 0..x.len() {
    ///         assert_eq!(*x_ptr.add(i), 1 << i);
    ///     }
    /// }
    /// ```
    ///
    /// [`as_mut_ptr`]: Vec::as_mut_ptr
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.base.as_ptr()
    }

    /// Returns an unsafe mutable pointer to the vector's buffer.
    ///
    /// The caller must ensure that the vector outlives the pointer this
    /// function returns, or else it will end up pointing to garbage.
    /// Modifying the vector may cause its buffer to be reallocated,
    /// which would also make any pointers to it invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// // Allocate vector big enough for 4 elements.
    /// let size = 4;
    /// let mut x: Vec<i32> = Vec::with_capacity(size);
    /// let x_ptr = x.as_mut_ptr();
    ///
    /// // Initialize elements via raw pointer writes, then set length.
    /// unsafe {
    ///     for i in 0..size {
    ///         *x_ptr.add(i) = i as i32;
    ///     }
    ///     x.set_len(size);
    /// }
    /// assert_eq!(&*x, &[0, 1, 2, 3]);
    /// ```
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.base.as_mut_ptr()
    }

    /// Forces the length of the vector to `new_len`.
    ///
    /// This is a low-level operation that maintains none of the normal
    /// invariants of the type. Normally changing the length of a vector
    /// is done using one of the safe operations instead, such as
    /// [`truncate`], [`resize`], [`extend`], or [`clear`].
    ///
    /// [`truncate`]: Vec::truncate
    /// [`resize`]: Vec::resize
    /// [`extend`]: Extend::extend
    /// [`clear`]: Vec::clear
    ///
    /// # Safety
    ///
    /// - `new_len` must be less than or equal to [`capacity()`].
    /// - The elements at `old_len..new_len` must be initialized.
    ///
    /// [`capacity()`]: Vec::capacity
    ///
    /// # Examples
    ///
    /// This method can be useful for situations in which the vector
    /// is serving as a buffer for other code, particularly over FFI:
    ///
    /// ```no_run
    /// # #![allow(dead_code)]
    /// # // This is just a minimal skeleton for the doc example;
    /// # // don't use this as a starting point for a real library.
    /// # pub struct StreamWrapper { strm: *mut std::ffi::c_void }
    /// # const Z_OK: i32 = 0;
    /// # extern "C" {
    /// #     fn deflateGetDictionary(
    /// #         strm: *mut std::ffi::c_void,
    /// #         dictionary: *mut u8,
    /// #         dictLength: *mut usize,
    /// #     ) -> i32;
    /// # }
    /// # impl StreamWrapper {
    /// pub fn get_dictionary(&self) -> Option<Vec<u8>> {
    ///     // Per the FFI method's docs, "32768 bytes is always enough".
    ///     let mut dict = Vec::with_capacity(32_768);
    ///     let mut dict_length = 0;
    ///     // SAFETY: When `deflateGetDictionary` returns `Z_OK`, it holds that:
    ///     // 1. `dict_length` elements were initialized.
    ///     // 2. `dict_length` <= the capacity (32_768)
    ///     // which makes `set_len` safe to call.
    ///     unsafe {
    ///         // Make the FFI call...
    ///         let r = deflateGetDictionary(self.strm, dict.as_mut_ptr(), &mut dict_length);
    ///         if r == Z_OK {
    ///             // ...and update the length to what was initialized.
    ///             dict.set_len(dict_length);
    ///             Some(dict)
    ///         } else {
    ///             None
    ///         }
    ///     }
    /// }
    /// # }
    /// ```
    ///
    /// While the following example is sound, there is a memory leak since
    /// the inner vectors were not freed prior to the `set_len` call:
    ///
    /// ```
    /// let mut vec = vec![vec![1, 0, 0],
    ///                    vec![0, 1, 0],
    ///                    vec![0, 0, 1]];
    /// // SAFETY:
    /// // 1. `old_len..0` is empty so no elements need to be initialized.
    /// // 2. `0 <= capacity` always holds whatever `capacity` is.
    /// unsafe {
    ///     vec.set_len(0);
    /// }
    /// ```
    ///
    /// Normally, here, one would use [`clear`] instead to correctly drop
    /// the contents and thus not leak memory.
    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.base.set_len(new_len)
    }

    /// Removes an element from the vector and returns it.
    ///
    /// The removed element is replaced by the last element of the vector.
    ///
    /// This does not preserve ordering, but is *O*(1).
    /// If you need to preserve the element order, use [`remove`] instead.
    ///
    /// [`remove`]: Vec::remove
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = vec!["foo", "bar", "baz", "qux"];
    ///
    /// assert_eq!(v.swap_remove(1), "bar");
    /// assert_eq!(v, ["foo", "qux", "baz"]);
    ///
    /// assert_eq!(v.swap_remove(0), "foo");
    /// assert_eq!(v, ["baz", "qux"]);
    /// ```
    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> T {
        self.base.swap_remove(index)
    }

    /// Inserts an element at position `index` within the vector, shifting all
    /// elements after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![1, 2, 3];
    /// vec.insert(1, 4);
    /// assert_eq!(vec, [1, 4, 2, 3]);
    /// vec.insert(4, 5);
    /// assert_eq!(vec, [1, 4, 2, 3, 5]);
    /// ```
    pub fn insert(&mut self, index: usize, element: T) {
        self.base.insert(index, element)
    }

    /// Removes and returns the element at position `index` within the vector,
    /// shifting all elements after it to the left.
    ///
    /// Note: Because this shifts over the remaining elements, it has a
    /// worst-case performance of *O*(*n*). If you don't need the order of elements
    /// to be preserved, use [`swap_remove`] instead.
    ///
    /// [`swap_remove`]: Vec::swap_remove
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = vec![1, 2, 3];
    /// assert_eq!(v.remove(1), 2);
    /// assert_eq!(v, [1, 3]);
    /// ```
    #[track_caller]
    pub fn remove(&mut self, index: usize) -> T {
        self.base.remove(index)
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` such that `f(&e)` returns `false`.
    /// This method operates in place, visiting each element exactly once in the
    /// original order, and preserves the order of the retained elements.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![1, 2, 3, 4];
    /// vec.retain(|&x| x % 2 == 0);
    /// assert_eq!(vec, [2, 4]);
    /// ```
    ///
    /// Because the elements are visited exactly once in the original order,
    /// external state may be used to decide which elements to keep.
    ///
    /// ```
    /// let mut vec = vec![1, 2, 3, 4, 5];
    /// let keep = [false, true, true, false, true];
    /// let mut iter = keep.iter();
    /// vec.retain(|_| *iter.next().unwrap());
    /// assert_eq!(vec, [2, 3, 5]);
    /// ```
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.base.retain(f)
    }

    /// Removes all but the first of consecutive elements in the vector that resolve to the same
    /// key.
    ///
    /// If the vector is sorted, this removes all duplicates.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![10, 20, 21, 30, 20];
    ///
    /// vec.dedup_by_key(|i| *i / 10);
    ///
    /// assert_eq!(vec, [10, 20, 30, 20]);
    /// ```
    #[inline]
    pub fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        self.base.dedup_by_key(key)
    }

    /// Removes all but the first of consecutive elements in the vector satisfying a given equality
    /// relation.
    ///
    /// The `same_bucket` function is passed references to two elements from the vector and
    /// must determine if the elements compare equal. The elements are passed in opposite order
    /// from their order in the slice, so if `same_bucket(a, b)` returns `true`, `a` is removed.
    ///
    /// If the vector is sorted, this removes all duplicates.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec!["foo", "bar", "Bar", "baz", "bar"];
    ///
    /// vec.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    ///
    /// assert_eq!(vec, ["foo", "bar", "baz", "bar"]);
    /// ```
    pub fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        self.base.dedup_by(same_bucket)
    }

    /// Appends an element to the back of a collection.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![1, 2];
    /// vec.push(3);
    /// assert_eq!(vec, [1, 2, 3]);
    /// ```
    #[inline]
    pub fn push(&mut self, value: T) {
        self.base.push(value)
    }

    /// Removes the last element from a vector and returns it, or [`None`] if it
    /// is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![1, 2, 3];
    /// assert_eq!(vec.pop(), Some(3));
    /// assert_eq!(vec, [1, 2]);
    /// ```
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.base.pop()
    }

    /// Moves all the elements of `other` into `Self`, leaving `other` empty.
    ///
    /// # Panics
    ///
    /// Panics if the number of elements in the vector overflows a `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![1, 2, 3];
    /// let mut vec2 = vec![4, 5, 6];
    /// vec.append(&mut vec2);
    /// assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
    /// assert_eq!(vec2, []);
    /// ```
    #[inline]
    pub fn append(&mut self, other: &mut Self) {
        self.base.append(&mut other.base)
    }

    /// Creates a draining iterator that removes the specified range in the vector
    /// and yields the removed items.
    ///
    /// When the iterator **is** dropped, all elements in the range are removed
    /// from the vector, even if the iterator was not fully consumed. If the
    /// iterator **is not** dropped (with [`mem::forget`] for example), it is
    /// unspecified how many elements are removed.
    ///
    /// # Panics
    ///
    /// Panics if the starting point is greater than the end point or if
    /// the end point is greater than the length of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = vec![1, 2, 3];
    /// let u: Vec<_> = v.drain(1..).collect();
    /// assert_eq!(v, &[1]);
    /// assert_eq!(u, &[2, 3]);
    ///
    /// // A full range clears the vector
    /// v.drain(..);
    /// assert_eq!(v, &[]);
    /// ```
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T>
    where
        R: RangeBounds<usize>,
    {
        self.base.drain(range)
    }

    /// Clears the vector, removing all values.
    ///
    /// Note that this method has no effect on the allocated capacity
    /// of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = vec![1, 2, 3];
    ///
    /// v.clear();
    ///
    /// assert!(v.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.base.clear()
    }

    /// Returns the number of elements in the vector, also referred to
    /// as its 'length'.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = vec![1, 2, 3];
    /// assert_eq!(a.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.base.len()
    }

    /// Returns `true` if the vector contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = Vec::new();
    /// assert!(v.is_empty());
    ///
    /// v.push(1);
    /// assert!(!v.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.base.is_empty()
    }

    /// Splits the collection into two at the given index.
    ///
    /// Returns a newly allocated vector containing the elements in the range
    /// `[at, len)`. After the call, the original vector will be left containing
    /// the elements `[0, at)` with its previous capacity unchanged.
    ///
    /// # Panics
    ///
    /// Panics if `at > len`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![1, 2, 3];
    /// let vec2 = vec.split_off(1);
    /// assert_eq!(vec, [1]);
    /// assert_eq!(vec2, [2, 3]);
    /// ```
    #[inline]
    #[must_use = "use `.truncate()` if you don't need the other half"]
    pub fn split_off(&mut self, at: usize) -> Self {
        Vec {
            base: self.base.split_off(at),
        }
    }

    /// Resizes the `Vec` in-place so that `len` is equal to `new_len`.
    ///
    /// If `new_len` is greater than `len`, the `Vec` is extended by the
    /// difference, with each additional slot filled with the result of
    /// calling the closure `f`. The return values from `f` will end up
    /// in the `Vec` in the order they have been generated.
    ///
    /// If `new_len` is less than `len`, the `Vec` is simply truncated.
    ///
    /// This method uses a closure to create new values on every push. If
    /// you'd rather [`Clone`] a given value, use [`Vec::resize`]. If you
    /// want to use the [`Default`] trait to generate values, you can
    /// pass [`Default::default`] as the second argument.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![1, 2, 3];
    /// vec.resize_with(5, Default::default);
    /// assert_eq!(vec, [1, 2, 3, 0, 0]);
    ///
    /// let mut vec = vec![];
    /// let mut p = 1;
    /// vec.resize_with(4, || { p *= 2; p });
    /// assert_eq!(vec, [2, 4, 8, 16]);
    /// ```
    pub fn resize_with<F>(&mut self, new_len: usize, f: F)
    where
        F: FnMut() -> T,
    {
        self.base.resize_with(new_len, f)
    }

    /// Consumes and leaks the `Vec`, returning a mutable reference to the contents,
    /// `&'a mut [T]`. Note that the type `T` must outlive the chosen lifetime
    /// `'a`. If the type has only static references, or none at all, then this
    /// may be chosen to be `'static`.
    ///
    /// As of Rust 1.57, this method does not reallocate or shrink the `Vec`,
    /// so the leaked allocation may include unused capacity that is not part
    /// of the returned slice.
    ///
    /// This function is mainly useful for data that lives for the remainder of
    /// the program's life. Dropping the returned reference will cause a memory
    /// leak.
    ///
    /// # Examples
    ///
    /// Simple usage:
    ///
    /// ```
    /// let x = vec![1, 2, 3];
    /// let static_ref: &'static mut [usize] = x.leak();
    /// static_ref[0] += 1;
    /// assert_eq!(static_ref, &[2, 2, 3]);
    /// ```
    #[inline]
    pub fn leak<'a>(self) -> &'a mut [T] {
        self.base.leak()
    }
}

impl<T: Clone> Vec<T> {
    /// Resizes the `Vec` in-place so that `len` is equal to `new_len`.
    ///
    /// If `new_len` is greater than `len`, the `Vec` is extended by the
    /// difference, with each additional slot filled with `value`.
    /// If `new_len` is less than `len`, the `Vec` is simply truncated.
    ///
    /// This method requires `T` to implement [`Clone`],
    /// in order to be able to clone the passed value.
    /// If you need more flexibility (or want to rely on [`Default`] instead of
    /// [`Clone`]), use [`Vec::resize_with`].
    /// If you only need to resize to a smaller size, use [`Vec::truncate`].
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec!["hello"];
    /// vec.resize(3, "world");
    /// assert_eq!(vec, ["hello", "world", "world"]);
    ///
    /// let mut vec = vec![1, 2, 3, 4];
    /// vec.resize(2, 0);
    /// assert_eq!(vec, [1, 2]);
    /// ```
    pub fn resize(&mut self, new_len: usize, value: T) {
        self.base.resize(new_len, value)
    }

    /// Clones and appends all elements in a slice to the `Vec`.
    ///
    /// Iterates over the slice `other`, clones each element, and then appends
    /// it to this `Vec`. The `other` slice is traversed in-order.
    ///
    /// Note that this function is same as [`extend`] except that it is
    /// specialized to work with slices instead. If and when Rust gets
    /// specialization this function will likely be deprecated (but still
    /// available).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![1];
    /// vec.extend_from_slice(&[2, 3, 4]);
    /// assert_eq!(vec, [1, 2, 3, 4]);
    /// ```
    ///
    /// [`extend`]: Vec::extend
    pub fn extend_from_slice(&mut self, other: &[T]) {
        self.base.extend_from_slice(other)
    }

    /// Copies elements from `src` range to the end of the vector.
    ///
    /// # Panics
    ///
    /// Panics if the starting point is greater than the end point or if
    /// the end point is greater than the length of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![0, 1, 2, 3, 4];
    ///
    /// vec.extend_from_within(2..);
    /// assert_eq!(vec, [0, 1, 2, 3, 4, 2, 3, 4]);
    ///
    /// vec.extend_from_within(..2);
    /// assert_eq!(vec, [0, 1, 2, 3, 4, 2, 3, 4, 0, 1]);
    ///
    /// vec.extend_from_within(4..8);
    /// assert_eq!(vec, [0, 1, 2, 3, 4, 2, 3, 4, 0, 1, 4, 2, 3, 4]);
    /// ```
    pub fn extend_from_within<R>(&mut self, src: R)
    where
        R: RangeBounds<usize>,
    {
        self.base.extend_from_within(src)
    }
}

impl<T: PartialEq> Vec<T> {
    /// Removes consecutive repeated elements in the vector according to the
    /// [`PartialEq`] trait implementation.
    ///
    /// If the vector is sorted, this removes all duplicates.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![1, 2, 2, 3, 2];
    ///
    /// vec.dedup();
    ///
    /// assert_eq!(vec, [1, 2, 3, 2]);
    /// ```
    #[inline]
    pub fn dedup(&mut self) {
        self.base.dedup()
    }
}

// ////////////////////////////////////////////////////////////////////////////////
// // Internal methods and functions
// ////////////////////////////////////////////////////////////////////////////////

// #[doc(hidden)]
// #[cfg(not(no_global_oom_handling))]
// #[stable(feature = "rust1", since = "1.0.0")]
// pub fn from_elem<T: Clone>(elem: T, n: usize) -> Vec<T> {
//     <T as SpecFromElem>::from_elem(elem, n, Global)
// }

// #[doc(hidden)]
// #[cfg(not(no_global_oom_handling))]
// #[unstable(feature = "allocator_api", issue = "32838")]
// pub fn from_elem_in<T: Clone, A: Allocator>(elem: T, n: usize, alloc: A) -> Vec<T, A> {
//     <T as SpecFromElem>::from_elem(elem, n, alloc)
// }

// trait ExtendFromWithinSpec {
//     /// # Safety
//     ///
//     /// - `src` needs to be valid index
//     /// - `self.capacity() - self.len()` must be `>= src.len()`
//     unsafe fn spec_extend_from_within(&mut self, src: Range<usize>);
// }

// impl<T: Clone, A: Allocator> ExtendFromWithinSpec for Vec<T, A> {
//     default unsafe fn spec_extend_from_within(&mut self, src: Range<usize>) {
//         // SAFETY:
//         // - len is increased only after initializing elements
//         let (this, spare, len) = unsafe { self.split_at_spare_mut_with_len() };

//         // SAFETY:
//         // - caller guaratees that src is a valid index
//         let to_clone = unsafe { this.get_unchecked(src) };

//         iter::zip(to_clone, spare)
//             .map(|(src, dst)| dst.write(src.clone()))
//             // Note:
//             // - Element was just initialized with `MaybeUninit::write`, so it's ok to increase len
//             // - len is increased after each element to prevent leaks (see issue #82533)
//             .for_each(|_| *len += 1);
//     }
// }

// impl<T: Copy, A: Allocator> ExtendFromWithinSpec for Vec<T, A> {
//     unsafe fn spec_extend_from_within(&mut self, src: Range<usize>) {
//         let count = src.len();
//         {
//             let (init, spare) = self.split_at_spare_mut();

//             // SAFETY:
//             // - caller guaratees that `src` is a valid index
//             let source = unsafe { init.get_unchecked(src) };

//             // SAFETY:
//             // - Both pointers are created from unique slice references (`&mut [_]`)
//             //   so they are valid and do not overlap.
//             // - Elements are :Copy so it's OK to to copy them, without doing
//             //   anything with the original values
//             // - `count` is equal to the len of `source`, so source is valid for
//             //   `count` reads
//             // - `.reserve(count)` guarantees that `spare.len() >= count` so spare
//             //   is valid for `count` writes
//             unsafe { ptr::copy_nonoverlapping(source.as_ptr(), spare.as_mut_ptr() as _, count) };
//         }

//         // SAFETY:
//         // - The elements were just initialized by `copy_nonoverlapping`
//         self.len += count;
//     }
// }

// ////////////////////////////////////////////////////////////////////////////////
// // Common trait implementations for Vec
// ////////////////////////////////////////////////////////////////////////////////

// #[stable(feature = "rust1", since = "1.0.0")]
// impl<T, A: Allocator> ops::Deref for Vec<T, A> {
//     type Target = [T];

//     fn deref(&self) -> &[T] {
//         unsafe { slice::from_raw_parts(self.as_ptr(), self.len) }
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// impl<T, A: Allocator> ops::DerefMut for Vec<T, A> {
//     fn deref_mut(&mut self) -> &mut [T] {
//         unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len) }
//     }
// }

// #[cfg(not(no_global_oom_handling))]
// trait SpecCloneFrom {
//     fn clone_from(this: &mut Self, other: &Self);
// }

// #[cfg(not(no_global_oom_handling))]
// impl<T: Clone, A: Allocator> SpecCloneFrom for Vec<T, A> {
//     default fn clone_from(this: &mut Self, other: &Self) {
//         // drop anything that will not be overwritten
//         this.truncate(other.len());

//         // self.len <= other.len due to the truncate above, so the
//         // slices here are always in-bounds.
//         let (init, tail) = other.split_at(this.len());

//         // reuse the contained values' allocations/resources.
//         this.clone_from_slice(init);
//         this.extend_from_slice(tail);
//     }
// }

// #[cfg(not(no_global_oom_handling))]
// impl<T: Copy, A: Allocator> SpecCloneFrom for Vec<T, A> {
//     fn clone_from(this: &mut Self, other: &Self) {
//         this.clear();
//         this.extend_from_slice(other);
//     }
// }

// #[cfg(not(no_global_oom_handling))]
// #[stable(feature = "rust1", since = "1.0.0")]
// impl<T: Clone, A: Allocator + Clone> Clone for Vec<T, A> {
//     #[cfg(not(test))]
//     fn clone(&self) -> Self {
//         let alloc = self.allocator().clone();
//         <[T]>::to_vec_in(&**self, alloc)
//     }

//     // HACK(japaric): with cfg(test) the inherent `[T]::to_vec` method, which is
//     // required for this method definition, is not available. Instead use the
//     // `slice::to_vec`  function which is only available with cfg(test)
//     // NB see the slice::hack module in slice.rs for more information
//     #[cfg(test)]
//     fn clone(&self) -> Self {
//         let alloc = self.allocator().clone();
//         crate::slice::to_vec(&**self, alloc)
//     }

//     fn clone_from(&mut self, other: &Self) {
//         SpecCloneFrom::clone_from(self, other)
//     }
// }

// /// The hash of a vector is the same as that of the corresponding slice,
// /// as required by the `core::borrow::Borrow` implementation.
// ///
// /// ```
// /// #![feature(build_hasher_simple_hash_one)]
// /// use std::hash::BuildHasher;
// ///
// /// let b = std::collections::hash_map::RandomState::new();
// /// let v: Vec<u8> = vec![0xa8, 0x3c, 0x09];
// /// let s: &[u8] = &[0xa8, 0x3c, 0x09];
// /// assert_eq!(b.hash_one(v), b.hash_one(s));
// /// ```
// #[stable(feature = "rust1", since = "1.0.0")]
// impl<T: Hash, A: Allocator> Hash for Vec<T, A> {
//     #[inline]
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         Hash::hash(&**self, state)
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// #[rustc_on_unimplemented(
//     message = "vector indices are of type `usize` or ranges of `usize`",
//     label = "vector indices are of type `usize` or ranges of `usize`"
// )]
// impl<T, I: SliceIndex<[T]>, A: Allocator> Index<I> for Vec<T, A> {
//     type Output = I::Output;

//     #[inline]
//     fn index(&self, index: I) -> &Self::Output {
//         Index::index(&**self, index)
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// #[rustc_on_unimplemented(
//     message = "vector indices are of type `usize` or ranges of `usize`",
//     label = "vector indices are of type `usize` or ranges of `usize`"
// )]
// impl<T, I: SliceIndex<[T]>, A: Allocator> IndexMut<I> for Vec<T, A> {
//     #[inline]
//     fn index_mut(&mut self, index: I) -> &mut Self::Output {
//         IndexMut::index_mut(&mut **self, index)
//     }
// }

// #[cfg(not(no_global_oom_handling))]
// #[stable(feature = "rust1", since = "1.0.0")]
// impl<T> FromIterator<T> for Vec<T> {
//     #[inline]
//     fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vec<T> {
//         <Self as SpecFromIter<T, I::IntoIter>>::from_iter(iter.into_iter())
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// impl<T, A: Allocator> IntoIterator for Vec<T, A> {
//     type Item = T;
//     type IntoIter = IntoIter<T, A>;

//     /// Creates a consuming iterator, that is, one that moves each value out of
//     /// the vector (from start to end). The vector cannot be used after calling
//     /// this.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// let v = vec!["a".to_string(), "b".to_string()];
//     /// for s in v.into_iter() {
//     ///     // s has type String, not &String
//     ///     println!("{}", s);
//     /// }
//     /// ```
//     #[inline]
//     fn into_iter(self) -> IntoIter<T, A> {
//         unsafe {
//             let mut me = ManuallyDrop::new(self);
//             let alloc = ptr::read(me.allocator());
//             let begin = me.as_mut_ptr();
//             let end = if mem::size_of::<T>() == 0 {
//                 arith_offset(begin as *const i8, me.len() as isize) as *const T
//             } else {
//                 begin.add(me.len()) as *const T
//             };
//             let cap = me.buf.capacity();
//             IntoIter {
//                 buf: NonNull::new_unchecked(begin),
//                 phantom: PhantomData,
//                 cap,
//                 alloc,
//                 ptr: begin,
//                 end,
//             }
//         }
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// impl<'a, T, A: Allocator> IntoIterator for &'a Vec<T, A> {
//     type Item = &'a T;
//     type IntoIter = slice::Iter<'a, T>;

//     fn into_iter(self) -> slice::Iter<'a, T> {
//         self.iter()
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// impl<'a, T, A: Allocator> IntoIterator for &'a mut Vec<T, A> {
//     type Item = &'a mut T;
//     type IntoIter = slice::IterMut<'a, T>;

//     fn into_iter(self) -> slice::IterMut<'a, T> {
//         self.iter_mut()
//     }
// }

// #[cfg(not(no_global_oom_handling))]
// #[stable(feature = "rust1", since = "1.0.0")]
// impl<T, A: Allocator> Extend<T> for Vec<T, A> {
//     #[inline]
//     fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
//         <Self as SpecExtend<T, I::IntoIter>>::spec_extend(self, iter.into_iter())
//     }

//     #[inline]
//     fn extend_one(&mut self, item: T) {
//         self.push(item);
//     }

//     #[inline]
//     fn extend_reserve(&mut self, additional: usize) {
//         self.reserve(additional);
//     }
// }

// impl<T, A: Allocator> Vec<T, A> {
//     // leaf method to which various SpecFrom/SpecExtend implementations delegate when
//     // they have no further optimizations to apply
//     #[cfg(not(no_global_oom_handling))]
//     fn extend_desugared<I: Iterator<Item = T>>(&mut self, mut iterator: I) {
//         // This is the case for a general iterator.
//         //
//         // This function should be the moral equivalent of:
//         //
//         //      for item in iterator {
//         //          self.push(item);
//         //      }
//         while let Some(element) = iterator.next() {
//             let len = self.len();
//             if len == self.capacity() {
//                 let (lower, _) = iterator.size_hint();
//                 self.reserve(lower.saturating_add(1));
//             }
//             unsafe {
//                 ptr::write(self.as_mut_ptr().add(len), element);
//                 // Since next() executes user code which can panic we have to bump the length
//                 // after each step.
//                 // NB can't overflow since we would have had to alloc the address space
//                 self.set_len(len + 1);
//             }
//         }
//     }

//     /// Creates a splicing iterator that replaces the specified range in the vector
//     /// with the given `replace_with` iterator and yields the removed items.
//     /// `replace_with` does not need to be the same length as `range`.
//     ///
//     /// `range` is removed even if the iterator is not consumed until the end.
//     ///
//     /// It is unspecified how many elements are removed from the vector
//     /// if the `Splice` value is leaked.
//     ///
//     /// The input iterator `replace_with` is only consumed when the `Splice` value is dropped.
//     ///
//     /// This is optimal if:
//     ///
//     /// * The tail (elements in the vector after `range`) is empty,
//     /// * or `replace_with` yields fewer or equal elements than `range`’s length
//     /// * or the lower bound of its `size_hint()` is exact.
//     ///
//     /// Otherwise, a temporary vector is allocated and the tail is moved twice.
//     ///
//     /// # Panics
//     ///
//     /// Panics if the starting point is greater than the end point or if
//     /// the end point is greater than the length of the vector.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// let mut v = vec![1, 2, 3, 4];
//     /// let new = [7, 8, 9];
//     /// let u: Vec<_> = v.splice(1..3, new).collect();
//     /// assert_eq!(v, &[1, 7, 8, 9, 4]);
//     /// assert_eq!(u, &[2, 3]);
//     /// ```
//     #[cfg(not(no_global_oom_handling))]
//     #[inline]
//     #[stable(feature = "vec_splice", since = "1.21.0")]
//     pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, A>
//     where
//         R: RangeBounds<usize>,
//         I: IntoIterator<Item = T>,
//     {
//         Splice {
//             drain: self.drain(range),
//             replace_with: replace_with.into_iter(),
//         }
//     }

//     /// Creates an iterator which uses a closure to determine if an element should be removed.
//     ///
//     /// If the closure returns true, then the element is removed and yielded.
//     /// If the closure returns false, the element will remain in the vector and will not be yielded
//     /// by the iterator.
//     ///
//     /// Using this method is equivalent to the following code:
//     ///
//     /// ```
//     /// # let some_predicate = |x: &mut i32| { *x == 2 || *x == 3 || *x == 6 };
//     /// # let mut vec = vec![1, 2, 3, 4, 5, 6];
//     /// let mut i = 0;
//     /// while i < vec.len() {
//     ///     if some_predicate(&mut vec[i]) {
//     ///         let val = vec.remove(i);
//     ///         // your code here
//     ///     } else {
//     ///         i += 1;
//     ///     }
//     /// }
//     ///
//     /// # assert_eq!(vec, vec![1, 4, 5]);
//     /// ```
//     ///
//     /// But `drain_filter` is easier to use. `drain_filter` is also more efficient,
//     /// because it can backshift the elements of the array in bulk.
//     ///
//     /// Note that `drain_filter` also lets you mutate every element in the filter closure,
//     /// regardless of whether you choose to keep or remove it.
//     ///
//     /// # Examples
//     ///
//     /// Splitting an array into evens and odds, reusing the original allocation:
//     ///
//     /// ```
//     /// #![feature(drain_filter)]
//     /// let mut numbers = vec![1, 2, 3, 4, 5, 6, 8, 9, 11, 13, 14, 15];
//     ///
//     /// let evens = numbers.drain_filter(|x| *x % 2 == 0).collect::<Vec<_>>();
//     /// let odds = numbers;
//     ///
//     /// assert_eq!(evens, vec![2, 4, 6, 8, 14]);
//     /// assert_eq!(odds, vec![1, 3, 5, 9, 11, 13, 15]);
//     /// ```
//     #[unstable(feature = "drain_filter", reason = "recently added", issue = "43244")]
//     pub fn drain_filter<F>(&mut self, filter: F) -> DrainFilter<'_, T, F, A>
//     where
//         F: FnMut(&mut T) -> bool,
//     {
//         let old_len = self.len();

//         // Guard against us getting leaked (leak amplification)
//         unsafe {
//             self.set_len(0);
//         }

//         DrainFilter {
//             vec: self,
//             idx: 0,
//             del: 0,
//             old_len,
//             pred: filter,
//             panic_flag: false,
//         }
//     }
// }

// /// Extend implementation that copies elements out of references before pushing them onto the Vec.
// ///
// /// This implementation is specialized for slice iterators, where it uses [`copy_from_slice`] to
// /// append the entire slice at once.
// ///
// /// [`copy_from_slice`]: slice::copy_from_slice
// #[cfg(not(no_global_oom_handling))]
// #[stable(feature = "extend_ref", since = "1.2.0")]
// impl<'a, T: Copy + 'a, A: Allocator + 'a> Extend<&'a T> for Vec<T, A> {
//     fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
//         self.spec_extend(iter.into_iter())
//     }

//     #[inline]
//     fn extend_one(&mut self, &item: &'a T) {
//         self.push(item);
//     }

//     #[inline]
//     fn extend_reserve(&mut self, additional: usize) {
//         self.reserve(additional);
//     }
// }

// /// Implements comparison of vectors, [lexicographically](core::cmp::Ord#lexicographical-comparison).
// #[stable(feature = "rust1", since = "1.0.0")]
// impl<T: PartialOrd, A: Allocator> PartialOrd for Vec<T, A> {
//     #[inline]
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         PartialOrd::partial_cmp(&**self, &**other)
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// impl<T: Eq, A: Allocator> Eq for Vec<T, A> {}

// /// Implements ordering of vectors, [lexicographically](core::cmp::Ord#lexicographical-comparison).
// #[stable(feature = "rust1", since = "1.0.0")]
// impl<T: Ord, A: Allocator> Ord for Vec<T, A> {
//     #[inline]
//     fn cmp(&self, other: &Self) -> Ordering {
//         Ord::cmp(&**self, &**other)
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// unsafe impl<#[may_dangle] T, A: Allocator> Drop for Vec<T, A> {
//     fn drop(&mut self) {
//         unsafe {
//             // use drop for [T]
//             // use a raw slice to refer to the elements of the vector as weakest necessary type;
//             // could avoid questions of validity in certain cases
//             ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.as_mut_ptr(), self.len))
//         }
//         // RawVec handles deallocation
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// #[rustc_const_unstable(feature = "const_default_impls", issue = "87864")]
// impl<T> const Default for Vec<T> {
//     /// Creates an empty `Vec<T>`.
//     fn default() -> Vec<T> {
//         Vec::new()
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// impl<T: fmt::Debug, A: Allocator> fmt::Debug for Vec<T, A> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         fmt::Debug::fmt(&**self, f)
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// impl<T, A: Allocator> AsRef<Vec<T, A>> for Vec<T, A> {
//     fn as_ref(&self) -> &Vec<T, A> {
//         self
//     }
// }

// #[stable(feature = "vec_as_mut", since = "1.5.0")]
// impl<T, A: Allocator> AsMut<Vec<T, A>> for Vec<T, A> {
//     fn as_mut(&mut self) -> &mut Vec<T, A> {
//         self
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// impl<T, A: Allocator> AsRef<[T]> for Vec<T, A> {
//     fn as_ref(&self) -> &[T] {
//         self
//     }
// }

// #[stable(feature = "vec_as_mut", since = "1.5.0")]
// impl<T, A: Allocator> AsMut<[T]> for Vec<T, A> {
//     fn as_mut(&mut self) -> &mut [T] {
//         self
//     }
// }

// #[cfg(not(no_global_oom_handling))]
// #[stable(feature = "rust1", since = "1.0.0")]
// impl<T: Clone> From<&[T]> for Vec<T> {
//     /// Allocate a `Vec<T>` and fill it by cloning `s`'s items.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// assert_eq!(Vec::from(&[1, 2, 3][..]), vec![1, 2, 3]);
//     /// ```
//     #[cfg(not(test))]
//     fn from(s: &[T]) -> Vec<T> {
//         s.to_vec()
//     }
//     #[cfg(test)]
//     fn from(s: &[T]) -> Vec<T> {
//         crate::slice::to_vec(s, Global)
//     }
// }

// #[cfg(not(no_global_oom_handling))]
// #[stable(feature = "vec_from_mut", since = "1.19.0")]
// impl<T: Clone> From<&mut [T]> for Vec<T> {
//     /// Allocate a `Vec<T>` and fill it by cloning `s`'s items.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// assert_eq!(Vec::from(&mut [1, 2, 3][..]), vec![1, 2, 3]);
//     /// ```
//     #[cfg(not(test))]
//     fn from(s: &mut [T]) -> Vec<T> {
//         s.to_vec()
//     }
//     #[cfg(test)]
//     fn from(s: &mut [T]) -> Vec<T> {
//         crate::slice::to_vec(s, Global)
//     }
// }

// #[cfg(not(no_global_oom_handling))]
// #[stable(feature = "vec_from_array", since = "1.44.0")]
// impl<T, const N: usize> From<[T; N]> for Vec<T> {
//     #[cfg(not(test))]
//     fn from(s: [T; N]) -> Vec<T> {
//         <[T]>::into_vec(box s)
//     }
//     /// Allocate a `Vec<T>` and move `s`'s items into it.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// assert_eq!(Vec::from([1, 2, 3]), vec![1, 2, 3]);
//     /// ```
//     #[cfg(test)]
//     fn from(s: [T; N]) -> Vec<T> {
//         crate::slice::into_vec(box s)
//     }
// }

// #[stable(feature = "vec_from_cow_slice", since = "1.14.0")]
// impl<'a, T> From<Cow<'a, [T]>> for Vec<T>
// where
//     [T]: ToOwned<Owned = Vec<T>>,
// {
//     /// Convert a clone-on-write slice into a vector.
//     ///
//     /// If `s` already owns a `Vec<T>`, it will be returned directly.
//     /// If `s` is borrowing a slice, a new `Vec<T>` will be allocated and
//     /// filled by cloning `s`'s items into it.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// # use std::borrow::Cow;
//     /// let o: Cow<[i32]> = Cow::Owned(vec![1, 2, 3]);
//     /// let b: Cow<[i32]> = Cow::Borrowed(&[1, 2, 3]);
//     /// assert_eq!(Vec::from(o), Vec::from(b));
//     /// ```
//     fn from(s: Cow<'a, [T]>) -> Vec<T> {
//         s.into_owned()
//     }
// }

// // note: test pulls in libstd, which causes errors here
// #[cfg(not(test))]
// #[stable(feature = "vec_from_box", since = "1.18.0")]
// impl<T, A: Allocator> From<Box<[T], A>> for Vec<T, A> {
//     /// Convert a boxed slice into a vector by transferring ownership of
//     /// the existing heap allocation.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// let b: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
//     /// assert_eq!(Vec::from(b), vec![1, 2, 3]);
//     /// ```
//     fn from(s: Box<[T], A>) -> Self {
//         s.into_vec()
//     }
// }

// // note: test pulls in libstd, which causes errors here
// #[cfg(not(no_global_oom_handling))]
// #[cfg(not(test))]
// #[stable(feature = "box_from_vec", since = "1.20.0")]
// impl<T, A: Allocator> From<Vec<T, A>> for Box<[T], A> {
//     /// Convert a vector into a boxed slice.
//     ///
//     /// If `v` has excess capacity, its items will be moved into a
//     /// newly-allocated buffer with exactly the right capacity.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// assert_eq!(Box::from(vec![1, 2, 3]), vec![1, 2, 3].into_boxed_slice());
//     /// ```
//     fn from(v: Vec<T, A>) -> Self {
//         v.into_boxed_slice()
//     }
// }

// #[cfg(not(no_global_oom_handling))]
// #[stable(feature = "rust1", since = "1.0.0")]
// impl From<&str> for Vec<u8> {
//     /// Allocate a `Vec<u8>` and fill it with a UTF-8 string.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// assert_eq!(Vec::from("123"), vec![b'1', b'2', b'3']);
//     /// ```
//     fn from(s: &str) -> Vec<u8> {
//         From::from(s.as_bytes())
//     }
// }

// #[stable(feature = "array_try_from_vec", since = "1.48.0")]
// impl<T, A: Allocator, const N: usize> TryFrom<Vec<T, A>> for [T; N] {
//     type Error = Vec<T, A>;

//     /// Gets the entire contents of the `Vec<T>` as an array,
//     /// if its size exactly matches that of the requested array.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use std::convert::TryInto;
//     /// assert_eq!(vec![1, 2, 3].try_into(), Ok([1, 2, 3]));
//     /// assert_eq!(<Vec<i32>>::new().try_into(), Ok([]));
//     /// ```
//     ///
//     /// If the length doesn't match, the input comes back in `Err`:
//     /// ```
//     /// use std::convert::TryInto;
//     /// let r: Result<[i32; 4], _> = (0..10).collect::<Vec<_>>().try_into();
//     /// assert_eq!(r, Err(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
//     /// ```
//     ///
//     /// If you're fine with just getting a prefix of the `Vec<T>`,
//     /// you can call [`.truncate(N)`](Vec::truncate) first.
//     /// ```
//     /// use std::convert::TryInto;
//     /// let mut v = String::from("hello world").into_bytes();
//     /// v.sort();
//     /// v.truncate(2);
//     /// let [a, b]: [_; 2] = v.try_into().unwrap();
//     /// assert_eq!(a, b' ');
//     /// assert_eq!(b, b'd');
//     /// ```
//     fn try_from(mut vec: Vec<T, A>) -> Result<[T; N], Vec<T, A>> {
//         if vec.len() != N {
//             return Err(vec);
//         }

//         // SAFETY: `.set_len(0)` is always sound.
//         unsafe { vec.set_len(0) };

//         // SAFETY: A `Vec`'s pointer is always aligned properly, and
//         // the alignment the array needs is the same as the items.
//         // We checked earlier that we have sufficient items.
//         // The items will not double-drop as the `set_len`
//         // tells the `Vec` not to also drop them.
//         let array = unsafe { ptr::read(vec.as_ptr() as *const [T; N]) };
//         Ok(array)
//     }
// }
