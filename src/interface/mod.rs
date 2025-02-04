mod select;

use crate::{
    data::{Contender, Environment, FillerTarget, Gladiator, Hero, HeroLike, Item, Location, TeamVillain, Variant, Villain, VillainLike},
    game::CurrentGame,
    state::{items::Items, State},
};
use select::{deselect_contenders, deselect_gladiator, deselect_hero, deselect_team_villain, deselect_variant, deselect_villain};
use std::fmt::Write;
use strum::IntoEnumIterator;
use web_sys::{window, Document, Element};

pub struct Interface {
    document: Document,
    heroes: Element,
    current_heroes: Vec<(usize, u8, Element)>,
    contenders: Element,
    current_contenders: Vec<(usize, Element)>,
    villains: Element,
    current_villains: Vec<(usize, Element)>,
    team_villains: Element,
    current_team_villains: Vec<(usize, Element)>,
    gladiators: Element,
    current_gladiators: Vec<(usize, Element)>,
    environments: Element,
    current_environments: Vec<(usize, Element)>,
    active_heroes: Element,
    active_villains: Element,
    active_environment: Element,
    active_variants: Element,
    active_filler: Element,
    goal: Element,
    pub current_game: CurrentGame,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectedHero {
    Hero(Hero),
    Variant(Variant),
    Contenders(Vec<Contender>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum CurrentVillains {
    Classic((Villain, u8, Element)),
    Team(Vec<(TeamVillain, u8, Element)>),
    Gladiators(Vec<(Gladiator, u8, Element)>),
    None,
}

const DIFFICULTY_CLASSES: [&str; 4] = ["normal", "advanced", "challenge", "ultimate"];
const DIFFICULTY_NAMES: [&str; 4] = ["Normal", "Advanced", "Challenge", "Ultimate"];

macro_rules! add_elem {
    ($self:ident, $current:expr, $parent:expr, $new:ident) => {
        let new = $self.document.create_element("li").expect("Failed to create child element");
        new.set_inner_html($new.as_str());
        new.set_id($new.as_ident());
        for (idx, (i, elem)) in $current.iter().enumerate() {
            if *i > $new as usize {
                let _ = elem.insert_adjacent_element("beforebegin", &new);
                $current.insert(idx, ($new as usize, new));
                return;
            }
        }

        let _ = $parent.append_child(&new);
        $current.push(($new as usize, new));
    };
    ($self:ident, $current:expr, $parent:expr, $new:ident, $completed:expr) => {
        let new = $self.document.create_element("li").expect("Failed to create child element");
        new.set_inner_html($new.as_str());
        new.set_id($new.as_ident());
        if $completed {
            let _ = new.class_list().add_1("completed");
        }
        for (idx, (i, elem)) in $current.iter().enumerate() {
            if *i > $new as usize {
                let _ = elem.insert_adjacent_element("beforebegin", &new);
                $current.insert(idx, ($new as usize, new));
                return;
            }
        }

        let _ = $parent.append_child(&new);
        $current.push(($new as usize, new));
    };
}

macro_rules! add_villain {
    ($self:ident, $current:expr, $parent:expr, $new:ident, $completion:expr) => {
        let new = $self.document.create_element("div").expect("Failed to create child element");
        new.set_inner_html(&format!(
            "<div id=\"completion-{}\">{}</div></div><span id=\"{}\">{}</span>",
            $new.as_ident(),
            completion_str($completion | if $new.no_challenge() { 0xC0 } else { 0 }),
            $new.as_ident(),
            $new.as_str()
        ));
        for (idx, (i, elem)) in $current.iter().enumerate() {
            if *i > $new as usize {
                let _ = elem.insert_adjacent_element("beforebegin", &new);
                $current.insert(idx, ($new as usize, new));
                return;
            }
        }

        let _ = $parent.append_child(&new);
        $current.push(($new as usize, new));
    };
}

fn completion_str(bitfield: u8) -> String {
    format!(
        "<div class=\"completion-indicator normal{}\"></div><div class=\"completion-indicator advanced{}\"></div><div class=\"completion-indicator challenge{}\"></div><div class=\"completion-indicator ultimate{}\">",
        completion_class(bitfield, 0),
        completion_class(bitfield, 1),
        completion_class(bitfield, 2),
        completion_class(bitfield, 3),
    )
}

fn completion_class(bitfield: u8, bit: u8) -> &'static str {
    if bitfield >> bit & 0x1 > 0 {
        " done"
    } else if bitfield >> (bit + 4) & 0x1 > 0 {
        " unavailable"
    } else {
        ""
    }
}

impl Interface {
    pub fn new() -> Interface {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                return Interface {
                    heroes: document.get_element_by_id("heroes").expect("Failed to get heroes element"),
                    current_heroes: vec![],
                    contenders: document.get_element_by_id("contenders").expect("Failed to get contenders element"),
                    current_contenders: vec![],
                    villains: document.get_element_by_id("villains").expect("Failed to get villains element"),
                    current_villains: vec![],
                    team_villains: document.get_element_by_id("team-villains").expect("Failed to get team-villains element"),
                    current_team_villains: vec![],
                    gladiators: document.get_element_by_id("gladiators").expect("Failed to get gladiators element"),
                    current_gladiators: vec![],
                    environments: document.get_element_by_id("environments").expect("Failed to get environments element"),
                    current_environments: vec![],
                    active_heroes: document.get_element_by_id("active-heroes").expect("Failed to get active-heroes element"),
                    active_villains: document.get_element_by_id("active-villains").expect("Failed to get active-villains element"),
                    active_environment: document.get_element_by_id("active-environment").expect("Failed to get active-environment element"),
                    active_variants: document.get_element_by_id("active-variants").expect("Failed to get active-variants element"),
                    active_filler: document.get_element_by_id("active-filler").expect("Failed to get active-filler element"),
                    goal: document.get_element_by_id("goal").expect("Failed to get goal element"),
                    current_game: CurrentGame::new(),
                    document,
                };
            }
        }

        panic!("Failed to get elements");
    }

