mod datapackage;
mod format_json;
mod persistent;
mod wrap_state;

use archipelago_protocol::{Connected, DeathlinkType, RoomInfo};
use client_lib::{
    data::{FillerTarget, HeroLike, Item, Location, VillainLike},
    datapackage::DatapackageStore,
    persistent::PersistentStore,
    Session,
};
use datapackage::WebDatapackageStore;
use format_json::format;
use persistent::WebPersistentStore;
use serde_json::from_str;
use std::fmt::Write;
use wasm_bindgen::prelude::wasm_bindgen;
use wrap_state::{wrap_state, WasmHero, WasmItem, WasmLocation, WasmState};

#[wasm_bindgen]
pub struct WasmSession {
    inner: Session<WebDatapackageStore, WebPersistentStore>,
}

#[wasm_bindgen]
pub fn new_session(mut datapackage_store: WebDatapackageStore, room_info: &str, connected: &str, slot: &str) -> WasmSession {
    if let (Ok(room_info), Ok(connected)) = (from_str::<RoomInfo>(room_info), from_str::<Connected>(connected)) {
        datapackage_store.build_player_map(&connected);
        let session = Session::new(&room_info.seed_name, datapackage_store, connected, slot);

        WasmSession { inner: session }
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

    pub fn get_location_ids(&mut self, locations: Vec<WasmLocation>) -> Vec<i64> {
        let mut location_ids = vec![];

        for location in locations.into_iter().map(|l| l.into_inner()) {
            self.inner.state.checked_locations.mark_location(location);

            for n in 0..self.inner.state.slot_data.locations_per[match location {
                Location::Variant(_) => 5,
                Location::Villain((_, d)) | Location::TeamVillain((_, d)) => d as usize,
                Location::Environment(_) => 4,
                Location::Victory => unreachable!(),
            }] {
                location_ids.push(location.as_id(n as i64));
            }
        }

        location_ids
    }

    pub fn get_state(&self) -> WasmState {
        wrap_state(&self.inner.state)
    }

    pub fn recieved_items(&mut self, items: Vec<i64>) {
        for item_id in items {
            self.inner.state.items.set_item(Item::from_id(item_id));
        }
    }

    pub fn get_filler_for_location(&self, target: &WasmLocation, show_desc: bool) -> String {
        if let Some(item) = target.as_inner().as_item() {
            self.get_filler_for_item_internal(item, show_desc)
        } else {
            String::new()
        }
    }

    pub fn get_filler_for_hero(&self, target: &WasmHero, show_desc: bool) -> String {
        if !target.real {
            return String::new();
        }

        let mut buf = String::new();

        if show_desc {
            if self.inner.state.items.has_base_hero(*target.as_inner()) {
                let mut inner_buf = String::new();

                for (filler, count) in self.inner.state.items.get_filler_for(FillerTarget::Hero(HeroLike::Hero(*target.as_inner()))) {
                    let _ = write!(
                        inner_buf,
                        "<li><span>{}</span><br /><span class=\"desc\">{}</span></li>",
                        filler.to_string(count),
                        filler.to_desc(count)
                    );
                }

                if !inner_buf.is_empty() {
                    let _ = write!(buf, "<li><span><b>{}</b></span><ul>{inner_buf}</ul></li>", target.as_inner().as_str());
                }
            }

            for variant in self.inner.state.items.variants_of(*target.as_inner()) {
                let mut inner_buf = String::new();

                for (filler, count) in self.inner.state.items.get_filler_for(FillerTarget::Hero(HeroLike::Variant(variant))) {
                    let _ = write!(
                        inner_buf,
                        "<li><span>{}</span><br /><span class=\"desc\">{}</span></li>",
                        filler.to_string(count),
                        filler.to_desc(count)
                    );
                }

                if !inner_buf.is_empty() {
                    let _ = write!(buf, "<li><span><b>{}</b></span><ul>{inner_buf}</ul></li>", variant.as_str());
                }
            }
        } else {
            if self.inner.state.items.has_base_hero(*target.as_inner()) {
                let mut inner_buf = String::new();

                for (filler, count) in self.inner.state.items.get_filler_for(FillerTarget::Hero(HeroLike::Hero(*target.as_inner()))) {
                    let _ = write!(inner_buf, "<li><span>{}</span></li>", filler.to_string(count));
                }

                if !inner_buf.is_empty() {
                    let _ = write!(buf, "<li><span><b>{}</b></span><ul>{inner_buf}</ul></li>", target.as_inner().as_str());
                }
            }

            for variant in self.inner.state.items.variants_of(*target.as_inner()) {
                let mut inner_buf = String::new();

                for (filler, count) in self.inner.state.items.get_filler_for(FillerTarget::Hero(HeroLike::Variant(variant))) {
                    let _ = write!(inner_buf, "<li><span>{}</span></li>", filler.to_string(count));
                }

                if !inner_buf.is_empty() {
                    let _ = write!(buf, "<li><span><b>{}</b></span><ul>{inner_buf}</ul></li>", variant.as_str());
                }
            }
        }

        buf
    }

    pub fn get_filler_for_item(&self, target: &WasmItem, show_desc: bool) -> String {
        self.get_filler_for_item_internal(*target.as_inner(), show_desc)
    }

    fn get_filler_for_item_internal(&self, item: Item, show_desc: bool) -> String {
        let mut buf = String::new();

        if show_desc {
            match item {
                Item::Villain(v) => {
                    if let Some((name, desc)) = v.challenge_desc() {
                        let _ = write!(buf, "<h3>Challenge - {name}</h3>");
                        for paragraph in desc {
                            let _ = write!(buf, "<p>{paragraph}</p>");
                        }
                    }
                    for (filler, count) in self.inner.state.items.get_filler_for(FillerTarget::Villain(VillainLike::Villain(v))) {
                        let _ = write!(buf, "<li><span>{}</span><br /><span class=\"desc\">{}</span></li>", filler.to_string(count), filler.to_desc(count));
                    }
                }
                Item::TeamVillain(v) => {
                    if let Some((name, desc)) = v.challenge_desc() {
                        let _ = write!(buf, "<h3>Challenge - {name}</h3>");
                        for paragraph in desc {
                            let _ = write!(buf, "<p>{paragraph}</p>");
                        }
                    }
                    for (filler, count) in self.inner.state.items.get_filler_for(FillerTarget::Villain(VillainLike::TeamVillain(v))) {
                        let _ = write!(buf, "<li><span>{}</span><br /><span class=\"desc\">{}</span></li>", filler.to_string(count), filler.to_desc(count));
                    }
                }
                Item::Environment(_) => {
                    for (filler, count) in self.inner.state.items.get_filler_for(FillerTarget::Other) {
                        let _ = write!(buf, "<li><span>{}</span><br /><span class=\"desc\">{}</span></li>", filler.to_string(count), filler.to_desc(count));
                    }
                }
                _ => (),
            }
        } else {
            match item {
                Item::Villain(v) => {
                    for (filler, count) in self.inner.state.items.get_filler_for(FillerTarget::Villain(VillainLike::Villain(v))) {
                        let _ = write!(buf, "<li><span>{}</span></li>", filler.to_string(count));
                    }
                }
                Item::TeamVillain(v) => {
                    for (filler, count) in self.inner.state.items.get_filler_for(FillerTarget::Villain(VillainLike::TeamVillain(v))) {
                        let _ = write!(buf, "<li><span>{}</span></li>", filler.to_string(count));
                    }
                }
                Item::Environment(_) => {
                    for (filler, count) in self.inner.state.items.get_filler_for(FillerTarget::Other) {
                        let _ = write!(buf, "<li><span>{}</span></li>", filler.to_string(count));
                    }
                }
                _ => (),
            }
        }

        buf
    }

    pub fn get_variant_desc(&self, target: &WasmLocation) -> String {
        match target.as_inner().as_item() {
            Some(Item::Variant(v)) => v.as_desc().replace('_', "\u{00A0}").to_string(),
            _ => String::new(),
        }
    }

    pub fn exit(&self) {
        self.inner.persistent_store.save(&self.inner.state.checked_locations);
    }

    pub fn victory(&mut self) {
        self.inner.state.checked_locations.victory = true;
    }

    pub fn deathlink(&self) -> u8 {
        match self.inner.slot_data.death_link {
            DeathlinkType::None => 0,
            DeathlinkType::Individual => 1,
            DeathlinkType::Team => 2,
        }
    }
}
