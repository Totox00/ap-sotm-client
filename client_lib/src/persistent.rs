// SAVE LAYOUT
// FIELD                      | TYPE
// -------------------------- | -----
// meta                       | [RRRRRRNV] R = reserved, N = new version (should always be 1), V = victory sent
// villain_locations_len      | u16
// team_villain_locations_len | u16
// gladiator_locations_len    | u16
// variant_locations_len      | u16
// environment_locations_len  | u16
// villain_locations          | [u8; villain_locations_len]
// team_villain_locations     | [u8; team_villain_locations_len]
// gladiator_locations        | [u8; gladiator_locations_len]
// variant_locations          | [u8; variant_locations_len]
// environments_locations     | [u8; environments_locations_len]

use crate::{
    data::{Environment, Gladiator, TeamVillain, Variant, Villain},
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
                if len > 10 + 1 + Villain::variant_count() + TeamVillain::variant_count() + Gladiator::variant_count() + Variant::variant_count() / 8 + 1 + Environment::variant_count() / 8 + 1 {
                    println!("Save file was made with a newer version and cannot be loaded.");
                    let _ = rename(Path::new("./persistent").join(&self.key), Path::new("./persistent").join(format!("{}-backup", self.key)));
                    return Locations::new();
                } else if buf[0] & 2 == 0 {
                    println!("Save file was made with a too old version and cannot be loaded.");
                    let _ = rename(Path::new("./persistent").join(&self.key), Path::new("./persistent").join(format!("{}-backup", self.key)));
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

        Locations::new()
    }

    fn save(&self, locations: &Locations) {
        if let Err(err) = create_dir_all("./persistent") {
            println!("Failed to create persistent storage with error {err}");
        }

        match File::create(Path::new("./persistent").join(&self.key)) {
            Ok(mut writer) => {
                let mut buf = vec![];
                buf.push(if locations.victory { 0b11 } else { 0b10 });
                buf.extend((Villain::variant_count() as u16).to_le_bytes());
                buf.extend((TeamVillain::variant_count() as u16).to_le_bytes());
                buf.extend((Gladiator::variant_count() as u16).to_le_bytes());
                buf.extend(((Variant::variant_count() / 8 + 1) as u16).to_le_bytes());
                buf.extend(((Environment::variant_count() / 8 + 1) as u16).to_le_bytes());
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