    pub fn add_item(&mut self, state: &State, item: Item) {
        match item {
            Item::Hero(hero) => self.add_hero(hero),
            Item::Contender(contender) => self.add_contender(contender),
            Item::Variant(variant) => self.add_variant(variant),
            Item::Villain(villain) => self.add_villain(state, villain),
            Item::TeamVillain(team_villain) => self.add_team_villain(state, team_villain),
            Item::Gladiator(gladiator) => self.add_gladiator(state, gladiator),
            Item::Environment(environment) => self.add_environment(state, environment),
            _ => (),
        }
    }

    pub fn add_hero(&mut self, hero: Hero) {
        let new = self.document.create_element("li").expect("Failed to create child element");
        new.set_inner_html(hero.as_str());
        new.set_id(hero.as_ident());
        for (idx, (i, v, elem)) in self.current_heroes.iter().enumerate() {
            if *i > hero as usize || (*i == hero as usize && *v > 0) {
                let _ = elem.insert_adjacent_element("beforebegin", &new);
                if self.current_game.has_hero_all_variants(hero) {
                    let _ = elem.class_list().add_1("secondary-selection");
                }
                self.current_heroes.insert(idx, (hero as usize, 0, new));
                return;
            }
        }

        let _ = self.heroes.append_child(&new);
        self.current_heroes.push((hero as usize, 0, new));
    }

    pub fn add_variant(&mut self, variant: Variant) {
        let base = variant.as_normal().unwrap();

        let new = self.document.create_element("li").expect("Failed to create child element");
        new.set_id(variant.as_ident());
        new.set_inner_html(variant.as_str());
        for (idx, (i, v, elem)) in self.current_heroes.iter().enumerate() {
            if *i > base as usize || (*i == base as usize && *v > variant.as_i()) {
                let _ = elem.insert_adjacent_element("beforebegin", &new);
                if self.current_game.has_hero_all_variants(base) {
                    let _ = elem.class_list().add_1("secondary-selection");
                }
                self.current_heroes.insert(idx, (base as usize, variant.as_i(), new));
                return;
            }
        }

        let _ = self.heroes.append_child(&new);
        self.current_heroes.push((base as usize, variant.as_i(), new));
    }

