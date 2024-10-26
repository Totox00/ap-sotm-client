use crate::{Data, FillerData, FillerType};
use std::fmt::Write;

pub fn push_filler<T>(str: &mut T, data: &Data)
where
    T: Write,
{
    let _ = write!(str, "#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]pub enum FillerType {{");

    for filler in &data.filler {
        let _ = write!(str, "{},", filler.enum_name);
    }

    let _ = write!(str, "}}#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]pub enum Filler {{");

    for filler in &data.filler {
        let r#type = filler.r#type;
        let damage_types = filler.damage_types;
        let enum_name = &filler.enum_name;

        let _ = match (r#type, damage_types) {
            (FillerType::Hero, true) => write!(str, "{enum_name}((HeroLike, DamageType)),"),
            (FillerType::Hero, false) => write!(str, "{enum_name}(HeroLike),"),
            (FillerType::Villain, true) => write!(str, "{enum_name}((VillainLike, DamageType)),"),
            (FillerType::Villain, false) => write!(str, "{enum_name}(VillainLike),"),
            (FillerType::Other, true) => write!(str, "{enum_name}(DamageType),"),
            (FillerType::Other, false) => write!(str, "{enum_name},"),
        };
    }

    let _ = write!(str, "}}impl FillerType {{pub fn as_i(&self) -> usize {{match self {{");

    for filler in &data.filler {
        let enum_name = &filler.enum_name;
        let i = filler.i;

        let _ = write!(str, "FillerType::{enum_name} => {i},");
    }

    let _ = write!(str, "}}}}pub fn as_idx(&self) -> usize {{match self {{");

    for (filler, idx) in data.filler.iter().filter(|filler| !filler.damage_types).zip(0..) {
        let enum_name = &filler.enum_name;
        let _ = write!(str, "FillerType::{enum_name} => {idx},");
    }
    for (filler, idx) in data.filler.iter().filter(|filler| filler.damage_types).zip(0..) {
        let enum_name = &filler.enum_name;
        let _ = write!(str, "FillerType::{enum_name} => {idx},");
    }

    let _ = write!(str, "}}}}}}impl Filler {{pub const fn variant_count() -> usize {{{}}}pub const fn variant_count_no_type() -> usize {{{}}}pub const fn variant_count_type() -> usize {{{}}}pub fn with_type(self, damage_type: DamageType) -> Filler {{match self {{", 
        data.filler.len(),
        data.filler.iter().filter(|filler| !filler.damage_types).count(),
        data.filler.iter().filter(|filler| filler.damage_types).count()
    );

    for filler in &data.filler {
        let r#type = filler.r#type;
        let damage_types = filler.damage_types;
        let enum_name = &filler.enum_name;

        if damage_types {
            let _ = match r#type {
                FillerType::Hero => write!(str, "Filler::{enum_name}((hero, _)) => Filler::{enum_name}((hero, damage_type)),"),
                FillerType::Villain => write!(str, "Filler::{enum_name}((villain, _)) => Filler::{enum_name}((villain, damage_type)),"),
                FillerType::Other => write!(str, "Filler::{enum_name}(_) => Filler::{enum_name}(damage_type),"),
            };
        }
    }

    let _ = write!(str, "_ => self}}}}pub fn to_string(&self, count: i32) -> String {{match self {{",);

    for filler in &data.filler {
        let r#type = filler.r#type;
        let damage_types = filler.damage_types;
        let enum_name = &filler.enum_name;
        let display_name_pos = &filler.display_name_pos;
        let display_name_neg = &filler.display_name_neg;

        match (r#type, damage_types) {
            (FillerType::Hero, true) | (FillerType::Villain, true) => {
                let _ = write!(
                    str,
                    "Filler::{enum_name}((variant, damage_type)) => if count > 0 {{format!(\"{}\", damage_type.as_str(), count.abs())}} else {{format!(\"{}\", damage_type.as_str(), count.abs())}},",
                    handle_pos_neg(display_name_pos.as_ref(), display_name_neg.as_ref()).replace("[COUNT]", "{}").replace("[TYPE]", "{}"),
                    handle_pos_neg(display_name_neg.as_ref(), display_name_pos.as_ref()).replace("[COUNT]", "{}").replace("[TYPE]", "{}")
                );
            }
            (FillerType::Hero, false) | (FillerType::Villain, false) => {
                let _ = write!(
                    str,
                    "Filler::{enum_name}(variant) => if count > 0 {{format!(\"{}\", count.abs())}} else {{format!(\"{}\", count.abs())}},",
                    handle_pos_neg(display_name_pos.as_ref(), display_name_neg.as_ref()).replace("[COUNT]", "{}"),
                    handle_pos_neg(display_name_neg.as_ref(), display_name_pos.as_ref()).replace("[COUNT]", "{}")
                );
            }
            (FillerType::Other, true) => {
                let _ = write!(
                    str,
                    "Filler::{enum_name}(damage_type) => if count > 0 {{format!(\"{}\", damage_type.as_str(), count.abs())}} else {{format!(\"{}\", damage_type.as_str(), count.abs())}},",
                    handle_pos_neg(display_name_pos.as_ref(), display_name_neg.as_ref()).replace("[COUNT]", "{}").replace("[TYPE]", "{}"),
                    handle_pos_neg(display_name_neg.as_ref(), display_name_pos.as_ref()).replace("[COUNT]", "{}").replace("[TYPE]", "{}")
                );
            }
            (FillerType::Other, false) => {
                let _ = write!(
                    str,
                    "Filler::{enum_name} => if count > 0 {{format!(\"{}\", count.abs())}} else {{format!(\"{}\", count.abs())}},",
                    handle_pos_neg(display_name_pos.as_ref(), display_name_neg.as_ref()).replace("[COUNT]", "{}"),
                    handle_pos_neg(display_name_neg.as_ref(), display_name_pos.as_ref()).replace("[COUNT]", "{}")
                );
            }
        }
    }

    let _ = write!(str, "}}}}pub fn to_desc(&self, count: i32) -> String {{match self {{");

    for filler in &data.filler {
        let r#type = filler.r#type;
        let damage_types = filler.damage_types;
        let enum_name = &filler.enum_name;
        let desc_pos = &filler.desc_pos;
        let desc_neg = &filler.desc_neg;

        match (r#type, damage_types) {
            (FillerType::Hero, true) | (FillerType::Villain, true) => {
                let _ = write!(
                    str,
                    "Filler::{enum_name}((variant, damage_type)) => if count > 0 {{format!(\"{}\", damage_type.as_str(), count.abs())}} else {{format!(\"{}\", damage_type.as_str(), count.abs())}},",
                    handle_pos_neg(desc_pos.as_ref(), desc_neg.as_ref()).replace("[COUNT]", "{}").replace("[TYPE]", "{}"),
                    handle_pos_neg(desc_neg.as_ref(), desc_pos.as_ref()).replace("[COUNT]", "{}").replace("[TYPE]", "{}")
                );
            }
            (FillerType::Hero, false) | (FillerType::Villain, false) => {
                let _ = write!(
                    str,
                    "Filler::{enum_name}(variant) => if count > 0 {{format!(\"{}\", count.abs())}} else {{format!(\"{}\", count.abs())}},",
                    handle_pos_neg(desc_pos.as_ref(), desc_neg.as_ref()).replace("[COUNT]", "{}"),
                    handle_pos_neg(desc_neg.as_ref(), desc_pos.as_ref()).replace("[COUNT]", "{}")
                );
            }
            (FillerType::Other, true) => {
                let _ = write!(
                    str,
                    "Filler::{enum_name}(damage_type) => if count > 0 {{format!(\"{}\", damage_type.as_str(), count.abs())}} else {{format!(\"{}\", damage_type.as_str(), count.abs())}},",
                    handle_pos_neg(desc_pos.as_ref(), desc_neg.as_ref()).replace("[COUNT]", "{}").replace("[TYPE]", "{}"),
                    handle_pos_neg(desc_neg.as_ref(), desc_pos.as_ref()).replace("[COUNT]", "{}").replace("[TYPE]", "{}")
                );
            }
            (FillerType::Other, false) => {
                let _ = write!(
                    str,
                    "Filler::{enum_name} => if count > 0 {{format!(\"{}\", count.abs())}} else {{format!(\"{}\", count.abs())}},",
                    handle_pos_neg(desc_pos.as_ref(), desc_neg.as_ref()).replace("[COUNT]", "{}"),
                    handle_pos_neg(desc_neg.as_ref(), desc_pos.as_ref()).replace("[COUNT]", "{}")
                );
            }
        }
    }

    let _ = write!(str, "}}}}");

    push_from_id(str, &data.filler);
    push_deconstruct(str, &data.filler);
    push_relevant_filler(str, &data.filler);

    let _ = write!(str, "/* other methods */}}");
}

