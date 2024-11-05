use crate::data::{DamageType, DeconstructedFiller, Environment, Filler, FillerTarget, Hero, HeroLike, Item, Location, TeamVillain, Variant, Villain, VillainLike};
use archipelago_protocol::SlotData;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub items: Items,
    pub checked_locations: Locations,
    pub slot_data: SlotData,
}

#[derive(Debug, Clone, Copy)]
pub struct Items {
    pub scions: u32,
    pub villains: [u8; Villain::variant_count() / 8 + 1],
    pub team_villains: [u8; TeamVillain::variant_count() / 8 + 1],
    pub heroes: [u8; Hero::variant_count()],
    pub environments: [u8; Environment::variant_count() / 8 + 1],
    pub filler: FillerItems,
}

type FillerCounts = [i32; Filler::variant_count_no_type()];

#[derive(Debug, Clone, Copy)]
pub struct FillerItems {
    pub variant_filler: [[FillerCounts; 8]; Hero::variant_count()],
    pub villain_filler: [FillerCounts; Villain::variant_count()],
    pub team_villain_filler: [FillerCounts; TeamVillain::variant_count()],
    pub other_filler: FillerCounts,
    pub typed_filler: [FillerItemsType; 11],
}

type FillerCountsType = [i32; Filler::variant_count_type()];

#[derive(Debug, Clone, Copy)]
pub struct FillerItemsType {
    pub variant_filler: [[FillerCountsType; 8]; Hero::variant_count()],
    pub villain_filler: [FillerCountsType; Villain::variant_count()],
    pub team_villain_filler: [FillerCountsType; TeamVillain::variant_count()],
    pub other_filler: FillerCountsType,
}

#[derive(Debug, Clone, Copy)]
pub struct Locations {
    pub victory: bool,
    pub villains: [u8; Villain::variant_count()],
    pub team_villains: [u8; TeamVillain::variant_count()],
    pub variants: [u8; Variant::variant_count() / 8 + 1],
    pub environments: [u8; Environment::variant_count() / 8 + 1],
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
            slot_data,
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
            team_villains: if self.items.team_villains.iter().map(|b| (*b).count_ones()).sum::<u32>() < 3 {
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
                .filter(|v| v.can_unlock(&self.items))
                .collect(),
            environments: Environment::iter()
                .filter(|v| self.checked_locations.has_unchecked_environment(*v))
                .filter(|v| self.items.has_environment(*v))
                .collect(),
        }
    }

    pub fn victory_available(&self) -> bool {
        self.items.scions >= self.slot_data.required_scions
            && self.checked_locations.variants.iter().map(|b| b.count_ones()).sum::<u32>() >= self.slot_data.required_variants
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
            villains: [0; Villain::variant_count() / 8 + 1],
            team_villains: [0; TeamVillain::variant_count() / 8 + 1],
            heroes: [0; Hero::variant_count()],
            environments: [0; Environment::variant_count() / 8 + 1],
            filler: FillerItems::new(),
        }
    }

    pub fn has_villain(&self, villain: Villain) -> bool {
        self.villains[villain as usize >> 3] & 1 << (villain as u8 & 0x7) > 0
    }

    pub fn has_team_villain(&self, team_villain: TeamVillain) -> bool {
        self.team_villains[team_villain as usize >> 3] & 1 << (team_villain as u8 & 0x7) > 0
    }

    pub fn team_villain_count(&self) -> bool {
        self.team_villains.iter().map(|b| b.count_ones()).sum::<u32>() >= 3
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

    pub fn variants_of(&self, hero: Hero) -> impl Iterator<Item = Variant> {
        let bitfield = self.heroes[hero as usize];
        (1..8).filter(move |i| bitfield & 1 << i > 0).filter_map(move |i| Variant::from_hero(hero, i as u32))
    }

    pub fn has_environment(&self, environment: Environment) -> bool {
        self.environments[environment as usize >> 3] & 1 << (environment as u8 & 0x7) > 0
    }

    pub fn set_villain(&mut self, villain: Villain) {
        self.villains[villain as usize >> 3] |= 1 << (villain as u8 & 0x7);
    }

    pub fn set_team_villain(&mut self, team_villain: TeamVillain) {
        self.team_villains[team_villain as usize >> 3] |= 1 << (team_villain as u8 & 0x7);
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
        self.environments[environment as usize >> 3] |= 1 << (environment as u8 & 0x7);
    }

    pub fn set_item(&mut self, item: Item) {
        match item {
            Item::Hero(v) => self.set_hero(v),
            Item::Variant(v) => self.set_hero_variant(v),
            Item::Villain(v) => self.set_villain(v),
            Item::TeamVillain(v) => self.set_team_villain(v),
            Item::Environment(v) => self.set_environment(v),
            Item::Scion => self.scions += 1,
            Item::Filler((filler, count)) => {
                self.filler.add(filler, count);
            }
        }
    }

    pub fn get_filler_for(&self, target: FillerTarget) -> Vec<(Filler, i32)> {
        let mut out: Vec<(Filler, i32)> = vec![];

        for filler in target.relevant_filler() {
            let deconstructed = filler.deconstruct();
            match deconstructed.damage_type {
                Some(DamageType::All) => {
                    let counts: Vec<_> = DamageType::iter()
                        .skip(1)
                        .filter_map(|damage_type| self.filler.typed_filler[damage_type as usize - 1].get_count(deconstructed).map(|count| (damage_type, count)))
                        .collect();

                    if counts.iter().skip(1).all(|(_, count)| counts[0].1 == *count) {
                        if counts[0].1 != 0 {
                            out.push((filler, counts[0].1));
                        }
                    } else {
                        for (damage_type, count) in counts {
                            if count != 0 {
                                out.push((filler.with_type(damage_type), count));
                            }
                        }
                    }
                }
                Some(damage_type) => {
                    if let Some(count) = self.filler.typed_filler[damage_type as usize - 1].get_count(deconstructed) {
                        if count != 0 {
                            out.push((filler, count));
                        }
                    }
                }
                None => {
                    if let Some(count) = self.filler.get_count(deconstructed) {
                        if count != 0 {
                            out.push((filler, count));
                        }
                    }
                }
            }
        }

        out
    }
}

