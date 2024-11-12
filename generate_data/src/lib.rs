mod data_py;
mod enums;
mod filler;
mod group_data;
mod id_py;
mod logic;

use data_py::generate_data_py;
use enums::{push_enum_defs, push_variant_defs, push_villain_defs};
use filler::push_filler;
use group_data::group_data;
use id_py::generate_id_py;
use logic::LogicTerm;
use proc_macro::TokenStream;

#[derive(Debug)]
struct Data {
    sources: Vec<SourceData>,
    villains: Vec<VillainData>,
    team_villains: Vec<VillainData>,
    heroes: Vec<EnumData>,
    contenders: Vec<EnumData>,
    environments: Vec<EnumData>,
    variants: Vec<VariantData>,
    filler: Vec<FillerData>,
}

#[derive(Debug)]
struct SourceData {
    enum_name: String,
    display_name: String,
    default: bool,
}

#[derive(Debug, Clone)]
struct VillainData {
    enum_name: String,
    display_name: String,
    source: String,
    i: usize,
    challenge: Option<(String, Vec<String>)>,
}

#[derive(Debug, Clone)]
struct EnumData {
    enum_name: String,
    display_name: String,
    source: String,
    i: usize,
}

#[derive(Debug, Clone)]
struct VariantData {
    enum_name: String,
    display_name: String,
    source: String,
    base: String,
    unlock_desc: Option<String>,
    unparsed_logic: Option<String>,
    logic: Option<Box<LogicTerm>>,
    i: usize,
    base_i: usize,
    is_villain: bool,
}

#[derive(Debug)]
struct FillerData {
    enum_name: String,
    display_name_pos: Option<String>,
    display_name_neg: Option<String>,
    damage_types: bool,
    r#type: FillerType,
    desc_pos: Option<String>,
    desc_neg: Option<String>,
    i: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FillerType {
    Hero,
    Villain,
    Other,
}

/// # Panics
///
/// Panics if the input file cannot be parsed into valid item data
#[proc_macro]
pub fn generate_data(_stream: TokenStream) -> TokenStream {
    let data = group_data();

    generate_id_py(&data);
    generate_data_py(&data);

    let mut str = String::new();

    push_villain_defs(&mut str, "Villain", &data.villains);
    push_villain_defs(&mut str, "TeamVillain", &data.team_villains);
    push_enum_defs(&mut str, "Hero", &data.heroes);
    push_enum_defs(&mut str, "Contender", &data.contenders);
    push_enum_defs(&mut str, "Environment", &data.environments);
    push_variant_defs(&mut str, &data.variants);
    push_filler(&mut str, &data);

    str.parse().unwrap()
}

impl Data {
    pub fn hero_variants(&self) -> impl Iterator<Item = &VariantData> {
        self.variants.iter().filter(|variant| !variant.is_villain)
    }

    pub fn villain_variants(&self) -> impl Iterator<Item = &VariantData> {
        self.variants.iter().filter(|variant| variant.is_villain)
    }
}