fn push_from_id<T>(str: &mut T, data: &[FillerData])
where
    T: Write,
{
    let _ = write!(
        str,
        "pub fn from_id(id: i64) -> (Filler, i32) {{let count = 1 - (((id >> 48) & 1) << 1) as i32; (match ((id & (0b1111 << 28)) >> 28, id & 0b1111_1111) {{"
    );

    for filler in data {
        let r#type = filler.r#type;
        let damage_types = filler.damage_types;
        let enum_name = &filler.enum_name;
        let i = filler.i;

        match (r#type, damage_types) {
            (FillerType::Hero, true) => {
                let _ = write!(
                    str,
                    "(0b1000, {i}) => Filler::{enum_name}((HeroLike::All, DamageType::from_i64((id & (0b1111 << 24)) >> 24).expect(\"Unknown damage type ID\"))),",
                );
                let _ = write!(str, "(0b1001, {i}) => Filler::{enum_name}((HeroLike::Hero(Hero::from_i64((id & (0b1111_1111_1111_1111 << 8)) >> 8).expect(\"Unknown hero ID in filler\")), DamageType::from_i64((id & (0b1111 << 24)) >> 24).expect(\"Unknown damage type ID\"))),",);
                let _ = write!(str, "(0b1010, {i}) => Filler::{enum_name}((HeroLike::Variant(Variant::from_hero(Hero::from_i64((id & (0b1111_1111_1111_1111 << 8)) >> 8).expect(\"Unknown hero ID in filler\"), ((id & (0b1111_1111 << 32)) >> 32) as u32).expect(\"Unknown variant ID for hero\")), DamageType::from_i64((id & (0b1111 << 24)) >> 24).expect(\"Unknown damage type ID\"))),",);
            }
            (FillerType::Hero, false) => {
                let _ = write!(str, "(0b1000, {i}) => Filler::{enum_name}(HeroLike::All),",);
                let _ = write!(
                    str,
                    "(0b1001, {i}) => Filler::{enum_name}(HeroLike::Hero(Hero::from_i64((id & (0b1111_1111_1111_1111 << 8)) >> 8).expect(\"Unknown hero ID in filler\"))),",
                );
                let _ = write!(str, "(0b1010, {i}) => Filler::{enum_name}(HeroLike::Variant(Variant::from_hero(Hero::from_i64((id & (0b1111_1111_1111_1111 << 8)) >> 8).expect(\"Unknown hero ID in filler\"), ((id & (0b1111_1111 << 32)) >> 32) as u32).expect(\"Unknown variant ID for hero\"))),",);
            }
            (FillerType::Villain, true) => {
                let _ = write!(
                    str,
                    "(0b0100, {i}) => Filler::{enum_name}((VillainLike::All, DamageType::from_i64((id & (0b1111 << 24)) >> 24).expect(\"Unknown damage type ID\"))),",
                );
                let _ = write!(str, "(0b0101, {i}) => Filler::{enum_name}((VillainLike::Villain(Villain::from_i64((id & (0b1111_1111_1111_1111 << 8)) >> 8).expect(\"Unknown villain ID in filler\")), DamageType::from_i64((id & (0b1111 << 24)) >> 24).expect(\"Unknown damage type ID\"))),",);
                let _ = write!(str, "(0b0110, {i}) => Filler::{enum_name}((VillainLike::TeamVillain(TeamVillain::from_i64((id & (0b1111_1111_1111_1111 << 8)) >> 8).expect(\"Unknown team villain ID in filler\")), DamageType::from_i64((id & (0b1111 << 24)) >> 24).expect(\"Unknown damage type ID\"))),",);
            }
            (FillerType::Villain, false) => {
                let _ = write!(str, "(0b0100, {i}) => Filler::{enum_name}(VillainLike::All),",);
                let _ = write!(
                    str,
                    "(0b0101, {i}) => Filler::{enum_name}(VillainLike::Villain(Villain::from_i64((id & (0b1111_1111_1111_1111 << 8)) >> 8).expect(\"Unknown villain ID in filler\"))),",
                );
                let _ = write!(
                    str,
                    "(0b0110, {i}) => Filler::{enum_name}(VillainLike::TeamVillain(TeamVillain::from_i64((id & (0b1111_1111_1111_1111 << 8)) >> 8).expect(\"Unknown team villain ID in filler\"))),",
                );
            }
            (FillerType::Other, true) => {
                let _ = write!(str, "(0b0000, {i}) => Filler::{enum_name}(DamageType::from_i64((id & (0b1111 << 24)) >> 24)),",);
            }
            (FillerType::Other, false) => {
                let _ = write!(str, "(0b0000, {i}) => Filler::{enum_name},",);
            }
        }
    }

    let _ = write!(str, "_ => panic!(\"Unknown filler ID\")}}, count)}}");
}