    pub fn add_contender(&mut self, contender: Contender) {
        add_elem!(self, self.current_contenders, self.contenders, contender);
    }

    pub fn add_villain(&mut self, state: &State, villain: Villain) {
        add_villain!(self, self.current_villains, self.villains, villain, state.checked_locations.villains[villain as usize]);
    }

    pub fn add_team_villain(&mut self, state: &State, team_villain: TeamVillain) {
        add_villain!(
            self,
            self.current_team_villains,
            self.team_villains,
            team_villain,
            state.checked_locations.team_villains[team_villain as usize]
        );
    }

    pub fn add_gladiator(&mut self, state: &State, gladiator: Gladiator) {
        add_villain!(self, self.current_gladiators, self.gladiators, gladiator, state.checked_locations.gladiators[gladiator as usize]);
    }

    pub fn add_environment(&mut self, state: &State, environment: Environment) {
        add_elem!(
            self,
            self.current_environments,
            self.environments,
            environment,
            !state.checked_locations.has_unchecked_environment(environment)
        );
    }

    pub fn advance_difficulty(&mut self, target: Item) {
        match target {
            Item::Villain(villain) => {
                if let CurrentVillains::Classic((_, diff, elem)) = &mut self.current_game.villains {
                    if let Ok(Some(elem)) = elem.query_selector("button") {
                        let mask = if villain.no_challenge() { 0x1 } else { 0x3 };
                        let _ = elem.class_list().remove_1(DIFFICULTY_CLASSES[*diff as usize]);
                        *diff += 1;
                        *diff &= mask;
                        let _ = elem.class_list().add_1(DIFFICULTY_CLASSES[*diff as usize]);
                        elem.set_inner_html(DIFFICULTY_NAMES[*diff as usize]);
                    }
                }
            }
            Item::TeamVillain(team_villain) => {
                if let CurrentVillains::Team(villains) = &mut self.current_game.villains {
                    if let Some((_, diff, elem)) = villains.iter_mut().find(|(v, _, _)| *v == team_villain) {
                        if let Ok(Some(elem)) = elem.query_selector("button") {
                            let mask = if team_villain.no_challenge() { 0x1 } else { 0x3 };
                            let _ = elem.class_list().remove_1(DIFFICULTY_CLASSES[*diff as usize]);
                            *diff += 1;
                            *diff &= mask;
                            let _ = elem.class_list().add_1(DIFFICULTY_CLASSES[*diff as usize]);
                            elem.set_inner_html(DIFFICULTY_NAMES[*diff as usize]);
                        }
                    }
                }
            }
            Item::Gladiator(gladiator) => {
                if let CurrentVillains::Gladiators(gladiators) = &mut self.current_game.villains {
                    if let Some((_, diff, elem)) = gladiators.iter_mut().find(|(g, _, _)| *g == gladiator) {
                        if let Ok(Some(elem)) = elem.query_selector("button") {
                            let mask = if gladiator.no_challenge() { 0x1 } else { 0x3 };
                            let _ = elem.class_list().remove_1(DIFFICULTY_CLASSES[*diff as usize]);
                            *diff += 1;
                            *diff &= mask;
                            let _ = elem.class_list().add_1(DIFFICULTY_CLASSES[*diff as usize]);
                            elem.set_inner_html(DIFFICULTY_NAMES[*diff as usize]);
                        }
                    }
                }
            }
            _ => (),
        }
    }

