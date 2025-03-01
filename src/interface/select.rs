use web_sys::Document;

use crate::{
    data::{Contender, Environment, Gladiator, Hero, Item, TeamVillain, Variant, Villain},
    protocol::DeathlinkType,
};

use super::{CurrentVillains, Interface, SelectedHero};

impl Interface {
    pub fn toggle_selection(&mut self, selected: Item, deathlink: DeathlinkType) {
        match selected {
            Item::Hero(selected) => {
                for (idx, (hero, elem)) in self.current_game.heroes.iter().enumerate() {
                    if SelectedHero::Hero(selected) == *hero {
                        deselect_hero(&self.document, &selected);
                        elem.remove();
                        self.current_game.heroes.remove(idx);
                        return;
                    } else if match hero {
                        SelectedHero::Variant(variant) => variant.as_normal() == Some(selected),
                        _ => false,
                    } {
                        change_hero_selection(&self.document, &selected);
                        if let Some(child) = elem.first_element_child() {
                            child.set_inner_html(selected.as_str());
                        }
                        if deathlink == DeathlinkType::Individual {
                            if let Some(button) = elem.last_element_child() {
                                button.set_id(&format!("deathlink-{}", selected.as_ident()));
                            }
                        }
                        self.current_game.heroes[idx].0 = SelectedHero::Hero(selected);
                        return;
                    }
                }

                select_hero(&self.document, &selected);
                self.prevent_hero_overflow();
                let new = self.document.create_element("div").expect("Failed to create child element");
                new.set_inner_html(&format!(
                    "<span>{}</span>{}",
                    selected.as_str(),
                    if deathlink == DeathlinkType::Individual {
                        format!("<button id=\"deathlink-{}\" class=\"deathlink\">Deathlink</button>", selected.as_ident())
                    } else {
                        String::new()
                    }
                ));
                let _ = self.active_heroes.append_child(&new);
                self.current_game.heroes.push((SelectedHero::Hero(selected), new));
            }
            Item::Contender(contender) => {
                let mut first_empty = None;
                for (idx, (hero, elem)) in self.current_game.heroes.iter_mut().enumerate() {
                    if let SelectedHero::Contenders(contenders) = hero {
                        if let Some(inner_idx) = contenders.iter().position(|c| *c == contender) {
                            contenders.remove(inner_idx);
                            deselect_contender(&self.document, &contender);
                            if contenders.is_empty() {
                                elem.remove();
                                self.current_game.heroes.remove(idx);
                            } else {
                                if let Some(child) = elem.first_element_child() {
                                    child.set_inner_html(&selected_contender_string(contenders));
                                }
                                if deathlink == DeathlinkType::Individual {
                                    if let Some(button) = elem.last_element_child() {
                                        button.set_id(&format!("deathlink-{}", contenders[0].as_ident()));
                                    }
                                }
                            }
                            return;
                        }

                        if first_empty.is_none() && contenders.len() < 3 {
                            first_empty = Some((contenders, elem));
                        }
                    }
                }

                select_contender(&self.document, &contender);
                if let Some((contenders, elem)) = first_empty {
                    contenders.push(contender);
                    elem.set_inner_html(&format!(
                        "<span>{}</span>{}",
                        selected_contender_string(contenders),
                        if deathlink == DeathlinkType::Individual {
                            format!("<button id=\"deathlink-{}\" class=\"deathlink\">Deathlink</button>", contenders[0].as_ident())
                        } else {
                            String::new()
                        }
                    ));
                } else {
                    self.prevent_hero_overflow();
                    let new = self.document.create_element("div").expect("Failed to create child element");
                    new.set_inner_html(&format!(
                        "<span>{}</span>{}",
                        contender.as_str(),
                        if deathlink == DeathlinkType::Individual {
                            format!("<button id=\"deathlink-{}\" class=\"deathlink\">Deathlink</button>", contender.as_ident())
                        } else {
                            String::new()
                        }
                    ));
                    let _ = self.active_heroes.append_child(&new);
                    self.current_game.heroes.push((SelectedHero::Contenders(vec![contender]), new));
                }
            }
            Item::Variant(variant) => {
                for (idx, (hero, elem)) in self.current_game.heroes.iter().enumerate() {
                    if SelectedHero::Variant(variant) == *hero {
                        deselect_variant(&self.document, &variant);
                        elem.remove();
                        self.current_game.heroes.remove(idx);
                        return;
                    } else if match hero {
                        SelectedHero::Hero(base) => variant.as_normal() == Some(*base),
                        SelectedHero::Variant(other) => variant.as_normal() == other.as_normal(),
                        _ => false,
                    } {
                        change_variant_selection(&self.document, &variant);
                        if let Some(child) = elem.first_element_child() {
                            child.set_inner_html(variant.as_str());
                        }
                        if deathlink == DeathlinkType::Individual {
                            if let Some(button) = elem.last_element_child() {
                                button.set_id(&format!("deathlink-{}", variant.as_ident()));
                            }
                        }
                        self.current_game.heroes[idx].0 = SelectedHero::Variant(variant);
                        return;
                    }
                }

                select_variant(&self.document, &variant);
                self.prevent_hero_overflow();
                let new = self.document.create_element("div").expect("Failed to create child element");
                new.set_inner_html(&format!(
                    "<span>{}</span>{}",
                    variant.as_str(),
                    if deathlink == DeathlinkType::Individual {
                        format!("<button id=\"deathlink-{}\" class=\"deathlink\">Deathlink</button>", variant.as_ident())
                    } else {
                        String::new()
                    }
                ));
                let _ = self.active_heroes.append_child(&new);
                self.current_game.heroes.push((SelectedHero::Variant(variant), new));
            }
            Item::Villain(villain) => {
                let add = match &self.current_game.villains {
                    CurrentVillains::Classic((current, _, _)) => *current != villain,
                    _ => true,
                };
                self.clear_selected_villain();
                if add {
                    select_villain(&self.document, &villain);
                    let new = self.document.create_element("div").expect("Failed to create child element");
                    new.set_inner_html(&format!(
                        "<h3>{}</h3><button id=\"diff-{}\" class=\"difficulty normal\">Normal</button>",
                        villain.as_str(),
                        villain.as_ident()
                    ));
                    let _ = self.active_villains.append_child(&new);
                    self.current_game.villains = CurrentVillains::Classic((villain, 0, new));
                }
            }
            Item::TeamVillain(team_villain) => {
                match &mut self.current_game.villains {
                    CurrentVillains::Classic(_) | CurrentVillains::Gladiators(_) => self.clear_selected_villain(),
                    CurrentVillains::Team(current) => {
                        for (idx, (villain, _, elem)) in current.iter_mut().enumerate() {
                            if *villain == team_villain {
                                elem.remove();
                                current.remove(idx);
                                deselect_team_villain(&self.document, &team_villain);
                                return;
                            }
                        }
                        if current.len() >= 5 {
                            let (first_villain, _, elem) = current.first().unwrap();
                            deselect_team_villain(&self.document, first_villain);
                            elem.remove();
                            current.remove(0);
                        }
                        select_team_villain(&self.document, &team_villain);
                        let new = self.document.create_element("div").expect("Failed to create child element");
                        new.set_inner_html(&format!(
                            "<h3>{}</h3><button id=\"diff-{}\" class=\"difficulty normal\">Normal</button>",
                            team_villain.as_str(),
                            team_villain.as_ident()
                        ));
                        let _ = self.active_villains.append_child(&new);
                        current.push((team_villain, 0, new));
                        return;
                    }
                    CurrentVillains::None => (),
                }
                select_team_villain(&self.document, &team_villain);
                let new = self.document.create_element("div").expect("Failed to create child element");
                new.set_inner_html(&format!(
                    "<h3>{}</h3><button id=\"diff-{}\" class=\"difficulty normal\">Normal</button>",
                    team_villain.as_str(),
                    team_villain.as_ident()
                ));
                let _ = self.active_villains.append_child(&new);
                self.current_game.villains = CurrentVillains::Team(vec![(team_villain, 0, new)]);
            }
            Item::Gladiator(gladiator) => {
                match &mut self.current_game.villains {
                    CurrentVillains::Classic(_) | CurrentVillains::Team(_) => self.clear_selected_villain(),
                    CurrentVillains::Gladiators(current) => {
                        for (idx, (villain, _, elem)) in current.iter_mut().enumerate() {
                            if *villain == gladiator {
                                elem.remove();
                                current.remove(idx);
                                deselect_gladiator(&self.document, &gladiator);
                                return;
                            }
                        }
                        if current.len() >= 5 {
                            let (first_gladiator, _, elem) = current.first().unwrap();
                            deselect_gladiator(&self.document, first_gladiator);
                            elem.remove();
                            current.remove(0);
                        }
                        select_gladiator(&self.document, &gladiator);
                        let new = self.document.create_element("div").expect("Failed to create child element");
                        new.set_inner_html(&format!(
                            "<h3>{}</h3><button id=\"diff-{}\" class=\"difficulty normal\">Normal</button>",
                            gladiator.as_str(),
                            gladiator.as_ident()
                        ));
                        let _ = self.active_villains.append_child(&new);
                        current.push((gladiator, 0, new));
                        return;
                    }
                    CurrentVillains::None => (),
                }
                select_gladiator(&self.document, &gladiator);
                let new = self.document.create_element("div").expect("Failed to create child element");
                new.set_inner_html(&format!(
                    "<h3>{}</h3><button id=\"diff-{}\" class=\"difficulty normal\">Normal</button>",
                    gladiator.as_str(),
                    gladiator.as_ident()
                ));
                let _ = self.active_villains.append_child(&new);
                self.current_game.villains = CurrentVillains::Gladiators(vec![(gladiator, 0, new)]);
            }
            Item::Environment(environment) => {
                if let Some(current) = self.current_game.environment {
                    if current == environment {
                        deselect_environment(&self.document, &environment);
                        self.active_environment.set_inner_html("");
                        self.current_game.environment = None;
                        return;
                    } else {
                        deselect_environment(&self.document, &current);
                    }
                }
                select_environment(&self.document, &environment);
                self.active_environment.set_inner_html(environment.as_str());
                self.current_game.environment = Some(environment);
            }
            _ => (),
        }
    }
}

