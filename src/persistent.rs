use crate::{
    data::{Environment, Gladiator, TeamVillain, Variant, Villain},
    state::locations::Locations,
    variant::state::PersistentVariantProgress,
};
use base64::{prelude::BASE64_STANDARD, Engine};

pub struct PersistentStore {
    key: String,
}

impl PersistentStore {
    pub fn new(seed: &str, name: &str) -> Self {
        PersistentStore { key: format!("{seed}-{name}") }
    }

    pub fn load_string(str: &str) -> (Locations, PersistentVariantProgress, Option<&'static str>) {
        if let Ok(buf) = BASE64_STANDARD.decode(str) {
            if buf.len()
                > 10 + 1
                    + Villain::variant_count()
                    + TeamVillain::variant_count()
                    + Gladiator::variant_count()
                    + Variant::variant_count() / 8
                    + 1
                    + Environment::variant_count() / 8
                    + 1
                    + PersistentVariantProgress::size()
            {
                return (Locations::new(), PersistentVariantProgress::default(), Some("Save was made with a newer version and cannot be loaded."));
            } else if buf[0] & 2 == 0 {
                return (
                    Locations::new(),
                    PersistentVariantProgress::default(),
                    Some("Save was made with a too old version and cannot be loaded."),
                );
            } else {
                let mut locations = Locations::new();
                locations.victory = buf[0] & 1 > 0;
                let villain_len = u16::from_le_bytes(buf[1..3].try_into().unwrap()) as usize;
                let team_villain_len = u16::from_le_bytes(buf[3..5].try_into().unwrap()) as usize;
                let gladiator_len = u16::from_le_bytes(buf[5..7].try_into().unwrap()) as usize;
                let variant_len = u16::from_le_bytes(buf[7..9].try_into().unwrap()) as usize;
                let environment_len = u16::from_le_bytes(buf[9..11].try_into().unwrap()) as usize;
                let mut start = 11;
                locations.villains[0..villain_len].copy_from_slice(&buf[start..start + villain_len]);
                start += villain_len;
                locations.team_villains[0..team_villain_len].copy_from_slice(&buf[start..start + team_villain_len]);
                start += team_villain_len;
                locations.gladiators[0..gladiator_len].copy_from_slice(&buf[start..start + gladiator_len]);
                start += gladiator_len;
                locations.variants[0..variant_len].copy_from_slice(&buf[start..start + variant_len]);
                start += variant_len;
                locations.environments[0..environment_len].copy_from_slice(&buf[start..start + environment_len]);
                start += environment_len;
                return (locations, PersistentVariantProgress::from(&buf[start..]), None);
            }
        }

        (Locations::new(), PersistentVariantProgress::default(), None)
    }

    pub fn save_string(&self, locations: &Locations, variant_progress: &PersistentVariantProgress) -> String {
        let mut buf = vec![];
        buf.push(if locations.victory { 0b11 } else { 0b10 });
        buf.extend((Villain::variant_count() as u16).to_le_bytes());
        buf.extend((TeamVillain::variant_count() as u16).to_le_bytes());
        buf.extend((Gladiator::variant_count() as u16).to_le_bytes());
        buf.extend(((Variant::variant_count() / 8 + 1) as u16).to_le_bytes());
        buf.extend(((Environment::variant_count() / 8 + 1) as u16).to_le_bytes());
        buf.extend(locations.villains);
        buf.extend(locations.team_villains);
        buf.extend(locations.gladiators);
        buf.extend(locations.variants);
        buf.extend(locations.environments);
        buf.extend(variant_progress.as_bytes());

        BASE64_STANDARD.encode(&buf)
    }
}