impl Default for Items {
    fn default() -> Self {
        Self::new()
    }
}

impl FillerItems {
    pub fn new() -> FillerItems {
        FillerItems {
            variant_filler: [[[0; Filler::variant_count_no_type()]; 8]; Hero::variant_count()],
            villain_filler: [[0; Filler::variant_count_no_type()]; Villain::variant_count()],
            team_villain_filler: [[0; Filler::variant_count_no_type()]; TeamVillain::variant_count()],
            other_filler: [0; Filler::variant_count_no_type()],
            typed_filler: [FillerItemsType::new(); 11],
        }
    }

    pub fn add(&mut self, filler: Filler, count: i32) {
        let deconstructed = filler.deconstruct();
        let idx = deconstructed.r#type.as_idx();

        match deconstructed.damage_type {
            Some(DamageType::All) => {
                for filler in self.typed_filler.each_mut() {
                    filler.add(deconstructed, count);
                }
            }
            Some(damage_type) => {
                self.typed_filler[damage_type as usize - 1].add(deconstructed, count);
            }
            None => match deconstructed.target {
                FillerTarget::Hero(HeroLike::All) => {
                    for ff in self.variant_filler.each_mut() {
                        for f in ff.each_mut() {
                            f[idx] += count;
                        }
                    }
                }
                FillerTarget::Hero(HeroLike::Hero(hero)) => {
                    for f in self.variant_filler[hero as usize].each_mut() {
                        f[idx] += count;
                    }
                }
                FillerTarget::Hero(HeroLike::Variant(variant)) => {
                    self.variant_filler[variant.as_normal().expect("Variant for filler target must be a hero variant") as usize][variant.as_i() as usize][idx] += count
                }
                FillerTarget::Villain(VillainLike::All) => {
                    for f in self.villain_filler.each_mut() {
                        f[idx] += count;
                    }

                    for f in self.team_villain_filler.each_mut() {
                        f[idx] += count;
                    }
                }
                FillerTarget::Villain(VillainLike::Villain(villain)) => self.villain_filler[villain as usize][idx] += count,
                FillerTarget::Villain(VillainLike::TeamVillain(villain)) => self.team_villain_filler[villain as usize][idx] += count,
                FillerTarget::Other => self.other_filler[idx] += count,
            },
        }
    }

