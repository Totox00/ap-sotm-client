// ITEMS                     52   48       40       32       24       16        8
//                        ____0000 00000000 00000000 00000000 00000000 00000000 00000000
//                  scion ____0000 00000000 00000000 00000000 00000000 00000000 00000001
//                villain ____0001 00000000 00000000 00000000 00000000 xxxxxxxx xxxxxxxx (x: villain index)
//           team villain ____0011 00000000 00000000 00000000 00000000 xxxxxxxx xxxxxxxx (x: team villain index)
//                   hero ____0010 00000000 00000000 00000000 00000000 xxxxxxxx xxxxxxxx (x: hero index)
//                variant ____0010 00000000 00000000 00000000 yyyyyyyy xxxxxxxx xxxxxxxx (x: hero index, y: variant index)
//            environment ____0100 00000000 00000000 00000000 00000000 xxxxxxxx xxxxxxxx (x: environment index)
//        all hero filler ____1000 00000000 00000000 1000aaaa yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: hero index, a: damage type index)
//       hero hero filler ____1000 00000000 00000000 1001aaaa yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: hero index, a: damage type index)
//    variant hero filler ____1000 00000000 zzzzzzzz 1010aaaa yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: hero index, z: variant index, a: damage type index)
//     all villain filler ____1000 00000000 00000000 0100aaaa yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: villain index, a: damage type index)
// classic villain filler ____1000 00000000 00000000 0101aaaa yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: villain index, a: damage type index)
//    team villain filler ____1000 00000000 00000000 0110aaaa yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: villain index, a: damage type index)
//           other filler ____1000 00000000 00000000 0000aaaa 00000000 00000000 xxxxxxxx (x: filler index, a: damage type index)
//          all hero trap ____1001 00000000 00000000 1000aaaa yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: hero index, a: damage type index)
//         hero hero trap ____1001 00000000 00000000 1001aaaa yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: hero index, a: damage type index)
//      variant hero trap ____1001 00000000 zzzzzzzz 1010aaaa yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: hero index, z: variant index, a: damage type index)
//       all villain trap ____1001 00000000 00000000 0100aaaa yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: villain index, a: damage type index)
//   classic villain trap ____1001 00000000 00000000 0101aaaa yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: villain index, a: damage type index)
//      team villain trap ____1001 00000000 00000000 0110aaaa yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: villain index, a: damage type index)
//             other trap ____1001 00000000 00000000 0000aaaa 00000000 00000000 xxxxxxxx (x: filler index, a: damage type index)
//
// LOCATIONS                 52   48       40       32       24       16        8
//                        ____0000 00000000 00000000 00000000 00000000 00000000 00000000
//                villain ____0001 00000000 00000000 00000000 zzyyyyyy xxxxxxxx xxxxxxxx (x: villain index, y: check #, z: difficulty)
//           team villain ____0011 00000000 00000000 00000000 zzyyyyyy xxxxxxxx xxxxxxxx (x: team villain index, y: check #, z: difficulty)
//           hero variant ____0010 00000000 00000000 zzzzzzzz 00yyyyyy xxxxxxxx xxxxxxxx (x: hero index, y: check #, z: variant index)
//        villain variant ____0010 00000000 00000000 00000000 00yyyyyy xxxxxxxx xxxxxxxx (x: variant index, y: check #)
//            environment ____0100 00000000 00000000 00000000 00yyyyyy xxxxxxxx xxxxxxxx (x: environment index, y: check #)

use std::{fs::OpenOptions, io::Write};

use crate::{Data, EnumData, FillerData, FillerType, VariantData};

pub fn generate_id_py(data: &Data) {
    if let Ok(mut writer) = OpenOptions::new().write(true).create(true).truncate(true).open("Id.py") {
        let _ = write!(writer, "# This file is generated as part of the compilation of the client\nitem_name_to_id={{\"Scion of Oblivaeon\":1,");

        item_id(&mut writer, &data.villains, 0b0001);
        item_id(&mut writer, &data.team_villains, 0b0011);
        item_id(&mut writer, &data.heroes, 0b0010);
        for (variant, idx) in data.hero_variants().map(|variant| (variant, get_base_idx(variant, &data.heroes))) {
            let _ = write!(writer, "\"{}\":{},", variant.display_name, (0b0010 << 48) | idx | ((variant.i as i64) << 16));
        }
        item_id(&mut writer, &data.environments, 0b0100);

        push_filler(&mut writer, data);

        let _ = write!(writer, "}}\nlocation_name_to_id={{");

        location_id(&mut writer, &data.villains, 0b0001);
        location_id(&mut writer, &data.team_villains, 0b0011);
        for (variant, idx) in data.variants.iter().map(|variant| (variant, get_base_idx(variant, &data.heroes))) {
            for c in 0..5 {
                let _ = write!(
                    writer,
                    "\"{} - Unlock #{}\":{},",
                    variant.display_name,
                    c + 1,
                    (0b0010 << 48) | c << 16 | if variant.base == "Villain" { variant.i as i64 } else { idx | (variant.i as i64) << 24 }
                );
            }
        }
        location_id(&mut writer, &data.environments, 0b0100);

        let _ = writeln!(writer, "}}");
    }
}

