use std::io::{stdout, Write};

use crate::{
    data::{Environment, Hero, Variant, Villain},
    state::State,
};
use console::Term;
use num::FromPrimitive;
use strum::IntoEnumIterator;

pub enum DisplayUpdate {
    Msg(String),
    State(State),
    Filter(String),
    CursorLeft,
    CursorRight,
    CursorUp,
    CursorDown,
    CursorHome,
    Select,
    Send,
}

const COLUMN_OFFSETS: [usize; 7] = [0, 20, 40, 70, 90, 110, 140];
// Search
// Villains  Environments Heroes     | Villains Environments Variants | Variant desc.
//                        > Variants |

pub fn print(term: &Term, state: &State, filter: &str, cursor_x: i32, cursor_y: i32) {
    let available = state.available_locations();
    dbg!(&available);
    dbg!(filter);

    let _ = term.clear_screen();
    let mut lock = stdout().lock();
    let _ = writeln!(lock, "{filter}");

    for (v, y) in Villain::iter().filter(|v| state.items.has_villain(*v)).zip(1..) {
        let _ = term.move_cursor_to(COLUMN_OFFSETS[0], y);
        let _ = write!(lock, "{}", v.to_str());
    }

    for (e, y) in Environment::iter().filter(|e| state.items.has_environment(*e)).zip(1..) {
        let _ = term.move_cursor_to(COLUMN_OFFSETS[1] - 1, y);
        let _ = write!(lock, " {}", e.to_str());
    }

    for ((bitfield, hero), y) in state.items.heroes.iter().zip(0..).filter(|(b, _)| b.count_ones() > 0).zip(1..) {
        let _ = term.move_cursor_to(COLUMN_OFFSETS[2] - 1, y);
        if *bitfield == 1 {
            if let Some(hero) = Hero::from_i32(hero) {
                let _ = write!(lock, " {}", hero.to_str());
            }
        } else if bitfield.count_ones() == 1 {
            if let Some(hero) = Hero::from_i32(hero) {
                if let Some(variant) = Variant::from_hero(hero, bitfield.trailing_zeros()) {
                    let _ = write!(lock, " {}", variant.to_str());
                }
            }
        } else {
            if let Some(hero) = Hero::from_i32(hero) {
                let _ = write!(lock, " {}{}", if bitfield & 1 == 1 { "" } else { "!! " }, hero.to_str());
            }
        }
    }

    for ((v, d), y) in available.villains.iter().zip(1..) {
        let _ = term.move_cursor_to(COLUMN_OFFSETS[3] - 1, y);
        let _ = write!(lock, " | {}", v.to_str());
    }

    for (e, y) in available.environments.iter().zip(1..) {
        let _ = term.move_cursor_to(COLUMN_OFFSETS[4] - 1, y);
        let _ = write!(lock, " {}", e.to_str());
    }

    for (v, y) in available.variants.iter().zip(1..) {
        let _ = term.move_cursor_to(COLUMN_OFFSETS[5] - 1, y);
        let _ = write!(lock, " {}", v.to_str());
    }

    let _ = lock.flush();
}