fn push_deconstruct<T>(str: &mut T, data: &[FillerData])
where
    T: Write,
{
    let _ = write!(str, "pub fn deconstruct(self) -> DeconstructedFiller {{match self {{");

    for filler in data {
        let r#type = filler.r#type;
        let damage_types = filler.damage_types;
        let enum_name = &filler.enum_name;

        let _ = match (r#type, damage_types) {
            (FillerType::Hero, true) => write!(
                str,
                "Filler::{enum_name}((hero, damage_type)) => DeconstructedFiller {{r#type: FillerType::{enum_name}, target: FillerTarget::Hero(hero), damage_type: Some(damage_type)}},"
            ),
            (FillerType::Hero, false) => write!(
                str,
                "Filler::{enum_name}(hero) => DeconstructedFiller {{r#type: FillerType::{enum_name}, target: FillerTarget::Hero(hero), damage_type: None}},"
            ),
            (FillerType::Villain, true) => write!(
                str,
                "Filler::{enum_name}((villain, damage_type)) => DeconstructedFiller {{r#type: FillerType::{enum_name}, target: FillerTarget::Villain(villain), damage_type: Some(damage_type)}},"
            ),
            (FillerType::Villain, false) => write!(
                str,
                "Filler::{enum_name}(villain) => DeconstructedFiller {{r#type: FillerType::{enum_name}, target: FillerTarget::Villain(villain), damage_type: None}},"
            ),
            (FillerType::Other, true) => write!(
                str,
                "Filler::{enum_name}(damage_type) => DeconstructedFiller {{r#type: FillerType::{enum_name}, target: FillerTarget::Other, damage_type: Some(damage_type)}},"
            ),
            (FillerType::Other, false) => write!(
                str,
                "Filler::{enum_name} => DeconstructedFiller {{r#type: FillerType::{enum_name}, target: FillerTarget::Other, damage_type: None}},"
            ),
        };
    }

    let _ = write!(str, "}}}}");
}