    fn prevent_hero_overflow(&mut self) {
        if self.current_game.heroes.len() >= 5 {
            let (first_hero, elem) = self.current_game.heroes.first().unwrap();
            match first_hero {
                SelectedHero::Hero(hero) => deselect_hero(&self.document, hero),
                SelectedHero::Variant(variant) => deselect_variant(&self.document, variant),
                SelectedHero::Contenders(contenders) => deselect_contenders(&self.document, contenders),
            }

            let _ = elem.remove();
            self.current_game.heroes.remove(0);
        }
    }

    fn clear_selected_villain(&mut self) {
        match &self.current_game.villains {
            CurrentVillains::Classic((villain, _, elem)) => {
                let _ = elem.remove();
                deselect_villain(&self.document, villain);
            }
            CurrentVillains::Team(villains) => {
                for (villain, _, elem) in villains {
                    let _ = elem.remove();
                    deselect_team_villain(&self.document, villain);
                }
            }
            CurrentVillains::Gladiators(gladiators) => {
                for (gladiator, _, elem) in gladiators {
                    let _ = elem.remove();
                    deselect_gladiator(&self.document, gladiator);
                }
            }
            CurrentVillains::None => (),
        }
        self.current_game.villains = CurrentVillains::None;
    }

    pub fn get_locations(&self) -> Vec<Location> {
        let mut locations = match &self.current_game.villains {
            CurrentVillains::Classic((villain, diff, _)) => (0..4).filter(|d| *d & *diff == *d).map(|diff| Location::Villain((*villain, diff))).collect(),
            CurrentVillains::Team(villains) => villains
                .iter()
                .flat_map(|(villain, diff, _)| (0..4).filter(|d| *d & *diff == *d).map(|diff| Location::TeamVillain((*villain, diff))))
                .collect(),
            CurrentVillains::Gladiators(gladiators) => gladiators
                .iter()
                .flat_map(|(gladiator, diff, _)| (0..4).filter(|d| *d & *diff == *d).map(|diff| Location::Gladiator((*gladiator, diff))))
                .collect(),
            CurrentVillains::None => vec![],
        };

        if let Some(environment) = self.current_game.environment {
            locations.push(Location::Environment(environment));
        }

        locations
    }

