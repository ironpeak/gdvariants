#[allow(clippy::module_inception)]
mod vec;
mod vec_impl;
mod vec_impl_as_mut;
mod vec_impl_as_ref;
mod vec_impl_borrow;
mod vec_impl_borrow_mut;
mod vec_impl_clone;
mod vec_impl_debug;
mod vec_impl_default;
mod vec_impl_deref;
mod vec_impl_deref_mut;
mod vec_impl_eq;
mod vec_impl_extend;
mod vec_impl_from;
mod vec_impl_from_iterator;
mod vec_impl_hash;
mod vec_impl_into_iterator;
mod vec_impl_ord;
mod vec_impl_partial_eq;
mod vec_impl_partial_ord;
mod vec_impl_slice_index;

pub use vec::Vec;