    fn get_count(&self, deconstructed: DeconstructedFiller) -> Option<i32> {
        let idx = deconstructed.r#type.as_idx();

        match deconstructed.target {
            FillerTarget::Hero(HeroLike::Hero(hero)) => Some(self.variant_filler[hero as usize][0][idx]),
            FillerTarget::Hero(HeroLike::Variant(variant)) => {
                Some(self.variant_filler[variant.as_normal().expect("Variant for filler target must be a hero variant") as usize][variant.as_i() as usize][idx])
            }
            FillerTarget::Villain(VillainLike::Villain(villain)) => Some(self.villain_filler[villain as usize][idx]),
            FillerTarget::Villain(VillainLike::TeamVillain(villain)) => Some(self.team_villain_filler[villain as usize][idx]),
            FillerTarget::Other => Some(self.other_filler[idx]),
            _ => None,
        }
    }
}

impl Default for FillerItemsType {
    fn default() -> Self {
        Self::new()
    }
}

impl FillerItemsType {
    pub fn new() -> FillerItemsType {
        FillerItemsType {
            variant_filler: [[[0; Filler::variant_count_type()]; 8]; Hero::variant_count()],
            villain_filler: [[0; Filler::variant_count_type()]; Villain::variant_count()],
            team_villain_filler: [[0; Filler::variant_count_type()]; TeamVillain::variant_count()],
            other_filler: [0; Filler::variant_count_type()],
        }
    }

    pub fn add(&mut self, deconstructed: DeconstructedFiller, count: i32) {
        let idx = deconstructed.r#type.as_idx();

        match deconstructed.target {
            FillerTarget::Hero(HeroLike::All) => {
                for ff in self.variant_filler.each_mut() {
                    for f in ff.each_mut() {
                        f[idx] += count;
                    }
                }
            }
            FillerTarget::Hero(HeroLike::Hero(hero)) => {
                for f in self.variant_filler[hero as usize].each_mut() {
                    f[idx] += count;
                }
            }
            FillerTarget::Hero(HeroLike::Variant(variant)) => {
                self.variant_filler[variant.as_normal().expect("Variant for filler target must be a hero variant") as usize][variant.as_i() as usize][idx] += count
            }
            FillerTarget::Villain(VillainLike::All) => {
                for f in self.villain_filler.each_mut() {
                    f[idx] += count;
                }

                for f in self.team_villain_filler.each_mut() {
                    f[idx] += count;
                }
            }
            FillerTarget::Villain(VillainLike::Villain(villain)) => self.villain_filler[villain as usize][idx] += count,
            FillerTarget::Villain(VillainLike::TeamVillain(villain)) => self.team_villain_filler[villain as usize][idx] += count,
            FillerTarget::Other => self.other_filler[idx] += count,
        }
    }

    fn get_count(&self, deconstructed: DeconstructedFiller) -> Option<i32> {
        let idx = deconstructed.r#type.as_idx();

        match deconstructed.target {
            FillerTarget::Hero(HeroLike::Hero(hero)) => Some(self.variant_filler[hero as usize][0][idx]),
            FillerTarget::Hero(HeroLike::Variant(variant)) => {
                Some(self.variant_filler[variant.as_normal().expect("Variant for filler target must be a hero variant") as usize][variant.as_i() as usize][idx])
            }
            FillerTarget::Villain(VillainLike::Villain(villain)) => Some(self.villain_filler[villain as usize][idx]),
            FillerTarget::Villain(VillainLike::TeamVillain(villain)) => Some(self.team_villain_filler[villain as usize][idx]),
            FillerTarget::Other => Some(self.other_filler[idx]),
            _ => None,
        }
    }
}

impl Default for FillerItems {
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
            variants: [0; Variant::variant_count() / 8 + 1],
            environments: [0; Environment::variant_count() / 8 + 1],
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
            self.variants[variant as usize >> 3] & 1 << (variant as u8 & 0x7) == 0
        }
    }

    pub fn has_unchecked_environment(&self, environment: Environment) -> bool {
        self.environments[environment as usize >> 3] & 1 << (environment as u8 & 0x7) == 0
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
        self.variants[variant as usize >> 3] |= 1 << (variant as u8 & 0x7);
    }

    pub fn mark_environment(&mut self, environment: Environment) {
        self.environments[environment as usize >> 3] |= 1 << (environment as u8 & 0x7);
    }

    pub fn mark_location(&mut self, location: Location) {
        match location {
            Location::Variant(v) => self.mark_variant(v),
            Location::Villain((v, d)) => self.mark_villain(v, d),
            Location::TeamVillain((v, d)) => self.mark_team_villain(v, d),
            Location::Environment(e) => self.mark_environment(e),
            Location::Victory => (),
        }
    }
}

impl Default for Locations {
    fn default() -> Self {
        Self::new()
    }
}
