use proc_macro::TokenStream;
use proc_macro::TokenTree;

#[proc_macro]
pub fn generate_data(stream: TokenStream) -> TokenStream {
    let mut villains = vec![];
    let mut team_villains = vec![];
    let mut heroes = vec![];
    let mut environments = vec![];
    let mut variants = vec![];

    for token in stream {
        if let TokenTree::Group(group) = token {
            let tokens: Vec<_> = group.stream().into_iter().filter(|token| !matches!(token, TokenTree::Punct(_))).collect();
            match tokens.len() {
                3 => {
                    if let (TokenTree::Ident(n), TokenTree::Ident(t), TokenTree::Literal(s)) = (&tokens[0], &tokens[1], &tokens[2]) {
                        match t.to_string().as_str() {
                            "Villain" => villains.push((n.to_string(), s.to_string(), s.to_string())),
                            "TeamVillain" => team_villains.push((n.to_string(), s.to_string(), s.to_string())),
                            "Hero" => heroes.push((n.to_string(), s.to_string(), s.to_string())),
                            "Environment" => environments.push((n.to_string(), s.to_string(), s.to_string())),
                            "Variant" | "VariantNoUnlock" => panic!("Variant is missing a base"),
                            _ => panic!("Invalid type"),
                        }
                    } else {
                        panic!("Invalid token");
                    }
                }
                4 => {
                    if let (TokenTree::Ident(n), TokenTree::Ident(t), TokenTree::Ident(b), TokenTree::Literal(s)) = (&tokens[0], &tokens[1], &tokens[2], &tokens[3]) {
                        if t.to_string().as_str() == "Variant" {
                            variants.push((n.to_string(), b.to_string(), s.to_string(), s.to_string(), true));
                        } else if t.to_string().as_str() == "VariantNoUnlock" {
                            variants.push((n.to_string(), b.to_string(), s.to_string(), s.to_string(), false));
                        } else {
                            panic!("Non-variants cannot have a base");
                        }
                    } else if let (TokenTree::Ident(n), TokenTree::Ident(t), TokenTree::Literal(s1), TokenTree::Literal(s2)) = (&tokens[0], &tokens[1], &tokens[2], &tokens[3]) {
                        match t.to_string().as_str() {
                            "Villain" => villains.push((n.to_string(), s1.to_string(), s2.to_string())),
                            "TeamVillain" => team_villains.push((n.to_string(), s1.to_string(), s2.to_string())),
                            "Hero" => heroes.push((n.to_string(), s1.to_string(), s2.to_string())),
                            "Environment" => environments.push((n.to_string(), s1.to_string(), s2.to_string())),
                            "Variant" | "VariantNoUnlock" => panic!("Variant is missing a base"),
                            _ => panic!("Invalid type"),
                        }
                    } else {
                        panic!("Invalid token");
                    }
                }
                5 => {
                    if let (TokenTree::Ident(n), TokenTree::Ident(t), TokenTree::Ident(b), TokenTree::Literal(s1), TokenTree::Literal(s2)) =
                        (&tokens[0], &tokens[1], &tokens[2], &tokens[3], &tokens[4])
                    {
                        if t.to_string().as_str() == "Variant" || t.to_string().as_str() == "VariantNoUnlock" {
                            variants.push((n.to_string(), b.to_string(), s1.to_string(), s2.to_string(), true));
                        } else if t.to_string().as_str() == "VariantNoUnlock" {
                            variants.push((n.to_string(), b.to_string(), s1.to_string(), s2.to_string(), false));
                        } else {
                            panic!("Non-variants cannot have a base");
                        }
                    } else {
                        panic!("Invalid token");
                    }
                }
                _ => panic!("Invalid token count"),
            }
        }
    }

    let mut variant_counts = vec![];

    for ((variant, base, _, _, _), i) in variants.iter().zip(1..) {
        if base == "Villain" {
            variant_counts.push((variant, base, 0));
        } else {
            variant_counts.push((variant, base, variants.iter().take(i).filter(|(_, b, _, _, _)| b == base).count()));
        }
    }

    format!(
        "
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, FromPrimitive)]
pub enum Villain {{
    {}
}}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, FromPrimitive)]
pub enum TeamVillain {{
    {}
}}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, FromPrimitive)]
pub enum Hero {{
    {}
}}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, FromPrimitive)]
pub enum Environment {{
    {}
}}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, FromPrimitive)]
pub enum Variant {{Base,
    {}
}}

