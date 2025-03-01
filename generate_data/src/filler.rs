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
        let enum_name = &filler.enum_name;

        let _ = match r#type {
            FillerType::Hero => write!(str, "{enum_name}(HeroLike),"),
            FillerType::Villain => write!(str, "{enum_name}(VillainLike),"),
            FillerType::Other => write!(str, "{enum_name},"),
        };
    }

    let _ = write!(str, "}}impl FillerType {{pub fn as_i(&self) -> u8 {{match self {{");

    for filler in &data.filler {
        let enum_name = &filler.enum_name;
        let i = filler.i;

        let _ = write!(str, "FillerType::{enum_name} => {i},");
    }

    let _ = write!(str, "}}}}pub fn from_i(i: u8) -> Option<FillerType> {{match i {{");

    for filler in &data.filler {
        let enum_name = &filler.enum_name;
        let i = filler.i;

        let _ = write!(str, "{i} => Some(FillerType::{enum_name}),");
    }

    let _ = write!(str, "_ => None}}}}pub fn no_target(&self) -> FillerTarget {{match self {{");

    for filler in &data.filler {
        let enum_name = &filler.enum_name;
        let r#type = &filler.r#type;
        let _ = match r#type {
            FillerType::Hero => write!(str, "FillerType::{enum_name} => FillerTarget::Hero(HeroLike::All),"),
            FillerType::Villain => write!(str, "FillerType::{enum_name} => FillerTarget::Villain(VillainLike::All),"),
            FillerType::Other => write!(str, "FillerType::{enum_name} => FillerTarget::Other,"),
        };
    }

    let _ = write!(
        str,
        "}}}}}}impl Filler {{pub const fn variant_count() -> usize {{{}}}pub fn as_str(&self, count: i32) -> &str {{match self {{",
        data.filler.len()
    );

    for filler in &data.filler {
        let r#type = filler.r#type;
        let enum_name = &filler.enum_name;
        let display_name_pos = &filler.display_name_pos;
        let display_name_neg = &filler.display_name_neg;

        match r#type {
            FillerType::Hero | FillerType::Villain => {
                let _ = write!(
                    str,
                    "Filler::{enum_name}(variant) => if count > 0 {{\"{}\"}} else {{\"{}\"}},",
                    handle_pos_neg(display_name_pos.as_ref(), display_name_neg.as_ref()),
                    handle_pos_neg(display_name_neg.as_ref(), display_name_pos.as_ref())
                );
            }
            FillerType::Other => {
                let _ = write!(
                    str,
                    "Filler::{enum_name} => if count > 0 {{\"{}\"}} else {{\"{}\"}},",
                    handle_pos_neg(display_name_pos.as_ref(), display_name_neg.as_ref()),
                    handle_pos_neg(display_name_neg.as_ref(), display_name_pos.as_ref())
                );
            }
        }
    }

    let _ = write!(str, "}}}}pub fn as_desc(&self, count: i32) -> &str {{match self {{");

    for filler in &data.filler {
        let r#type = filler.r#type;
        let enum_name = &filler.enum_name;
        let desc_pos = &filler.desc_pos;
        let desc_neg = &filler.desc_neg;

        match r#type {
            FillerType::Hero | FillerType::Villain => {
                let _ = write!(
                    str,
                    "Filler::{enum_name}(variant) => if count > 0 {{\"{}\"}} else {{\"{}\"}},",
                    handle_pos_neg(desc_pos.as_ref(), desc_neg.as_ref()),
                    handle_pos_neg(desc_neg.as_ref(), desc_pos.as_ref())
                );
            }
            FillerType::Other => {
                let _ = write!(
                    str,
                    "Filler::{enum_name} => if count > 0 {{\"{}\"}} else {{\"{}\"}},",
                    handle_pos_neg(desc_pos.as_ref(), desc_neg.as_ref()),
                    handle_pos_neg(desc_neg.as_ref(), desc_pos.as_ref())
                );
            }
        }
    }

    let _ = write!(str, "}}}}");

    push_from_id(str, &data.filler);
    push_deconstruct(str, &data.filler);

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
        let enum_name = &filler.enum_name;
        let i = filler.i;

        match r#type {
            FillerType::Hero => {
                let _ = write!(str, "(0b1000, {i}) => Filler::{enum_name}(HeroLike::All),",);
                let _ = write!(
                    str,
                    "(0b1001, {i}) => Filler::{enum_name}(HeroLike::Hero(Hero::from_i64((id & (0b1111_1111_1111_1111 << 8)) >> 8).expect(\"Unknown hero ID in filler\"))),",
                );
                let _ = write!(str, "(0b1010, {i}) => Filler::{enum_name}(Variant::from_hero(Hero::from_i64((id & (0b1111_1111_1111_1111 << 8)) >> 8).expect(\"Unknown hero ID in filler\"), ((id & (0b1111_1111 << 32)) >> 32) as u32).map(HeroLike::Variant).unwrap_or(HeroLike::Base(Hero::from_i64((id & (0b1111_1111_1111_1111 << 8)) >> 8).expect(\"Unknown hero ID in filler\")))),",);
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

    let _ = write!(str, "_ => panic!(\"Unknown filler ID\")}}, count)}}");
}

fn push_deconstruct<T>(str: &mut T, data: &[FillerData])
where
    T: Write,
{
    let _ = write!(str, "pub fn deconstruct(self) -> DeconstructedFiller {{match self {{");

    for filler in data {
        let r#type = filler.r#type;
        let enum_name = &filler.enum_name;

        let _ = match r#type {
            FillerType::Hero => write!(
                str,
                "Filler::{enum_name}(hero) => DeconstructedFiller {{r#type: FillerType::{enum_name}, target: FillerTarget::Hero(hero)}},"
            ),
            FillerType::Villain => write!(
                str,
                "Filler::{enum_name}(villain) => DeconstructedFiller {{r#type: FillerType::{enum_name}, target: FillerTarget::Villain(villain)}},"
            ),
            FillerType::Other => write!(str, "Filler::{enum_name} => DeconstructedFiller {{r#type: FillerType::{enum_name}, target: FillerTarget::Other}},"),
        };
    }

    let _ = write!(
        str,
        "}}}}pub fn construct(deconstructed: DeconstructedFiller) -> Option<Self> {{match (deconstructed.r#type, deconstructed.target) {{"
    );

    for filler in data {
        let r#type = filler.r#type;
        let enum_name = &filler.enum_name;

        let _ = match r#type {
            FillerType::Hero => write!(str, "(FillerType::{enum_name}, FillerTarget::Hero(hero)) => Some(Filler::{enum_name}(hero)),"),
            FillerType::Villain => write!(str, "(FillerType::{enum_name}, FillerTarget::Villain(villain)) => Some(Filler::{enum_name}(villain)),"),
            FillerType::Other => write!(str, "(FillerType::{enum_name}, FillerTarget::Other) => Some(Filler::{enum_name}),"),
        };
    }

    let _ = write!(str, "_ => None}}}}");
}

fn handle_pos_neg<'a>(preferred: Option<&'a String>, other: Option<&'a String>) -> &'a String {
    if let Some(preferred) = preferred {
        preferred
    } else {
        other.expect("Filler must have either pos, neg, or both")
    }
}
