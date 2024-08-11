use crate::{Data, FillerData, FillerType};
use std::fmt::Write;

pub fn push_filler<T>(str: &mut T, data: &Data)
where
    T: Write,
{
    let _ = write!(str, "#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]pub enum Filler {{");

    for FillerData {
        enum_name,
        damage_types,
        r#type,
        display_name_pos: _,
        display_name_neg: _,
        desc_pos: _,
        desc_neg: _,
        i: _,
    } in &data.filler
    {
        let _ = if *damage_types {
            match r#type {
                FillerType::Hero => write!(str, "{enum_name}((HeroLike, DamageType)),"),
                FillerType::Villain => write!(str, "{enum_name}((VillainLike, DamageType)),"),
                FillerType::Other => write!(str, "{enum_name}(DamageType),"),
            }
        } else {
            match r#type {
                FillerType::Hero => write!(str, "{enum_name}(HeroLike),"),
                FillerType::Villain => write!(str, "{enum_name}(VillainLike),"),
                FillerType::Other => write!(str, "{enum_name},"),
            }
        };
    }

    let _ = write!(str, "}}impl Filler {{pub fn to_string(&self, count: i8) -> String {{match self {{");

    for FillerData {
        enum_name,
        damage_types,
        r#type,
        display_name_pos,
        display_name_neg,
        desc_pos: _,
        desc_neg: _,
        i: _,
    } in &data.filler
    {
        if *damage_types {
            match r#type {
                FillerType::Hero | FillerType::Villain => {
                    let _ = write!(
                            str,
                            "Filler::{enum_name}((variant, damage_type)) => if count > 0 {{format!(\"{}{{}}\", damage_type.as_str(), count.abs(), variant.as_str())}} else {{format!(\"{}{{}}\", damage_type.as_str(), count.abs(), variant.as_str())}},",
                            handle_pos_neg(display_name_pos, display_name_neg).replace("[COUNT]", "{}").replace("[TYPE]", "{}"),
                            handle_pos_neg(display_name_neg, display_name_pos).replace("[COUNT]", "{}").replace("[TYPE]", "{}")
                        );
                }
                FillerType::Other => {
                    let _ = write!(
                        str,
                        "Filler::{enum_name}(damage_type) => if count > 0 {{format!(\"{}\", damage_type.as_str(), count.abs())}} else {{format!(\"{}\", damage_type.as_str(), count.abs())}},",
                        handle_pos_neg(display_name_pos, display_name_neg).replace("[COUNT]", "{}").replace("[TYPE]", "{}"),
                        handle_pos_neg(display_name_neg, display_name_pos).replace("[COUNT]", "{}").replace("[TYPE]", "{}")
                    );
                }
            }
        } else {
            match r#type {
                FillerType::Hero | FillerType::Villain => {
                    let _ = write!(
                        str,
                        "Filler::{enum_name}(variant) => if count > 0 {{format!(\"{}{{}}\", count.abs(), variant.as_str())}} else {{format!(\"{}{{}}\", count.abs(), variant.as_str())}},",
                        handle_pos_neg(display_name_pos, display_name_neg).replace("[COUNT]", "{}"),
                        handle_pos_neg(display_name_neg, display_name_pos).replace("[COUNT]", "{}")
                    );
                }
                FillerType::Other => {
                    let _ = write!(
                        str,
                        "Filler::{enum_name} => if count > 0 {{format!(\"{}\", count.abs())}} else {{format!(\"{}\", count.abs())}},",
                        handle_pos_neg(display_name_pos, display_name_neg).replace("[COUNT]", "{}"),
                        handle_pos_neg(display_name_neg, display_name_pos).replace("[COUNT]", "{}")
                    );
                }
            }
        };
    }

    let _ = write!(str, "}}}}pub fn as_desc(&self, count: i8) -> &str {{match self {{");

    for FillerData {
        enum_name,
        damage_types,
        r#type,
        display_name_pos: _,
        display_name_neg: _,
        desc_pos,
        desc_neg,
        i: _,
    } in &data.filler
    {
        if *r#type == FillerType::Other && !*damage_types {
            let _ = write!(str, "Filler::{enum_name} => if count > 0 {{\"{desc_pos}\"}} else {{\"{desc_neg}\"}},");
        } else {
            let _ = write!(str, "Filler::{enum_name}(_) => if count > 0 {{\"{desc_pos}\"}} else {{\"{desc_neg}\"}},");
        }
    }

    let _ = write!(str, "}}}}");

    push_from_id(str, &data.filler);

    let _ = write!(str, "/* other methods */}}");
}

