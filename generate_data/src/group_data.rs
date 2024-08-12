use proc_macro::{Literal, TokenStream, TokenTree};

use crate::{Data, EnumData, FillerData, FillerType, VariantData};

pub fn group_data(stream: TokenStream) -> Data {
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
                    "Villain" => villains.push(EnumData {
                        enum_name: enum_name.to_string(),
                        display_name: trim_quotes(display_name),
                        i: villains.len(),
                    }),
                    "TeamVillain" => team_villains.push(EnumData {
                        enum_name: enum_name.to_string(),
                        display_name: trim_quotes(display_name),
                        i: team_villains.len(),
                    }),
                    "Hero" => heroes.push(EnumData {
                        enum_name: enum_name.to_string(),
                        display_name: trim_quotes(display_name),
                        i: heroes.len(),
                    }),
                    "Environment" => environments.push(EnumData {
                        enum_name: enum_name.to_string(),
                        display_name: trim_quotes(display_name),
                        i: environments.len(),
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
                        i: 0,
                        base_i: 0,
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
                        i: 0,
                        base_i: 0,
                    })
                }
                [TokenTree::Ident(enum_name), TokenTree::Ident(r#type), TokenTree::Ident(group), TokenTree::Literal(display_name_pos), TokenTree::Literal(display_name_neg), TokenTree::Literal(desc_pos), TokenTree::Literal(desc_neg)]
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
                        i: filler.len() as i64,
                    })
                }
                [TokenTree::Ident(enum_name), TokenTree::Ident(r#type), TokenTree::Ident(group), TokenTree::Ident(variant_type), TokenTree::Literal(display_name_pos), TokenTree::Literal(display_name_neg), TokenTree::Literal(desc_pos), TokenTree::Literal(desc_neg)]
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
                        i: filler.len() as i64,
                    })
                }
                invalid => panic!("Invalid token count: {:?}", invalid),
            }
        }
    }

    for i in 0..variants.len() {
        let new_i = variants.iter().take(i + 1).filter(|variant| variant.base == variants[i].base).count();
        variants[i].i = new_i;
        variants[i].base_i = heroes.iter().find(|hero| hero.enum_name == variants[i].base).map(|hero| hero.i).unwrap_or(0)
    }

    Data {
        villains,
        team_villains,
        heroes,
        environments,
        variants,
        filler,
    }
}

fn trim_quotes(lit: &Literal) -> String {
    let str = lit.to_string();
    str[1..(str.len() - 1)].to_string()
}
