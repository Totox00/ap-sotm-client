use utils::greatest;

#[derive(Debug, Clone, Copy, Default)]
pub struct PersistentVariantProgress {
    pub eternal_haka: bool,
    pub omnitron_u: u8,
    pub action_hero_stuntman: u8,
    pub benchmark_supply_and_demand: bool,
    pub heroic_luminary: u8,
    pub hunted_naturalist: u8,
    pub termi_nation_bunker: bool,
    pub freedom_six: u8,
    pub prime_wardens_argent_adept: bool,
    pub freedom_five: u8,
    pub first_response_echelon: u8,
    pub necro_last_of_the_forgotten_order: bool,
    pub first_response_vanish: bool,
    pub baron_blade_black_hole_generator: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TemporaryVariantProgress {
    pub spite_agent_of_gloom: bool,
    pub skinwalker_gloomweaver: u8,
    pub trickster_kismet: (u8, u8),
    pub heroic_infinitor: u8,
    pub americas_greatest_legacy: bool,
    pub dark_visionary: bool,
    pub eternal_haka: u8,
    pub gi_bunker: u8,
    pub ra_setting_sun: u8,
    pub redeemer_fanatic: u8,
    pub the_visionary_unleashed: u8,
    pub captain_cosmic_requital: u8,
    pub chrono_ranger_best_of_times: u8,
    pub dark_conductor_argent_adept: u8,
    pub sky_scraper_extremist: u8,
    pub omnitron_u: u8,
    pub santa_guise: u8,
    pub the_scholar_of_the_infinite: u8,
    pub action_hero_stuntman: bool,
    pub akash_thriya_spirit_of_the_void: (u8, u8),
    pub benchmark_supply_and_demand: (u8, u8),
    pub heroic_luminary: u8,
    pub knyfe_rogue_agent: u8,
    pub la_comodora_curse_of_the_black_spot: u8,
    pub lifeline_blood_mage: (u8, u8),
    pub parse_fugue_state: u8,
    pub adamant_sentinels: u8,
    pub hunted_naturalist: u8,
    pub termi_nation_bunker: bool,
    pub freedom_six_absolute_zero: u8,
    pub freedom_six_bunker: u8,
    pub freedom_six_tachyon: u8,
    pub freedom_six_tempest: u8,
    pub freedom_six_wraith: u8,
    pub freedom_six_unity: bool,
    pub dark_watch_expatriette: bool,
    pub dark_watch_harpy: u8,
    pub prime_wardens_argent_adept: u8,
    pub prime_wardens_captain_cosmic: u8,
    pub prime_wardens_fanatic: u8,
    pub prime_wardens_haka: u8,
    pub prime_wardens_tempest: u8,
    pub xtreme_prime_wardens_argent_adept: u16,
    pub xtreme_prime_wardens_captain_cosmic: u8,
    pub xtreme_prime_wardens_tempest: u8,
    pub xtreme_prime_wardens_fanatic: u8,
    pub xtreme_prime_wardens_haka: u8,
    pub freedom_five_prereq_advanced: bool,
    pub freedom_five_absolute_zero: (u8, u8, bool),
    pub freedom_five_bunker: (u8, u16),
    pub freedom_five_wraith: u8,
    pub freedom_five_tachyon: u8,
    pub freedom_five_legacy: (u8, u8, bool),
    pub super_sentai_idealist: u8,
    pub cosmic_inventor_writhe: u8,
    pub dr_medico_malpractice: u8,
    pub road_warrior_mainstay: u8,
    pub hydra_tiamat: u8,
    pub first_response_cricket: u8,
    pub the_cricket_renegade: u8,
    pub first_response_cypher: u8,
    pub cypher_swarming_protocol: u8,
    pub first_response_doc_havoc: (u8, u16),
    pub first_response_echelon: bool,
    pub necro_last_of_the_forgotten_order: bool,
    pub first_response_vanish: u8,
    pub omnitron_tech_terror: u8,
    pub baron_blade_black_hole_generator: u8,
    pub akash_bhuta_primordial_creator: bool,
    pub gloomweaver_ritual_of_gnophos: u8,
    pub omnitron_vi_hunter_killer: u8,
    pub berserk_haka: u8,
    pub citizen_dawn_solar_empress: u8,
    pub deadline_angel_of_extinction: bool,
    pub wounded_shape_anathema: u16,
    pub the_thaumaturgy_scholar: u8,
}

impl PersistentVariantProgress {
    pub const fn size() -> usize {
        14
    }

    pub fn as_bytes(&self) -> [u8; Self::size()] {
        [
            if self.eternal_haka { 1 } else { 0 },
            self.omnitron_u,
            self.action_hero_stuntman,
            if self.benchmark_supply_and_demand { 1 } else { 0 },
            self.heroic_luminary,
            self.hunted_naturalist,
            if self.termi_nation_bunker { 1 } else { 0 },
            self.freedom_six,
            if self.prime_wardens_argent_adept { 1 } else { 0 },
            self.freedom_five,
            self.first_response_echelon,
            if self.necro_last_of_the_forgotten_order { 1 } else { 0 },
            if self.first_response_vanish { 1 } else { 0 },
            if self.baron_blade_black_hole_generator { 1 } else { 0 },
        ]
    }

    pub fn update(&mut self, other: &PersistentVariantProgress) {
        self.eternal_haka |= other.eternal_haka;
        greatest!(omnitron_u);
        greatest!(action_hero_stuntman);
        self.benchmark_supply_and_demand |= other.benchmark_supply_and_demand;
        greatest!(heroic_luminary);
        self.hunted_naturalist |= other.hunted_naturalist;
        self.termi_nation_bunker |= other.termi_nation_bunker;
        greatest!(freedom_six);
        self.prime_wardens_argent_adept |= other.prime_wardens_argent_adept;
        greatest!(freedom_five);
        self.first_response_echelon |= other.first_response_echelon;
        self.necro_last_of_the_forgotten_order |= other.necro_last_of_the_forgotten_order;
        self.first_response_vanish |= other.first_response_vanish;
        self.baron_blade_black_hole_generator |= other.baron_blade_black_hole_generator;
    }
}

impl From<&[u8]> for PersistentVariantProgress {
    fn from(value: &[u8]) -> Self {
        PersistentVariantProgress {
            eternal_haka: value.first().copied().unwrap_or_default() > 0,
            omnitron_u: value.get(1).copied().unwrap_or_default(),
            action_hero_stuntman: value.get(2).copied().unwrap_or_default(),
            benchmark_supply_and_demand: value.get(3).copied().unwrap_or_default() > 0,
            heroic_luminary: value.get(4).copied().unwrap_or_default(),
            hunted_naturalist: value.get(5).copied().unwrap_or_default(),
            termi_nation_bunker: value.get(6).copied().unwrap_or_default() > 0,
            freedom_six: value.get(7).copied().unwrap_or_default(),
            prime_wardens_argent_adept: value.get(8).copied().unwrap_or_default() > 0,
            freedom_five: value.get(9).copied().unwrap_or_default(),
            first_response_echelon: value.get(10).copied().unwrap_or_default(),
            necro_last_of_the_forgotten_order: value.get(11).copied().unwrap_or_default() > 0,
            first_response_vanish: value.get(12).copied().unwrap_or_default() > 0,
            baron_blade_black_hole_generator: value.get(13).copied().unwrap_or_default() > 0
        }
    }
}
