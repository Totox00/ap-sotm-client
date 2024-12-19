// ITEMS                     52   48       40       32       24       16        8
//                        ____0000 00000000 00000000 00000000 00000000 00000000 00000000
//                  scion ____0000 00000000 00000000 00000000 00000000 00000000 00000001
//                villain ____0001 00000000 00000000 00000000 00000000 xxxxxxxx xxxxxxxx (x: villain index)
//           team villain ____0011 00000000 00000000 00000000 00000000 xxxxxxxx xxxxxxxx (x: team villain index)
//              gladiator ____0101 00000000 00000000 00000000 00000000 xxxxxxxx xxxxxxxx (x: gladiator index)
//                   hero ____0010 00000000 00000000 00000000 00000000 xxxxxxxx xxxxxxxx (x: hero index)
//                variant ____0010 00000000 00000000 00000000 yyyyyyyy xxxxxxxx xxxxxxxx (x: hero index, y: variant index)
//              contender ____0110 00000000 00000000 00000000 00000000 xxxxxxxx xxxxxxxx (x: contender index)
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
//              gladiator ____0101 00000000 00000000 00000000 zzyyyyyy xxxxxxxx xxxxxxxx (x: gladiator index, y: check #, z: difficulty)
//           hero variant ____0010 00000000 00000000 zzzzzzzz 00yyyyyy xxxxxxxx xxxxxxxx (x: hero index, y: check #, z: variant index)
//        villain variant ____0010 00000000 00000000 00000000 01yyyyyy xxxxxxxx xxxxxxxx (x: variant index, y: check #)
//            environment ____0100 00000000 00000000 00000000 00yyyyyy xxxxxxxx xxxxxxxx (x: environment index, y: check #)

use std::{fs::OpenOptions, io::Write};

use crate::{Data, EnumData, FillerData, FillerType, VillainData, MAX_LOCATION_DENSITY};