pub fn select_hero(doc: &Document, hero: &Hero) {
    if let Some(elem) = doc.get_element_by_id(hero.as_ident()) {
        let _ = elem.class_list().add_1("selected");
        for variant in hero.variants() {
            if let Some(elem) = doc.get_element_by_id(variant.as_ident()) {
                let _ = elem.class_list().add_1("secondary-selection");
            }
        }
    }
}

pub fn deselect_hero(doc: &Document, hero: &Hero) {
    if let Some(elem) = doc.get_element_by_id(hero.as_ident()) {
        let _ = elem.class_list().remove_1("selected");
    }
    for variant in hero.variants() {
        if let Some(elem) = doc.get_element_by_id(variant.as_ident()) {
            let _ = elem.class_list().remove_1("secondary-selection");
        }
    }
}

pub fn change_hero_selection(doc: &Document, hero: &Hero) {
    if let Some(elem) = doc.get_element_by_id(hero.as_ident()) {
        let _ = elem.class_list().remove_1("secondary-selection");
        let _ = elem.class_list().add_1("selected");
    }
    for variant in hero.variants() {
        if let Some(elem) = doc.get_element_by_id(variant.as_ident()) {
            let _ = elem.class_list().remove_1("selected");
            let _ = elem.class_list().add_1("secondary-selection");
        }
    }
}

