use std::fmt::Write;

use crate::{EnumData, VariantData};

pub fn push_enum_defs<T>(str: &mut T, ident: &str, enum_data: &[EnumData])
where
    T: Write,
{
    let _ = write!(
        str,
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, FromPrimitive, ToPrimitive, Hash)]pub enum {ident} {{"
    );

    for data in enum_data {
        let _ = write!(str, "{},", data.enum_name);
    }

    let _ = write!(
        str,
        "}}impl {ident} {{pub const fn variant_count() -> usize {{{}}}pub fn as_str(&self) -> &str {{match self {{",
        enum_data.len()
    );

    for data in enum_data {
        let _ = write!(str, "{ident}::{} => \"{}\",", data.enum_name, data.display_name);
    }

    let _ = write!(str, "}}}}}}");
}

pub fn push_variant_defs<T>(str: &mut T, variant_data: &[VariantData])
where
    T: Write,
{
    let _ = write!(
        str,
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, FromPrimitive, ToPrimitive, Hash)]pub enum Variant {{Base,"
    );

    for variant in variant_data {
        let _ = write!(str, "{},", variant.enum_name);
    }

    let _ = write!(
        str,
        "}}impl Variant {{pub const fn variant_count() -> usize {{{}}}pub fn as_i(&self) -> u8 {{match self {{Variant::Base => 0,",
        variant_data.len()
    );

    for variant in variant_data {
        let enum_name = &variant.enum_name;
        let i = variant.i;

        let _ = write!(str, "Variant::{enum_name} => {i},");
    }

    let _ = write!(str, "}}}}pub fn as_normal(&self) -> Option<Hero> {{match self {{Variant::Base => None,");

    for variant in variant_data {
        let enum_name = &variant.enum_name;
        let base = &variant.base;
        let is_villain = variant.is_villain;

        let _ = write!(str, "Variant::{enum_name} => {},", if is_villain { String::from("None") } else { format!("Some(Hero::{base})") });
    }

    let _ = write!(str, "}}}}pub fn from_hero(hero: Hero, i: u32) -> Option<Variant> {{match (hero, i) {{");

    for variant in variant_data {
        let enum_name = &variant.enum_name;
        let i = variant.i;
        let base = &variant.base;
        let is_villain = variant.is_villain;

        if !is_villain {
            let _ = write!(str, "(Hero::{base}, {i}) => Some(Variant::{enum_name}),");
        }
    }

    let _ = write!(str, "_ => None}}}}pub fn as_str(&self) -> &str {{match self {{Variant::Base => \"Base\",");

    for variant in variant_data {
        let _ = write!(str, "Variant::{} => \"{}\",", variant.enum_name, variant.display_name);
    }

    let _ = write!(str, "}}}}pub fn as_desc(&self) -> &str {{match self {{");

    for variant in variant_data {
        let enum_name = &variant.enum_name;
        let unlock_desc = &variant.unlock_desc;

        if let Some(desc) = unlock_desc {
            let _ = write!(str, "Variant::{enum_name} => \"{desc}\",");
        }
    }

    let _ = write!(str, "_ => \"\",}}}}pub fn can_unlock(&self, items: &Items) -> bool {{match self {{");
    
    for variant in variant_data {
        let enum_name = &variant.enum_name;

        if let Some(logic) = &variant.logic {
            let _ = write!(str, "Variant::{enum_name} => {},", logic.as_rust_expr());
        }
    }
    
    let _ = write!(str, "_ => false,}}}}}}");
}