    pub fn update_current_filler(&self, items: &Items) {
        let mut buf = String::new();

        match &self.current_game.villains {
            CurrentVillains::Classic((villain, diff, _)) => {
                if *diff > 1 {
                    if let Some((name, desc)) = villain.challenge_desc() {
                        if !buf.is_empty() {
                            let _ = write!(buf, "<hr />");
                        }
                        let _ = write!(buf, "<h4>Challenge - {}</h4><p>{}</p>", name, desc.join("<br />"));
                    }
                }

                for (filler, count) in items.get_filler_for(FillerTarget::Villain(VillainLike::Villain(*villain))) {
                    let _ = write!(buf, "<h4>{}</h4><p>{}</p>", filler.to_string(count), filler.to_desc(count));
                }
            }
            CurrentVillains::Team(villains) => {
                for (villain, diff, _) in villains {
                    let mut name_written = false;

                    if *diff > 1 {
                        if let Some((name, desc)) = villain.challenge_desc() {
                            name_written = true;
                            if !buf.is_empty() {
                                let _ = write!(buf, "<hr />");
                            }
                            let _ = write!(buf, "<h3>{}</h3><h4>Challenge - {}</h4><p>{}</p>", villain.as_str(), name, desc.join("<br />"));
                        }
                    }

                    let relevant_filler = items.get_filler_for(FillerTarget::Villain(VillainLike::TeamVillain(*villain)));
                    if !relevant_filler.is_empty() && !name_written {
                        if !buf.is_empty() {
                            let _ = write!(buf, "<hr />");
                        }
                        let _ = write!(buf, "<h3>{}</h3>", villain.as_str());
                    }

                    for (filler, count) in relevant_filler {
                        let _ = write!(buf, "<h4>{}</h4><p>{}</p>", filler.to_string(count), filler.to_desc(count));
                    }
                }
            }
            CurrentVillains::Gladiators(gladiators) => {
                for (gladiator, diff, _) in gladiators {
                    if *diff > 1 {
                        if let Some((name, desc)) = gladiator.challenge_desc() {
                            if !buf.is_empty() {
                                let _ = write!(buf, "<hr />");
                            }
                            let _ = write!(buf, "<h3>{}</h3><h4>Challenge - {}</h4><p>{}</p>", gladiator.as_str(), name, desc.join("<br />"));
                        }
                    }
                }
            }
            CurrentVillains::None => (),
        }

        for (hero, _) in &self.current_game.heroes {
            match hero {
                SelectedHero::Hero(hero) => {
                    let relevant_filler = items.get_filler_for(FillerTarget::Hero(HeroLike::Hero(*hero)));
                    if !relevant_filler.is_empty() {
                        if !buf.is_empty() {
                            let _ = write!(buf, "<hr />");
                        }

                        let _ = write!(buf, "<h3>{}</h3>", hero.as_str());
                    }

                    for (filler, count) in relevant_filler {
                        let _ = write!(buf, "<h4>{}</h4><p>{}</p>", filler.to_string(count), filler.to_desc(count));
                    }
                }
                SelectedHero::Variant(variant) => {
                    let relevant_filler = items.get_filler_for(FillerTarget::Hero(HeroLike::Variant(*variant)));
                    if !relevant_filler.is_empty() {
                        if !buf.is_empty() {
                            let _ = write!(buf, "<hr />");
                        }

                        let _ = write!(buf, "<h3>{}</h3>", variant.as_str());
                    }

                    for (filler, count) in relevant_filler {
                        let _ = write!(buf, "<h4>{}</h4><p>{}</p>", filler.to_string(count), filler.to_desc(count));
                    }
                }
                SelectedHero::Contenders(_) => (),
            }
        }

        let relevant_filler = items.get_filler_for(FillerTarget::Other);
        if !relevant_filler.is_empty() && !buf.is_empty() {
            let _ = write!(buf, "<hr />");
        }

        for (filler, count) in relevant_filler {
            let _ = write!(buf, "<h4>{}</h4><p>{}</p>", filler.to_string(count), filler.to_desc(count));
        }

        self.active_filler.set_inner_html(&buf);
    }

    pub fn update_completion(&self, state: &State) {
        match &self.current_game.villains {
            CurrentVillains::Classic((villain, _, _)) => {
                if let Some(elem) = self.document.get_element_by_id(&format!("completion-{}", villain.as_ident())) {
                    elem.set_inner_html(&completion_str(state.checked_locations.villains[(*villain) as usize] | if villain.no_challenge() { 0xC0 } else { 0 }));
                }
            }
            CurrentVillains::Team(villains) => {
                for (villain, _, _) in villains {
                    if let Some(elem) = self.document.get_element_by_id(&format!("completion-{}", villain.as_ident())) {
                        elem.set_inner_html(&completion_str(
                            state.checked_locations.team_villains[(*villain) as usize] | if villain.no_challenge() { 0xC0 } else { 0 },
                        ));
                    }
                }
            }
            CurrentVillains::Gladiators(gladiators) => {
                for (gladiator, _, _) in gladiators {
                    if let Some(elem) = self.document.get_element_by_id(&format!("completion-{}", gladiator.as_ident())) {
                        elem.set_inner_html(&completion_str(
                            state.checked_locations.gladiators[(*gladiator) as usize] | if gladiator.no_challenge() { 0xC0 } else { 0 },
                        ));
                    }
                }
            }
            CurrentVillains::None => (),
        }
        if let Some(environment) = self.current_game.environment {
            if let Some(elem) = self.document.get_element_by_id(environment.as_ident()) {
                let _ = elem.class_list().add_1("completed");
            }
        }
    }

