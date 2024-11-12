use crate::state::Items;
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
    Variant(Variant),
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
    pub damage_type: Option<DamageType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, FromPrimitive, ToPrimitive, Hash)]
pub enum DamageType {
    All,
    Cold,
    Energy,
    Fire,
    Infernal,
    Lightning,
    Melee,
    Projectile,
    Psychic,
    Radiant,
    Sonic,
    Toxic,
}

impl DamageType {
    pub fn as_str(&self) -> &str {
        match self {
            DamageType::All => "",
            DamageType::Cold => "Cold ",
            DamageType::Energy => "Energy ",
            DamageType::Fire => "Fire ",
            DamageType::Infernal => "Infernal ",
            DamageType::Lightning => "Lightning ",
            DamageType::Melee => "Melee ",
            DamageType::Projectile => "Projectile ",
            DamageType::Psychic => "Psychic ",
            DamageType::Radiant => "Radiant ",
            DamageType::Sonic => "Sonic ",
            DamageType::Toxic => "Toxic ",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FillerTarget {
    Hero(HeroLike),
    Villain(VillainLike),
    Other,
}

impl FillerTarget {
    pub fn relevant_filler(&self) -> Vec<Filler> {
        match self {
            FillerTarget::Hero(hero) => Filler::hero_filler(*hero),
            FillerTarget::Villain(villain) => Filler::villain_filler(*villain),
            FillerTarget::Other => Filler::other_filler(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HeroLike {
    All,
    Hero(Hero),
    Variant(Variant),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VillainLike {
    All,
    Villain(Villain),
    TeamVillain(TeamVillain),
}

impl HeroLike {
    pub fn as_str(&self) -> String {
        match self {
            HeroLike::All => String::new(),
            HeroLike::Hero(hero) => format!(": {} (All variants)", hero.as_str()),
            HeroLike::Variant(variant) => format!(": {}", variant.as_str()),
        }
    }
}

impl VillainLike {
    pub fn as_str(&self) -> String {
        match self {
            VillainLike::All => String::new(),
            VillainLike::Villain(villain) => format!(": {}", villain.as_str()),
            VillainLike::TeamVillain(team_villain) => format!(": {}", team_villain.as_str()),
        }
    }
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

    pub fn from_id(id: i64) -> Item {
        match (id & (0b1111 << 48)) >> 48 {
            0b0001 => Item::Villain(Villain::from_i64(id & 0b1111_1111_1111_1111).expect("Unknown villain ID")),
            0b0011 => Item::TeamVillain(TeamVillain::from_i64(id & 0b1111_1111_1111_1111).expect("Unknown team villain ID")),
            0b0101 => Item::Gladiator(Gladiator::from_i64(id & 0b1111_1111_1111_1111).expect("Unknown team villain ID")),
            0b0010 => {
                let hero = Hero::from_i64(id & 0b1111_1111_1111_1111).expect("Unknown hero ID");
                let variant_i = ((id as u32) & (0b1111_1111 << 16)) >> 16;

                if variant_i == 0 {
                    Item::Hero(hero)
                } else {
                    Item::Variant(Variant::from_hero(hero, variant_i).expect("Unknown variant ID"))
                }
            }
            0b0110 => Item::Contender(Contender::from_i64(id & 0b1111_1111_1111_1111).expect("Unknown contender ID")),
            0b0100 => Item::Environment(Environment::from_i64(id & 0b1111_1111_1111_1111).expect("Unknown environment ID")),
            0b1000 | 0b1001 => Item::Filler(Filler::from_id(id)),
            0b0000 => {
                if id == 1 {
                    Item::Scion
                } else {
                    panic!("Unknown item ID")
                }
            }
            _ => panic!("Unknown item ID"),
        }
    }
}

impl Location {
    pub fn as_id(&self, n: i64) -> i64 {
        (match self {
            Location::Villain((v, d)) => (0b0001 << 48) | *v as i64 | ((*d as i64) << 22),
            Location::TeamVillain((v, d)) => (0b0011 << 48) | *v as i64 | ((*d as i64) << 22),
            Location::Gladiator((v, d)) => (0b0101 << 48) | *v as i64 | ((*d as i64) << 22),
            Location::Variant(v) => {
                (0b0010 << 48)
                    | if let Some(h) = v.as_normal() {
                        h as i64 | (v.as_i() as i64) << 24
                    } else {
                        v.as_i() as i64 | 1 << 22
                    }
            }
            Location::Environment(v) => (0b0100 << 48) | *v as i64,
            Location::Victory => 0,
        }) | n << 16
    }

    pub fn as_item(&self) -> Option<Item> {
        match self {
            Location::Variant(v) => Some(Item::Variant(*v)),
            Location::Villain((v, _)) => Some(Item::Villain(*v)),
            Location::TeamVillain((v, _)) => Some(Item::TeamVillain(*v)),
            Location::Gladiator((v, _)) => Some(Item::Gladiator(*v)),
            Location::Environment(v) => Some(Item::Environment(*v)),
            Location::Victory => None,
        }
    }
}
