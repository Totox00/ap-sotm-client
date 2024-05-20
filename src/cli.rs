use std::{
    collections::VecDeque,
    io::{stdout, StdoutLock, Write},
};

use crate::{
    data::{Environment, Hero, Item, Location, TeamVillain, Variant, Villain},
    state::State,
};
use console::{style, StyledObject, Term};
use num::FromPrimitive;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub enum DisplayUpdate {
    Msg(String),
    Items(Vec<Item>),
    Filter(String),
    CursorLeft,
    CursorRight,
    CursorUp,
    CursorDown,
    CursorHome,
    Select,
    Send,
    Exit,
}

const COLUMN_SIZES: [usize; 7] = [20, 20, 30, 30, 20, 30, 50];
const COLUMN_OFFSETS: [usize; 7] = [0, 20, 40, 70, 100, 120, 150];

#[allow(clippy::too_many_lines)]
pub fn print(term: &Term, state: &State, filter: &str, cursor_x: usize, cursor_y: usize, msg_buffer: &VecDeque<String>, multi_send: bool) -> bool {
    let available = state.available_locations();
    let scroll = if cursor_y < 10 { 0 } else { cursor_y - 10 };
    let _ = term.clear_screen();
    let mut lock = stdout().lock();
    let _ = writeln!(lock, "{filter}");

    for msg in msg_buffer {
        let _ = writeln!(lock, "{msg}");
    }

    if available.victory {
        if filter.to_lowercase() == "oblivaeon" {
            return true;
        }
        let (_, cols) = term.size();
        let _ = term.move_cursor_to(cols as usize - 18, 1);
        let _ = write!(lock, " {}", style("Victory available").bright());
        let _ = term.move_cursor_to(cols as usize - 34, 2);
        let _ = write!(lock, " {}", style("Send by filtering for \"oblivaeon\"").bright());
    }

    if multi_send {
        let (_, cols) = term.size();
        let _ = term.move_cursor_to(cols as usize - 19, 3);
        let _ = write!(lock, " {}", style("Multi-send enabled").bright());
    }

    let mut offset = 1;
    for (v, y) in Villain::iter().filter(|v| state.items.has_villain(*v)).filter(|v| filter_match(filter, v.as_str())).zip(0..) {
        if scroll == 0 || y > scroll {
            let _ = term.move_cursor_to(COLUMN_OFFSETS[0], y + 11 - scroll);
            if cursor_x == 0 && cursor_y == y {
                let _ = write!(lock, "[{}]", style(trunc(v.as_str(), COLUMN_SIZES[0] - 2)).bold());
            } else {
                let _ = write!(lock, "{}", trunc(v.as_str(), COLUMN_SIZES[0]));
            }
        }
        offset += 1;
    }

    for (v, y) in TeamVillain::iter()
        .filter(|v| state.items.has_team_villain(*v))
        .filter(|v| filter_match(filter, v.as_str()))
        .zip(offset..)
    {
        if scroll == 0 || y > scroll {
            let _ = term.move_cursor_to(COLUMN_OFFSETS[0], y + 11 - scroll);
            if cursor_x == 0 && cursor_y == y {
                let _ = write!(lock, "[{}]", style(trunc(v.as_str(), COLUMN_SIZES[0] - 2)).bold());
            } else {
                let _ = write!(lock, "{}", trunc(v.as_str(), COLUMN_SIZES[0]));
            }
        }
    }

    for (e, y) in Environment::iter().filter(|e| state.items.has_environment(*e)).filter(|e| filter_match(filter, e.as_str())).zip(0..) {
        if scroll == 0 || y > scroll {
            let _ = term.move_cursor_to(COLUMN_OFFSETS[1], y + 11 - scroll);
            if cursor_x == 1 && cursor_y == y {
                let _ = write!(lock, "[{}]", style(trunc(e.as_str(), COLUMN_SIZES[1] - 2)).bold());
            } else {
                let _ = write!(lock, "{}", trunc(e.as_str(), COLUMN_SIZES[1]));
            }
        }
    }

    let mut offset = 0;
    for ((bitfield, hero), base_y) in state
        .items
        .heroes
        .iter()
        .zip(0..)
        .filter_map(|(b, h)| Hero::from_i32(h).map(|hero| (b, hero)))
        .filter(|(b, _)| b.count_ones() > 0)
        .filter(|(b, h)| {
            filter_match(filter, h.as_str())
                || (1..7)
                    .filter(move |v| *b & 1 << v > 0)
                    .filter_map(move |v| Variant::from_hero(*h, v))
                    .any(|v| filter_match(filter, v.as_str()))
        })
        .zip(0..)
    {
        if scroll == 0 || base_y > scroll {
            let y = base_y + offset;
            let _ = term.move_cursor_to(COLUMN_OFFSETS[2], y + 11 - scroll);
            if *bitfield == 1 {
                if cursor_x == 2 && cursor_y == y {
                    let _ = write!(lock, "[{}]", style(trunc(hero.as_str(), COLUMN_SIZES[2] - 2)).bold());
                } else {
                    let _ = write!(lock, "{}", trunc(hero.as_str(), COLUMN_SIZES[2]));
                }
            } else if bitfield.count_ones() == 1 {
                if let Some(variant) = Variant::from_hero(hero, bitfield.trailing_zeros()) {
                    if cursor_x == 2 && cursor_y == y {
                        let _ = write!(lock, "[{}]", style(trunc(variant.as_str(), COLUMN_SIZES[2] - 2)).bold());
                    } else {
                        let _ = write!(lock, "{}", trunc(variant.as_str(), COLUMN_SIZES[2]));
                    }
                }
            } else if cursor_x == 2 && cursor_y == y {
                let _ = write!(lock, "[{}{}]", if bitfield & 1 == 1 { "++ " } else { "!! " }, style(trunc(hero.as_str(), COLUMN_SIZES[2] - 5)).bold());
                for (variant, v_offset) in (1..7).filter(|o| bitfield & 1 << o > 1).zip(1..) {
                    if let Some(variant) = Variant::from_hero(hero, variant) {
                        let _ = term.move_cursor_to(COLUMN_OFFSETS[2], y + v_offset + 11 - scroll);
                        let _ = write!(lock, " - {}", trunc(variant.as_str(), COLUMN_SIZES[2] - 3));
                        offset += 1;
                    }
                }
            } else {
                let _ = write!(lock, "{}{}", if bitfield & 1 == 1 { "++ " } else { "!! " }, trunc(hero.as_str(), COLUMN_SIZES[2] - 3));
            }
        }
    }

    let mut offset = 1;
    for ((v, d), y) in available.villains.iter().filter(|(v, _)| filter_match(filter, v.as_str())).zip(0..) {
        if scroll == 0 || y > scroll {
            let _ = term.move_cursor_to(COLUMN_OFFSETS[3], y + 11 - scroll);
            if cursor_x == 3 && cursor_y == y {
                let _ = write!(lock, "[{} - {}]", style(trunc(v.as_str(), COLUMN_SIZES[3] - 2)).bold(), to_dif(*d));
            } else {
                let _ = write!(lock, "{} - {}", trunc(v.as_str(), COLUMN_SIZES[3]), to_dif(*d));
            }
        }
        offset += 1;
    }

    for ((v, d), y) in available.team_villains.iter().filter(|(v, _)| filter_match(filter, v.as_str())).zip(offset..) {
        if scroll == 0 || y > scroll {
            let _ = term.move_cursor_to(COLUMN_OFFSETS[3], y + 11 - scroll);
            if cursor_x == 3 && cursor_y == y {
                let _ = write!(lock, "[{} - {}]", style(trunc(v.as_str(), COLUMN_SIZES[3] - 2)).bold(), to_dif(*d));
            } else {
                let _ = write!(lock, "{} - {}", trunc(v.as_str(), COLUMN_SIZES[3]), to_dif(*d));
            }
        }
    }

    for (e, y) in available.environments.iter().filter(|e| filter_match(filter, e.as_str())).zip(0..) {
        if scroll == 0 || y > scroll {
            let _ = term.move_cursor_to(COLUMN_OFFSETS[4], y + 11 - scroll);
            if cursor_x == 4 && cursor_y == y {
                let _ = write!(lock, "[{}]", style(trunc(e.as_str(), COLUMN_SIZES[4] - 2)).bold());
            } else {
                let _ = write!(lock, "{}", trunc(e.as_str(), COLUMN_SIZES[4]));
            }
        }
    }

    for (v, y) in available.variants.iter().filter(|v| filter_match(filter, v.as_str())).zip(0..) {
        if scroll == 0 || y > scroll {
            let _ = term.move_cursor_to(COLUMN_OFFSETS[5], y + 11 - scroll);
            if cursor_x == 5 && cursor_y == y {
                let _ = write!(lock, "[{}]", style(trunc(v.as_str(), COLUMN_SIZES[5] - 2)).bold());
                write_variant_desc(term, &mut lock, *v);
            } else {
                let _ = write!(lock, "{}", trunc(v.as_str(), COLUMN_SIZES[5]));
            }
        }
    }

    let _ = lock.flush();
    false
}