impl Variant {{
    pub const fn variant_count() -> usize {{
        {}
    }}

    pub fn as_i(&self) -> u8 {{
        match self {{
            Variant::Base => 0,
            {}
        }}
    }}

    pub fn as_normal(&self) -> Option<Hero> {{
        match self {{
            Variant::Base => None,
            {}
        }}
    }}

    pub fn as_str(&self) -> &str {{
        match self {{
            Variant::Base => \"Base\",
            {}
        }}
    }}

    pub fn from_hero(hero: Hero, i: u32) -> Option<Variant> {{
        match (hero, i) {{
            {},
            _ => None
        }}
    }}
}}


impl Hero {{
    pub const fn variant_count() -> usize {{
        {}
    }}

    pub fn as_str(&self) -> &str {{
        match self {{
            {}
        }}
    }}
}}

impl Villain {{
    pub const fn variant_count() -> usize {{
        {}
    }}

    pub fn as_str(&self) -> &str {{
        match self {{
            {}
        }}
    }}
}}

impl TeamVillain {{
    pub const fn variant_count() -> usize {{
        {}
    }}

    pub fn as_str(&self) -> &str {{
        match self {{
            {}
        }}
    }}
}}

impl Environment {{
    pub const fn variant_count() -> usize {{
        {}
    }}

    pub fn as_str(&self) -> &str {{
        match self {{
            {}
        }}
    }}
}}

impl Item {{
    pub fn from_str(str: &str) -> Option<Item> {{
        match str {{
            {},{},{},{},{},
            \"1 Undo\" => Some(Item::Filler),
            \"Scion of Oblivaeon\" => Some(Item::Scion),
            _ => None
        }}
    }}
}}

