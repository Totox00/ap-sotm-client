pub mod filler;
pub mod items;
pub mod locations;

use crate::data::Variant;
use crate::protocol::SlotData;
use crate::variant::state::{PersistentVariantProgress, TemporaryVariantProgress};
use items::Items;
use locations::Locations;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub items: Items,
    pub checked_locations: Locations,
    pub slot_data: SlotData,
    pub persistent_variant_progress: PersistentVariantProgress,
    pub temporary_variant_progress: TemporaryVariantProgress,
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
        }
    }

    pub fn available_variants(&self) -> impl Iterator<Item = Variant> + use<'_> {
        Variant::iter().filter(|v| self.checked_locations.has_unchecked_variant(*v)).filter(|v| v.can_unlock(&self.items))
    }

    pub fn goal_progress(&self) -> GoalProgress {
        GoalProgress {
            scions: self.items.scions,
            required_scions: self.slot_data.required_scions,
            villains: self
                .checked_locations
                .villains
                .iter()
                .map(|bitfield| (0..4).map(move |b| (*bitfield & (1 << b)) as u32 * self.slot_data.villain_difficulty_points[b]).sum::<u32>())
                .sum::<u32>(),
            required_villains: self.slot_data.required_villains,
            variants: self.checked_locations.variants.iter().map(|b| b.count_ones()).sum::<u32>(),
            required_variants: self.slot_data.required_variants,
        }
    }
}

impl GoalProgress {
    pub fn available(&self) -> bool {
        self.scions >= self.required_scions && self.villains >= self.required_villains && self.variants >= self.required_variants
    }
}