pub fn find_location(state: &State, filter: &str, cursor_x: usize, cursor_y: usize) -> Option<Location> {
    let available = state.available_locations();
    match cursor_x {
        3 => {
            let villain_count = available.villains.iter().filter(|(v, _)| filter_match(filter, v.as_str())).count();
            if cursor_y < villain_count {
                available.villains.iter().filter(|(v, _)| filter_match(filter, v.as_str())).nth(cursor_y).map(|v| Location::Villain(*v))
            } else {
                available
                    .team_villains
                    .iter()
                    .filter(|(v, _)| filter_match(filter, v.as_str()))
                    .nth(cursor_y - villain_count - 1)
                    .map(|v| Location::TeamVillain(*v))
            }
        }
        4 => available
            .environments
            .iter()
            .filter(|e| filter_match(filter, e.as_str()))
            .nth(cursor_y)
            .map(|e| Location::Environment(*e)),
        5 => available.variants.iter().filter(|v| filter_match(filter, v.as_str())).nth(cursor_y).map(|v| Location::Variant(*v)),
        _ => None,
    }
}

fn trunc(str: &str, max_len: usize) -> &str {
    if str.len() > max_len {
        &str[0..max_len]
    } else {
        str
    }
}

fn to_dif(d: u8) -> StyledObject<&'static str> {
    match d {
        0 => style("Normal").green(),
        1 => style("Advanced").yellow(),
        2 => style("Challenge").red(),
        3 => style("Ultimate").magenta(),
        _ => style(""),
    }
}

