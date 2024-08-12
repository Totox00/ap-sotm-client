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

    for data in variant_data {
        let _ = write!(str, "{},", data.enum_name);
    }

    let _ = write!(
        str,
        "}}impl Variant {{pub const fn variant_count() -> usize {{{}}}pub fn as_i(&self) -> u8 {{match self {{Variant::Base => 0,",
        variant_data.len()
    );

    for VariantData {
        enum_name,
        base: _,
        display_name: _,
        unlock_desc: _,
        i,
        base_i: _,
    } in variant_data
    {
        let _ = write!(str, "Variant::{enum_name} => {i},");
    }

    let _ = write!(str, "}}}}pub fn as_normal(&self) -> Option<Hero> {{match self {{Variant::Base => None,");

    for VariantData {
        enum_name,
        base,
        unlock_desc: _,
        display_name: _,
        i: _,
        base_i: _,
    } in variant_data
    {
        let _ = write!(str, "Variant::{enum_name} => {},", if base == "Villain" { String::from("None") } else { format!("Some(Hero::{base})") });
    }

    let _ = write!(str, "}}}}pub fn from_hero(hero: Hero, i: u32) -> Option<Variant> {{match (hero, i) {{");

    for VariantData {
        base,
        i,
        enum_name,
        display_name: _,
        unlock_desc: _,
        base_i: _,
    } in variant_data
    {
        if base.as_str() != "Villain" {
            let _ = write!(str, "(Hero::{base}, {i}) => Some(Variant::{enum_name}),");
        }
    }

    let _ = write!(str, "_ => None}}}}pub fn as_str(&self) -> &str {{match self {{Variant::Base => \"Base\",");

    for data in variant_data {
        let _ = write!(str, "Variant::{} => \"{}\",", data.enum_name, data.display_name);
    }

    let _ = write!(str, "}}}}pub fn as_desc(&self) -> &str {{match self {{");

    for VariantData {
        enum_name,
        base: _,
        unlock_desc,
        display_name: _,
        i: _,
        base_i: _,
    } in variant_data
    {
        if let Some(desc) = unlock_desc {
            let _ = write!(str, "Variant::{enum_name} => \"{desc}\",");
        }
    }

    let _ = write!(str, "_ => \"\",}}}}}}");
}
