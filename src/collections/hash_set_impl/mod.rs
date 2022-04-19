#[allow(clippy::module_inception)]
mod hash_set_impl;
mod hash_set_impl_bit_and;
mod hash_set_impl_bit_or;
mod hash_set_impl_bit_xor;
mod hash_set_impl_clone;
mod hash_set_impl_debug;
mod hash_set_impl_default;
mod hash_set_impl_eq;
mod hash_set_impl_extend;
mod hash_set_impl_from;
mod hash_set_impl_from_iterator;
mod hash_set_impl_into_iterator;
mod hash_set_impl_partial_eq;
mod hash_set_impl_sub;

#[cfg(feature = "serde")]
pub mod serde;
