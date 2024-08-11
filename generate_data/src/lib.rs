mod enums;
mod filler;
mod group_data;
mod id;

use enums::{push_enum_defs, push_variant_defs};
use filler::push_filler;
use group_data::group_data;
use id::generate_id_py;
use proc_macro::TokenStream;
use std::fmt::Write;

struct Data {
    villains: Vec<EnumData>,
    team_villains: Vec<EnumData>,
    heroes: Vec<EnumData>,
    environments: Vec<EnumData>,
    variants: Vec<VariantData>,
    filler: Vec<FillerData>,
}

struct EnumData {
    enum_name: String,
    display_name: String,
}

struct VariantData {
    enum_name: String,
    display_name: String,
    base: String,
    unlock_desc: Option<String>,
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
    i: i64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FillerType {
    Hero,
    Villain,
    Other,
}

macro_rules! get_from_str {
    ($vec: expr, $t:literal) => {
        $vec.iter().fold(String::new(), |mut str, data| {
            let _ = write!(str, "\"{}\" => Some(Item::{}({}::{})),", data.display_name, $t, $t, data.enum_name);
            str
        })
    };
}

/// # Panics
///
/// Panics if the input table cannot be parsed into valid item data
#[proc_macro]
pub fn generate_data(stream: TokenStream) -> TokenStream {
    let data = group_data(stream);
    generate_id_py(&data);

    let mut str = String::new();

    push_enum_defs(&mut str, "Villain", &data.villains);
    push_enum_defs(&mut str, "TeamVillain", &data.team_villains);
    push_enum_defs(&mut str, "Hero", &data.heroes);
    push_enum_defs(&mut str, "Environment", &data.environments);
    push_variant_defs(&mut str, &data.variants);
    push_filler(&mut str, &data);

    let villain_from_str = get_from_str!(data.villains, "Villain");
    let team_villain_from_str = get_from_str!(data.team_villains, "TeamVillain");
    let hero_from_str = get_from_str!(data.heroes, "Hero");
    let environment_from_str = get_from_str!(data.environments, "Environment");
    let variant_from_str = get_from_str!(data.variants, "Variant");

    let _ = write!(
        str,
        "
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
        data.villains
            .iter()
            .filter(|EnumData { enum_name, display_name: _ }| enum_name != "SpiteAgentOfGloom" && enum_name != "SkinwalkerGloomweaver")
            .fold(String::new(), |mut str, EnumData { enum_name, display_name }| {
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
        data.team_villains.iter().fold(String::new(), |mut str, EnumData { enum_name, display_name }| {
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
        data.environments.iter().fold(String::new(), |mut str, EnumData { enum_name, display_name }| {
            let _ = write!(str, "\"{display_name} - Any Difficulty\" => Some(Location::Environment(Environment::{enum_name})),");
            str
        }),
        data.variants
            .iter()
            .filter(
                |VariantData {
                     enum_name: _,
                     base: _,
                     unlock_desc,
                     display_name: _,
                     i: _,
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
                     i: _,
                 }| {
                    let _ = write!(str, "\"{display_name} - Unlock\" => Some(Location::Variant(Variant::{enum_name})),");
                    str
                }
            )
    );

    str.parse().unwrap()
}

impl Data {
    pub fn hero_variants(&self) -> impl Iterator<Item = &VariantData> {
        self.variants.iter().filter(|variant| variant.base != "Villain")
    }
}