fn filter_match(filter: &str, item: &str) -> bool {
    let lowercase = filter.to_ascii_lowercase();
    let mut filter_iter = lowercase.chars().peekable();
    for char in item.to_ascii_lowercase().chars() {
        if let Some(next) = filter_iter.peek() {
            if *next == char {
                filter_iter.next();
            }
        } else {
            return true;
        }
    }

    filter_iter.next().is_none()
}

fn write_variant_desc(term: &Term, lock: &mut StdoutLock, v: Variant) {
    let desc = v.as_desc();
    if desc.is_empty() {
        return;
    }

    let mut lines = vec![String::new()];
    for word in desc.split(' ') {
        let word: String = word.chars().map(|c| if c == '_' { ' ' } else { c }).collect();
        if let Some((a, b)) = word.split_once('\\') {
            if lines.last().unwrap().len() + a.len() > COLUMN_SIZES[6] {
                lines.push(String::new());
            }

            lines.last_mut().unwrap().push_str(a);
            lines.push(String::from(b));
        } else {
            if lines.last().unwrap().len() + word.len() > COLUMN_SIZES[6] {
                lines.push(String::new());
            }
            lines.last_mut().unwrap().push_str(&word);
        }
        lines.last_mut().unwrap().push(' ');
    }

    for (line, y) in lines.iter().zip(0..) {
        let _ = term.move_cursor_to(COLUMN_OFFSETS[6], y + 11);
        let _ = write!(lock, "{line}");
    }
}
