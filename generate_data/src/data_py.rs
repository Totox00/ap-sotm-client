use crate::Data;
use std::{fs::OpenOptions, io::Write};

const PREFIX: &str = include_str!("prefix.py");
const SUFFIX: &str = include_str!("suffix.py");

pub fn generate_data_py(data: &Data) {
    if let Ok(mut writer) = OpenOptions::new().write(true).create(true).truncate(true).open("Data.py") {
        let _ = write!(writer, "{PREFIX}\nclass SotmSource(IntEnum):");

        let _ = write!(
            writer,
            "{}",
            data.sources.iter().enumerate().map(|(i, source)| format!("{}={i}", source.enum_name)).collect::<Vec<_>>().join(";")
        );

        let _ = write!(writer, "\nsources={{");

        for source in &data.sources {
            let _ = write!(
                writer,
                "SotmSource.{}:{{\"name\":\"{}\",\"default\":{}}},",
                source.enum_name,
                source.display_name,
                if source.default { "True" } else { "False" }
            );
        }

        let _ = write!(writer, "}}\nclass SotmData(NamedTuple):name:str;sources:list[SotmSource];category:SotmCategory;no_challenge:bool;base:Optional[str]=None;rule:Optional[Callable[[CollectionState|SotmState,int],bool]]=None;dependencies:Optional[list[str]]=None\ndata=[");

        for villain in &data.villains {
            if let Some(variant) = data.villain_variants().find(|variant| villain.enum_name == variant.enum_name) {
                if let Some(logic) = &variant.logic {
                    let base = data
                        .villains
                        .iter()
                        .find(|villain| villain.enum_name == variant.base)
                        .unwrap_or_else(|| panic!("Failed to find base for variant {}", variant.enum_name));

                    let _ = write!(
                        writer,
                        "SotmData(\"{}\",[{}],SotmCategory.VillainVariant,{},\"{}\",lambda state,player:{},[{}]),",
                        villain.display_name,
                        map_source(&villain.source),
                        if villain.challenge.is_none() { "True" } else { "False" },
                        base.display_name,
                        logic.as_py_expr(),
                        logic.as_dependencies(data).iter().map(|dependency| format!("\"{dependency}\"")).collect::<Vec<_>>().join(",")
                    );
                } else {
                    let _ = write!(
                        writer,
                        "SotmData(\"{}\",[{}],SotmCategory.Villain,{}),",
                        villain.display_name,
                        map_source(&villain.source),
                        if villain.challenge.is_none() { "True" } else { "False" }
                    );
                }
            } else {
                let _ = write!(
                    writer,
                    "SotmData(\"{}\",[{}],SotmCategory.Villain,{}),",
                    villain.display_name,
                    map_source(&villain.source),
                    if villain.challenge.is_none() { "True" } else { "False" }
                );
            }
        }

        for team_villain in &data.team_villains {
            let _ = write!(
                writer,
                "SotmData(\"{}\",[{}],SotmCategory.TeamVillain,{}),",
                team_villain.display_name,
                map_source(&team_villain.source),
                if team_villain.challenge.is_none() { "True" } else { "False" }
            );
        }

        for gladiator in &data.gladiators {
            let _ = write!(
                writer,
                "SotmData(\"{}\",[{}],SotmCategory.Gladiator,{}),",
                gladiator.display_name,
                map_source(&gladiator.source),
                if gladiator.challenge.is_none() { "True" } else { "False" }
            );
        }

        for hero in &data.heroes {
            let _ = write!(writer, "SotmData(\"{}\",[{}],SotmCategory.Hero,False),", hero.display_name, map_source(&hero.source));
        }

        for contender in &data.contenders {
            let _ = write!(writer, "SotmData(\"{}\",[{}],SotmCategory.Contender,False),", contender.display_name, map_source(&contender.source));
        }

        for environment in &data.environments {
            let _ = write!(
                writer,
                "SotmData(\"{}\",[{}],SotmCategory.Environment,False),",
                environment.display_name,
                map_source(&environment.source)
            );
        }

        for variant in data.hero_variants() {
            let base = data
                .heroes
                .iter()
                .find(|hero| hero.enum_name == variant.base)
                .unwrap_or_else(|| panic!("Failed to find base for variant {}", variant.enum_name));
            if let Some(logic) = &variant.logic {
                let _ = write!(
                    writer,
                    "SotmData(\"{}\",[{}],SotmCategory.Variant,False,\"{}\",lambda state,player:{},[{}]),",
                    variant.display_name,
                    map_source(&variant.source),
                    base.display_name,
                    logic.as_py_expr(),
                    logic.as_dependencies(data).iter().map(|dependency| format!("\"{dependency}\"")).collect::<Vec<_>>().join(",")
                );
            } else {
                let _ = write!(
                    writer,
                    "SotmData(\"{}\",[{}],SotmCategory.Variant,False,\"{}\"),",
                    variant.display_name,
                    map_source(&variant.source),
                    base.display_name
                );
            }
        }

        let _ = write!(writer, "]\nfiller=[");

        for filler in &data.filler {
            let _ = write!(
                writer,
                "FillerData(\"{}\",FillerType.{}{}{}),",
                filler.enum_name,
                match filler.r#type {
                    crate::FillerType::Hero => "Hero",
                    crate::FillerType::Villain => "Villain",
                    crate::FillerType::Other => "Other",
                },
                if let Some(name_pos) = &filler.display_name_pos {
                    format!(",name_pos=\"{}\"", name_pos.replace("[COUNT]", "1"))
                } else {
                    String::new()
                },
                if let Some(name_neg) = &filler.display_name_neg {
                    format!(",name_neg=\"{}\"", name_neg.replace("[COUNT]", "1"))
                } else {
                    String::new()
                },
            );
        }

        let _ = write!(writer, "FillerData(\"Scion\", FillerType.Other, name_pos=\"Scion of Oblivaeon\")]\n{SUFFIX}");
    }
}

fn map_source(source: &str) -> String {
    if source.is_empty() {
        String::new()
    } else {
        source.split(' ').map(|source| format!("SotmSource.{source}")).collect::<Vec<_>>().join(",")
    }
}
