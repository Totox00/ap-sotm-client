use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{Data, EnumData, FillerData, FillerType, SourceData, VariantData};

#[derive(Default)]
struct Fields {
    data_type: DataType,
    enum_name: Option<String>,
    display_name: Option<String>,
    source: Option<String>,
    default: Option<bool>,
    no_challenge: Option<bool>,
    base: Option<String>,
    unlock_desc: Option<String>,
    unlock_logic: Option<String>,
    display_name_pos: Option<String>,
    display_name_neg: Option<String>,
    damage_types: Option<bool>,
    r#type: Option<FillerType>,
    desc_pos: Option<String>,
    desc_neg: Option<String>,
}

enum DataType {
    None,
    Source,
    Villain,
    TeamVillain,
    Hero,
    Contender,
    Environment,
    Variant,
    Filler,
}

macro_rules! push_current {
    ($new_type: expr, $value: ident, $current: ident, $sources: ident, $villains: ident, $team_villains: ident, $heroes: ident, $contenders: ident, $environments: ident, $variants: ident, $filler: ident) => {{
        match $current.data_type {
            DataType::None => (),
            DataType::Source => $sources.push(SourceData {
                enum_name: $current.enum_name.expect("Villains must have enum_name"),
                display_name: $current.display_name.expect("Villains must have display_name"),
                default: $current.default.unwrap_or(false),
            }),
            DataType::Villain => $villains.push(EnumData {
                enum_name: $current.enum_name.expect("Villains must have enum_name"),
                display_name: $current.display_name.expect("Villains must have display_name"),
                source: $current.source.unwrap_or(String::new()),
                i: $villains.len(),
                no_challenge: $current.no_challenge.unwrap_or(false),
            }),
            DataType::TeamVillain => $team_villains.push(EnumData {
                enum_name: $current.enum_name.expect("Team villains must have enum_name"),
                display_name: $current.display_name.expect("Team villains must have display_name"),
                source: $current.source.unwrap_or(String::new()),
                i: $team_villains.len(),
                no_challenge: $current.no_challenge.unwrap_or(false),
            }),
            DataType::Hero => $heroes.push(EnumData {
                enum_name: $current.enum_name.expect("Heroes must have enum_name"),
                display_name: $current.display_name.expect("Heroes must have display_name"),
                source: $current.source.unwrap_or(String::new()),
                i: $heroes.len(),
                no_challenge: false,
            }),
            DataType::Contender => $contenders.push(EnumData {
                enum_name: $current.enum_name.expect("Contenders must have enum_name"),
                display_name: $current.display_name.expect("Contenders must have display_name"),
                source: $current.source.unwrap_or(String::new()),
                i: $contenders.len(),
                no_challenge: false,
            }),
            DataType::Environment => $environments.push(EnumData {
                enum_name: $current.enum_name.expect("Environments must have enum_name"),
                display_name: $current.display_name.expect("Environments must have display_name"),
                source: $current.source.unwrap_or(String::new()),
                i: $environments.len(),
                no_challenge: false,
            }),
            DataType::Variant => {
                let enum_name = $current.enum_name.expect("Variants must have enum_name");
                let display_name = $current.display_name.expect("Variants must have display_name");
                let source = $current.source.unwrap_or(String::new());
                let base = $current.base.expect("Variants must have base");
                let is_villain = $villains.iter().any(|v| v.enum_name == base);

                if is_villain {
                    $villains.push(EnumData {
                        enum_name: enum_name.clone(),
                        display_name: display_name.clone(),
                        source: source.clone(),
                        i: $villains.len(),
                        no_challenge: $current.no_challenge.unwrap_or(false),
                    })
                }

                $variants.push(VariantData {
                    enum_name,
                    display_name,
                    source,
                    base,
                    unlock_desc: $current.unlock_desc,
                    unparsed_logic: $current.unlock_logic,
                    logic: None,
                    i: 0,
                    base_i: 0,
                    is_villain,
                });
            }
            DataType::Filler => $filler.push(FillerData {
                enum_name: $current.enum_name.expect("Filler must have enum_name"),
                display_name_pos: $current.display_name_pos,
                display_name_neg: $current.display_name_neg,
                damage_types: $current.damage_types.unwrap_or(false),
                r#type: $current.r#type.expect("Filler must have type"),
                desc_pos: $current.desc_pos,
                desc_neg: $current.desc_neg,
                i: $filler.len(),
            }),
        };
        $current = Fields::default();
        $current.data_type = $new_type;
        $current.enum_name = Some($value.to_owned());
    }};
}

