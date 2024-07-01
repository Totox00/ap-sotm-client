use client_lib::{
    data::{Environment, Hero, Location, TeamVillain, Variant, Villain},
    state::State,
};
use console::{style, StyledObject, Term};
use num::FromPrimitive;
use std::{
    collections::VecDeque,
    io::{stdout, StdoutLock, Write},
};
use strum::IntoEnumIterator;

#[allow(clippy::too_many_lines)]
pub fn print(term: &Term, state: &State, filter: &str, cursor_x: usize, cursor_y: usize, msg_buffer: &VecDeque<String>, multi_send: bool) -> bool {
    let available = state.available_locations();
    let scroll_y = if cursor_y < 10 { 0 } else { cursor_y - 10 };
    let _ = term.clear_screen();
    let mut lock = stdout().lock();
    let _ = writeln!(lock, "{}", if filter.is_empty() { "Type to filter..." } else { filter });

    let (rows, cols) = term.size();
    let column_sizes = calc_columns(cols as usize, cursor_x);
    let mut lines_needed = 0;
    let mut start_i = 0;

    for (msg, i) in msg_buffer.iter().zip(0..10).rev() {
        lines_needed += (msg.len() as f32 / cols as f32).ceil() as usize;
        if lines_needed > 10 {
            start_i = i;
            break;
        }
    }

    for msg in msg_buffer.iter().skip(start_i) {
        let _ = writeln!(lock, "{msg}");
    }

    if available.victory {
        if filter.to_lowercase() == "oblivaeon" {
            return true;
        }
        let _ = term.move_cursor_to(cols as usize - 18, 1);
        let _ = write!(lock, " {}", style("Victory available").bright());
        let _ = term.move_cursor_to(cols as usize - 34, 2);
        let _ = write!(lock, " {}", style("Send by filtering for \"oblivaeon\"").bright());
    }

    if multi_send {
        let _ = term.move_cursor_to(cols as usize - 19, 3);
        let _ = write!(lock, " {}", style("Multi-send enabled").bright());
    }

    let mut offset_x = 0;
    if column_sizes[0] > 0 {
        let mut offset = 1;
        for (v, y) in Villain::iter().filter(|v| state.items.has_villain(*v)).filter(|v| filter_match(filter, v.as_str())).zip(0..) {
            if scroll_y == 0 || y > scroll_y {
                let _ = term.move_cursor_to(offset_x, y + 11 - scroll_y);
                if cursor_x == 0 && cursor_y == y {
                    let _ = write!(lock, "[{}]", style(trunc(v.as_str(), column_sizes[0] - 2)).bold());
                } else {
                    let _ = write!(lock, "{}", trunc(v.as_str(), column_sizes[0]));
                }
            }
            offset += 1;
        }

        for (v, y) in TeamVillain::iter()
            .filter(|v| state.items.has_team_villain(*v))
            .filter(|v| filter_match(filter, v.as_str()))
            .zip(offset..)
        {
            if scroll_y == 0 || y > scroll_y {
                let _ = term.move_cursor_to(offset_x, y + 11 - scroll_y);
                if cursor_x == 0 && cursor_y == y {
                    let _ = write!(lock, "[{}]", style(trunc(v.as_str(), column_sizes[0] - 2)).bold());
                } else {
                    let _ = write!(lock, "{}", trunc(v.as_str(), column_sizes[0]));
                }
            }
        }
    }

    offset_x += column_sizes[0];

    if column_sizes[1] > 0 {
        for (e, y) in Environment::iter().filter(|e| state.items.has_environment(*e)).filter(|e| filter_match(filter, e.as_str())).zip(0..) {
            if scroll_y == 0 || y > scroll_y {
                let _ = term.move_cursor_to(offset_x, y + 11 - scroll_y);
                if cursor_x == 1 && cursor_y == y {
                    let _ = write!(lock, "[{}]", style(trunc(e.as_str(), column_sizes[1] - 2)).bold());
                } else {
                    let _ = write!(lock, "{}", trunc(e.as_str(), column_sizes[1]));
                }
            }
        }
    }

    offset_x += column_sizes[1];

    if column_sizes[2] > 0 {
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
            if scroll_y == 0 || base_y > scroll_y {
                let y = base_y + offset;
                let _ = term.move_cursor_to(offset_x, y + 11 - scroll_y);
                if *bitfield == 1 {
                    if cursor_x == 2 && cursor_y == y {
                        let _ = write!(lock, "[{}]", style(trunc(hero.as_str(), column_sizes[2] - 2)).bold());
                    } else {
                        let _ = write!(lock, "{}", trunc(hero.as_str(), column_sizes[2]));
                    }
                } else if bitfield.count_ones() == 1 {
                    if let Some(variant) = Variant::from_hero(hero, bitfield.trailing_zeros()) {
                        if cursor_x == 2 && cursor_y == y {
                            let _ = write!(lock, "[{}]", style(trunc(variant.as_str(), column_sizes[2] - 2)).bold());
                        } else {
                            let _ = write!(lock, "{}", trunc(variant.as_str(), column_sizes[2]));
                        }
                    }
                } else if cursor_x == 2 && cursor_y == y {
                    let _ = write!(lock, "[{}{}]", if bitfield & 1 == 1 { "++ " } else { "!! " }, style(trunc(hero.as_str(), column_sizes[2] - 5)).bold());
                    for (variant, v_offset) in (1..7).filter(|o| bitfield & 1 << o > 1).zip(1..) {
                        if let Some(variant) = Variant::from_hero(hero, variant) {
                            let _ = term.move_cursor_to(offset_x, y + v_offset + 11 - scroll_y);
                            let _ = write!(lock, " - {}", trunc(variant.as_str(), column_sizes[2] - 3));
                            offset += 1;
                        }
                    }
                } else {
                    let _ = write!(lock, "{}{}", if bitfield & 1 == 1 { "++ " } else { "!! " }, trunc(hero.as_str(), column_sizes[2] - 3));
                }
            }
        }
    }

    offset_x += column_sizes[2];

    if column_sizes[3] > 0 {
        let mut offset = 1;
        for ((v, d), y) in available.villains.iter().filter(|(v, _)| filter_match(filter, v.as_str())).zip(0..) {
            if scroll_y == 0 || y > scroll_y {
                let _ = term.move_cursor_to(offset_x, y + 11 - scroll_y);
                if cursor_x == 3 && cursor_y == y {
                    let _ = write!(lock, "[{} - {}]", style(trunc(v.as_str(), column_sizes[3] - 2)).bold(), to_dif(*d));
                } else {
                    let _ = write!(lock, "{} - {}", trunc(v.as_str(), column_sizes[3]), to_dif(*d));
                }
            }
            offset += 1;
        }

        for ((v, d), y) in available.team_villains.iter().filter(|(v, _)| filter_match(filter, v.as_str())).zip(offset..) {
            if scroll_y == 0 || y > scroll_y {
                let _ = term.move_cursor_to(offset_x, y + 11 - scroll_y);
                if cursor_x == 3 && cursor_y == y {
                    let _ = write!(lock, "[{} - {}]", style(trunc(v.as_str(), column_sizes[3] - 2)).bold(), to_dif(*d));
                } else {
                    let _ = write!(lock, "{} - {}", trunc(v.as_str(), column_sizes[3]), to_dif(*d));
                }
            }
        }
    }

    offset_x += column_sizes[3];

    if column_sizes[4] > 0 {
        for (e, y) in available.environments.iter().filter(|e| filter_match(filter, e.as_str())).zip(0..) {
            if scroll_y == 0 || y > scroll_y {
                let _ = term.move_cursor_to(offset_x, y + 11 - scroll_y);
                if cursor_x == 4 && cursor_y == y {
                    let _ = write!(lock, "[{}]", style(trunc(e.as_str(), column_sizes[4] - 2)).bold());
                } else {
                    let _ = write!(lock, "{}", trunc(e.as_str(), column_sizes[4]));
                }
            }
        }
    }

    offset_x += column_sizes[4];

    if column_sizes[5] > 0 {
        for (v, y) in available.variants.iter().filter(|v| filter_match(filter, v.as_str())).zip(0..) {
            if scroll_y == 0 || y > scroll_y {
                let _ = term.move_cursor_to(offset_x, y + 11 - scroll_y);
                if cursor_x == 5 && cursor_y == y {
                    let _ = write!(lock, "[{}]", style(trunc(v.as_str(), column_sizes[5] - 2)).bold());
                    if column_sizes[6] > 0 {
                        write_variant_desc(term, column_sizes[6], offset_x + column_sizes[5], &mut lock, *v);
                    }
                } else {
                    let _ = write!(lock, "{}", trunc(v.as_str(), column_sizes[5]));
                }
            }
        }
    }

    print_keybinds(term, &mut lock, rows as usize - 3);

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

