#[derive(Debug, Clone, Copy)]
pub struct ReceivedFiller {
    handsize: i32,
    hero_hp: i32,
    mulligan: i32,
    hero_damage_taken: [i32; 12],
    hero_card_draw: i32,
    hero_buffer: i32,
    villain_hp: i32,
    villain_target_hp: i32,
    hero_damage_dealt: [i32; 12],
    hero_cannot_play: i32,
    hero_cannot_power: i32,
    hero_cannot_draw: i32,
    hero_cannot_damage: [i32; 12],
}

impl ReceivedFiller {
    pub fn new() -> Self {
        ReceivedFiller {
            handsize: 0,
            hero_hp: 0,
            mulligan: 0,
            hero_damage_taken: [0; 12],
            hero_card_draw: 0,
            hero_buffer: 0,
            villain_hp: 0,
            villain_target_hp: 0,
            hero_damage_dealt: [0; 12],
            hero_cannot_play: 0,
            hero_cannot_power: 0,
            hero_cannot_draw: 0,
            hero_cannot_damage: [0; 12],
        }
    }

    pub fn add(&mut self, filler: Filler) {}
}

impl Default for ReceivedFiller {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Filler {
    Handsize,
    HeroHp,
    Mulligan,
    HeroDamageTaken(DamageType),
    HeroCardDraw,
    HeroBuffer,
    VillainHp,
    VillainTargetHp,
    HeroDamageDealt(DamageType),
    HeroCannotPlay,
    HeroCannotPower,
    HeroCannotDraw,
    HeroCannotDamage(DamageType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DamageType {
    All,
    Cold,
    Energy,
    Fire,
    Infernal,
    Lightning,
    Melee,
    Projectile,
    Psychic,
    Radiant,
    Sonic,
    Toxic,
}
