use crate::{Data, EnumData, VariantData, VillainData};

#[derive(Debug, Clone)]
pub enum LogicTerm {
    AnyHero(EnumData),
    Villain(VillainData),
    TeamVillain(VillainData),
    Hero(EnumData),
    Variant(VariantData),
    Environment(EnumData),
    Or(Vec<LogicTerm>),
    And(Vec<LogicTerm>),
    True,
}

impl Data {
    pub fn parse_logic(&self, str: &str) -> LogicTerm {
        if str.is_empty() {
            LogicTerm::True
        } else if str.contains(' ') {
            LogicTerm::And(str.split(' ').map(|term| self.parse_logic(term)).collect())
        } else if str.contains('|') {
            LogicTerm::Or(str.split('|').map(|term| self.parse_logic(term)).collect())
        } else {
            let loose = str.ends_with('?');

            if loose {
                let key = &str[..str.len() - 1];
                if let Some(hero) = self.heroes.iter().find(|hero| hero.enum_name == key) {
                    LogicTerm::AnyHero(hero.clone())
                } else if let Some(villain) = self.villains.iter().find(|villain| villain.enum_name == key) {
                    let mut variants = vec![LogicTerm::Villain(villain.clone())];

                    variants.extend(self.variants.iter().filter(|variant| variant.base == key).map(|variant| LogicTerm::Variant(variant.clone())));
                    variants.extend(
                        self.team_villains
                            .iter()
                            .filter(|team_villain| team_villain.enum_name[4..] == *key)
                            .map(|team_villain| LogicTerm::TeamVillain(team_villain.clone())),
                    );

                    LogicTerm::Or(variants)
                } else if self.variants.iter().any(|variant| variant.enum_name == key) {
                    panic!("Villain variants cannot have loose matching {str}, use the base villain instead")
                } else if self.team_villains.iter().any(|team_villain| team_villain.enum_name == key) {
                    panic!("Team villains cannot have loose matching {str}, use the non-team villain instead")
                } else if self.environments.iter().any(|environment| environment.enum_name == key) {
                    panic!("Environments cannot have loose matching {str}")
                } else {
                    panic!("Failed to resolve logic {str}")
                }
            } else if let Some(hero) = self.heroes.iter().find(|hero| hero.enum_name == str) {
                LogicTerm::Hero(hero.clone())
            } else if let Some(variant) = self.variants.iter().find(|variant| variant.enum_name == str) {
                LogicTerm::Variant(variant.clone())
            } else if let Some(villain) = self.villains.iter().find(|villain| villain.enum_name == str) {
                LogicTerm::Villain(villain.clone())
            } else if let Some(team_villain) = self.team_villains.iter().find(|team_villain| team_villain.enum_name == str) {
                LogicTerm::TeamVillain(team_villain.clone())
            } else if let Some(environment) = self.environments.iter().find(|environment| environment.enum_name == str) {
                LogicTerm::Environment(environment.clone())
            } else {
                panic!("Failed to resolve logic {str}")
            }
        }
    }
}

impl LogicTerm {
    pub fn as_rust_expr(&self) -> String {
        match self {
            LogicTerm::AnyHero(enum_data) => format!("items.has_hero(Hero::{})", enum_data.enum_name),
            LogicTerm::Villain(enum_data) => format!("items.has_villain(Villain::{})", enum_data.enum_name),
            LogicTerm::TeamVillain(enum_data) => format!("(items.has_team_villain(TeamVillain::{}) && items.team_villain_count())", enum_data.enum_name),
            LogicTerm::Hero(enum_data) => format!("items.has_base_hero(Hero::{})", enum_data.enum_name),
            LogicTerm::Variant(variant_data) => {
                if variant_data.is_villain {
                    format!("items.has_villain(Villain::{})", variant_data.enum_name)
                } else {
                    format!("items.has_hero_variant(Variant::{})", variant_data.enum_name)
                }
            }
            LogicTerm::Environment(enum_data) => format!("items.has_environment(Environment::{})", enum_data.enum_name),
            LogicTerm::Or(terms) => terms.iter().map(|term| term.as_rust_expr()).collect::<Vec<_>>().join("||"),
            LogicTerm::And(terms) => terms.iter().map(|term| format!("({})", term.as_rust_expr())).collect::<Vec<_>>().join("&&"),
            LogicTerm::True => String::from("True"),
        }
    }