fn write_variant_desc(term: &Term, width: usize, offset: usize, lock: &mut StdoutLock, v: Variant) {
    let desc = v.as_desc();
    if desc.is_empty() {
        return;
    }

    let mut lines = vec![String::new()];
    for word in desc.split(' ') {
        let word: String = word.chars().map(|c| if c == '_' { ' ' } else { c }).collect();
        if let Some((a, b)) = word.split_once('\\') {
            if lines.last().unwrap().len() + a.len() > width {
                lines.push(String::new());
            }

            lines.last_mut().unwrap().push_str(a);
            lines.push(String::from(b));
        } else {
            if lines.last().unwrap().len() + word.len() > width {
                lines.push(String::new());
            }
            lines.last_mut().unwrap().push_str(&word);
        }
        lines.last_mut().unwrap().push(' ');
    }

    for (line, y) in lines.iter().zip(0..) {
        let _ = term.move_cursor_to(offset, y + 11);
        let _ = write!(lock, "{line}");
    }
}

const COLUMN_RATIOS: [usize; 7] = [2, 2, 3, 3, 2, 3, 5];

fn calc_columns(cols: usize, cursor_x: usize) -> [usize; 7] {
    let (normal, with_variant, offset) = match cols {
        0..=39 => (1, 1, 0),
        40..=79 => (2, 2, 0),
        80..=99 => (3, 2, 1),
        100..=129 => (4, 3, 1),
        130..=149 => (5, 4, 2),
        150..=199 => (6, 5, 4),
        200.. => (7, 7, 0),
    };

    let mut sizes = [0; 7];
    if cursor_x == 5 && with_variant > 1 {
        let multiplier = (cols as f32 / COLUMN_RATIOS.iter().skip(7 - with_variant).sum::<usize>() as f32).floor() as usize;
        for col in (7 - with_variant)..7 {
            sizes[col] = COLUMN_RATIOS[col] * multiplier;
        }
    } else {
        let start_col = if cursor_x <= offset || cursor_x - offset + normal > 7 { 0 } else { cursor_x - offset };
        let multiplier = (cols as f32 / COLUMN_RATIOS.iter().skip(start_col).take(normal).sum::<usize>() as f32).floor() as usize;
        for col in start_col..(start_col + normal) {
            sizes[col] = COLUMN_RATIOS[col] * multiplier;
        }
    }

    sizes
}

fn print_keybinds(term: &Term, lock: &mut StdoutLock, row: usize) {
    let _ = term.move_cursor_to(0, row);
    let _ = writeln!(
        lock,
        " {}: Move cursor   {}: Send location   {}: Move cursor to start",
        style("Arrow Keys").black().on_white(),
        style("Enter").black().on_white(),
        style("Home").black().on_white()
    );
    let _ = writeln!(
        lock,
        "     {}: Disconnect   {}: Clear filter     {}: Toggle multisend",
        style("Ctrl+D").black().on_white(),
        style("Ctrl+C").black().on_white(),
        style("Tab").black().on_white()
    );
}