pub fn group_data() -> Data {
    let mut sources = vec![];
    let mut villains = vec![];
    let mut team_villains = vec![];
    let mut heroes = vec![];
    let mut contenders = vec![];
    let mut environments = vec![];
    let mut variants = vec![];
    let mut filler = vec![];

    let mut current = Fields::default();

    for (i, line) in BufReader::new(File::open(Path::new(file!()).parent().unwrap().join("data")).expect("Failed to open file"))
        .lines()
        .enumerate()
        .map(|(i, line)| (i, line.unwrap_or_else(|_| panic!("Failed to read line {i}"))))
    {
        if let Some((field, value)) = line.split_once(' ') {
            match field {
                "source" => push_current!(DataType::Source, value, current, sources, villains, team_villains, heroes, contenders, environments, variants, filler),
                "villain" => push_current!(DataType::Villain, value, current, sources, villains, team_villains, heroes, contenders, environments, variants, filler),
                "teamvillain" => push_current!(DataType::TeamVillain, value, current, sources, villains, team_villains, heroes, contenders, environments, variants, filler),
                "hero" => push_current!(DataType::Hero, value, current, sources, villains, team_villains, heroes, contenders, environments, variants, filler),
                "environment" => push_current!(DataType::Environment, value, current, sources, villains, team_villains, heroes, contenders, environments, variants, filler),
                "variant" => push_current!(DataType::Variant, value, current, sources, villains, team_villains, heroes, contenders, environments, variants, filler),
                "filler" => push_current!(DataType::Filler, value, current, sources, villains, team_villains, heroes, contenders, environments, variants, filler),
                "contender" => push_current!(DataType::Contender, value, current, sources, villains, team_villains, heroes, contenders, environments, variants, filler),
                "name" => current.display_name = Some(value.to_owned().escape_debug().to_string()),
                "from" => current.source = Some(value.to_owned()),
                "base" => current.base = Some(value.to_owned().escape_debug().to_string()),
                "unlock" => current.unlock_desc = Some(value.to_owned().escape_debug().to_string()),
                "requires" => current.unlock_logic = Some(value.to_owned()),
                "type" => {
                    current.r#type = Some(match value {
                        "Hero" => FillerType::Hero,
                        "Villain" => FillerType::Villain,
                        "Other" => FillerType::Other,
                        _ => panic!("Invalid filler type"),
                    })
                }
                "posname" => current.display_name_pos = Some(value.to_owned().escape_debug().to_string()),
                "negname" => current.display_name_neg = Some(value.to_owned().escape_debug().to_string()),
                "posdesc" => current.desc_pos = Some(value.to_owned().escape_debug().to_string()),
                "negdesc" => current.desc_neg = Some(value.to_owned().escape_debug().to_string()),
                _ => panic!("Unrecognised field {field} at line {i}"),
            }
        } else if !line.is_empty() {
            match line.as_str() {
                "damagetypes" => current.damage_types = Some(true),
                "default" => current.default = Some(true),
                "nochallenge" => current.no_challenge = Some(true),
                _ => panic!("Unrecognised bool field {line} at line {i}"),
            }
        }
    }

    let mut villain_variant_i = 0;
    for i in 0..variants.len() {
        if variants[i].is_villain {
            variants[i].i = villain_variant_i;
            villain_variant_i += 1;
        } else {
            let new_i = variants.iter().take(i + 1).filter(|variant| variant.base == variants[i].base).count();
            variants[i].i = new_i;
            variants[i].base_i = heroes.iter().find(|hero| hero.enum_name == variants[i].base).map(|hero| hero.i).unwrap_or(0)
        }
    }

    let mut data = Data {
        sources,
        villains,
        team_villains,
        heroes,
        contenders,
        environments,
        variants,
        filler,
    };

    for i in 0..data.variants.len() {
        if let Some(unparsed_logic) = &data.variants[i].unparsed_logic {
            data.variants[i].logic = Some(Box::new(data.parse_logic(unparsed_logic)));
        }
    }

    data
}

impl Default for DataType {
    fn default() -> Self {
        Self::None
    }
}
