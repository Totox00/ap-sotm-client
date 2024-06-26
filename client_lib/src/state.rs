use strum::IntoEnumIterator;

use crate::{
    data::{Environment, Hero, Item, TeamVillain, Variant, Villain},
    logic::can_unlock,
};
use archipelago_rs::protocol::SlotData;

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub items: Items,
    pub checked_locations: Locations,
    pub slot_data: CleanedSlotData,
}

#[derive(Debug, Clone, Copy)]
pub struct CleanedSlotData {
    pub required_scions: u32,
    pub required_villains: u32,
    pub required_variants: u32,
    pub villain_difficulty_points: [u32; 4],
    pub locations_per: [u8; 6],
}

#[derive(Debug, Clone, Copy)]
pub struct Items {
    pub scions: u32,
    pub villains: u64,
    pub team_villains: u16,
    pub heroes: [u8; Hero::variant_count()],
    pub environments: u64,
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
    pub fn new(slot_data: SlotData) -> Self {
        State {
            items: Items::new(),
            checked_locations: Locations::new(),
            slot_data: slot_data.into(),
        }
    }

    pub fn available_locations(&self) -> AvailableLocations {
        AvailableLocations {
            victory: !self.checked_locations.victory && self.victory_available(),
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
                .map(|(v, d)| if v == Villain::SkinwalkerGloomweaver { (v, d & 0b11) } else { (v, d) })
                .filter(|(_, d)| *d > 0)
                .map(|(v, d)| {
                    if v != Villain::SpiteAgentOfGloom || self.items.has_villain(Villain::SkinwalkerGloomweaver) {
                        (v, d)
                    } else {
                        (v, d & 0b11)
                    }
                })
                .flat_map(|(v, b)| [0, 1, 2, 3].iter().filter(move |d| b & 1 << *d > 0).map(move |d| (v, *d)))
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
                    .flat_map(|(v, b)| [0, 1, 2, 3].iter().filter(move |d| b & 1 << *d > 0).map(move |d| (v, *d)))
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

    pub fn victory_available(&self) -> bool {
        self.items.scions >= self.slot_data.required_scions
            && self.checked_locations.variants.count_ones() >= self.slot_data.required_variants
            && self
                .checked_locations
                .villains
                .iter()
                .map(|bitfield| (0..4).map(move |b| (*bitfield & (1 << b)) as u32 * self.slot_data.villain_difficulty_points[b]).sum::<u32>())
                .sum::<u32>()
                >= self.slot_data.required_villains
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
        if let Some(normal) = variant.as_normal() {
            self.heroes[normal as usize] & 1 << variant.as_i() > 0
        } else {
            false
        }
    }

    pub fn has_environment(&self, environment: Environment) -> bool {
        self.environments & 1 << environment as u64 > 0
    }

    pub fn set_villain(&mut self, villain: Villain) {
        self.villains |= 1 << villain as u64;
    }

    pub fn set_team_villain(&mut self, team_villain: TeamVillain) {
        self.team_villains |= 1 << team_villain as u16;
    }

    pub fn set_hero(&mut self, hero: Hero) {
        self.heroes[hero as usize] |= 1;
    }

    pub fn set_hero_variant(&mut self, variant: Variant) {
        if let Some(normal) = variant.as_normal() {
            self.heroes[normal as usize] |= 1 << variant.as_i();
        }
    }

    pub fn set_environment(&mut self, environment: Environment) {
        self.environments |= 1 << environment as u64;
    }

    pub fn set_item(&mut self, item: Item) {
        match item {
            Item::Hero(v) => self.set_hero(v),
            Item::Variant(v) => self.set_hero_variant(v),
            Item::Villain(v) => self.set_villain(v),
            Item::TeamVillain(v) => self.set_team_villain(v),
            Item::Environment(v) => self.set_environment(v),
            Item::Scion => self.scions += 1,
            Item::Filler(_filler) => (),
        }
    }
}

impl Default for Items {
    fn default() -> Self {
        Self::new()
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
            false
        } else {
            self.variants & 1 << variant as u128 == 0
        }
    }

    pub fn has_unchecked_environment(&self, environment: Environment) -> bool {
        self.environments & 1 << environment as u64 == 0
    }

    pub fn mark_villain(&mut self, villain: Villain, difficulty: u8) {
        self.villains[villain as usize] |= 1 << difficulty;
    }

    pub fn mark_team_villain(&mut self, team_villain: TeamVillain, difficulty: u8) {
        self.team_villains[team_villain as usize] |= 1 << difficulty;
    }

    pub fn mark_variant(&mut self, variant: Variant) {
        if variant as usize >= Variant::BaccaratAceOfSwords as usize {
            return;
        }
        self.variants |= 1 << variant as u128;
    }

    pub fn mark_environment(&mut self, environment: Environment) {
        self.environments |= 1 << environment as u64;
    }
}

impl Default for Locations {
    fn default() -> Self {
        Self::new()
    }
}

impl From<SlotData> for CleanedSlotData {
    fn from(value: SlotData) -> Self {
        Self {
            required_scions: if value.required_scions < 0 { 0 } else { value.required_scions as u32 },
            required_villains: if value.required_villains < 0 { 0 } else { value.required_villains as u32 },
            required_variants: if value.required_variants < 0 { 0 } else { value.required_variants as u32 },
            villain_difficulty_points: value.villain_difficulty_points.map(|e| if e < 0 { 0 } else { e as u32 }),
            locations_per: value.locations_per.map(|e| if e < 0 { 0 } else { e as u8 }),
        }
    }
}
