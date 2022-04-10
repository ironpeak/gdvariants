// Source: https://doc.rust-lang.org/src/alloc/vec/mod.rs.html

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
