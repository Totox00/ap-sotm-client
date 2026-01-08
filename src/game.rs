use web_sys::Element;

use crate::{
    data::{Environment, Hero, TeamVillain, Variant, Villain},
    interface::{CurrentVillains, SelectedHero},
};

pub struct CurrentGame {
    pub heroes: Vec<(SelectedHero, Element)>,
    pub villains: CurrentVillains,
    pub environment: Option<Environment>,
}

impl CurrentGame {
    pub fn new() -> CurrentGame {
        CurrentGame {
            heroes: vec![],
            villains: CurrentVillains::None,
            environment: None,
        }
    }

    pub fn is_classic(&self, villain: Villain) -> bool {
        match &self.villains {
            CurrentVillains::Classic((v, _, _)) => *v == villain,
            _ => false,
        }
    }

    pub fn is_classic_any_variant(&self, base: Villain) -> bool {
        match &self.villains {
            CurrentVillains::Classic((v, _, _)) => base.variants().contains(v),
            _ => false,
        }
    }

    pub fn has_team(&self, villain: TeamVillain) -> bool {
        match &self.villains {
            CurrentVillains::Team(villains) => villains.iter().any(|(v, _, _)| *v == villain),
            _ => false,
        }
    }

    pub fn current_bases(&self) -> impl Iterator<Item = Hero> + use<'_> {
        self.heroes.iter().filter_map(|(hero, _)| match hero {
            SelectedHero::Hero(hero) => Some(*hero),
            SelectedHero::Variant(variant) => variant.as_normal(),
            SelectedHero::Contenders(_) => None,
        })
    }

    pub fn has_hero(&self, hero: Hero) -> bool {
        self.heroes.iter().any(|(h, _)| h.is_hero(hero))
    }

    pub fn has_variant(&self, variant: Variant) -> bool {
        self.heroes.iter().any(|(v, _)| v.is_variant(variant))
    }

    pub fn has_hero_all_variants(&self, hero: Hero) -> bool {
        self.heroes.iter().any(|(h, _)| match h {
            SelectedHero::Hero(h) => *h == hero,
            SelectedHero::Variant(v) => v.as_normal().is_some_and(|base| base == hero),
            _ => false,
        })
    }

    pub fn has_heroes(&self, heroes: &[Hero]) -> bool {
        self.heroes
            .iter()
            .filter(|(hero, _)| match hero {
                SelectedHero::Hero(hero) => heroes.contains(hero),
                _ => false,
            })
            .count()
            == heroes.len()
    }

    pub fn has_variants(&self, variants: &[Variant]) -> bool {
        self.heroes
            .iter()
            .filter(|(hero, _)| match hero {
                SelectedHero::Variant(variant) => variants.contains(variant),
                _ => false,
            })
            .count()
            == variants.len()
    }

    pub fn has_all_any_variants(&self, bases: &[Hero]) -> bool {
        self.current_bases().filter(|base| bases.contains(base)).count() == bases.len()
    }

    pub fn first_hero(&self, hero: Hero) -> bool {
        self.heroes.first().is_some_and(|(h, _)| match h {
            SelectedHero::Hero(h) => *h == hero,
            _ => false,
        })
    }

    pub fn last_hero(&self, hero: Hero) -> bool {
        self.heroes.last().is_some_and(|(h, _)| match h {
            SelectedHero::Hero(h) => *h == hero,
            _ => false,
        })
    }

    pub fn first_variant(&self, variant: Variant) -> bool {
        self.heroes.first().is_some_and(|(hero, _)| match hero {
            SelectedHero::Variant(v) => *v == variant,
            _ => false,
        })
    }

    pub fn first_hero_any_variant(&self, hero: Hero) -> bool {
        self.heroes.first().is_some_and(|(h, _)| match h {
            SelectedHero::Hero(h) => hero == *h,
            SelectedHero::Variant(variant) => variant.as_normal().is_some_and(|base| base == hero),
            _ => false,
        })
    }

    pub fn environment(&self, environment: Environment) -> bool {
        self.environment.is_some_and(|e| e == environment)
    }

    pub fn any_ambuscade(&self) -> bool {
        self.is_classic_any_variant(Villain::Ambuscade) || self.has_team(TeamVillain::TeamAmbuscade)
    }

    pub fn any_baron_blade(&self) -> bool {
        self.is_classic_any_variant(Villain::BaronBlade) || self.has_team(TeamVillain::TeamBaronBlade)
    }

    pub fn freedom_five(&self, progress: u8) -> bool {
        self.is_classic_any_variant(Villain::Progeny)
            && match progress {
                0 => {
                    !self.environment(Environment::RookCity)
                        && !self.environment(Environment::Megalopolis)
                        && self.has_variants(&[
                            Variant::PrimeWardensArgentAdept,
                            Variant::PrimeWardensCaptainCosmic,
                            Variant::PrimeWardensFanatic,
                            Variant::PrimeWardensHaka,
                            Variant::PrimeWardensTempest,
                        ])
                }
                1 => {
                    self.environment(Environment::RookCity)
                        && self.has_variants(&[
                            Variant::DarkWatchExpatriette,
                            Variant::DarkWatchMisterFixer,
                            Variant::DarkWatchNightmist,
                            Variant::DarkWatchSetback,
                            Variant::DarkWatchHarpy,
                        ])
                }
                2 => self.environment(Environment::RookCity) && self.has_heroes(&[Hero::AbsoluteZero, Hero::Bunker, Hero::Wraith, Hero::Tachyon, Hero::Legacy]),
                3 => self.environment(Environment::Megalopolis) && self.has_heroes(&[Hero::AbsoluteZero, Hero::Bunker, Hero::Wraith, Hero::Tachyon, Hero::Legacy]),
                _ => false,
            }
    }
}

impl SelectedHero {
    pub fn is_hero(&self, hero: Hero) -> bool {
        match self {
            SelectedHero::Hero(h) => *h == hero,
            _ => false,
        }
    }

    pub fn is_variant(&self, variant: Variant) -> bool {
        match self {
            SelectedHero::Variant(v) => *v == variant,
            _ => false,
        }
    }
}
