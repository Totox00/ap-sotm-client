use std::collections::HashMap;

use crate::{
    data::{DeconstructedFiller, Environment, Filler, FillerTarget, FillerType, Gladiator, Hero, HeroLike, TeamVillain, Variant, Villain, VillainLike},
    state::locations::Locations,
    variant::state::PersistentVariantProgress,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use num::FromPrimitive;

pub fn load_string(str: &str) -> (Locations, PersistentVariantProgress, Vec<(Filler, i32)>, Option<&'static str>) {
    if let Ok(buf) = BASE64_STANDARD.decode(str) {
        if buf[0] & 0x6 != 0x4 {
            return (
                Locations::new(),
                PersistentVariantProgress::default(),
                vec![],
                Some("Save was made with a too old version and cannot be loaded."),
            );
        } else {
            let mut locations = Locations::new();
            locations.victory = buf[0] & 1 > 0;
            let villain_len = u16::from_le_bytes(buf[1..3].try_into().unwrap()) as usize;
            let team_villain_len = u16::from_le_bytes(buf[3..5].try_into().unwrap()) as usize;
            let gladiator_len = u16::from_le_bytes(buf[5..7].try_into().unwrap()) as usize;
            let variant_unlock_len = u16::from_le_bytes(buf[7..9].try_into().unwrap()) as usize;
            let hero_len = u16::from_le_bytes(buf[9..11].try_into().unwrap()) as usize;
            let environment_len = u16::from_le_bytes(buf[11..13].try_into().unwrap()) as usize;
            let persistent_variant_progress_len = u16::from_le_bytes(buf[13..15].try_into().unwrap()) as usize;
            let mut start = 15;
            locations.villains[0..villain_len].copy_from_slice(&buf[start..start + villain_len]);
            start += villain_len;
            locations.team_villains[0..team_villain_len].copy_from_slice(&buf[start..start + team_villain_len]);
            start += team_villain_len;
            locations.gladiators[0..gladiator_len].copy_from_slice(&buf[start..start + gladiator_len]);
            start += gladiator_len;
            locations.variant_unlocks[0..variant_unlock_len].copy_from_slice(&buf[start..start + variant_unlock_len]);
            start += variant_unlock_len;
            locations.heroes[0..hero_len].copy_from_slice(&buf[start..start + hero_len]);
            start += hero_len;
            locations.environments[0..environment_len].copy_from_slice(&buf[start..start + environment_len]);
            start += environment_len;
            let persistent_variant_progress = PersistentVariantProgress::from(&buf[start..start + persistent_variant_progress_len]);
            start += persistent_variant_progress_len;

            let expended_filler = expended_from_save_string(&buf[start..]);

            if let Some(expended_filler) = expended_filler {
                return (locations, persistent_variant_progress, expended_filler, None);
            } else {
                return (locations, persistent_variant_progress, vec![], Some("Failed to read expended filler"));
            }
        }
    }

    (Locations::new(), PersistentVariantProgress::default(), vec![], None)
}

pub fn save_string(locations: &Locations, variant_progress: &PersistentVariantProgress, expended_filler: &HashMap<Filler, i32>) -> String {
    let mut buf = vec![];
    buf.push(if locations.victory { 0b101 } else { 0b100 });
    buf.extend((Villain::variant_count() as u16).to_le_bytes());
    buf.extend((TeamVillain::variant_count() as u16).to_le_bytes());
    buf.extend((Gladiator::variant_count() as u16).to_le_bytes());
    buf.extend(((Variant::variant_count() / 8 + 1) as u16).to_le_bytes());
    buf.extend((Hero::variant_count() as u16).to_le_bytes());
    buf.extend(((Environment::variant_count() / 8 + 1) as u16).to_le_bytes());
    buf.extend((PersistentVariantProgress::size() as u16).to_le_bytes());
    buf.extend(locations.villains);
    buf.extend(locations.team_villains);
    buf.extend(locations.gladiators);
    buf.extend(locations.variant_unlocks);
    buf.extend(locations.heroes);
    buf.extend(locations.environments);
    buf.extend(variant_progress.as_bytes());

    for (filler, duration) in expended_filler {
        let deconstructed = filler.deconstruct();
        buf.push(deconstructed.r#type.as_i());
        match deconstructed.target {
            FillerTarget::Hero(HeroLike::All) | FillerTarget::Villain(VillainLike::All) | FillerTarget::Other => buf.push(0),
            FillerTarget::Hero(HeroLike::Hero(hero)) => {
                buf.push(1);
                buf.extend((hero as u16).to_le_bytes());
            }
            FillerTarget::Hero(HeroLike::Base(base)) => {
                buf.push(2);
                buf.extend((base as u16).to_le_bytes());
            }
            FillerTarget::Hero(HeroLike::Variant(variant)) => {
                buf.push(3);
                buf.extend((variant.as_normal().unwrap() as u16).to_le_bytes());
                buf.push(variant.as_i());
            }
            FillerTarget::Villain(VillainLike::Villain(villain)) => {
                buf.push(4);
                buf.extend((villain as u16).to_le_bytes());
            }
            FillerTarget::Villain(VillainLike::TeamVillain(team_villain)) => {
                buf.push(5);
                buf.extend((team_villain as u16).to_le_bytes());
            }
        }
        buf.extend(duration.to_le_bytes());
    }
    BASE64_STANDARD.encode(&buf)
}

fn expended_from_save_string(buf: &[u8]) -> Option<Vec<(Filler, i32)>> {
    let mut out = vec![];

    let mut iter = buf.iter().copied();

    while let Some(type_i) = iter.next() {
        let r#type = FillerType::from_i(type_i)?;
        let target = match iter.next() {
            Some(0) => r#type.no_target(),
            Some(1) => FillerTarget::Hero(HeroLike::Hero(Hero::from_u16(u16::from_le_bytes(iter.next_chunk().ok()?))?)),
            Some(2) => FillerTarget::Hero(HeroLike::Base(Hero::from_u16(u16::from_le_bytes(iter.next_chunk().ok()?))?)),
            Some(3) => FillerTarget::Hero(HeroLike::Variant(Variant::from_hero(
                Hero::from_u16(u16::from_le_bytes(iter.next_chunk().ok()?))?,
                iter.next()? as u32,
            )?)),
            Some(4) => FillerTarget::Villain(VillainLike::Villain(Villain::from_u16(u16::from_le_bytes(iter.next_chunk().ok()?))?)),
            Some(5) => FillerTarget::Villain(VillainLike::TeamVillain(TeamVillain::from_u16(u16::from_le_bytes(iter.next_chunk().ok()?))?)),
            _ => return None,
        };

        out.push((Filler::construct(DeconstructedFiller { r#type, target })?, i32::from_le_bytes(iter.next_chunk::<4>().ok()?)));
    }

    Some(out)
}
