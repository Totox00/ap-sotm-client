#![feature(iter_next_chunk)]

mod data;
mod datapackage;
mod format_json;
mod game;
mod interface;
mod persistent;
mod protocol;
mod state;
mod variant;

use data::{Item, Location};
use datapackage::DatapackageStore;
use format_json::format;
use interface::Interface;
use protocol::{Connected, DeathlinkType, SlotData};
use serde_json::from_str;
use state::State;
use std::collections::HashMap;
use variant::{on_click::on_click, state::TemporaryVariantProgress};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Session {
    datapackage_store: DatapackageStore,
    players: HashMap<i32, String>,
    slot: String,
    slot_data: SlotData,
    state: State,
    interface: Interface,
}

#[wasm_bindgen]
pub struct Action {
    locations: Vec<i64>,
    pub push: bool,
    deathlink: String,
    pub victory: bool,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn new_session(mut datapackage_store: DatapackageStore, connected: &str, slot: &str) -> Session {
    let connected: Connected = from_str(connected).unwrap_or_else(|err| {
        log!("Failed to parse Connected packet: {err}");
        panic!()
    });

    datapackage_store.build_player_map(&connected);

    let state = State::new(connected.slot_data);

    let mut players = HashMap::new();
    for player in connected.players {
        players.insert(player.slot, player.alias);
    }

    let interface = Interface::new();
    interface.update_goal(&state);

    Session {
        datapackage_store,
        players,
        slot: slot.to_string(),
        slot_data: state.slot_data,
        state,
        interface,
    }
}

#[wasm_bindgen]
impl Session {
    pub fn try_format_json(&self, json: &str) -> String {
        if let Ok(msg) = from_str(json) {
            format(&self.datapackage_store, msg, &self.players, &self.slot)
        } else {
            String::new()
        }
    }

    pub fn handle_click(&mut self, target: &str) -> Action {
        if target == "victory" {
            let mut locations = self.interface.get_locations();
            let mut push = false;
            for variant in self.state.available_variants().collect::<Vec<_>>() {
                if variant.is_available(&self.state, &self.interface.current_game) && variant.game_end(&mut self.state, &self.interface.current_game, true, &mut push) {
                    locations.push(Location::VariantUnlock(variant));
                }
            }
            let (victory, locations) = self.location_ids(&locations);
            self.interface.update_completion(&self.state);
            self.state.expend_filler(&self.interface.current_game);
            self.reset();
            return Action {
                deathlink: String::new(),
                push,
                locations,
                victory,
            };
        } else if target == "defeat" {
            let mut locations = vec![];
            let mut push = false;
            for variant in self.state.available_variants().collect::<Vec<_>>() {
                if variant.is_available(&self.state, &self.interface.current_game) && variant.game_end(&mut self.state, &self.interface.current_game, false, &mut push) {
                    locations.push(Location::VariantUnlock(variant));
                }
            }
            let (victory, locations) = self.location_ids(&locations);
            self.state.expend_filler(&self.interface.current_game);
            self.reset();
            return Action {
                deathlink: if self.slot_data.death_link == DeathlinkType::Team {
                    format!("{} was defeated by {}", self.slot, self.interface.current_game.villains.deathlink_name())
                } else {
                    String::new()
                },
                push,
                locations,
                victory,
            };
        } else if target == "goal" {
            if self.state.goal_progress().available() {
                return Action {
                    deathlink: String::new(),
                    push: false,
                    locations: vec![],
                    victory: true,
                };
            }
        } else if let Some(item) = Item::from_ident(target) {
            self.interface.toggle_selection(item, self.slot_data.death_link);
            self.interface.update_current_filler(&self.state.items, &self.state.expended_filler);
            self.interface.update_current_variants(&self.state);
        } else if target.starts_with("deathlink-") {
            if let Some(item) = Item::from_ident(target.split_at(10).1) {
                return Action {
                    locations: vec![],
                    push: false,
                    deathlink: format!("{} was incapacitated by {}", item.as_str(), self.interface.current_game.villains.deathlink_name()),
                    victory: false,
                };
            }
        } else if target.starts_with("diff-") {
            if let Some(item) = Item::from_ident(target.split_at(5).1) {
                self.interface.advance_difficulty(item);
                self.interface.update_current_filler(&self.state.items, &self.state.expended_filler);
                self.interface.update_current_variants(&self.state);
            }
        } else if target.starts_with("unlock-") {
            let item = Item::from_ident(target.split_at(7).1);
            if let Some(Item::Variant(variant)) = item {
                let (victory, locations) = self.location_ids(&[Location::VariantUnlock(variant)]);
                self.interface.update_current_variants(&self.state);
                self.interface.update_goal(&self.state);
                return Action {
                    deathlink: String::new(),
                    push: false,
                    locations,
                    victory,
                };
            } else if let Some(Item::Villain(villain)) = item {
                if let Some(variant) = villain.variant() {
                    let (victory, locations) = self.location_ids(&[Location::VariantUnlock(variant)]);
                    self.interface.update_current_variants(&self.state);
                    self.interface.update_goal(&self.state);
                    return Action {
                        deathlink: String::new(),
                        push: false,
                        locations,
                        victory,
                    };
                }
            }
        } else {
            let (victory, locations) = if let Some(variant) = on_click(target, &mut self.state, &self.interface.current_game) {
                self.location_ids(&[Location::VariantUnlock(variant)])
            } else {
                (false, vec![])
            };
            self.interface.update_current_variants(&self.state);
            self.interface.update_goal(&self.state);
            return Action {
                deathlink: String::new(),
                push: false,
                locations,
                victory,
            };
        }
        Action::none()
    }

