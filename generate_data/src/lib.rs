mod enums;
mod filler;
mod group_data;
mod id;

use enums::{push_enum_defs, push_variant_defs};
use filler::push_filler;
use group_data::group_data;
use id::generate_id_py;
use proc_macro::TokenStream;

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
    i: usize,
}

struct VariantData {
    enum_name: String,
    display_name: String,
    base: String,
    unlock_desc: Option<String>,
    i: usize,
    base_i: usize,
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

    str.parse().unwrap()
}

impl Data {
    pub fn hero_variants(&self) -> impl Iterator<Item = &VariantData> {
        self.variants.iter().filter(|variant| variant.base != "Villain")
    }
}