pub fn select_variant(doc: &Document, variant: &Variant) {
    if let Some(elem) = doc.get_element_by_id(variant.as_ident()) {
        let _ = elem.class_list().add_1("selected");
    }
    if let Some(base) = variant.as_normal() {
        if let Some(elem) = doc.get_element_by_id(base.as_ident()) {
            let _ = elem.class_list().add_1("secondary-selection");
        }
        for other_variant in base.variants() {
            if other_variant != *variant {
                if let Some(elem) = doc.get_element_by_id(other_variant.as_ident()) {
                    let _ = elem.class_list().add_1("secondary-selection");
                }
            }
        }
    }
}

pub fn deselect_variant(doc: &Document, variant: &Variant) {
    if let Some(elem) = doc.get_element_by_id(variant.as_ident()) {
        let _ = elem.class_list().remove_1("selected");
    }
    if let Some(base) = variant.as_normal() {
        if let Some(elem) = doc.get_element_by_id(base.as_ident()) {
            let _ = elem.class_list().remove_1("secondary-selection");
        }
        for other_variant in base.variants() {
            if other_variant != *variant {
                if let Some(elem) = doc.get_element_by_id(other_variant.as_ident()) {
                    let _ = elem.class_list().remove_1("secondary-selection");
                }
            }
        }
    }
}

