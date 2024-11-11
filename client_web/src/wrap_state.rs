use client_lib::{
    data::{Contender, Environment, Hero, Item, Location, TeamVillain, Variant, Villain},
    state::State,
};
use num::FromPrimitive;
use std::fmt::Write;
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct WasmState {
    available: WasmAvailable,
    villains: Vec<WasmItem>,
    team_villains: Vec<WasmItem>,
    environments: Vec<WasmItem>,
    heroes: Vec<WasmHero>,
    pub scions: i32,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmAvailable {
    pub victory: bool,
    villains: Vec<WasmLocation>,
    team_villains: Vec<WasmLocation>,
    environments: Vec<WasmLocation>,
    variants: Vec<WasmLocation>,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmLocation {
    inner: Location,
    name: String,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmItem {
    inner: Item,
    name: String,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmHero {
    inner: Hero,
    bitfield: u8,
    name: String,
    pub real: bool,
}

pub fn wrap_state(state: &State) -> WasmState {
    let available = state.available_locations();
    let mut heroes: Vec<WasmHero> = state
        .items
        .heroes
        .iter()
        .zip(0..)
        .filter_map(|(b, h)| Hero::from_i32(h).map(|hero| (b, hero)))
        .filter(|(b, _)| b.count_ones() > 0)
        .map(|(bitfield, hero)| WasmHero {
            inner: hero,
            bitfield: *bitfield,
            name: if *bitfield == 1 {
                hero.as_str().to_owned()
            } else if bitfield.count_ones() == 1 {
                if let Some(variant) = Variant::from_hero(hero, bitfield.trailing_zeros()) {
                    variant.as_str().to_owned()
                } else {
                    String::new()
                }
            } else {
                let mut buf = String::new();

                let _ = write!(&mut buf, "{}<ul>", hero.as_str());
                if bitfield & 1 > 0 {
                    let _ = write!(&mut buf, "<li class=\"variant\">{}</li>", hero.as_str());
                }
                for variant in (1..7).filter(|o| bitfield & 1 << o > 1) {
                    if let Some(variant) = Variant::from_hero(hero, variant) {
                        let _ = write!(&mut buf, "<li class=\"variant\">{}</li>", variant.as_str());
                    }
                }
                let _ = write!(&mut buf, "</ul>");

                buf
            },
            real: true,
        })
        .collect();
    if state.items.contenders.iter().copied().map(u8::count_ones).sum::<u32>() >= 3 {
        heroes.push(WasmHero {
            inner: Hero::Legacy,
            bitfield: 0,
            name: {
                let mut buf = String::new();

                let _ = write!(&mut buf, "The Contenders<ul>");
                for contender in 0..Contender::variant_count() {
                    if let Some(contender) = Contender::from_usize(contender) {
                        if state.items.has_contender(contender) {
                            let _ = write!(&mut buf, "<li class=\"variant\">{}</li>", contender.as_str());
                        }
                    }
                }
                let _ = write!(&mut buf, "</ul>");

                buf
            },
            real: false,
        });
    }

    WasmState {
        available: WasmAvailable {
            victory: available.victory,
            villains: available
                .villains
                .into_iter()
                .map(|(v, d)| WasmLocation {
                    inner: Location::Villain((v, d)),
                    name: format!("{} - {}", v.as_str(), difficulty(&d)),
                })
                .collect(),
            team_villains: available
                .team_villains
                .into_iter()
                .map(|(v, d)| WasmLocation {
                    inner: Location::TeamVillain((v, d)),
                    name: format!("{} - {}", v.as_str(), difficulty(&d)),
                })
                .collect(),
            environments: available
                .environments
                .into_iter()
                .map(|e| WasmLocation {
                    inner: Location::Environment(e),
                    name: format!("{} - Any Difficulty", e.as_str()),
                })
                .collect(),
            variants: available
                .variants
                .into_iter()
                .map(|v| WasmLocation {
                    inner: Location::Variant(v),
                    name: format!("{} - Unlock", v.as_str()),
                })
                .collect(),
        },
        villains: Villain::iter()
            .filter(|v| state.items.has_villain(*v))
            .map(|v| WasmItem {
                inner: Item::Villain(v),
                name: v.as_str().to_owned(),
            })
            .collect(),
        team_villains: TeamVillain::iter()
            .filter(|v| state.items.has_team_villain(*v))
            .map(|v| WasmItem {
                inner: Item::TeamVillain(v),
                name: v.as_str().to_owned(),
            })
            .collect(),
        environments: Environment::iter()
            .filter(|e| state.items.has_environment(*e))
            .map(|e| WasmItem {
                inner: Item::Environment(e),
                name: e.as_str().to_owned(),
            })
            .collect(),
        heroes,
        scions: state.items.scions as i32,
    }
}

impl WasmLocation {
    pub fn as_inner(&self) -> &Location {
        &self.inner
    }

    pub fn into_inner(self) -> Location {
        self.inner
    }
}

#[wasm_bindgen]
impl WasmLocation {
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl WasmItem {
    pub fn as_inner(&self) -> &Item {
        &self.inner
    }

    pub fn into_inner(self) -> Item {
        self.inner
    }
}

#[wasm_bindgen]
impl WasmItem {
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl WasmHero {
    pub fn as_inner(&self) -> &Hero {
        &self.inner
    }

    pub fn into_inner(self) -> (Hero, u8) {
        (self.inner, self.bitfield)
    }
}

#[wasm_bindgen]
impl WasmHero {
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[wasm_bindgen]
impl WasmState {
    pub fn available(&self) -> WasmAvailable {
        self.available.clone()
    }

    pub fn villains(&self) -> Vec<WasmItem> {
        self.villains.clone()
    }

    pub fn team_villains(&self) -> Vec<WasmItem> {
        self.team_villains.clone()
    }

    pub fn environments(&self) -> Vec<WasmItem> {
        self.environments.clone()
    }

    pub fn heroes(&self) -> Vec<WasmHero> {
        self.heroes.clone()
    }
}

#[wasm_bindgen]
impl WasmAvailable {
    pub fn villains(&self) -> Vec<WasmLocation> {
        self.villains.clone()
    }

    pub fn team_villains(&self) -> Vec<WasmLocation> {
        self.team_villains.clone()
    }

    pub fn environments(&self) -> Vec<WasmLocation> {
        self.environments.clone()
    }

    pub fn variants(&self) -> Vec<WasmLocation> {
        self.variants.clone()
    }
}

fn difficulty(d: &u8) -> &str {
    match d {
        0 => "Normal",
        1 => "Advanced",
        2 => "Challenge",
        3 => "Ultimate",
        _ => unreachable!(),
    }
}
