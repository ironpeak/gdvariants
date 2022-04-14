use std::borrow::{Borrow, BorrowMut};

use gdnative::{
    core_types::VariantArray,
    export::{Export, ExportInfo},
    prelude::{FromVariant, FromVariantError, OwnedToVariant, ToVariant, Variant},
};

pub struct Vec<T> {
    pub(crate) base: std::vec::Vec<T>,
}

pub enum NoHint {}

impl<T> Export for Vec<T>
where
    T: ToVariant,
{
    type Hint = NoHint;

    fn export_info(_hint: Option<Self::Hint>) -> ExportInfo {
        ExportInfo::new(gdnative::core_types::VariantType::VariantArray)
    }
}

impl<T> FromVariant for Vec<T>
where
    T: FromVariant,
{
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        let array = VariantArray::from_variant(variant)?;
        let mut vec: Vec<T> = Vec::with_capacity(array.len() as usize);
        for variant in array.iter() {
            let value = T::from_variant(&variant)?;
            vec.push(value);
        }
        Ok(vec)
    }
}

impl<T> ToVariant for Vec<T>
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

impl<T> Borrow<std::vec::Vec<T>> for Vec<T> {
    fn borrow(&self) -> &std::vec::Vec<T> {
        &self.base
    }
}

impl<T> BorrowMut<std::vec::Vec<T>> for Vec<T> {
    fn borrow_mut(&mut self) -> &mut std::vec::Vec<T> {
        &mut self.base
    }
}

impl<T> From<std::vec::Vec<T>> for Vec<T> {
    fn from(vec: std::vec::Vec<T>) -> Vec<T> {
        Vec { base: vec }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::*;
    use std::vec::*;

    macro_rules! __test_method {
        ($name:ident, [$($args:tt)*]) => {
            paste::item! {
                #[test]
                fn [< test_ $name >] () {
                    let stdvec = vec![2, 1, 3];
                    let cratevec = Vec::from(vec![2, 1, 3]);

                    let stdres = stdvec.[< $name >]($($args)*);
                    let crateres = cratevec.[< $name >]($($args)*);

                    assert_eq!(stdres, crateres);
                }
            }
        };
    }

    __test_method!(index, [0]);
}
