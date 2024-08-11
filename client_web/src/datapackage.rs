use archipelago_protocol::{Connected, DataPackageObject, GameData as ArchipelagoGameData, RoomInfo};
use client_lib::datapackage::DatapackageStore;
use serde_json::from_str;
use std::{collections::HashMap, sync::Arc};
use wasm_bindgen::prelude::wasm_bindgen;

pub type Requested = HashMap<String, String>;

#[derive(Debug, Clone)]
struct GameData {
    item_id_to_name: HashMap<i64, String>,
    location_id_to_name: HashMap<i64, String>,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct WebDatapackageStore {
    data: HashMap<String, Arc<GameData>>,
    missing: Vec<String>,
    missing_checksums: Vec<String>,
    player_to_game: HashMap<i32, Arc<GameData>>,
}

#[wasm_bindgen]
impl WebDatapackageStore {
    pub fn add_game(&mut self, game: String, data: &str) {
        if let Ok(data) = from_str(data) {
            self.add_game_internal(game, data)
        }
    }

    fn add_game_internal(&mut self, game: String, data: ArchipelagoGameData) {
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

    pub fn item(&self, id: i64) -> String {
        if let Some(game_data) = self.data.get("Sentinels of the Multiverse") {
            game_data.item_id_to_name.get(&id).map(|i| i.to_owned()).unwrap_or(format!("Unknown item {id}"))
        } else {
            format!("Unknown item {id}")
        }
    }

    pub fn location(&self, id: i64) -> String {
        if let Some(game_data) = self.data.get("Sentinels of the Multiverse") {
            game_data.location_id_to_name.get(&id).map(|i| i.to_owned()).unwrap_or(format!("Unknown location {id}"))
        } else {
            format!("Unknown location {id}")
        }
    }

    pub fn get_missing_games(&self) -> Vec<String> {
        self.missing_games().into()
    }
}

impl DatapackageStore for WebDatapackageStore {
    fn new(requested: Requested) -> Self {
        let mut new = Self {
            data: HashMap::new(),
            missing: vec![],
            missing_checksums: vec![],
            player_to_game: HashMap::new(),
        };

        for (game, checksum) in requested {
            new.missing.push(game);
            new.missing_checksums.push(checksum);
        }

        new
    }

    fn missing_games(&self) -> Box<[String]> {
        self.missing.clone().into()
    }

    fn cache(&mut self, data: DataPackageObject) {
        for (game, data) in data.games {
            self.add_game_internal(game, data);
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
}

#[wasm_bindgen]
pub fn new_datapackage_store(room_info: &str) -> WebDatapackageStore {
    if let Ok(room_info) = from_str::<RoomInfo>(room_info) {
        WebDatapackageStore::new(room_info.datapackage_checksums)
    } else {
        panic!("Failed to parse room_info")
    }
}
