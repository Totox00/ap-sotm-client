use base64::{prelude::BASE64_STANDARD, Engine};
use client_lib::{
    data::{TeamVillain, Villain},
    persistent::PersistentStore,
    state::Locations,
};
use web_sys::window;

pub struct WebPersistentStore {
    seed: String,
}

impl PersistentStore for WebPersistentStore {
    fn new(seed: &str) -> Self {
        WebPersistentStore { seed: seed.to_string() }
    }

    fn load(&self) -> Locations {
        if let Some(window) = window() {
            if let Ok(Some(local_storage)) = window.local_storage() {
                if let Ok(Some(base64)) = local_storage.get_item(&self.seed) {
                    if let Ok(buf) = BASE64_STANDARD.decode(&base64) {
                        if buf.len() != 1 + Villain::variant_count() + TeamVillain::variant_count() + 16 + 8 {
                            return Locations::new();
                        } else {
                            let mut locations = Locations::new();
                            locations.victory = buf[0] > 0;
                            let mut start = 1;
                            locations.villains.copy_from_slice(&buf[start..start + Villain::variant_count()]);
                            start += Villain::variant_count();
                            locations.team_villains.copy_from_slice(&buf[start..start + TeamVillain::variant_count()]);
                            start += TeamVillain::variant_count();
                            locations.variants = u128::from_le_bytes(buf[start..start + 16].try_into().unwrap());
                            start += 16;
                            locations.environments = u64::from_le_bytes(buf[start..start + 8].try_into().unwrap());
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
                let mut buf = vec![if locations.victory { 1 } else { 0 }];
                buf.extend(locations.villains);
                buf.extend(locations.team_villains);
                buf.extend(locations.variants.to_le_bytes());
                buf.extend(locations.environments.to_le_bytes());

                let _ = local_storage.set_item(&self.seed, &BASE64_STANDARD.encode(&buf));
            }
        }
    }
}
