use crate::{
    data::{TeamVillain, Villain},
    state::Locations,
};

use std::{
    fs::{create_dir_all, rename, File},
    io::{Read, Write},
    path::Path,
};

pub trait PersistentStore {
    fn new(seed: &str) -> Self;
    fn load(&self) -> Locations;
    fn save(&self, locations: &Locations);
}

pub struct DefaultPersistentStore {
    seed: String,
}

impl PersistentStore for DefaultPersistentStore {
    fn new(seed: &str) -> Self {
        DefaultPersistentStore { seed: seed.to_string() }
    }

    fn load(&self) -> Locations {
        if let Ok(mut reader) = File::open(Path::new("./persistent").join(&self.seed)) {
            let mut buf = vec![];
            if let Ok(len) = reader.read_to_end(&mut buf) {
                if len != 1 + Villain::variant_count() + TeamVillain::variant_count() + 16 + 8 {
                    println!("Save file is invalid. Was it made with an older version?");
                    let _ = rename(Path::new("./persistent").join(&self.seed), Path::new("./persistent").join(format!("{}-backup", self.seed)));
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

        Locations::new()
    }

    fn save(&self, locations: &Locations) {
        if let Err(err) = create_dir_all("./persistent") {
            println!("Failed to create persistent storage with error {err}");
        }

        match File::create(Path::new("./persistent").join(&self.seed)) {
            Ok(mut writer) => {
                let mut buf = vec![if locations.victory { 1 } else { 0 }];
                buf.extend(locations.villains);
                buf.extend(locations.team_villains);
                buf.extend(locations.variants.to_le_bytes());
                buf.extend(locations.environments.to_le_bytes());
                if let Err(err) = writer.write_all(&buf) {
                    println!("Failed to save locations to persistent storage with error {err}");
                }
            }
            Err(err) => println!("Failed to save locations to persistent storage with error {err}"),
        }
    }
}
