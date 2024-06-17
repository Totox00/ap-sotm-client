use crate::data::{Item, Location};
use archipelago_rs::protocol::{Connected, DataPackageObject, GameData as ArchipelagoGameData};
use serde_json::{from_reader, to_writer};
use std::{
    collections::HashMap,
    fs::{create_dir, create_dir_all, remove_file, File},
    io::{stdout, Write},
    path::Path,
    sync::Arc,
};

pub type Requested = HashMap<String, String>;

pub trait DatapackageStore {
    fn new(requested: Requested) -> Self;
    fn missing_games(&self) -> Box<[String]>;
    fn cache(&mut self, data: DataPackageObject);
    fn build_player_map(&mut self, connected: &Connected);
    fn get_item(&self, player: i32, id: i64) -> &str;
    fn get_location(&self, player: i32, id: i64) -> &str;
    fn id_to_own_item(&self, id: i64) -> Option<Item>;
    fn id_to_own_location(&self, id: i64) -> Option<Location>;
    fn id_from_own_item(&self, item: Item) -> Option<i64>;
    fn id_from_own_location(&self, location: (Location, u8)) -> Option<i64>;
}

#[derive(Debug, Clone)]
struct GameData {
    item_id_to_name: HashMap<i64, String>,
    location_id_to_name: HashMap<i64, String>,
}

#[derive(Debug, Clone)]
pub struct DefaultDatapackageStore {
    items_from_id: HashMap<i64, Item>,
    items_to_id: HashMap<Item, i64>,
    locations_from_id: HashMap<i64, (Location, u8)>,
    locations_to_id: HashMap<(Location, u8), i64>,
    data: HashMap<String, Arc<GameData>>,
    missing: Vec<String>,
    missing_checksums: Vec<String>,
    player_to_game: HashMap<i32, Arc<GameData>>,
}

impl DefaultDatapackageStore {
    fn add_game(&mut self, game: String, data: ArchipelagoGameData) {
        if *game == *"Sentinels of the Multiverse" {
            for (item, id) in &data.item_name_to_id {
                if let Some(item) = Item::from_str(item) {
                    self.items_from_id.insert(*id, item);
                    self.items_to_id.insert(item, *id);
                }
            }

            for (location, id) in &data.location_name_to_id {
                if let Some(location) = Location::from_str(location) {
                    self.locations_from_id.insert(*id, location);
                    self.locations_to_id.insert(location, *id);
                }
            }
        }

        let mut item_id_to_name = HashMap::new();
        let mut location_id_to_name = HashMap::new();

        for (item, id) in data.item_name_to_id {
            item_id_to_name.insert(id, item);
        }
        for (location, id) in data.location_name_to_id {
            location_id_to_name.insert(id, location);
        }

        self.data.insert(game, Arc::new(GameData { item_id_to_name, location_id_to_name }));
    }
}

impl DatapackageStore for DefaultDatapackageStore {
    fn new(requested: Requested) -> Self {
        let mut new = Self {
            data: HashMap::new(),
            missing: vec![],
            missing_checksums: vec![],
            player_to_game: HashMap::new(),
            items_from_id: HashMap::new(),
            items_to_id: HashMap::new(),
            locations_from_id: HashMap::new(),
            locations_to_id: HashMap::new(),
        };

        for (game, checksum) in requested {
            if let Ok(reader) = File::open(Path::new("./datapackage").join(&game).join(&checksum)) {
                if let Ok(data) = from_reader::<File, archipelago_rs::protocol::GameData>(reader) {
                    new.add_game(game, data);
                } else {
                    new.missing.push(game);
                    new.missing_checksums.push(checksum);
                }
            } else {
                new.missing.push(game);
                new.missing_checksums.push(checksum);
            }
        }

        new
    }

    fn missing_games(&self) -> Box<[String]> {
        let mut lock = stdout().lock();
        for game in &self.missing {
            let _ = writeln!(lock, "Missing datapackage for {game}...");
        }

        self.missing.clone().into()
    }

    fn cache(&mut self, data: DataPackageObject) {
        create_dir_all("./datapackage").expect("Failed to create datapackage cache");

        for (game, checksum) in self.missing.iter().zip(&self.missing_checksums) {
            if let Some(data) = data.games.get(game) {
                let _ = create_dir(Path::new("./datapackage").join(game));
                if let Ok(writer) = File::create(Path::new("./datapackage").join(game).join(checksum)) {
                    let res = to_writer(writer, data);
                    if res.is_err() {
                        remove_file(Path::new("./datapackage").join(game).join(checksum)).unwrap_or_else(|_| panic!("Failed to clean failed write for {game}-{checksum}. Data may be corrupt"));
                    }
                }
            }
        }

        for (game, data) in data.games {
            self.add_game(game, data);
        }
    }

    fn build_player_map(&mut self, connected: &Connected) {
        for player in connected.players.iter().map(|p| p.slot) {
            if let Some(slot) = connected.slot_info.get(&*player.to_string()) {
                if let Some(rc) = self.data.get(&*slot.game) {
                    self.player_to_game.insert(player, rc.clone());
                }
            }
        }
    }

    fn get_item(&self, player: i32, id: i64) -> &str {
        if let Some(data) = self.player_to_game.get(&player) {
            if let Some(item) = data.item_id_to_name.get(&id) {
                return item;
            }
        }

        "Unknown item"
    }

    fn get_location(&self, player: i32, id: i64) -> &str {
        if let Some(data) = self.player_to_game.get(&player) {
            if let Some(location) = data.location_id_to_name.get(&id) {
                return location;
            }
        }

        "Unknown location"
    }

    fn id_to_own_item(&self, id: i64) -> Option<Item> {
        self.items_from_id.get(&id).copied()
    }

    fn id_to_own_location(&self, id: i64) -> Option<Location> {
        self.locations_from_id.get(&id).map(|(l, _)| *l)
    }

    fn id_from_own_item(&self, item: Item) -> Option<i64> {
        self.items_to_id.get(&item).copied()
    }

    fn id_from_own_location(&self, location: (Location, u8)) -> Option<i64> {
        self.locations_to_id.get(&location).copied()
    }
}
