use std::{env::var, sync::Arc};

use strum::IntoEnumIterator;

use crate::{
    archipelago_rs::protocol::{Connected, DataPackageObject, ReceivedItems},
    data::{Environment, Hero, Item, Location, TeamVillain, Variant, Villain},
    idmap::IdMap,
    logic::can_unlock,
};

#[derive(Debug, Clone)]
pub struct State {
    pub items: Items,
    pub checked_locations: Locations,
    pub idmap: Arc<IdMap>,
}

#[derive(Debug, Clone, Copy)]
pub struct Items {
    pub scions: u8,
    pub villains: u64,
    pub team_villains: u16,
    pub heroes: [u8; Hero::variant_count()],
    pub environments: u64,
    pub filler: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Locations {
    pub victory: bool,
    pub villains: [u8; Villain::variant_count()],
    pub team_villains: [u8; TeamVillain::variant_count()],
    pub variants: u128,
    pub environments: u64,
}

#[derive(Debug, Clone)]
pub struct AvailableLocations {
    pub victory: bool,
    pub villains: Vec<(Villain, u8)>,
    pub team_villains: Vec<(TeamVillain, u8)>,
    pub variants: Vec<Variant>,
    pub environments: Vec<Environment>,
}

impl State {
    pub fn new(datapackage: &DataPackageObject) -> Self {
        State {
            items: Items::new(),
            checked_locations: Locations::new(),
            idmap: Arc::new(IdMap::new(datapackage)),
        }
    }

    pub fn sync(&mut self, connected: Connected, sync: ReceivedItems) {
        for location_id in connected.checked_locations {
            match self.idmap.locations_from_id.get(&location_id) {
                Some(Location::Variant(v)) => self.checked_locations.mark_variant(*v),
                Some(Location::Villain((v, d))) => self.checked_locations.mark_villain(*v, *d),
                Some(Location::TeamVillain((v, d))) => self.checked_locations.mark_team_villain(*v, *d),
                Some(Location::Environment(v)) => self.checked_locations.mark_environment(*v),
                Some(Location::Victory) => self.checked_locations.victory = true,
                None => (),
            }
        }

        for item in sync.items {
            match self.idmap.items_from_id.get(&item.item) {
                Some(Item::Hero(v)) => self.items.set_hero(*v),
                Some(Item::Variant(v)) => self.items.set_hero_variant(*v),
                Some(Item::Villain(v)) => self.items.set_villain(*v),
                Some(Item::TeamVillain(v)) => self.items.set_team_villain(*v),
                Some(Item::Environment(v)) => self.items.set_environment(*v),
                Some(Item::Scion) => self.items.scions += 1,
                Some(Item::Filler) => self.items.filler += 1,
                None => (),
            }
        }
    }

    pub fn available_locations(&self) -> AvailableLocations {
        AvailableLocations {
            victory: !self.checked_locations.victory && self.items.scions >= 10,
            villains: Villain::iter()
                .filter(|v| self.items.has_villain(*v))
                .map(|v| {
                    (
                        v,
                        [0, 1, 2, 3]
                            .iter()
                            .filter(|d| self.checked_locations.has_unchecked_villain(v, **d))
                            .map(|d| 1 << d)
                            .fold(0, |acc, x| acc | x),
                    )
                })
                .map(|(v, d)| if v != Villain::SkinwalkerGloomweaver { (v, d) } else { (v, d & 0b11) })
                .filter(|(_, d)| *d > 0)
                .map(|(v, d)| {
                    if v != Villain::SpiteAgentOfGloom || self.items.has_villain(Villain::SkinwalkerGloomweaver) {
                        (v, d)
                    } else {
                        (v, d & 0b11)
                    }
                })
                .collect(),
            team_villains: if self.items.team_villains.count_ones() < 3 {
                vec![]
            } else {
                TeamVillain::iter()
                    .filter(|v| self.items.has_team_villain(*v))
                    .map(|v| {
                        (
                            v,
                            [0, 1, 2, 3]
                                .iter()
                                .filter(|d| self.checked_locations.has_unchecked_team_villain(v, **d))
                                .map(|d| 1 << d)
                                .fold(0, |acc, x| acc | x),
                        )
                    })
                    .filter(|(_, d)| *d > 0)
                    .collect()
            },
            variants: Variant::iter()
                .filter(|v| self.checked_locations.has_unchecked_variant(*v))
                .filter(|v| can_unlock(*v, &self.items))
                .collect(),
            environments: Environment::iter()
                .filter(|v| self.checked_locations.has_unchecked_environment(*v))
                .filter(|v| self.items.has_environment(*v))
                .collect(),
        }
    }
}

impl Items {
    pub fn new() -> Self {
        Items {
            scions: 0,
            villains: 0,
            team_villains: 0,
            heroes: [0; Hero::variant_count()],
            environments: 0,
            filler: 0,
        }
    }