fn item_id<T>(writer: &mut T, data: &[EnumData], prefix: i64)
where
    T: Write,
{
    for (data, idx) in data.iter().zip(0..) {
        let _ = write!(writer, "\"{}\":{},", data.display_name, (prefix << 48) | idx);
    }
}

const DIFFICULTIES: [&str; 4] = ["Normal", "Advanced", "Challenge", "Ultimate"];

fn location_id<T>(writer: &mut T, data: &[EnumData], prefix: i64)
where
    T: Write,
{
    if (prefix & 1) == 1 {
        for (data, idx) in data.iter().zip(0..) {
            for c in 0..5 {
                for d in 0..4 {
                    if d >= 2 && data.display_name == "Spite: Agent of Gloom" {
                        let _ = write!(
                            writer,
                            "\"Spite: Agent of Gloom and Skinwalker Gloomweaver - {} #{}\":{},",
                            DIFFICULTIES[d as usize],
                            c + 1,
                            (prefix << 48) | idx | c << 16 | d << 22
                        );
                    } else if d < 2 || data.display_name != "Skinwalker Gloomweaver" {
                        let _ = write!(
                            writer,
                            "\"{} - {} #{}\":{},",
                            data.display_name,
                            DIFFICULTIES[d as usize],
                            c + 1,
                            (prefix << 48) | idx | c << 16 | d << 22
                        );
                    }
                }
            }
        }
    } else {
        for (data, idx) in data.iter().zip(0..) {
            for c in 0..5 {
                let _ = write!(writer, "\"{} - Any Difficulty #{}\":{},", data.display_name, c + 1, (prefix << 48) | idx | c << 16);
            }
        }
    }
}

const DAMAGE_TYPES: [&str; 12] = [
    "",
    "Cold ",
    "Energy ",
    "Fire ",
    "Infernal ",
    "Lightning ",
    "Melee ",
    "Projectile ",
    "Psychic ",
    "Radiant ",
    "Sonic ",
    "Toxic ",
];

fn push_filler<T>(writer: &mut T, data: &Data)
where
    T: Write,
{
    for filler in &data.filler {
        match filler.r#type {
            FillerType::Hero => handle_pos_neg(filler, |name, b| {
                handle_damage_types(filler.damage_types, |t| {
                    let _ = write!(writer, "\"{}\":{},", normalize(name, t), 0b1000 << 48 | b | filler.i | t << 24 | 0b1000 << 28);
                    for (hero, idx) in data.heroes.iter().zip(0..) {
                        let _ = write!(
                            writer,
                            "\"{} (Any {})\":{},",
                            normalize(name, t),
                            hero.display_name,
                            0b1000 << 48 | b | filler.i | t << 24 | 0b1001 << 28 | idx << 8
                        );
                    }
                    for variant in data.hero_variants() {
                        let _ = write!(
                            writer,
                            "\"{} ({})\":{},",
                            normalize(name, t),
                            variant.display_name,
                            0b1000 << 48 | b | filler.i | t << 24 | 0b1010 << 28 | get_base_idx(variant, &data.heroes) << 8 | (variant.i as i64) << 32
                        );
                    }
                })
            }),
            FillerType::Villain => handle_pos_neg(filler, |name, b| {
                handle_damage_types(filler.damage_types, |t| {
                    let _ = write!(writer, "\"{}\":{},", normalize(name, t), 0b1000 << 48 | b | filler.i | t << 24 | 0b0100 << 28);
                    for (villain, idx) in data.villains.iter().zip(0..) {
                        let _ = write!(
                            writer,
                            "\"{} ({})\":{},",
                            normalize(name, t),
                            villain.display_name,
                            0b1000 << 48 | b | filler.i | t << 24 | 0b0101 << 28 | idx << 8
                        );
                    }
                    for (villain, idx) in data.team_villains.iter().zip(0..) {
                        let _ = write!(
                            writer,
                            "\"{} ({})\":{},",
                            normalize(name, t),
                            villain.display_name,
                            0b1000 << 48 | b | filler.i | t << 24 | 0b0110 << 28 | idx << 8
                        );
                    }
                })
            }),
            FillerType::Other => handle_pos_neg(filler, |name, b| {
                handle_damage_types(filler.damage_types, |t| {
                    let _ = write!(writer, "\"{}\":{},", normalize(name, t), 0b1000 << 48 | b | filler.i | t << 24);
                })
            }),
        }
    }
}

fn handle_pos_neg<F>(filler: &FillerData, mut r#fn: F)
where
    F: FnMut(&str, i64),
{
    if !filler.display_name_pos.is_empty() {
        r#fn(&filler.display_name_pos, 0)
    }
    if !filler.display_name_neg.is_empty() {
        r#fn(&filler.display_name_neg, 1 << 48)
    }
}

fn handle_damage_types<F>(damage_types: bool, mut r#fn: F)
where
    F: FnMut(i64),
{
    if damage_types {
        for t in 0..12 {
            r#fn(t)
        }
    } else {
        r#fn(0)
    }
}

fn normalize(name: &str, t: i64) -> String {
    name.replace("[COUNT]", "1").replace("[TYPE]", DAMAGE_TYPES[t as usize])
}

fn get_base_idx(variant: &VariantData, heroes: &[EnumData]) -> i64 {
    heroes.iter().position(|hero| hero.enum_name == variant.base).unwrap_or(0) as i64
}
