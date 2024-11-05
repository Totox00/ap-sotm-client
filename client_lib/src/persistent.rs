// SAVE LAYOUT
// FIELD                      | TYPE
// -------------------------- | -----
// villain_locations_len      | u16
// team_villain_locations_len | u16
// variant_locations_len      | u16
// environment_locations_len  | u16
// victory_sent               | u8
// villain_locations          | [u8; villain_locations_len]
// team_villain_locations     | [u8; team_villain_locations_len]
// variant_locations          | [u8; variant_locations_len]
// environments_locations     | [u8; environments_locations_len]

use crate::{
    data::{Environment, TeamVillain, Variant, Villain},
    state::Locations,
};
use std::{
    fs::{create_dir_all, rename, File},
    io::{Read, Write},
    path::Path,
};

pub trait PersistentStore {
    fn new(seed: &str, name: &str) -> Self;
    fn load(&self) -> Locations;
    fn save(&self, locations: &Locations);
}

pub struct DefaultPersistentStore {
    key: String,
}

impl PersistentStore for DefaultPersistentStore {
    fn new(seed: &str, name: &str) -> Self {
        DefaultPersistentStore { key: format!("{seed}-{name}") }
    }

    fn load(&self) -> Locations {
        if let Ok(mut reader) = File::open(Path::new("./persistent").join(&self.key)) {
            let mut buf = vec![];
            if let Ok(len) = reader.read_to_end(&mut buf) {
                if len > 8 + 1 + Villain::variant_count() + TeamVillain::variant_count() + Variant::variant_count() / 8 + 1 + Environment::variant_count() / 8 + 1 {
                    println!("Save file was made with a newer version and cannot be loaded.");
                    let _ = rename(Path::new("./persistent").join(&self.key), Path::new("./persistent").join(format!("{}-backup", self.key)));
                    return Locations::new();
                } else {
                    let mut locations = Locations::new();
                    let villain_len = u16::from_le_bytes(buf[0..2].try_into().unwrap()) as usize;
                    let team_villain_len = u16::from_le_bytes(buf[2..4].try_into().unwrap()) as usize;
                    let variant_len = u16::from_le_bytes(buf[4..6].try_into().unwrap()) as usize;
                    let environment_len = u16::from_le_bytes(buf[6..8].try_into().unwrap()) as usize;
                    locations.victory = buf[8] > 0;
                    let mut start = 9;
                    locations.villains.copy_from_slice(&buf[start..start + villain_len]);
                    start += villain_len;
                    locations.team_villains.copy_from_slice(&buf[start..start + team_villain_len]);
                    start += team_villain_len;
                    locations.variants.copy_from_slice(&buf[start..start + variant_len]);
                    start += variant_len;
                    locations.environments.copy_from_slice(&buf[start..start + environment_len]);
                    return locations;
                }
            }
        }

        Locations::new()
    }

    fn save(&self, locations: &Locations) {
        if let Err(err) = create_dir_all("./persistent") {
            println!("Failed to create persistent storage with error {err}");
        }

        match File::create(Path::new("./persistent").join(&self.key)) {
            Ok(mut writer) => {
                let mut buf = vec![];
                buf.extend((Villain::variant_count() as u16).to_le_bytes());
                buf.extend((TeamVillain::variant_count() as u16).to_le_bytes());
                buf.extend(((Variant::variant_count() / 8 + 1) as u16).to_le_bytes());
                buf.extend(((Environment::variant_count() / 8 + 1) as u16).to_le_bytes());
                buf.push(if locations.victory { 1 } else { 0 });
                buf.extend(locations.villains);
                buf.extend(locations.team_villains);
                buf.extend(locations.variants);
                buf.extend(locations.environments);
                if let Err(err) = writer.write_all(&buf) {
                    println!("Failed to save locations to persistent storage with error {err}");
                }
            }
            Err(err) => println!("Failed to save locations to persistent storage with error {err}"),
        }
    }
}
