use crate::data::{Environment, Gladiator, Hero, Location, TeamVillain, Variant, Villain};

#[derive(Debug, Clone, Copy)]
pub struct Locations {
    pub victory: bool,
    pub villains: [u8; Villain::variant_count()],
    pub team_villains: [u8; TeamVillain::variant_count()],
    pub gladiators: [u8; Gladiator::variant_count()],
    pub heroes: [u8; Hero::variant_count()],
    pub variant_unlocks: [u8; Variant::variant_count() / 8 + 1],
    pub environments: [u8; Environment::variant_count() / 8 + 1],
}

impl Locations {
    pub fn new() -> Self {
        Locations {
            victory: false,
            villains: [0; Villain::variant_count()],
            team_villains: [0; TeamVillain::variant_count()],
            gladiators: [0; Gladiator::variant_count()],
            heroes: [0; Hero::variant_count()],
            variant_unlocks: [0; Variant::variant_count() / 8 + 1],
            environments: [0; Environment::variant_count() / 8 + 1],
        }
    }

    pub fn has_unchecked_location(&self, location: Location) -> bool {
        match location {
            Location::Hero(hero) => self.has_unchecked_hero(hero),
            Location::Variant(variant) => self.has_unchecked_variant(variant),
            Location::VariantUnlock(variant) => self.has_unchecked_variant_unlock(variant),
            Location::Villain((villain, diff)) => self.has_unchecked_villain(villain, diff),
            Location::TeamVillain((villain, diff)) => self.has_unchecked_team_villain(villain, diff),
            Location::Gladiator((gladiator, diff)) => self.has_unchecked_gladiator(gladiator, diff),
            Location::Environment(environment) => self.has_unchecked_environment(environment),
            Location::Victory => !self.victory
        }
    }

    pub fn has_unchecked_villain(&self, villain: Villain, difficulty: u8) -> bool {
        self.villains[villain as usize] & 1 << difficulty == 0
    }

    pub fn has_unchecked_team_villain(&self, team_villain: TeamVillain, difficulty: u8) -> bool {
        self.team_villains[team_villain as usize] & 1 << difficulty == 0
    }

    pub fn has_unchecked_gladiator(&self, gladiator: Gladiator, difficulty: u8) -> bool {
        self.gladiators[gladiator as usize] & 1 << difficulty == 0
    }

    pub fn has_unchecked_hero(&self, hero: Hero) -> bool {
        self.heroes[hero as usize] & 1 == 0
    }

    pub fn has_unchecked_variant(&self, variant: Variant) -> bool {
        if let Some(base) = variant.as_normal() {
            self.heroes[base as usize] >> variant.as_i() & 1 == 0
        } else {
            false
        }
    }

    pub fn has_unchecked_variant_unlock(&self, variant: Variant) -> bool {
        self.variant_unlocks[variant as usize >> 3] & 1 << (variant as u8 & 0x7) == 0
    }

    pub fn has_unchecked_environment(&self, environment: Environment) -> bool {
        self.environments[environment as usize >> 3] & 1 << (environment as u8 & 0x7) == 0
    }

    pub fn mark_location(&mut self, location: Location) {
        match location {
            Location::Hero(hero) => self.mark_hero(hero),
            Location::Variant(variant) => self.mark_variant(variant),
            Location::VariantUnlock(variant) => self.mark_variant_unlock(variant),
            Location::Villain((v, d)) => self.mark_villain(v, d),
            Location::TeamVillain((v, d)) => self.mark_team_villain(v, d),
            Location::Gladiator((v, d)) => self.mark_gladiator(v, d),
            Location::Environment(e) => self.mark_environment(e),
            Location::Victory => (),
        }
    }

    pub fn mark_villain(&mut self, villain: Villain, difficulty: u8) {
        self.villains[villain as usize] |= 1 << difficulty;
    }

    pub fn mark_team_villain(&mut self, team_villain: TeamVillain, difficulty: u8) {
        self.team_villains[team_villain as usize] |= 1 << difficulty;
    }

    pub fn mark_gladiator(&mut self, gladiator: Gladiator, difficulty: u8) {
        self.gladiators[gladiator as usize] |= 1 << difficulty;
    }

    pub fn mark_hero(&mut self, hero: Hero) {
        self.heroes[hero as usize] |= 1;
    }

    pub fn mark_variant(&mut self, variant: Variant) {
        if let Some(base) = variant.as_normal() {
            self.heroes[base as usize] |= 1 << variant.as_i();
        }
    }

    pub fn mark_variant_unlock(&mut self, variant: Variant) {
        self.variant_unlocks[variant as usize >> 3] |= 1 << (variant as u8 & 0x7);
    }

    pub fn mark_environment(&mut self, environment: Environment) {
        self.environments[environment as usize >> 3] |= 1 << (environment as u8 & 0x7);
    }

    pub fn update(&mut self, other: &Locations) {
        self.victory |= other.victory;
        for (current, other) in self.villains.iter_mut().zip(other.villains.into_iter()) {
            *current |= other;
        }
        for (current, other) in self.team_villains.iter_mut().zip(other.team_villains.into_iter()) {
            *current |= other;
        }
        for (current, other) in self.gladiators.iter_mut().zip(other.gladiators.into_iter()) {
            *current |= other;
        }
        for (current, other) in self.heroes.iter_mut().zip(other.heroes.into_iter()) {
            *current |= other;
        }
        for (current, other) in self.variant_unlocks.iter_mut().zip(other.variant_unlocks.into_iter()) {
            *current |= other;
        }
        for (current, other) in self.environments.iter_mut().zip(other.environments.into_iter()) {
            *current |= other;
        }
    }
}

impl Default for Locations {
    fn default() -> Self {
        Self::new()
    }
}