pub fn change_variant_selection(doc: &Document, variant: &Variant) {
    if let Some(elem) = doc.get_element_by_id(variant.as_ident()) {
        let _ = elem.class_list().remove_1("secondary-selection");
        let _ = elem.class_list().add_1("selected");
    }
    if let Some(base) = variant.as_normal() {
        if let Some(elem) = doc.get_element_by_id(base.as_ident()) {
            let _ = elem.class_list().remove_1("selected");
            let _ = elem.class_list().add_1("secondary-selection");
        }
        for other_variant in base.variants() {
            if other_variant != *variant {
                if let Some(elem) = doc.get_element_by_id(other_variant.as_ident()) {
                    let _ = elem.class_list().remove_1("selected");
                    let _ = elem.class_list().add_1("secondary-selection");
                }
            }
        }
    }
}

pub fn deselect_contenders(doc: &Document, contenders: &[Contender]) {
    for contender in contenders {
        deselect_contender(doc, contender);
    }
}

pub fn select_contender(doc: &Document, contender: &Contender) {
    if let Some(elem) = doc.get_element_by_id(contender.as_ident()) {
        let _ = elem.class_list().add_1("selected");
    }
}

pub fn deselect_contender(doc: &Document, contender: &Contender) {
    if let Some(elem) = doc.get_element_by_id(contender.as_ident()) {
        let _ = elem.class_list().remove_1("selected");
    }
}

pub fn selected_contender_string(contenders: &[Contender]) -> String {
    contenders.iter().map(|contender| contender.as_str()).collect::<Vec<_>>().join(" | ")
}

pub fn select_villain(doc: &Document, villain: &Villain) {
    if let Some(elem) = doc.get_element_by_id(villain.as_ident()) {
        let _ = elem.class_list().add_1("selected");
    }
}

pub fn deselect_villain(doc: &Document, villain: &Villain) {
    if let Some(elem) = doc.get_element_by_id(villain.as_ident()) {
        let _ = elem.class_list().remove_1("selected");
    }
}

pub fn select_team_villain(doc: &Document, team_villain: &TeamVillain) {
    if let Some(elem) = doc.get_element_by_id(team_villain.as_ident()) {
        let _ = elem.class_list().add_1("selected");
    }
}

pub fn deselect_team_villain(doc: &Document, team_villain: &TeamVillain) {
    if let Some(elem) = doc.get_element_by_id(team_villain.as_ident()) {
        let _ = elem.class_list().remove_1("selected");
    }
}

pub fn select_gladiator(doc: &Document, gladiator: &Gladiator) {
    if let Some(elem) = doc.get_element_by_id(gladiator.as_ident()) {
        let _ = elem.class_list().add_1("selected");
    }
}

pub fn deselect_gladiator(doc: &Document, gladiator: &Gladiator) {
    if let Some(elem) = doc.get_element_by_id(gladiator.as_ident()) {
        let _ = elem.class_list().remove_1("selected");
    }
}

pub fn select_environment(doc: &Document, environment: &Environment) {
    if let Some(elem) = doc.get_element_by_id(environment.as_ident()) {
        let _ = elem.class_list().add_1("selected");
    }
}

pub fn deselect_environment(doc: &Document, environment: &Environment) {
    if let Some(elem) = doc.get_element_by_id(environment.as_ident()) {
        let _ = elem.class_list().remove_1("selected");
    }
}
