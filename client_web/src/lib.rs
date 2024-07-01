mod datapackage;
mod format_json;
mod persistent;
mod wrap_state;

use archipelago_protocol::{Connected, RoomInfo};
use client_lib::{data::Location, datapackage::DatapackageStore, persistent::PersistentStore, Session};
use datapackage::WebDatapackageStore;
use format_json::format;
use persistent::WebPersistentStore;
use serde_json::from_str;
use wasm_bindgen::prelude::wasm_bindgen;
use wrap_state::{wrap_state, WasmLocation, WasmState};

#[wasm_bindgen]
pub struct WasmSession {
    inner: Session<WebDatapackageStore, WebPersistentStore>,
}

#[wasm_bindgen]
pub fn new_session(mut datapackage_store: WebDatapackageStore, room_info: &str, connected: &str, slot: &str) -> WasmSession {
    if let (Ok(room_info), Ok(connected)) = (from_str::<RoomInfo>(room_info), from_str::<Connected>(connected)) {
        datapackage_store.build_player_map(&connected);

        WasmSession {
            inner: Session::new(&room_info.seed_name, datapackage_store, connected, slot),
        }
    } else {
        panic!("Failed to parse session info");
    }
}

#[wasm_bindgen]
impl WasmSession {
    pub fn try_format_json(&self, json: &str) -> String {
        if let Ok(msg) = from_str(json) {
            format(&self.inner.datapackage_store, msg, &self.inner.players, &self.inner.slot)
        } else {
            String::new()
        }
    }

    pub fn get_location_ids(&mut self, locations: Vec<WasmLocation>) -> Vec<i32> {
        let mut location_ids = vec![];

        for location in locations.into_iter().map(|l| l.into_inner()) {
            self.inner.state.checked_locations.mark_location(location);

            for n in 0..=self.inner.state.slot_data.locations_per[match location {
                Location::Variant(_) => 5,
                Location::Villain((_, d)) | Location::TeamVillain((_, d)) => d as usize,
                Location::Environment(_) => 4,
                Location::Victory => unreachable!(),
            }] {
                if n > 0 {
                    if let Some(id) = self.inner.datapackage_store.id_from_own_location((location, n)) {
                        location_ids.push(id as i32)
                    }
                }
            }
        }

        location_ids
    }

    pub fn get_state(&self) -> WasmState {
        wrap_state(&self.inner.state)
    }

    pub fn recieved_items(&mut self, items: Vec<i64>) {
        for item in items.into_iter().filter_map(|id| self.inner.datapackage_store.id_to_own_item(id)) {
            self.inner.state.items.set_item(item);
        }
    }

    pub fn exit(&self) {
        self.inner.persistent_store.save(&self.inner.state.checked_locations);
    }

    pub fn victory(&mut self) {
        self.inner.state.checked_locations.victory = true;
    }
}