impl Location {{
    pub fn from_str(str: &str) -> Option<Location> {{
        match str {{
            {},{},{},{},
            \"Spite - Agent of Gloom - Normal\" => Some(Location::Villain((Villain::SpiteAgentOfGloom, 0))),
            \"Spite - Agent of Gloom - Advanced\" => Some(Location::Villain((Villain::SpiteAgentOfGloom, 1))),
            \"Spite - Agent of Gloom & Skinwalker Gloomweaver - Challenge\" => Some(Location::Villain((Villain::SpiteAgentOfGloom, 2))),
            \"Spite - Agent of Gloom & Skinwalker Gloomweaver - Ultimate\" => Some(Location::Villain((Villain::SpiteAgentOfGloom, 3))),
            \"Skinwalker Gloomweaver - Normal\" => Some(Location::Villain((Villain::SkinwalkerGloomweaver, 0))),
            \"Skinwalker Gloomweaver - Advanced\" => Some(Location::Villain((Villain::SkinwalkerGloomweaver, 1))),            
            \"Oblivaeon\" => Some(Location::Victory),
            _ => None
        }}
    }}
}}
        ",
        villains.iter().map(|(v, _, _)| v).cloned().collect::<Vec<_>>().join(","),
        team_villains.iter().map(|(v, _, _)| v).cloned().collect::<Vec<_>>().join(","),
        heroes.iter().map(|(v, _, _)| v).cloned().collect::<Vec<_>>().join(","),
        environments.iter().map(|(v, _, _)| v).cloned().collect::<Vec<_>>().join(","),
        variants.iter().map(|(v, _, _, _, _)| v).cloned().collect::<Vec<_>>().join(","),
        variants.len() + 1,
        variant_counts.iter().map(|(v, _, c)| format!("Variant::{v} => {c}")).collect::<Vec<_>>().join(","),
        variants
            .iter()
            .map(|(v, b, _, _, _)| format!("Variant::{v} => {}", if b == "Villain" { String::from("None") } else { format!("Some(Hero::{b})") }))
            .collect::<Vec<_>>()
            .join(","),
        variants.iter().map(|(v, _, _, s, _)| format!("Variant::{v} => {s}")).collect::<Vec<_>>().join(","),
        variant_counts
            .iter()
            .filter(|(_, b, _)| b.as_str() != "Villain")
            .map(|(v, b, c)| format!("(Hero::{b}, {c}) => Some(Variant::{v})"))
            .collect::<Vec<_>>()
            .join(","),
        heroes.len(),
        heroes.iter().map(|(v, _, s)| format!("Hero::{v} => {s}")).collect::<Vec<_>>().join(","),
        villains.len(),
        villains.iter().map(|(v, _, s)| format!("Villain::{v} => {s}")).collect::<Vec<_>>().join(","),
        team_villains.len(),
        team_villains.iter().map(|(v, _, s)| format!("TeamVillain::{v} => {s}")).collect::<Vec<_>>().join(","),
        environments.len(),
        environments.iter().map(|(v, _, s)| format!("Environment::{v} => {s}")).collect::<Vec<_>>().join(","),
        villains.iter().map(|(v, s, _)| format!("{s} => Some(Item::Villain(Villain::{v}))")).collect::<Vec<_>>().join(","),
        team_villains
            .iter()
            .map(|(v, s, _)| format!("{s} => Some(Item::TeamVillain(TeamVillain::{v}))"))
            .collect::<Vec<_>>()
            .join(","),
        heroes.iter().map(|(v, s, _)| format!("{s} => Some(Item::Hero(Hero::{v}))")).collect::<Vec<_>>().join(","),
        environments
            .iter()
            .map(|(v, s, _)| format!("{s} => Some(Item::Environment(Environment::{v}))"))
            .collect::<Vec<_>>()
            .join(","),
        variants.iter().map(|(v, _, s, _, _)| format!("{s} => Some(Item::Variant(Variant::{v}))")).collect::<Vec<_>>().join(","),
        villains.iter()
            .filter(|(v, _, _)| v != "SpiteAgentOfGloom" && v != "SkinwalkerGloomweaver")
            .map(|(v, s1, s2)| (v, s1[0..(s1.len() - 1)].to_string(), s2))
            .map(|(v, s, _)| format!("{s} - Normal\" => Some(Location::Villain((Villain::{v}, 0))),{s} - Advanced\" => Some(Location::Villain((Villain::{v}, 1))),{s} - Challenge\" => Some(Location::Villain((Villain::{v}, 2))),{s} - Ultimate\" => Some(Location::Villain((Villain::{v}, 3)))"))
            .collect::<Vec<_>>()
            .join(","),
        team_villains.iter()
            .map(|(v, s, _)| (v, s[0..(s.len() - 1)].to_string()))
            .map(|(v, s)| format!("{s} - Normal\" => Some(Location::TeamVillain((TeamVillain::{v}, 0))),{s} - Advanced\" => Some(Location::TeamVillain((TeamVillain::{v}, 1))),{s} - Challenge\" => Some(Location::TeamVillain((TeamVillain::{v}, 2))),{s} - Ultimate\" => Some(Location::TeamVillain((TeamVillain::{v}, 3)))"))
            .collect::<Vec<_>>()
            .join(","),
        environments.iter()
            .map(|(v, s, _)| (v, s[0..(s.len() - 1)].to_string()))
            .map(|(v, s)| format!("{s} - Any difficulty\" => Some(Location::Environment(Environment::{v}))"))
            .collect::<Vec<_>>()
            .join(","),
        variants.iter()
            .filter(|(_, _, _, _, u)| *u)
            .map(|(v, _, s, _, _)| (v, s[0..(s.len() - 1)].to_string()))
            .map(|(v, s)| format!("{s} - Unlock\" => Some(Location::Variant(Variant::{v}))"))
            .collect::<Vec<_>>()
            .join(",")
    )
    .parse()
    .unwrap()
}
