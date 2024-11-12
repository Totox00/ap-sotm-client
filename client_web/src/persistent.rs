use base64::{prelude::BASE64_STANDARD, Engine};
use client_lib::{
    data::{Environment, Gladiator, TeamVillain, Variant, Villain},
    persistent::PersistentStore,
    state::Locations,
};
use web_sys::window;

pub struct WebPersistentStore {
    key: String,
}

impl PersistentStore for WebPersistentStore {
    fn new(seed: &str, name: &str) -> Self {
        WebPersistentStore { key: format!("{seed}-{name}") }
    }

    fn load(&self) -> Locations {
        if let Some(window) = window() {
            if let Ok(Some(local_storage)) = window.local_storage() {
                if let Ok(Some(base64)) = local_storage.get_item(&self.key) {
                    if let Ok(buf) = BASE64_STANDARD.decode(&base64) {
                        if buf.len()
                            > 10 + 1 + Villain::variant_count() + TeamVillain::variant_count() + Gladiator::variant_count() + Variant::variant_count() / 8 + 1 + Environment::variant_count() / 8 + 1
                        {
                            let _ = window.alert_with_message("Save file was made with a newer version and cannot be loaded.");
                            return Locations::new();
                        } else if buf[0] & 2 == 0 {
                            let _ = window.alert_with_message("Save file was made with a too old version and cannot be loaded.");
                            return Locations::new();
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
                            return locations;
                        }
                    }
                }
            }
        }

        Locations::new()
    }

    fn save(&self, locations: &Locations) {
        if let Some(window) = window() {
            if let Ok(Some(local_storage)) = window.local_storage() {
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

                let _ = local_storage.set_item(&self.key, &BASE64_STANDARD.encode(&buf));
            }
        }
    }
}
