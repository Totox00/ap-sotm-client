use crate::data::{DamageType, DeconstructedFiller, Filler, FillerTarget, Hero, HeroLike, TeamVillain, Villain, VillainLike};

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

    pub fn get_count(&self, deconstructed: DeconstructedFiller) -> Option<i32> {
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

    pub fn get_count(&self, deconstructed: DeconstructedFiller) -> Option<i32> {
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
