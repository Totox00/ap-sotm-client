pub mod filler;
pub mod items;
pub mod locations;

use std::collections::HashMap;

use crate::data::{Filler, Variant};
use crate::game::CurrentGame;
use crate::protocol::SlotData;
use crate::variant::state::{PersistentVariantProgress, TemporaryVariantProgress};
use items::Items;
use locations::Locations;
use strum::IntoEnumIterator;

#[derive(Debug, Clone)]
pub struct State {
    pub items: Items,
    pub checked_locations: Locations,
    pub slot_data: SlotData,
    pub persistent_variant_progress: PersistentVariantProgress,
    pub temporary_variant_progress: TemporaryVariantProgress,
    pub expended_filler: HashMap<Filler, i32>,
}

pub struct GoalProgress {
    pub scions: u32,
    pub required_scions: u32,
    pub villains: u32,
    pub required_villains: u32,
    pub variants: u32,
    pub required_variants: u32,
}

impl State {
    pub fn new(slot_data: SlotData) -> Self {
        State {
            items: Items::new(),
            checked_locations: Locations::new(),
            slot_data,
            persistent_variant_progress: PersistentVariantProgress::default(),
            temporary_variant_progress: TemporaryVariantProgress::default(),
            expended_filler: HashMap::new(),
        }
    }

    pub fn available_variants(&self) -> impl Iterator<Item = Variant> + use<'_> {
        Variant::iter()
            .filter(|v| self.checked_locations.has_unchecked_variant_unlock(*v))
            .filter(|v| v.can_unlock(&self.items))
    }

    pub fn goal_progress(&self) -> GoalProgress {
        GoalProgress {
            scions: self.items.scions,
            required_scions: self.slot_data.required_scions as u32,
            villains: (self
                .checked_locations
                .villains
                .iter()
                .map(|bitfield| (0..4).map(move |b| ((*bitfield >> b) & 1) as i32 * self.slot_data.villain_difficulty_points[b]).sum::<i32>())
                .sum::<i32>()
                + self
                    .checked_locations
                    .team_villains
                    .iter()
                    .map(|bitfield| (0..4).map(move |b| ((*bitfield >> b) & 1) as i32 * self.slot_data.villain_difficulty_points[b]).sum::<i32>())
                    .sum::<i32>()
                + self
                    .checked_locations
                    .gladiators
                    .iter()
                    .map(|bitfield| (0..4).map(move |b| ((*bitfield >> b) & 1) as i32 * self.slot_data.villain_difficulty_points[b]).sum::<i32>())
                    .sum::<i32>()) as u32,
            required_villains: self.slot_data.required_villains as u32,
            variants: self.checked_locations.variant_unlocks.iter().map(|b| b.count_ones()).sum::<u32>(),
            required_variants: self.slot_data.required_variants as u32,
        }
    }

    pub fn expend_filler(&mut self, game: &CurrentGame) {
        for filler in &self.items.filler {
            let duration = filler.duration + self.expended_filler.get(&filler.filler).copied().unwrap_or(0);

            if duration != 0 && filler.is_relevant(game) {
                if duration > 0 {
                    if let Some(expended) = self.expended_filler.get_mut(&filler.filler) {
                        *expended -= 1;
                    } else {
                        self.expended_filler.insert(filler.filler, -1);
                    }
                } else if let Some(expended) = self.expended_filler.get_mut(&filler.filler) {
                    *expended += 1;
                } else {
                    self.expended_filler.insert(filler.filler, 1);
                }
            }
        }
    }

    pub fn update_expended_filler(&mut self, expended: &[(Filler, i32)]) {
        for (filler, duration) in expended {
            self.expended_filler.insert(*filler, *duration);
        }
    }
}

impl GoalProgress {
    pub fn available(&self) -> bool {
        self.scions >= self.required_scions && self.villains >= self.required_villains && self.variants >= self.required_variants
    }
}