    pub fn update_completion_all(&self, state: &State) {
        for villain in Villain::iter() {
            if let Some(elem) = self.document.get_element_by_id(&format!("completion-{}", villain.as_ident())) {
                elem.set_inner_html(&completion_str(state.checked_locations.villains[villain as usize] | if villain.no_challenge() { 0xC0 } else { 0 }));
            }
        }
        for team_villain in TeamVillain::iter() {
            if let Some(elem) = self.document.get_element_by_id(&format!("completion-{}", team_villain.as_ident())) {
                elem.set_inner_html(&completion_str(
                    state.checked_locations.team_villains[team_villain as usize] | if team_villain.no_challenge() { 0xC0 } else { 0 },
                ));
            }
        }
        for gladiator in Gladiator::iter() {
            if let Some(elem) = self.document.get_element_by_id(&format!("completion-{}", gladiator.as_ident())) {
                elem.set_inner_html(&completion_str(
                    state.checked_locations.gladiators[gladiator as usize] | if gladiator.no_challenge() { 0xC0 } else { 0 },
                ));
            }
        }
        for environment in Environment::iter() {
            if !state.checked_locations.has_unchecked_environment(environment) {
                if let Some(elem) = self.document.get_element_by_id(environment.as_ident()) {
                    let _ = elem.class_list().add_1("completed");
                }
            }
        }
    }

    pub fn update_current_variants(&self, state: &State) {
        let mut available = vec![];
        let mut unavailable = vec![];

        for variant in state.available_variants() {
            if variant.is_available(state, &self.current_game) {
                available.push(variant);
            } else {
                unavailable.push(variant);
            }
        }

        self.active_variants.set_inner_html("");
        for variant in available {
            self.create_variant_elem(state, variant, true);
        }

        for variant in unavailable {
            self.create_variant_elem(state, variant, false);
        }
    }

    fn create_variant_elem(&self, state: &State, variant: Variant, available: bool) {
        let new = self.document.create_element("div").expect("Failed to create child element");
        if available {
            let _ = new.class_list().add_1("variant-unlock");
        } else {
            let _ = new.class_list().add_2("variant-unlock", "unavailable");
        }
        let mut buf = format!(
            "<h3>{}</h3><p>{}</p><div class=\"buttons\">",
            variant.as_str(),
            variant.as_desc().replace("_", "\u{00A0}").replace("\\", "<br />")
        );
        for button_row in variant.buttons(state, &self.current_game) {
            let _ = write!(buf, "<div>");
            for button in button_row {
                if available {
                    let _ = write!(
                        buf,
                        "<button{} class=\"{}\">{}</button>",
                        if let Some(id) = button.id { format!(" id=\"{id}\"") } else { String::new() },
                        button.class_name(),
                        button.text
                    );
                } else {
                    let _ = write!(
                        buf,
                        "<button{} class=\"disabled\">{}</button>",
                        if let Some(id) = button.id { format!(" id=\"{id}\"") } else { String::new() },
                        button.text
                    );
                }
            }
            let _ = write!(buf, "</div>");
        }
        let _ = write!(buf, "</div>");
        new.set_inner_html(&buf);
        let _ = self.active_variants.append_child(&new);
    }

    pub fn update_goal(&self, state: &State) {
        let progress = state.goal_progress();

        if progress.available() {
            let _ = self.goal.class_list().add_1("available");
            self.goal.set_inner_html("Goal");
        } else {
            let _ = self.goal.class_list().remove_1("available");
            self.goal.set_inner_html(
                &[
                    (progress.scions, progress.required_scions, "Scions"),
                    (progress.villains, progress.required_villains, "Villains"),
                    (progress.variants, progress.required_variants, "Variants"),
                ]
                .iter()
                .filter(|(current, required, _)| current < required)
                .map(|(current, required, label)| format!("{current}/{required} {label}"))
                .collect::<Vec<_>>()
                .join("<br />"),
            );
        }
    }
}

impl CurrentVillains {
    pub fn deathlink_name(&self) -> &'static str {
        match self {
            CurrentVillains::Classic((villain, _, _)) => villain.as_str(),
            CurrentVillains::Team(_) => "a team of villains",
            CurrentVillains::Gladiators(_) => "the gladiators",
            CurrentVillains::None => "a mysterious force",
        }
    }
}
