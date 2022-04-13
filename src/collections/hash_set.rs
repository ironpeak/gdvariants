use std::{
    borrow::{Borrow, BorrowMut},
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash},
};

use gdnative::{
    core_types::VariantArray,
    export::{Export, ExportInfo},
    prelude::{FromVariant, FromVariantError, OwnedToVariant, ToVariant, Variant},
};

/// A [hash set] implemented as a `HashMap` where the value is `()`.
///
/// As with the [`HashMap`] type, a `HashSet` requires that the elements
/// implement the [`Eq`] and [`Hash`] traits. This can frequently be achieved by
/// using `#[derive(PartialEq, Eq, Hash)]`. If you implement these yourself,
/// it is important that the following property holds:
///
/// ```text
/// k1 == k2 -> hash(k1) == hash(k2)
/// ```
///
/// In other words, if two keys are equal, their hashes must be equal.
///
///
/// It is a logic error for an item to be modified in such a way that the
/// item's hash, as determined by the [`Hash`] trait, or its equality, as
/// determined by the [`Eq`] trait, changes while it is in the set. This is
/// normally only possible through [`Cell`], [`RefCell`], global state, I/O, or
/// unsafe code. The behavior resulting from such a logic error is not
/// specified (it could include panics, incorrect results, aborts, memory
/// leaks, or non-termination) but will not be undefined behavior.
///
/// # Examples
///
/// ```
/// use gdvariants::collections::HashSet;
/// // Type inference lets us omit an explicit type signature (which
/// // would be `HashSet<String>` in this example).
/// let mut books = HashSet::new();
///
/// // Add some books.
/// books.insert("A Dance With Dragons".to_string());
/// books.insert("To Kill a Mockingbird".to_string());
/// books.insert("The Odyssey".to_string());
/// books.insert("The Great Gatsby".to_string());
///
/// // Check for a specific one.
/// if !books.contains("The Winds of Winter") {
///     println!("We have {} books, but The Winds of Winter ain't one.",
///              books.len());
/// }
///
/// // Remove a book.
/// books.remove("The Odyssey");
///
/// // Iterate over everything.
/// for book in &books {
///     println!("{}", book);
/// }
/// ```
///
/// The easiest way to use `HashSet` with a custom type is to derive
/// [`Eq`] and [`Hash`]. We must also derive [`PartialEq`], this will in the
/// future be implied by [`Eq`].
///
/// ```
/// use gdvariants::collections::HashSet;
/// #[derive(Hash, Eq, PartialEq, Debug)]
/// struct Viking {
///     name: String,
///     power: usize,
/// }
///
/// let mut vikings = HashSet::new();
///
/// vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
/// vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
/// vikings.insert(Viking { name: "Olaf".to_string(), power: 4 });
/// vikings.insert(Viking { name: "Harald".to_string(), power: 8 });
///
/// // Use derived implementation to print the vikings.
/// for x in &vikings {
///     println!("{:?}", x);
/// }
/// ```
///
/// A `HashSet` with a known list of items can be initialized from an array:
///
/// ```
/// use gdvariants::collections::HashSet;
///
/// let viking_names = HashSet::from(["Einar", "Olaf", "Harald"]);
/// ```
///
/// [hash set]: crate::collections#use-the-set-variant-of-any-of-these-maps-when
/// [`HashMap`]: crate::collections::HashMap
pub struct HashSet<T, S = RandomState> {
    pub(crate) base: std::collections::HashSet<T, S>,
}

pub enum NoHint {}

impl<T> Export for HashSet<T>
where
    T: ToVariant,
{
    type Hint = NoHint;

    fn export_info(_hint: Option<Self::Hint>) -> ExportInfo {
        ExportInfo::new(gdnative::core_types::VariantType::VariantArray)
    }
}

impl<T, S> FromVariant for HashSet<T, S>
where
    T: Eq + Hash + FromVariant,
    S: BuildHasher + Default,
{
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        let array = VariantArray::from_variant(variant)?;
        let mut hash_set: HashSet<T, S> =
            HashSet::with_capacity_and_hasher(array.len() as usize, S::default());
        for variant_value in array.iter() {
            let value = T::from_variant(&variant_value)?;
            hash_set.insert(value);
        }
        Ok(hash_set)
    }
}

impl<T, S> ToVariant for HashSet<T, S>
where
    T: ToVariant,
{
    fn to_variant(&self) -> Variant {
        let array = VariantArray::new();
        for value in &self.base {
            array.push(value.to_variant());
        }
        array.owned_to_variant()
    }
}

impl<T, S> Borrow<std::collections::HashSet<T, S>> for HashSet<T, S> {
    fn borrow(&self) -> &std::collections::HashSet<T, S> {
        &self.base
    }
}

impl<T, S> BorrowMut<std::collections::HashSet<T, S>> for HashSet<T, S> {
    fn borrow_mut(&mut self) -> &mut std::collections::HashSet<T, S> {
        &mut self.base
    }
}