    fn reset(&mut self) {
        self.state.temporary_variant_progress = TemporaryVariantProgress::default();
        self.interface.update_current_filler(&self.state.items, &self.state.expended_filler);
        self.interface.update_current_variants(&self.state);
        self.interface.update_goal(&self.state);
    }

    fn location_ids(&mut self, locations: &[Location]) -> (bool, Vec<i64>) {
        let mut victory = false;
        let mut location_ids = vec![];
        for location in locations {
            if *location == Location::Victory {
                victory = true;
            } else if self.state.checked_locations.has_unchecked_location(*location) {
                self.state.checked_locations.mark_location(*location);
                for n in 0..self.slot_data.locations_per[match location {
                    Location::Villain((_, d)) | Location::TeamVillain((_, d)) | Location::Gladiator((_, d)) => *d as usize,
                    Location::Hero(_) | Location::Variant(_) => 4,
                    Location::Environment(_) => 5,
                    Location::VariantUnlock(_) => 6,
                    Location::Victory => unreachable!(),
                }] {
                    location_ids.push(location.as_id(n as i64));
                }
            }
        }
        (victory, location_ids)
    }

    pub fn recieved_items(&mut self, items: Vec<i64>) {
        for item_id in items {
            if let Some(item) = Item::from_id(item_id) {
                if let Item::Filler((filler, duration)) = item {
                    self.state.items.add_filler(filler, duration * self.slot_data.filler_duration);
                } else {
                    self.interface.add_item(&self.state, item);
                    self.state.items.set_item(item);
                }
            }
        }
        self.interface.update_current_filler(&self.state.items, &self.state.expended_filler);
        self.interface.update_current_variants(&self.state);
        self.interface.update_goal(&self.state);
    }

    pub fn save_string(&self) -> String {
        persistent::save_string(&self.state.checked_locations, &self.state.persistent_variant_progress, &self.state.expended_filler)
    }

    pub fn update_save(&mut self, save_str: &str) {
        let (locations, variant_progress, expended_filler, err_msg) = persistent::load_string(save_str);
        if let Some(msg) = err_msg {
            log!("{msg}");
        }
        self.state.update_expended_filler(&expended_filler);
        self.state.checked_locations.update(&locations);
        self.state.persistent_variant_progress.update(&variant_progress);
        self.interface.update_goal(&self.state);
        self.interface.update_current_filler(&self.state.items, &self.state.expended_filler);
        self.interface.update_current_variants(&self.state);
        self.interface.update_completion_all(&self.state);
    }

    pub fn deathlink_type(&self) -> u8 {
        match self.slot_data.death_link {
            DeathlinkType::None => 0,
            DeathlinkType::Individual => 1,
            DeathlinkType::Team => 2,
        }
    }
}

impl Action {
    fn none() -> Action {
        Action {
            locations: vec![],
            push: false,
            deathlink: String::new(),
            victory: false,
        }
    }
}

#[wasm_bindgen]
impl Action {
    pub fn locations(&self) -> Vec<i64> {
        self.locations.clone()
    }

    pub fn deathlink(&self) -> String {
        self.deathlink.clone()
    }
}