    pub fn as_py_expr(&self) -> String {
        let mut requires_team_villains = false;

        let expr = match self {
            LogicTerm::AnyHero(enum_data) => format!("state.prog_items[player].get(\"Any {}\",False)", enum_data.display_name),
            LogicTerm::TeamVillain(enum_data) => {
                requires_team_villains = true;
                format!("state.prog_items[player].get(\"{}\",False)", enum_data.display_name)
            }
            LogicTerm::Villain(enum_data) => format!("state.prog_items[player].get(\"{}\",False)", enum_data.display_name),
            LogicTerm::Hero(enum_data) | LogicTerm::Environment(enum_data) => format!("state.prog_items[player].get(\"{}\",False)", enum_data.display_name),
            LogicTerm::Variant(variant_data) => format!("state.prog_items[player].get(\"{}\",False)", variant_data.display_name),
            LogicTerm::Or(terms) => {
                let mut required_items = vec![];
                let mut required_items_team_villains = vec![];
                let mut additional_exprs = vec![];

                for term in terms {
                    if let Some((item, rtv)) = term.try_py_required_item() {
                        if rtv {
                            required_items_team_villains.push(item);
                        } else {
                            required_items.push(item);
                        }
                    } else {
                        additional_exprs.push(term.as_py_expr());
                    }
                }

                let items_part = match required_items.len() {
                    0 => String::new(),
                    1 => format!("state.prog_items[player].get(\"{}\",False)", required_items[0]),
                    2.. => format!("state.has_any(({}),player)", required_items.into_iter().map(|item| format!("\"{item}\"")).collect::<Vec<_>>().join(",")),
                };
                let rtv_items_part = match required_items_team_villains.len() {
                    0 => String::new(),
                    1 => format!(
                        "state.prog_items[player][\"Team Villains\"] >= 3 and state.prog_items[player].get(\"{}\",False)",
                        required_items_team_villains[0]
                    ),
                    2.. => format!(
                        "state.prog_items[player][\"Team Villains\"] >= 3 and state.has_any(({}),player)",
                        required_items_team_villains.into_iter().map(|item| format!("\"{item}\"")).collect::<Vec<_>>().join(",")
                    ),
                };
                let additional_part = if additional_exprs.is_empty() { String::new() } else { additional_exprs.join(" or ") };

                return [items_part, rtv_items_part, additional_part]
                    .into_iter()
                    .filter(|part| !part.is_empty())
                    .collect::<Vec<_>>()
                    .join(" or ");
            }
            LogicTerm::And(terms) => {
                let mut required_items = vec![];
                let mut additional_exprs = vec![];

                for term in terms {
                    if let Some((item, rtv)) = term.try_py_required_item() {
                        required_items.push(item);
                        requires_team_villains |= rtv;
                    } else {
                        additional_exprs.push(format!("({})", term.as_py_expr()));
                    }
                }

                let rtv_part = if requires_team_villains { "state.prog_items[player][\"Team Villains\"] >= 3" } else { "" };
                let items_part = match required_items.len() {
                    0 => String::new(),
                    1 => format!("state.prog_items[player].get(\"{}\",False)", required_items[0]),
                    2.. => format!("state.has_all(({}),player)", required_items.into_iter().map(|item| format!("\"{item}\"")).collect::<Vec<_>>().join(",")),
                };
                let additional_part = if additional_exprs.is_empty() { String::new() } else { additional_exprs.join(" and ") };

                return [rtv_part, &items_part, &additional_part].into_iter().filter(|part| !part.is_empty()).collect::<Vec<_>>().join(" and ");
            }
            LogicTerm::True => return String::from("True"),
        };

        if requires_team_villains {
            format!("state.prog_items[player][\"Team Villains\"] >= 3 and {expr}")
        } else {
            expr
        }
    }

    fn try_py_required_item(&self) -> Option<(String, bool)> {
        match self {
            LogicTerm::AnyHero(enum_data) => Some((format!("Any {}", enum_data.display_name), false)),
            LogicTerm::TeamVillain(enum_data) => Some((enum_data.display_name.to_owned(), true)),
            LogicTerm::Villain(enum_data) => Some((enum_data.display_name.to_owned(), false)),
            LogicTerm::Hero(enum_data) | LogicTerm::Environment(enum_data) => Some((enum_data.display_name.to_owned(), false)),
            LogicTerm::Variant(variant_data) => Some((variant_data.display_name.to_owned(), false)),
            LogicTerm::Or(_) | LogicTerm::And(_) | LogicTerm::True => None,
        }
    }

    pub fn as_dependencies(&self, data: &Data) -> Vec<String> {
        match self {
            LogicTerm::AnyHero(enum_data) => {
                let mut dependencies: Vec<String> = data
                    .hero_variants()
                    .filter(|variant| variant.base == enum_data.enum_name)
                    .map(|variant| variant.display_name.clone())
                    .collect();
                dependencies.push(enum_data.display_name.clone());
                dependencies
            }
            LogicTerm::TeamVillain(enum_data) => {
                let mut dependencies: Vec<String> = data.team_villains.iter().map(|team_villain| team_villain.display_name.clone()).collect();
                if !dependencies.contains(&enum_data.display_name) {
                    dependencies.push(enum_data.display_name.clone());
                }
                dependencies
            }
            LogicTerm::Villain(enum_data) => vec![enum_data.display_name.clone()],
            LogicTerm::Hero(enum_data) | LogicTerm::Environment(enum_data) => vec![enum_data.display_name.clone()],
            LogicTerm::Variant(variant_data) => vec![variant_data.display_name.clone()],
            LogicTerm::Or(terms) | LogicTerm::And(terms) => {
                let mut dependencies = vec![];
                for term in terms {
                    for dependency in term.as_dependencies(data) {
                        if !dependencies.contains(&dependency) {
                            dependencies.push(dependency);
                        }
                    }
                }
                dependencies
            }
            LogicTerm::True => vec![],
        }
    }
}
