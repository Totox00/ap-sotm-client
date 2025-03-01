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
//        all hero filler ____1000 00000000 00000000 10000000 yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: hero index)
//       hero hero filler ____1000 00000000 00000000 10010000 yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: hero index)
//    variant hero filler ____1000 00000000 zzzzzzzz 10100000 yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: hero index, z: variant index)
//     all villain filler ____1000 00000000 00000000 01000000 yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: villain index)
// classic villain filler ____1000 00000000 00000000 01010000 yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: villain index)
//    team villain filler ____1000 00000000 00000000 01100000 yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: villain index)
//           other filler ____1000 00000000 00000000 00000000 00000000 00000000 xxxxxxxx (x: filler index)
//          all hero trap ____1001 00000000 00000000 10000000 yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: hero index)
//         hero hero trap ____1001 00000000 00000000 10010000 yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: hero index)
//      variant hero trap ____1001 00000000 zzzzzzzz 10100000 yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: hero index, z: variant index)
//       all villain trap ____1001 00000000 00000000 01000000 yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: villain index)
//   classic villain trap ____1001 00000000 00000000 01010000 yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: villain index)
//      team villain trap ____1001 00000000 00000000 01100000 yyyyyyyy yyyyyyyy xxxxxxxx (x: filler index, y: villain index)
//             other trap ____1001 00000000 00000000 00000000 00000000 00000000 xxxxxxxx (x: filler index)
//
// LOCATIONS                 52   48       40       32       24       16        8
//                        ____0000 00000000 00000000 00000000 00000000 00000000 00000000
//                villain ____0001 00000000 00000000 00000000 zzyyyyyy xxxxxxxx xxxxxxxx (x: villain index, y: check #, z: difficulty)
//           team villain ____0011 00000000 00000000 00000000 zzyyyyyy xxxxxxxx xxxxxxxx (x: team villain index, y: check #, z: difficulty)
//              gladiator ____0101 00000000 00000000 00000000 zzyyyyyy xxxxxxxx xxxxxxxx (x: gladiator index, y: check #, z: difficulty)
//    hero variant unlock ____0010 00000000 00000000 zzzzzzzz 00yyyyyy xxxxxxxx xxxxxxxx (x: hero index, y: check #, z: variant index)
// villain variant unlock ____0010 00000000 00000000 00000000 01yyyyyy xxxxxxxx xxxxxxxx (x: variant index, y: check #)
//            environment ____0100 00000000 00000000 00000000 00yyyyyy xxxxxxxx xxxxxxxx (x: environment index, y: check #)
//                   hero ____0010 00000000 00000000 00000000 10yyyyyy xxxxxxxx xxxxxxxx (x: hero index, y: check #)
//           hero variant ____0010 00000000 00000000 zzzzzzzz 10yyyyyy xxxxxxxx xxxxxxxx (x: hero index, y: check #, z: variant index)

use std::{fs::OpenOptions, io::Write};

use crate::{Data, EnumData, FillerData, FillerType, VariantData, VillainData, MAX_LOCATION_DENSITY};

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
        hero_location_id(&mut writer, &data.heroes, 0b0010);
        variant_location_id(&mut writer, data.hero_variants(), 0b0010);

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

fn hero_location_id<T>(writer: &mut T, data: &[EnumData], prefix: i64)
where
    T: Write,
{
    for data in data {
        for c in 0..MAX_LOCATION_DENSITY {
            let _ = write!(
                writer,
                "\"{} - Any Difficulty #{}\":{},",
                data.display_name,
                c + 1,
                (prefix << 48) | data.i as i64 | c << 16 | 0b10 << 22
            );
        }
    }
}

fn variant_location_id<'a, T>(writer: &mut T, data: impl Iterator<Item = &'a VariantData>, prefix: i64)
where
    T: Write,
{
    for data in data {
        for c in 0..MAX_LOCATION_DENSITY {
            let _ = write!(
                writer,
                "\"{} - Any Difficulty #{}\":{},",
                data.display_name,
                c + 1,
                (prefix << 48) | data.base_i as i64 | (data.i as i64) << 24 | c << 16 | 0b10 << 22
            );
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

fn push_filler<T>(writer: &mut T, data: &Data)
where
    T: Write,
{
    for filler in &data.filler {
        match filler.r#type {
            FillerType::Hero => handle_pos_neg(filler, |name, b| {
                let _ = write!(writer, "\"{}\":{},", name, 0b1000 << 48 | b | filler.i as i64 | 0b1000 << 28);
                for hero in &data.heroes {
                    let _ = write!(
                        writer,
                        "\"{} (Any {})\":{},",
                        name,
                        hero.display_name,
                        0b1000 << 48 | b | filler.i as i64 | 0b1001 << 28 | (hero.i as i64) << 8
                    );
                    let _ = write!(
                        writer,
                        "\"{} ({})\":{},",
                        name,
                        hero.display_name,
                        0b1000 << 48 | b | filler.i as i64 | 0b1010 << 28 | (hero.i as i64) << 8
                    );
                }
                for variant in data.hero_variants() {
                    let _ = write!(
                        writer,
                        "\"{} ({})\":{},",
                        name,
                        variant.display_name,
                        0b1000 << 48 | b | filler.i as i64 | 0b1010 << 28 | (variant.base_i as i64) << 8 | (variant.i as i64) << 32
                    );
                }
            }),
            FillerType::Villain => handle_pos_neg(filler, |name, b| {
                let _ = write!(writer, "\"{}\":{},", name, 0b1000 << 48 | b | filler.i as i64 | 0b0100 << 28);
                for villain in &data.villains {
                    let _ = write!(
                        writer,
                        "\"{} ({})\":{},",
                        name,
                        villain.display_name,
                        0b1000 << 48 | b | filler.i as i64 | 0b0101 << 28 | (villain.i as i64) << 8
                    );
                }
                for villain in &data.team_villains {
                    let _ = write!(
                        writer,
                        "\"{} ({})\":{},",
                        name,
                        villain.display_name,
                        0b1000 << 48 | b | filler.i as i64 | 0b0110 << 28 | (villain.i as i64) << 8
                    );
                }
            }),
            FillerType::Other => handle_pos_neg(filler, |name, b| {
                let _ = write!(writer, "\"{}\":{},", name, 0b1000 << 48 | b | filler.i as i64);
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