    pub fn has_villain(&self, villain: Villain) -> bool {
        self.villains & 1 << villain as u64 > 0
    }

    pub fn has_team_villain(&self, team_villain: TeamVillain) -> bool {
        self.team_villains & 1 << team_villain as u16 > 0
    }

    pub fn has_hero(&self, hero: Hero) -> bool {
        self.heroes[hero as usize] > 0
    }

    pub fn has_base_hero(&self, hero: Hero) -> bool {
        self.heroes[hero as usize] & 1 > 0
    }

    pub fn has_hero_variant(&self, variant: Variant) -> bool {
        if let Some(normal) = variant.to_normal() {
            self.heroes[normal as usize] & 1 << variant.to_i() > 0
        } else {
            false
        }
    }

    pub fn has_environment(&self, environment: Environment) -> bool {
        self.environments & 1 << environment as u64 > 0
    }

    pub fn set_villain(&mut self, villain: Villain) {
        self.villains |= 1 << villain as u64
    }

    pub fn set_team_villain(&mut self, team_villain: TeamVillain) {
        self.team_villains |= 1 << team_villain as u16
    }

    pub fn set_hero(&mut self, hero: Hero) {
        self.heroes[hero as usize] |= 1;
    }

    pub fn set_hero_variant(&mut self, variant: Variant) {
        if let Some(normal) = variant.to_normal() {
            self.heroes[normal as usize] |= 1 << variant.to_i()
        }
    }

    pub fn set_environment(&mut self, environment: Environment) {
        self.environments |= 1 << environment as u64
    }
}

impl Locations {
    pub fn new() -> Self {
        Locations {
            victory: false,
            villains: [0; Villain::variant_count()],
            team_villains: [0; TeamVillain::variant_count()],
            variants: 0,
            environments: 0,
        }
    }

    pub fn has_unchecked_villain(&self, villain: Villain, difficulty: u8) -> bool {
        self.villains[villain as usize] & 1 << difficulty == 0
    }

    pub fn has_unchecked_team_villain(&self, team_villain: TeamVillain, difficulty: u8) -> bool {
        self.team_villains[team_villain as usize] & 1 << difficulty == 0
    }

    pub fn has_unchecked_variant(&self, variant: Variant) -> bool {
        if variant as usize >= Variant::BaccaratAceOfSwords as usize {
            return false;
        }
        self.variants & 1 << variant as u128 == 0
    }

    pub fn has_unchecked_environment(&self, environment: Environment) -> bool {
        self.environments & 1 << environment as u64 == 0
    }

    pub fn mark_villain(&mut self, villain: Villain, difficulty: u8) {
        self.villains[villain as usize] |= 1 << difficulty
    }

    pub fn mark_team_villain(&mut self, team_villain: TeamVillain, difficulty: u8) {
        self.team_villains[team_villain as usize] |= 1 << difficulty
    }

    pub fn mark_variant(&mut self, variant: Variant) {
        if variant as usize >= Variant::BaccaratAceOfSwords as usize {
            return;
        }
        self.variants |= 1 << variant as u128
    }

    pub fn mark_environment(&mut self, environment: Environment) {
        self.environments |= 1 << environment as u64
    }
}