fn push_from_id<T>(str: &mut T, data: &[FillerData])
where
    T: Write,
{
    let _ = write!(
        str,
        "pub fn from_id(id: i64) -> (Filler, i32) {{let count = 1 - ((id & (1 << 48)) << 2) as i32; (match ((id & (0b1111 << 28)) >> 28, id & 0b1111_1111) {{"
    );

    for FillerData {
        enum_name,
        display_name_pos: _,
        display_name_neg: _,
        damage_types,
        r#type,
        desc_pos: _,
        desc_neg: _,
        i,
    } in data
    {
        if *damage_types {
            match r#type {
                FillerType::Hero => {
                    let _ = write!(
                        str,
                        "(0b1000, {i}) => Filler::{enum_name}((HeroLike::All, DamageType::from_i64((id & (0b1111 << 24)) >> 24).expect(\"Unknown damage type ID\"))),",
                    );
                    let _ = write!(str, "(0b1001, {i}) => Filler::{enum_name}((HeroLike::Hero(Hero::from_i64((id & (0b1111_1111_1111_1111 << 8)) >> 8).expect(\"Unknown hero ID in filler\")), DamageType::from_i64((id & (0b1111 << 24)) >> 24).expect(\"Unknown damage type ID\"))),",);
                    let _ = write!(str, "(0b1010, {i}) => Filler::{enum_name}((HeroLike::Variant(Variant::from_hero(Hero::from_i64((id & (0b1111_1111_1111_1111 << 8)) >> 8).expect(\"Unknown hero ID in filler\"), ((id & (0b1111_1111 << 32)) >> 32) as u32).expect(\"Unknown variant ID for hero\")), DamageType::from_i64((id & (0b1111 << 24)) >> 24).expect(\"Unknown damage type ID\"))),",);
                }
                FillerType::Villain => {
                    let _ = write!(
                        str,
                        "(0b0100, {i}) => Filler::{enum_name}((VillainLike::All, DamageType::from_i64((id & (0b1111 << 24)) >> 24).expect(\"Unknown damage type ID\"))),",
                    );
                    let _ = write!(str, "(0b0101, {i}) => Filler::{enum_name}((VillainLike::Villain(Villain::from_i64((id & (0b1111_1111_1111_1111 << 8)) >> 8).expect(\"Unknown villain ID in filler\")), DamageType::from_i64((id & (0b1111 << 24)) >> 24).expect(\"Unknown damage type ID\"))),",);
                    let _ = write!(str, "(0b0110, {i}) => Filler::{enum_name}((VillainLike::TeamVillain(TeamVillain::from_i64((id & (0b1111_1111_1111_1111 << 8)) >> 8).expect(\"Unknown team villain ID in filler\")), DamageType::from_i64((id & (0b1111 << 24)) >> 24).expect(\"Unknown damage type ID\"))),",);
                }
                FillerType::Other => {
                    let _ = write!(str, "(0b0000, {i}) => Filler::{enum_name}(DamageType::from_i64((id & (0b1111 << 24)) >> 24)),",);
                }
            }
        } else {
            match r#type {
                FillerType::Hero => {
                    let _ = write!(str, "(0b1000, {i}) => Filler::{enum_name}(HeroLike::All),",);
                    let _ = write!(
                        str,
                        "(0b1001, {i}) => Filler::{enum_name}(HeroLike::Hero(Hero::from_i64((id & (0b1111_1111_1111_1111 << 8)) >> 8).expect(\"Unknown hero ID in filler\"))),",
                    );
                    let _ = write!(str, "(0b1010, {i}) => Filler::{enum_name}(HeroLike::Variant(Variant::from_hero(Hero::from_i64((id & (0b1111_1111_1111_1111 << 8)) >> 8).expect(\"Unknown hero ID in filler\"), ((id & (0b1111_1111 << 32)) >> 32) as u32).expect(\"Unknown variant ID for hero\"))),",);
                }
                FillerType::Villain => {
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
                FillerType::Other => {
                    let _ = write!(str, "(0b0000, {i}) => Filler::{enum_name},",);
                }
            }
        }
    }

    let _ = write!(str, "_ => panic!(\"Unknown filler ID\")}}, count)}}");
}

fn handle_pos_neg<'a>(preferred: &'a str, other: &'a str) -> &'a str {
    if preferred.is_empty() {
        other
    } else {
        preferred
    }
}
