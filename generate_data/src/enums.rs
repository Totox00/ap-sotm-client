use std::fmt::Write;

use crate::{Data, EnumData, VariantData, VillainData};

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
        "}}impl {ident} {{pub const fn variant_count() -> usize {{{}}}pub fn as_str(&self) -> &'static str {{match self {{",
        enum_data.len()
    );

    for data in enum_data {
        let _ = write!(str, "{ident}::{} => \"{}\",", data.enum_name, data.display_name);
    }

    let _ = write!(str, "}}}}pub fn as_ident(&self) -> &'static str {{match self {{",);

    for data in enum_data {
        let _ = write!(str, "{ident}::{} => \"{}\",", data.enum_name, data.enum_name);
    }

    let _ = write!(str, "}}}}}}");
}

pub fn push_villain_defs<T>(str: &mut T, ident: &str, enum_data: &[VillainData], villain_variant_data: &[&VariantData])
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
        "}}impl {ident} {{pub const fn variant_count() -> usize {{{}}}pub fn as_str(&self) -> &'static str {{match self {{",
        enum_data.len()
    );

    for data in enum_data {
        let _ = write!(str, "{ident}::{} => \"{}\",", data.enum_name, data.display_name);
    }

    let _ = write!(str, "}}}}pub fn as_ident(&self) -> &'static str {{match self {{",);

    for data in enum_data {
        let _ = write!(str, "{ident}::{} => \"{}\",", data.enum_name, data.enum_name);
    }

    let _ = write!(str, "}}}}pub fn no_challenge(&self) -> bool {{match self {{");

    for data in enum_data {
        let _ = write!(str, "{ident}::{} => {},", data.enum_name, data.challenge.is_none());
    }

    let _ = write!(str, "}}}}pub fn challenge_desc(&self) -> Option<(&str, &[&str])> {{match self {{");

    for data in enum_data {
        if let Some((name, desc)) = &data.challenge {
            let _ = write!(str, "{ident}::{} => Some((\"{}\", &[\"{}\"])),", data.enum_name, name, desc.join("\",\""));
        }
    }

    let _ = write!(str, "_=>None}}}}pub fn can_do_challenge(&self, items: &Items) -> bool {{match self {{");

    for data in enum_data {
        let enum_name = &data.enum_name;

        if let Some(logic) = &data.challenge_req {
            let _ = write!(str, "{ident}::{enum_name} => {},", logic.as_rust_expr());
        }
    }

    let _ = write!(str, "_=>true}}}}pub fn variants(&self) -> &[Self] {{match self {{");

    for data in enum_data {
        let enum_name = &data.enum_name;

        let variants: Vec<_> = villain_variant_data
            .iter()
            .filter(|variant_data| variant_data.base == data.enum_name)
            .map(|data| format!("{ident}::{}", data.enum_name))
            .collect();

        let _ = write!(str, "{ident}::{enum_name} => &[{ident}::{enum_name},{}],", variants.join(","));
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

    let _ = write!(str, "_ => None}}}}pub fn as_str(&self) -> &'static str {{match self {{");

    for variant in variant_data {
        let _ = write!(str, "Variant::{} => \"{}\",", variant.enum_name, variant.display_name);
    }

    let _ = write!(str, "Variant::Base => \"Base\"}}}}pub fn as_ident(&self) -> &'static str {{match self {{",);

    for data in variant_data {
        let _ = write!(str, "Variant::{} => \"{}\",", data.enum_name, data.enum_name);
    }

    let _ = write!(str, "Variant::Base => \"Base\"}}}}pub fn as_desc(&self) -> &str {{match self {{");

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

macro_rules! write_data {
    ($str:ident, $data:expr, $ident:expr) => {
        for item in $data {
            let _ = write!($str, "\"{}\" => Some(Item::{}({}::{})),", item.enum_name, $ident, $ident, item.enum_name);
        }
    };
}

pub fn push_from_ident<T>(str: &mut T, data: &Data)
where
    T: Write,
{
    let _ = write!(str, "impl Item {{pub fn from_ident(ident: &str) -> Option<Item> {{match ident {{");

    write_data!(str, &data.heroes, "Hero");
    write_data!(str, &data.contenders, "Contender");
    write_data!(str, data.hero_variants(), "Variant");
    write_data!(str, data.villain_variants(), "Villain");
    write_data!(str, &data.villains, "Villain");
    write_data!(str, &data.team_villains, "TeamVillain");
    write_data!(str, &data.gladiators, "Gladiator");
    write_data!(str, &data.environments, "Environment");

    let _ = write!(str, "_ => None}}}}}}");
}

pub fn push_hero_variants<T>(str: &mut T, data: &Data)
where
    T: Write,
{
    let _ = write!(str, "impl Hero {{pub fn variants(&self) -> Box<dyn Iterator<Item = Variant>> {{match self {{");

    for hero in &data.heroes {
        let _ = write!(str, "Hero::{} => Box::new([", hero.enum_name);

        for variant in &data.variants {
            if !variant.is_villain && variant.base_i == hero.i {
                let _ = write!(str, "Variant::{},", variant.enum_name);
            }
        }

        let _ = write!(str, "].into_iter()),");
    }

    let _ = write!(str, "}}}}}}");
}

pub fn push_villain_variants<T>(str: &mut T, data: &Data)
where
    T: Write,
{
    let _ = write!(str, "impl Villain {{pub fn variant(&self) -> Option<Variant> {{match self {{");

    for variant in data.villain_variants() {
        let _ = write!(str, "Villain::{} => Some(Variant::{}),", variant.enum_name, variant.enum_name);
    }

    let _ = write!(str, "_ => None}}}}}}");
}