fn push_relevant_filler<T>(str: &mut T, data: &[FillerData])
where
    T: Write,
{
    let _ = write!(str, "pub fn hero_filler(hero: HeroLike) -> Vec<Filler> {{vec![");

    for filler in data {
        let r#type = filler.r#type;
        let damage_types = filler.damage_types;
        let enum_name = &filler.enum_name;

        if r#type == FillerType::Hero {
            if damage_types {
                let _ = write!(str, "Filler::{enum_name}((hero, DamageType::All)),");
            } else {
                let _ = write!(str, "Filler::{enum_name}(hero),");
            }
        }
    }

    let _ = write!(str, "]}}pub fn villain_filler(villain: VillainLike) -> Vec<Filler> {{vec![");

    for filler in data {
        let r#type = filler.r#type;
        let damage_types = filler.damage_types;
        let enum_name = &filler.enum_name;

        if r#type == FillerType::Villain {
            if damage_types {
                let _ = write!(str, "Filler::{enum_name}((villain, DamageType::All)),");
            } else {
                let _ = write!(str, "Filler::{enum_name}(villain),");
            }
        }
    }

    let _ = write!(str, "]}}pub fn other_filler() -> Vec<Filler> {{vec![");

    for filler in data {
        let r#type = filler.r#type;
        let damage_types = filler.damage_types;
        let enum_name = &filler.enum_name;

        if r#type == FillerType::Other {
            if damage_types {
                let _ = write!(str, "Filler::{enum_name}(DamageType::All),");
            } else {
                let _ = write!(str, "Filler::{enum_name},");
            }
        }
    }

    let _ = write!(str, "]}}");
}

fn handle_pos_neg<'a>(preferred: Option<&'a String>, other: Option<&'a String>) -> &'a String {
    if let Some(preferred) = preferred {
        preferred
    } else {
        other.expect("Filler must have either pos, neg, or both")
    }
}
