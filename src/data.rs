use crate::state::items::Items;
use generate_data::generate_data;
use num::FromPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};
use strum::EnumIter;

generate_data!();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Hero(Hero),
    Contender(Contender),
    Variant(Variant),
    Villain(Villain),
    TeamVillain(TeamVillain),
    Gladiator(Gladiator),
    Environment(Environment),
    Scion,
    Filler((Filler, i32)),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Location {
    Hero(Hero),
    Variant(Variant),
    VariantUnlock(Variant),
    Villain((Villain, u8)),
    TeamVillain((TeamVillain, u8)),
    Gladiator((Gladiator, u8)),
    Environment(Environment),
    Victory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeconstructedFiller {
    pub r#type: FillerType,
    pub target: FillerTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FillerTarget {
    Hero(HeroLike),
    Villain(VillainLike),
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HeroLike {
    All,
    Hero(Hero),
    Base(Hero),
    Variant(Variant),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VillainLike {
    All,
    Villain(Villain),
    TeamVillain(TeamVillain),
}

impl Item {
    pub fn as_str(&self) -> &str {
        match self {
            Item::Hero(i) => i.as_str(),
            Item::Contender(i) => i.as_str(),
            Item::Variant(i) => i.as_str(),
            Item::Villain(i) => i.as_str(),
            Item::TeamVillain(i) => i.as_str(),
            Item::Gladiator(i) => i.as_str(),
            Item::Environment(i) => i.as_str(),
            Item::Scion => "Scion of Oblivaeon",
            Item::Filler(_) => "Filler",
        }
    }

    pub fn from_id(id: i64) -> Option<Item> {
        match (id & (0b1111 << 48)) >> 48 {
            0b0001 => Villain::from_i64(id & 0b1111_1111_1111_1111).map(Item::Villain),
            0b0011 => TeamVillain::from_i64(id & 0b1111_1111_1111_1111).map(Item::TeamVillain),
            0b0101 => Gladiator::from_i64(id & 0b1111_1111_1111_1111).map(Item::Gladiator),
            0b0010 => {
                if let Some(hero) = Hero::from_i64(id & 0b1111_1111_1111_1111) {
                    let variant_i = ((id as u32) & (0b1111_1111 << 16)) >> 16;

                    if variant_i == 0 {
                        Some(Item::Hero(hero))
                    } else {
                        Variant::from_hero(hero, variant_i).map(Item::Variant)
                    }
                } else {
                    None
                }
            }
            0b0110 => Contender::from_i64(id & 0b1111_1111_1111_1111).map(Item::Contender),
            0b0100 => Environment::from_i64(id & 0b1111_1111_1111_1111).map(Item::Environment),
            0b1000 | 0b1001 => Some(Item::Filler(Filler::from_id(id))),
            0b0000 => {
                if id == 1 {
                    Some(Item::Scion)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Location {
    pub fn as_id(&self, n: i64) -> i64 {
        (match self {
            Location::Villain((v, d)) => (0b0001 << 48) | *v as i64 | ((*d as i64) << 22),
            Location::TeamVillain((v, d)) => (0b0011 << 48) | *v as i64 | ((*d as i64) << 22),
            Location::Gladiator((v, d)) => (0b0101 << 48) | *v as i64 | ((*d as i64) << 22),
            Location::VariantUnlock(v) => {
                (0b0010 << 48)
                    | if let Some(h) = v.as_normal() {
                        h as i64 | (v.as_i() as i64) << 24
                    } else {
                        v.as_i() as i64 | 0b10 << 22
                    }
            }
            Location::Hero(v) => (0b0010 << 48) | *v as i64 | 0b10 << 22,
            Location::Variant(v) => {
                if let Some(h) = v.as_normal() {
                    (0b0010 << 48) | h as i64 | (v.as_i() as i64) << 24 | 0b10 << 22
                } else {
                    return 0;
                }
            }
            Location::Environment(v) => (0b0100 << 48) | *v as i64,
            Location::Victory => 0,
        }) | n << 16
    }
}
