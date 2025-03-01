use crate::{
    data::{Filler, FillerTarget, FillerType, HeroLike, VillainLike},
    game::CurrentGame,
};

#[derive(Debug, Clone, Copy)]
pub struct FillerItem {
    pub filler: Filler,
    pub r#type: FillerType,
    pub target: FillerTarget,
    pub duration: i32,
}

impl FillerItem {
    pub fn new(filler: Filler, duration: i32) -> Self {
        let deconstructed = filler.deconstruct();

        Self {
            filler,
            r#type: deconstructed.r#type,
            target: deconstructed.target,
            duration,
        }
    }

    pub fn is_relevant(&self, game: &CurrentGame) -> bool {
        match self.target {
            FillerTarget::Hero(HeroLike::All) => true,
            FillerTarget::Hero(HeroLike::Hero(hero)) => game.has_hero_all_variants(hero),
            FillerTarget::Hero(HeroLike::Base(base)) => game.has_hero(base),
            FillerTarget::Hero(HeroLike::Variant(variant)) => game.has_variant(variant),
            FillerTarget::Villain(VillainLike::All) => true,
            FillerTarget::Villain(VillainLike::Villain(villain)) => game.is_classic(villain),
            FillerTarget::Villain(VillainLike::TeamVillain(team_villain)) => game.has_team(team_villain),
            FillerTarget::Other => true,
        }
    }

    pub fn as_str(&self) -> &str {
        self.filler.as_str(self.duration)
    }

    pub fn as_desc(&self) -> &str {
        self.filler.as_desc(self.duration)
    }
}
