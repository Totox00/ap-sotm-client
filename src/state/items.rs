use crate::data::{Contender, Environment, Filler, Gladiator, Hero, Item, TeamVillain, Variant, Villain};

use super::filler::FillerItem;

#[derive(Debug, Clone)]
pub struct Items {
    pub scions: u32,
    pub villains: [u8; Villain::variant_count() / 8 + 1],
    pub team_villains: [u8; TeamVillain::variant_count() / 8 + 1],
    pub gladiators: [u8; Gladiator::variant_count() / 8 + 1],
    pub heroes: [u8; Hero::variant_count()],
    pub contenders: [u8; Contender::variant_count() / 8 + 1],
    pub environments: [u8; Environment::variant_count() / 8 + 1],
    pub filler: Vec<FillerItem>,
}

impl Items {
    pub fn new() -> Self {
        Items {
            scions: 0,
            villains: [0; Villain::variant_count() / 8 + 1],
            team_villains: [0; TeamVillain::variant_count() / 8 + 1],
            gladiators: [0; Gladiator::variant_count() / 8 + 1],
            heroes: [0; Hero::variant_count()],
            contenders: [0; Contender::variant_count() / 8 + 1],
            environments: [0; Environment::variant_count() / 8 + 1],
            filler: vec![],
        }
    }

    pub fn has_villain(&self, villain: Villain) -> bool {
        self.villains[villain as usize >> 3] & 1 << (villain as u8 & 0x7) > 0
    }

    pub fn has_team_villain(&self, team_villain: TeamVillain) -> bool {
        self.team_villains[team_villain as usize >> 3] & 1 << (team_villain as u8 & 0x7) > 0
    }

    pub fn has_gladiator(&self, gladiator: Gladiator) -> bool {
        self.gladiators[gladiator as usize >> 3] & 1 << (gladiator as u8 & 0x7) > 0
    }

    pub fn team_villain_count(&self) -> bool {
        self.team_villains.iter().map(|b| b.count_ones()).sum::<u32>() >= 3
    }

    pub fn gladiator_count(&self) -> bool {
        self.gladiators.iter().map(|b| b.count_ones()).sum::<u32>() >= 3
    }

    pub fn has_hero(&self, hero: Hero) -> bool {
        self.heroes[hero as usize] > 0
    }

    pub fn has_contender(&self, contender: Contender) -> bool {
        self.contenders[contender as usize >> 3] & 1 << (contender as u8 & 0x7) > 0
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

    pub fn set_gladiator(&mut self, gladiator: Gladiator) {
        self.gladiators[gladiator as usize >> 3] |= 1 << (gladiator as u8 & 0x7);
    }

    pub fn set_hero(&mut self, hero: Hero) {
        self.heroes[hero as usize] |= 1;
    }

    pub fn set_contender(&mut self, contender: Contender) {
        self.contenders[contender as usize >> 3] |= 1 << (contender as u8 & 0x7);
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
            Item::Contender(v) => self.set_contender(v),
            Item::Variant(v) => self.set_hero_variant(v),
            Item::Villain(v) => self.set_villain(v),
            Item::TeamVillain(v) => self.set_team_villain(v),
            Item::Gladiator(v) => self.set_gladiator(v),
            Item::Environment(v) => self.set_environment(v),
            Item::Scion => self.scions += 1,
            Item::Filler((filler, duration)) => self.add_filler(filler, duration),
        }
    }

    pub fn add_filler(&mut self, filler: Filler, duration: i32) {
        for existing in self.filler.iter_mut() {
            if existing.filler == filler {
                existing.duration += duration;
                return;
            }
        }

        self.filler.push(FillerItem::new(filler, duration));
    }
}

impl Default for Items {
    fn default() -> Self {
        Self::new()
    }
}
