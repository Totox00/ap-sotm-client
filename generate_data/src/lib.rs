use proc_macro::Literal;
use proc_macro::TokenStream;
use proc_macro::TokenTree;
use std::fmt::Write;

struct VillainData {
    enum_name: String,
    display_name: String,
}

struct TeamVillainData {
    enum_name: String,
    display_name: String,
}

struct HeroData {
    enum_name: String,
    display_name: String,
}

struct EnvironmentData {
    enum_name: String,
    display_name: String,
}

struct VariantData {
    enum_name: String,
    display_name: String,
    base: String,
    unlock_desc: Option<String>,
}

struct VariantCount {
    enum_name: String,
    base: String,
    i: usize,
}

struct FillerData {
    enum_name: String,
    display_name_pos: String,
    display_name_neg: String,
    damage_types: bool,
    r#type: FillerType,
    desc_pos: String,
    desc_neg: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FillerType {
    Hero,
    Villain,
    Other,
}

macro_rules! get_enum_names {
    ($vec: ident) => {
        $vec.iter().fold(String::new(), |mut str, data| {
            let _ = write!(str, "{},", data.enum_name);
            str
        })
    };
}

macro_rules! get_as_str {
    ($vec: ident, $t:literal) => {
        $vec.iter().fold(String::new(), |mut str, data| {
            let _ = write!(str, "{}::{} => \"{}\",", $t, data.enum_name, data.display_name);
            str
        })
    };
}

macro_rules! get_from_str {
    ($vec: ident, $t:literal) => {
        $vec.iter().fold(String::new(), |mut str, data| {
            let _ = write!(str, "\"{}\" => Some(Item::{}({}::{})),", data.display_name, $t, $t, data.enum_name);
            str
        })
    };
}

/// # Panics
///
/// Panics if the input table cannot be parsed into valid item data
#[allow(clippy::too_many_lines)]
#[proc_macro]
pub fn generate_data(stream: TokenStream) -> TokenStream {
    let (villains, team_villains, heroes, environments, variants, filler, variant_counts) = group_data(stream);

    let villain_enum_names = get_enum_names!(villains);
    let team_villain_enum_names = get_enum_names!(team_villains);
    let hero_enum_names = get_enum_names!(heroes);
    let environment_enum_names = get_enum_names!(environments);
    let variant_enum_names = get_enum_names!(variants);
    let filler_enum_names = filler.iter().fold(
        String::new(),
        |mut str,
         FillerData {
             enum_name,
             damage_types,
             r#type,
             display_name_pos: _,
             display_name_neg: _,
             desc_pos: _,
             desc_neg: _,
         }| {
            let _ = if *damage_types {
                match r#type {
                    FillerType::Hero => write!(str, "{enum_name}((HeroLike, DamageType)),"),
                    FillerType::Villain => write!(str, "{enum_name}((VillainLike, DamageType)),"),
                    FillerType::Other => write!(str, "{enum_name}(DamageType),"),
                }
            } else {
                match r#type {
                    FillerType::Hero => write!(str, "{enum_name}(HeroLike),"),
                    FillerType::Villain => write!(str, "{enum_name}(VillainLike),"),
                    FillerType::Other => write!(str, "{enum_name},"),
                }
            };

            str
        },
    );

    let variant_variant_count = variants.len() + 1;
    let team_villain_variant_count = team_villains.len();
    let villain_variant_count = villains.len();
    let hero_variant_count = heroes.len();
    let environment_variant_count = environments.len();

    let variant_as_i = variant_counts.iter().fold(String::new(), |mut str, VariantCount { enum_name, base: _, i }| {
        let _ = write!(str, "Variant::{enum_name} => {i},");
        str
    });

    let variant_as_normal = variants.iter().fold(
        String::new(),
        |mut str,
         VariantData {
             enum_name,
             base,
             unlock_desc: _,
             display_name: _,
         }| {
            let _ = write!(str, "Variant::{enum_name} => {},", if base == "Villain" { String::from("None") } else { format!("Some(Hero::{base})") });
            str
        },
    );

    let variant_from_hero = variant_counts
        .iter()
        .filter(|VariantCount { enum_name: _, base, i: _ }| base.as_str() != "Villain")
        .fold(String::new(), |mut str, VariantCount { enum_name, base, i }| {
            let _ = write!(str, "(Hero::{base}, {i}) => Some(Variant::{enum_name}),");
            str
        });

    let variant_as_desc = variants.iter().fold(
        String::new(),
        |mut str,
         VariantData {
             enum_name,
             display_name: _,
             base: _,
             unlock_desc,
         }| {
            if let Some(desc) = unlock_desc {
                let _ = write!(str, "Variant::{enum_name} => \"{desc}\",");
            }
            str
        },
    );

    let villain_as_str = get_as_str!(villains, "Villain");
    let team_villain_as_str = get_as_str!(team_villains, "TeamVillain");
    let hero_as_str = get_as_str!(heroes, "Hero");
    let environment_as_str = get_as_str!(environments, "Environment");
    let variant_as_str = get_as_str!(variants, "Variant");
    let filler_to_string = filler_to_string(&filler);

    let filler_as_desc = filler.iter().fold(
        String::new(),
        |mut str,
         FillerData {
             enum_name,
             damage_types,
             r#type,
             display_name_pos: _,
             display_name_neg: _,
             desc_pos,
             desc_neg,
         }| {
            if *r#type == FillerType::Other && !*damage_types {
                let _ = write!(str, "Filler::{enum_name} => if count > 0 {{\"{desc_pos}\"}} else {{\"{desc_neg}\"}},");
            } else {
                let _ = write!(str, "Filler::{enum_name}(_) => if count > 0 {{\"{desc_pos}\"}} else {{\"{desc_neg}\"}},");
            }

            str
        },
    );

    let villain_from_str = get_from_str!(villains, "Villain");
    let team_villain_from_str = get_from_str!(team_villains, "TeamVillain");
    let hero_from_str = get_from_str!(heroes, "Hero");
    let environment_from_str = get_from_str!(environments, "Environment");
    let variant_from_str = get_from_str!(variants, "Variant");

    format!(
        "
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, FromPrimitive, Hash)]
            pub enum Villain {{{villain_enum_names}}}
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, FromPrimitive, Hash)]
            pub enum TeamVillain {{{team_villain_enum_names}}}
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, FromPrimitive, Hash)]
            pub enum Hero {{{hero_enum_names}}}
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, FromPrimitive, Hash)]
            pub enum Environment {{{environment_enum_names}}}
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, FromPrimitive, Hash)]
            pub enum Variant {{Base,{variant_enum_names}}}
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub enum Filler {{{filler_enum_names}}}

            impl Variant {{
                pub const fn variant_count() -> usize {{{variant_variant_count}}}
                pub fn as_i(&self) -> u8 {{match self {{Variant::Base => 0,{variant_as_i}}}}}
                pub fn as_normal(&self) -> Option<Hero> {{match self {{Variant::Base => None,{variant_as_normal}}}}}
                pub fn from_hero(hero: Hero, i: u32) -> Option<Variant> {{match (hero, i) {{{variant_from_hero}_ => None}}}}
                pub fn as_str(&self) -> &str {{match self {{Variant::Base => \"Base\",{variant_as_str}}}}}
                pub fn as_desc(&self) -> &str {{match self {{{variant_as_desc}_ => \"\",}}}}
            }}

            impl Hero {{
                pub const fn variant_count() -> usize {{{hero_variant_count}}}
                pub fn as_str(&self) -> &str {{match self {{{hero_as_str}}}}}
            }}

            impl Villain {{
                pub const fn variant_count() -> usize {{{villain_variant_count}}}
                pub fn as_str(&self) -> &str {{match self {{{villain_as_str}}}}}
            }}

            impl TeamVillain {{
                pub const fn variant_count() -> usize {{{team_villain_variant_count}}}
                pub fn as_str(&self) -> &str {{match self {{{team_villain_as_str}}}}}
            }}

            impl Environment {{
                pub const fn variant_count() -> usize {{{environment_variant_count}}}
                pub fn as_str(&self) -> &str {{match self {{{environment_as_str}}}}}
            }}

            impl Filler {{
                pub fn to_string(&self, count: i8) -> String {{match self {{{filler_to_string}}}}}
                pub fn as_desc(&self, count: i8) -> &str {{match self {{{filler_as_desc}}}}}
            }}

            impl Item {{
                pub fn from_str(str: &str) -> Option<Item> {{
                    match str {{
                        {villain_from_str}{team_villain_from_str}{hero_from_str}{environment_from_str}{variant_from_str}
                        \"Scion of Oblivaeon\" => Some(Item::Scion),
                        _ => None
                    }}
                }}
            }}

            impl Location {{
                pub fn from_str(str: &str) -> Option<(Location, u8)> {{
                    match &str[..str.len() - 3] {{
                        {}{}{}{}
                        \"Spite: Agent of Gloom - Normal\" => Some(Location::Villain((Villain::SpiteAgentOfGloom, 0))),
                        \"Spite: Agent of Gloom - Advanced\" => Some(Location::Villain((Villain::SpiteAgentOfGloom, 1))),
                        \"Spite: Agent of Gloom and Skinwalker Gloomweaver - Challenge\" => Some(Location::Villain((Villain::SpiteAgentOfGloom, 2))),
                        \"Spite: Agent of Gloom and Skinwalker Gloomweaver - Ultimate\" => Some(Location::Villain((Villain::SpiteAgentOfGloom, 3))),
                        \"Skinwalker Gloomweaver - Normal\" => Some(Location::Villain((Villain::SkinwalkerGloomweaver, 0))),
                        \"Skinwalker Gloomweaver - Advanced\" => Some(Location::Villain((Villain::SkinwalkerGloomweaver, 1))),            
                        _ => None
                    }}.map(|l| (l, str[str.len() - 1..str.len()].parse().unwrap()))
                }}
            }}
        ",
        villains
            .iter()
            .filter(|VillainData { enum_name, display_name: _ }| enum_name != "SpiteAgentOfGloom" && enum_name != "SkinwalkerGloomweaver")
            .fold(String::new(), |mut str, VillainData { enum_name, display_name }| {
                let _ = write!(
                    str,
                    "
                        \"{display_name} - Normal\" => Some(Location::Villain((Villain::{enum_name}, 0))),
                        \"{display_name} - Advanced\" => Some(Location::Villain((Villain::{enum_name}, 1))),
                        \"{display_name} - Challenge\" => Some(Location::Villain((Villain::{enum_name}, 2))),
                        \"{display_name} - Ultimate\" => Some(Location::Villain((Villain::{enum_name}, 3))),
                    "
                );
                str
            }),
        team_villains.iter().fold(String::new(), |mut str, TeamVillainData { enum_name, display_name }| {
            let _ = write!(
                str,
                "
                    \"{display_name} - Normal\" => Some(Location::TeamVillain((TeamVillain::{enum_name}, 0))),
                    \"{display_name} - Advanced\" => Some(Location::TeamVillain((TeamVillain::{enum_name}, 1))),
                    \"{display_name} - Challenge\" => Some(Location::TeamVillain((TeamVillain::{enum_name}, 2))),
                    \"{display_name} - Ultimate\" => Some(Location::TeamVillain((TeamVillain::{enum_name}, 3))),
                "
            );
            str
        }),
        environments.iter().fold(String::new(), |mut str, EnvironmentData { enum_name, display_name }| {
            let _ = write!(str, "\"{display_name} - Any Difficulty\" => Some(Location::Environment(Environment::{enum_name})),");
            str
        }),
        variants
            .iter()
            .filter(
                |VariantData {
                     enum_name: _,
                     base: _,
                     unlock_desc,
                     display_name: _,
                 }| unlock_desc.is_some()
            )
            .fold(
                String::new(),
                |mut str,
                 VariantData {
                     enum_name,
                     base: _,
                     unlock_desc: _,
                     display_name,
                 }| {
                    let _ = write!(str, "\"{display_name} - Unlock\" => Some(Location::Variant(Variant::{enum_name})),");
                    str
                }
            )
    )
    .parse()
    .unwrap()
}

type Groups = (
    Vec<VillainData>,
    Vec<TeamVillainData>,
    Vec<HeroData>,
    Vec<EnvironmentData>,
    Vec<VariantData>,
    Vec<FillerData>,
    Vec<VariantCount>,
);

fn group_data(stream: TokenStream) -> Groups {
    let mut villains = vec![];
    let mut team_villains = vec![];
    let mut heroes = vec![];
    let mut environments = vec![];
    let mut variants = vec![];
    let mut filler = vec![];

    for token in stream {
        if let TokenTree::Group(group) = token {
            let tokens: Vec<_> = group.stream().into_iter().filter(|token| !matches!(token, TokenTree::Punct(_))).collect();
            match &tokens[..] {
                [TokenTree::Ident(enum_name), TokenTree::Ident(r#type), TokenTree::Literal(display_name)] => match r#type.to_string().as_str() {
                    "Villain" => villains.push(VillainData {
                        enum_name: enum_name.to_string(),
                        display_name: trim_quotes(display_name),
                    }),
                    "TeamVillain" => team_villains.push(TeamVillainData {
                        enum_name: enum_name.to_string(),
                        display_name: trim_quotes(display_name),
                    }),
                    "Hero" => heroes.push(HeroData {
                        enum_name: enum_name.to_string(),
                        display_name: trim_quotes(display_name),
                    }),
                    "Environment" => environments.push(EnvironmentData {
                        enum_name: enum_name.to_string(),
                        display_name: trim_quotes(display_name),
                    }),
                    "Variant" => panic!("Variant is missing a base"),
                    _ => panic!("Invalid type"),
                },
                [TokenTree::Ident(enum_name), TokenTree::Ident(r#type), TokenTree::Ident(base), TokenTree::Literal(display_name)] if r#type.to_string().as_str() == "Variant" => {
                    variants.push(VariantData {
                        enum_name: enum_name.to_string(),
                        base: base.to_string(),
                        unlock_desc: None,
                        display_name: trim_quotes(display_name),
                    })
                }
                [TokenTree::Ident(enum_name), TokenTree::Ident(r#type), TokenTree::Ident(base), TokenTree::Literal(display_name), TokenTree::Literal(unlock_desc)]
                    if r#type.to_string().as_str() == "Variant" =>
                {
                    variants.push(VariantData {
                        enum_name: enum_name.to_string(),
                        base: base.to_string(),
                        unlock_desc: Some(trim_quotes(unlock_desc)),
                        display_name: trim_quotes(display_name),
                    })
                }
                [TokenTree::Ident(enum_name), TokenTree::Ident(r#type), TokenTree::Ident(group), TokenTree::Literal(display_name_pos), TokenTree::Literal(desc_pos), TokenTree::Literal(display_name_neg), TokenTree::Literal(desc_neg)]
                    if r#type.to_string().as_str() == "Filler" =>
                {
                    filler.push(FillerData {
                        enum_name: enum_name.to_string(),
                        display_name_pos: trim_quotes(display_name_pos),
                        display_name_neg: trim_quotes(display_name_neg),
                        damage_types: false,
                        r#type: match group.to_string().as_str() {
                            "Hero" => FillerType::Hero,
                            "Villain" => FillerType::Villain,
                            "Other" => FillerType::Other,
                            _ => panic!("Invalid filler type"),
                        },
                        desc_pos: trim_quotes(desc_pos),
                        desc_neg: trim_quotes(desc_neg),
                    })
                }
                [TokenTree::Ident(enum_name), TokenTree::Ident(r#type), TokenTree::Ident(group), TokenTree::Ident(variant_type), TokenTree::Literal(display_name_pos), TokenTree::Literal(desc_pos), TokenTree::Literal(display_name_neg), TokenTree::Literal(desc_neg)]
                    if r#type.to_string().as_str() == "Filler" && variant_type.to_string().as_str() == "DamageType" =>
                {
                    filler.push(FillerData {
                        enum_name: enum_name.to_string(),
                        display_name_pos: trim_quotes(display_name_pos),
                        display_name_neg: trim_quotes(display_name_neg),
                        damage_types: true,
                        r#type: match group.to_string().as_str() {
                            "Hero" => FillerType::Hero,
                            "Villain" => FillerType::Villain,
                            "Other" => FillerType::Other,
                            _ => panic!("Invalid filler type"),
                        },
                        desc_pos: trim_quotes(desc_pos),
                        desc_neg: trim_quotes(desc_neg),
                    })
                }
                invalid => panic!("Invalid token count: {:?}", invalid),
            }
        }
    }

    let mut variant_counts = vec![];

    for (
        VariantData {
            enum_name,
            base,
            unlock_desc: _,
            display_name: _,
        },
        i,
    ) in variants.iter().zip(1..)
    {
        if base == "Villain" {
            variant_counts.push(VariantCount {
                enum_name: enum_name.to_owned(),
                base: base.to_owned(),
                i: 0,
            });
        } else {
            variant_counts.push(VariantCount {
                enum_name: enum_name.to_owned(),
                base: base.to_owned(),
                i: variants
                    .iter()
                    .take(i)
                    .filter(
                        |VariantData {
                             enum_name: _,
                             base: b,
                             unlock_desc: _,
                             display_name: _,
                         }| b == base,
                    )
                    .count(),
            });
        }
    }

    (villains, team_villains, heroes, environments, variants, filler, variant_counts)
}

fn trim_quotes(lit: &Literal) -> String {
    let str = lit.to_string();
    str[1..(str.len() - 1)].to_string()
}

fn filler_to_string(filler: &[FillerData]) -> String {
    filler.iter().fold(
        String::new(),
        |mut str,
         FillerData {
             enum_name,
             damage_types,
             r#type,
             display_name_pos,
             display_name_neg,
             desc_pos: _,
             desc_neg: _,
         }| {
            if *damage_types {
                match r#type {
                    FillerType::Hero | FillerType::Villain => {
                        let _ = write!(
                            str,
                            "Filler::{enum_name}((variant, damage_type)) => if count > 0 {{format!(\"{}{{}}\", damage_type.as_str(), count.abs(), variant.as_str())}} else {{format!(\"{}{{}}\", damage_type.as_str(), count.abs(), variant.as_str())}},",
                            display_name_pos.replace("[COUNT]", "{}").replace("[TYPE]", "{}"),
                            display_name_neg.replace("[COUNT]", "{}").replace("[TYPE]", "{}")
                        );
                    }
                    FillerType::Other => {
                        let _ = write!(
                            str,
                            "Filler::{enum_name}(damage_type) => if count > 0 {{format!(\"{}\", damage_type.as_str(), count.abs())}} else {{format!(\"{}\", damage_type.as_str(), count.abs())}},",
                            display_name_pos.replace("[COUNT]", "{}").replace("[TYPE]", "{}"),
                            display_name_neg.replace("[COUNT]", "{}").replace("[TYPE]", "{}")
                        );
                    }
                }
            } else {
                match r#type {
                    FillerType::Hero | FillerType::Villain => {
                        let _ = write!(
                            str,
                            "Filler::{enum_name}(variant) => if count > 0 {{format!(\"{}{{}}\", count.abs(), variant.as_str())}} else {{format!(\"{}{{}}\", count.abs(), variant.as_str())}},",
                            display_name_pos.replace("[COUNT]", "{}"),
                            display_name_neg.replace("[COUNT]", "{}")
                        );
                    }
                    FillerType::Other => {
                        let _ = write!(str, "Filler::{enum_name} => if count > 0 {{format!(\"{}\", count.abs())}} else {{format!(\"{}\", count.abs())}},", 
                        display_name_pos.replace("[COUNT]", "{}"),
                        display_name_neg.replace("[COUNT]", "{}"));
                    }
                }
            };

            str
        },
    )
}