pub fn generate_id_py(data: &Data) {
    if let Ok(mut writer) = OpenOptions::new().write(true).create(true).truncate(true).open("Id.py") {
        let _ = write!(writer, "# This file is generated as part of the compilation of the client\nitem_name_to_id={{\"Scion of Oblivaeon\":1,");

        villain_item_id(&mut writer, &data.villains, 0b0001);
        villain_item_id(&mut writer, &data.team_villains, 0b0011);
        villain_item_id(&mut writer, &data.gladiators, 0b0101);
        item_id(&mut writer, &data.heroes, 0b0010);
        item_id(&mut writer, &data.contenders, 0b0110);
        for variant in data.hero_variants() {
            let _ = write!(writer, "\"{}\":{},", variant.display_name, (0b0010 << 48) | variant.base_i as i64 | ((variant.i as i64) << 16));
        }
        item_id(&mut writer, &data.environments, 0b0100);

        push_filler(&mut writer, data);

        let _ = write!(writer, "}}\nlocation_name_to_id={{");

        villain_location_id(&mut writer, &data.villains, 0b0001);
        villain_location_id(&mut writer, &data.team_villains, 0b0011);
        villain_location_id(&mut writer, &data.gladiators, 0b0101);
        for variant in data.variants.iter() {
            for c in 0..MAX_LOCATION_DENSITY {
                let _ = write!(
                    writer,
                    "\"{} - Unlock #{}\":{},",
                    variant.display_name,
                    c + 1,
                    (0b0010 << 48)
                        | c << 16
                        | if variant.is_villain {
                            variant.i as i64 | 1 << 22
                        } else {
                            variant.base_i as i64 | (variant.i as i64) << 24
                        }
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
    for data in data {
        let _ = write!(writer, "\"{}\":{},", data.display_name, (prefix << 48) | data.i as i64);
    }
}

fn villain_item_id<T>(writer: &mut T, data: &[VillainData], prefix: i64)
where
    T: Write,
{
    for data in data {
        let _ = write!(writer, "\"{}\":{},", data.display_name, (prefix << 48) | data.i as i64);
    }
}

const DIFFICULTIES: [&str; 4] = ["Normal", "Advanced", "Challenge", "Ultimate"];

fn location_id<T>(writer: &mut T, data: &[EnumData], prefix: i64)
where
    T: Write,
{
    for data in data {
        for c in 0..MAX_LOCATION_DENSITY {
            let _ = write!(writer, "\"{} - Any Difficulty #{}\":{},", data.display_name, c + 1, (prefix << 48) | data.i as i64 | c << 16);
        }
    }
}

fn villain_location_id<T>(writer: &mut T, data: &[VillainData], prefix: i64)
where
    T: Write,
{
    for data in data {
        for c in 0..MAX_LOCATION_DENSITY {
            for d in 0..2 {
                let _ = write!(
                    writer,
                    "\"{} - {} #{}\":{},",
                    data.display_name,
                    DIFFICULTIES[d as usize],
                    c + 1,
                    (prefix << 48) | data.i as i64 | c << 16 | d << 22
                );
            }

            if data.challenge.is_some() {
                for d in 2..4 {
                    let _ = write!(
                        writer,
                        "\"{} - {} #{}\":{},",
                        data.display_name,
                        DIFFICULTIES[d as usize],
                        c + 1,
                        (prefix << 48) | data.i as i64 | c << 16 | d << 22
                    );
                }
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
                    let _ = write!(writer, "\"{}\":{},", normalize(name, t), 0b1000 << 48 | b | filler.i as i64 | t << 24 | 0b1000 << 28);
                    for hero in &data.heroes {
                        let _ = write!(
                            writer,
                            "\"{} (Any {})\":{},",
                            normalize(name, t),
                            hero.display_name,
                            0b1000 << 48 | b | filler.i as i64 | t << 24 | 0b1001 << 28 | (hero.i as i64) << 8
                        );
                    }
                    for variant in data.hero_variants() {
                        let _ = write!(
                            writer,
                            "\"{} ({})\":{},",
                            normalize(name, t),
                            variant.display_name,
                            0b1000 << 48 | b | filler.i as i64 | t << 24 | 0b1010 << 28 | (variant.base_i as i64) << 8 | (variant.i as i64) << 32
                        );
                    }
                })
            }),
            FillerType::Villain => handle_pos_neg(filler, |name, b| {
                handle_damage_types(filler.damage_types, |t| {
                    let _ = write!(writer, "\"{}\":{},", normalize(name, t), 0b1000 << 48 | b | filler.i as i64 | t << 24 | 0b0100 << 28);
                    for villain in &data.villains {
                        let _ = write!(
                            writer,
                            "\"{} ({})\":{},",
                            normalize(name, t),
                            villain.display_name,
                            0b1000 << 48 | b | filler.i as i64 | t << 24 | 0b0101 << 28 | (villain.i as i64) << 8
                        );
                    }
                    for villain in &data.team_villains {
                        let _ = write!(
                            writer,
                            "\"{} ({})\":{},",
                            normalize(name, t),
                            villain.display_name,
                            0b1000 << 48 | b | filler.i as i64 | t << 24 | 0b0110 << 28 | (villain.i as i64) << 8
                        );
                    }
                })
            }),
            FillerType::Other => handle_pos_neg(filler, |name, b| {
                handle_damage_types(filler.damage_types, |t| {
                    let _ = write!(writer, "\"{}\":{},", normalize(name, t), 0b1000 << 48 | b | filler.i as i64 | t << 24);
                })
            }),
        }
    }
}

fn handle_pos_neg<F>(filler: &FillerData, mut r#fn: F)
where
    F: FnMut(&str, i64),
{
    if let Some(pos) = &filler.display_name_pos {
        r#fn(pos, 0)
    }
    if let Some(neg) = &filler.display_name_neg {
        r#fn(neg, 1 << 48)
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
