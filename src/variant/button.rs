use crate::{
    data::{Environment, Hero, TeamVillain, Variant, Villain},
    game::CurrentGame,
    interface::CurrentVillains,
    state::State,
};

#[derive(Debug, Clone)]
pub struct Button {
    pub text: String,
    pub class: ButtonClass,
    pub id: Option<&'static str>,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonClass {
    Done,
    Available,
    Restricted,
}

impl Variant {
    pub fn buttons(&self, state: &State, game: &CurrentGame) -> Vec<Vec<Button>> {
        match self {
            Variant::MadBomberBaronBlade => vec![vec![
                Button::auto("Baron Blade", !state.checked_locations.has_unchecked_villain(Villain::BaronBlade, 0)),
                Button::restricted_unlock(
                    !state.checked_locations.has_unchecked_villain(Villain::BaronBlade, 0) && (game.is_classic(Villain::CitizenDawn) || game.is_classic(Villain::CitizenDawnSolarEmpress)),
                    "unlock-MadBomberBaronBlade",
                ),
            ]],
            Variant::OmnitronII => vec![vec![
                Button::auto("Omnitron", !state.checked_locations.has_unchecked_villain(Villain::Omnitron, 0)),
                Button::restricted_unlock(
                    !state.checked_locations.has_unchecked_villain(Villain::Omnitron, 0) && (game.is_classic(Villain::GrandWarlordVoss) || game.is_classic(Villain::GrandWarlordVossGeneBoundGeneral)),
                    "unlock-OmnitronII",
                ),
            ]],
            Variant::SpiteAgentOfGloom => vec![vec![
                Button::auto("Spite", !state.checked_locations.has_unchecked_villain(Villain::Spite, 0)),
                Button::restricted_toggle(
                    "GloomWeaver",
                    !state.checked_locations.has_unchecked_villain(Villain::Spite, 0) && game.is_classic(Villain::Gloomweaver),
                    state.temporary_variant_progress.spite_agent_of_gloom,
                    "toggle-SpiteAgentOfGloom",
                ),
            ]],
            Variant::SkinwalkerGloomweaver => {
                let final_game = !state.checked_locations.has_unchecked_villain(Villain::Spite, 0)
                    && !state.checked_locations.has_unchecked_villain(Villain::Gloomweaver, 0)
                    && game.is_classic(Villain::SpiteAgentOfGloom)
                    && game.environment(Environment::RookCity);

                vec![
                    vec![
                        Button::auto("Spite", !state.checked_locations.has_unchecked_villain(Villain::Spite, 0)),
                        Button::auto("GloomWeaver", !state.checked_locations.has_unchecked_villain(Villain::Gloomweaver, 0)),
                    ],
                    vec![Button::neutral_toggle(
                        if state.temporary_variant_progress.skinwalker_gloomweaver & 0x1 == 0 {
                            "Agent of Gloom"
                        } else {
                            "Broken Vessel"
                        },
                        "toggle-SkinwalkerGloomweaver-side",
                    )],
                    vec![
                        Button::restricted_toggle(
                            "Empty Safe House",
                            final_game,
                            state.temporary_variant_progress.skinwalker_gloomweaver & 0x2 > 0,
                            "toggle-SkinwalkerGloomweaver-safehouse",
                        ),
                        Button::restricted_toggle(
                            "Max 2 Drugs",
                            final_game,
                            state.temporary_variant_progress.skinwalker_gloomweaver & 0x4 == 0,
                            "toggle-SkinwalkerGloomweaver-drugs",
                        ),
                    ],
                    vec![
                        Button::restricted_toggle(
                            "5 Victims",
                            final_game,
                            state.temporary_variant_progress.skinwalker_gloomweaver & 0x8 > 0,
                            "toggle-SkinwalkerGloomweaver-victims",
                        ),
                        Button::restricted_toggle(
                            format!("Destroyed Victims: {}", state.temporary_variant_progress.skinwalker_gloomweaver >> 4),
                            final_game,
                            state.temporary_variant_progress.skinwalker_gloomweaver >> 4 >= 5,
                            "counter-SkinwalkerGloomweaver-victims",
                        ),
                    ],
                ]
            }
            Variant::TricksterKismet => {
                let done = state.temporary_variant_progress.trickster_kismet.0 > state.temporary_variant_progress.trickster_kismet.1;
                vec![vec![
                    Button::toggle(format!("Hero Turns: {}", state.temporary_variant_progress.trickster_kismet.0), done, "counter-TricksterKismet-hero"),
                    Button::toggle(
                        format!("Villain Turns: {}", state.temporary_variant_progress.trickster_kismet.1),
                        done,
                        "counter-TricksterKismet-villain",
                    ),
                ]]
            }
            Variant::HeroicInfinitor => {
                vec![
                    vec![
                        Button::toggle("Construct Final Blow", state.temporary_variant_progress.heroic_infinitor & 0x20 > 0, "toggle-HeroicInfinitor-construct"),
                        Button::toggle(
                            "No Incapacitated",
                            state.temporary_variant_progress.heroic_infinitor & 0x40 == 0,
                            "toggle-HeroicInfinitor-incapacitated",
                        ),
                    ],
                    Button::buttons(
                        &[
                            ("Crushing Cage", "toggle-HeroicInfinitor-0"),
                            ("Lambent Reaper", "toggle-HeroicInfinitor-1"),
                            ("Ocular Swarm", "toggle-HeroicInfinitor-2"),
                        ],
                        state.temporary_variant_progress.heroic_infinitor,
                    ),
                    Button::buttons(
                        &[("Recalescent Hellion", "toggle-HeroicInfinitor-3"), ("Twisted Miscreation", "toggle-HeroicInfinitor-4")],
                        state.temporary_variant_progress.heroic_infinitor >> 3,
                    ),
                ]
            }
            Variant::AmericasGreatestLegacy => vec![vec![Button::toggle(
                "All Max 9 HP",
                state.temporary_variant_progress.americas_greatest_legacy,
                "toggle-AmericasGreatestLegacy",
            )]],
            Variant::AmericasNewestLegacy => Button::unlock_single("unlock-AmericasNewestLegacy"),
            Variant::DarkVisionary => vec![vec![Button::toggle("Final Blow", state.temporary_variant_progress.dark_visionary, "toggle-DarkVisionary")]],
            Variant::TheEternalHaka => vec![
                vec![Button::toggle(
                    "Only non-Incapacitated",
                    state.temporary_variant_progress.eternal_haka & 0x8 > 0,
                    "toggle-TheEternalHaka-last",
                )],
                Button::restricted_buttons(
                    &[
                        ("Battle", "toggle-TheEternalHaka-0"),
                        ("Shielding", "toggle-TheEternalHaka-1"),
                        ("Restoration", "toggle-TheEternalHaka-2"),
                    ],
                    state.temporary_variant_progress.eternal_haka,
                    state.persistent_variant_progress.eternal_haka,
                ),
            ],
            Variant::GIBunker => vec![Button::buttons(
                &[("Research", "toggle-GIBunker-0"), ("Turret", "toggle-GIBunker-1"), ("Upgrade", "toggle-GIBunker-2")],
                state.temporary_variant_progress.gi_bunker,
            )],
            Variant::RaHorusOfTwoHorizons => Button::unlock_single("unlock-RaHorusOfTwoHorizons"),
            Variant::RaSettingSun => {
                let reduced = state.temporary_variant_progress.ra_setting_sun & 0x1 > 0;
                let restored = state.temporary_variant_progress.ra_setting_sun & 0x2 > 0;
                vec![
                    vec![
                        Button::toggle("Reduced", reduced, "toggle-RaSettingSun-reduced"),
                        Button::restricted_toggle("Restored", reduced, restored, "toggle-RaSettingSun-restored"),
                    ],
                    vec![Button::toggle(
                        format!("Final Blows: {}", state.temporary_variant_progress.ra_setting_sun >> 2),
                        state.temporary_variant_progress.ra_setting_sun >> 2 >= 3,
                        "counter-RaSettingSun",
                    )],
                ]
            }
            Variant::RedeemerFanatic => {
                let undaunted = state.temporary_variant_progress.redeemer_fanatic & 0x1 > 0;
                let restored = state.temporary_variant_progress.redeemer_fanatic & 0x2 > 0;
                let prayer = state.temporary_variant_progress.redeemer_fanatic & 0x4 > 0;
                let absolution = state.temporary_variant_progress.redeemer_fanatic & 0x8 > 0;
                vec![
                    vec![
                        Button::toggle("Undaunted", undaunted, "toggle-RedeemerFanatic-undaunted"),
                        Button::toggle("Restored", restored, "toggle-RedeemerFanatic-restored"),
                    ],
                    vec![
                        Button::restricted_toggle("Prayer", restored, prayer, "toggle-RedeemerFanatic-prayer"),
                        Button::restricted_toggle("Absolution", prayer, absolution, "toggle-RedeemerFanatic-absolution"),
                    ],
                ]
            }
            Variant::RookCityWraith => Button::unlock_single("unlock-RookCityWraith"),
            Variant::TheSuperScientificTachyon => Button::unlock_single("unlock-TheSuperScientificTachyon"),
            Variant::TheVisionaryUnleashed => {
                vec![
                    Button::buttons(
                        &[
                            ("Melody", "toggle-TheVisionaryUnleashed-0"),
                            ("Harmony", "toggle-TheVisionaryUnleashed-1"),
                            ("Rhythm", "toggle-TheVisionaryUnleashed-2"),
                        ],
                        state.temporary_variant_progress.the_visionary_unleashed,
                    ),
                    vec![Button::toggle(
                        "Dark Visionary Incapacitated",
                        state.temporary_variant_progress.the_visionary_unleashed >> 3 & 1 > 0,
                        "toggle-TheVisionaryUnleashed-visionary",
                    )],
                    vec![Button::toggle(
                        "Argent Adept non-Incapacitated",
                        state.temporary_variant_progress.the_visionary_unleashed >> 4 & 1 == 0,
                        "toggle-TheVisionaryUnleashed-argentadept",
                    )],
                ]
            }
            Variant::CaptainCosmicRequital => {
                let manifestations = state.temporary_variant_progress.captain_cosmic_requital & 0xF;
                let constructs = state.temporary_variant_progress.captain_cosmic_requital >> 4 & 0x7;
                vec![
                    vec![Button::toggle(
                        "Final Blow",
                        state.temporary_variant_progress.captain_cosmic_requital & 0x80 > 0,
                        "toggle-CaptainCosmicRequital",
                    )],
                    vec![
                        Button::toggle(format!("Manifestations: {}", manifestations), manifestations >= 10, "counter-CaptainCosmicRequital-manifestations"),
                        Button::toggle(format!("Constructs: {}", constructs), constructs >= 5, "counter-CaptainCosmicRequital-constructs"),
                    ],
                ]
            }
            Variant::ChronoRangerTheBestOfTimes => vec![vec![
                Button::toggle(
                    "Tachyon",
                    state.temporary_variant_progress.chrono_ranger_best_of_times & 0x1 == 0,
                    "toggle-ChronoRangerTheBestOfTimes-tachyon",
                ),
                Button::toggle(
                    "Single Bounty",
                    state.temporary_variant_progress.chrono_ranger_best_of_times & 0x2 == 0,
                    "toggle-ChronoRangerTheBestOfTimes-bounty",
                ),
                Button::toggle(
                    "Final Blow",
                    state.temporary_variant_progress.chrono_ranger_best_of_times & 0x4 > 0,
                    "toggle-ChronoRangerTheBestOfTimes-final",
                ),
            ]],
            Variant::DarkConductorArgentAdept => vec![vec![
                Button::toggle("Selfish", state.temporary_variant_progress.dark_conductor_argent_adept & 0x1 == 0, "toggle-DarkConductorArgentAdept"),
                Button::toggle(
                    format!("Damage: {}", state.temporary_variant_progress.dark_conductor_argent_adept >> 1),
                    state.temporary_variant_progress.dark_conductor_argent_adept >> 1 >= 20,
                    "counter-DarkConductorArgentAdept",
                ),
            ]],
            Variant::ExtremistSkyScraper => vec![vec![Button::toggle(
                format!("Extremism: {}", state.temporary_variant_progress.sky_scraper_extremist),
                state.temporary_variant_progress.sky_scraper_extremist >= 3,
                "counter-ExtremistSkyScraper",
            )]],
            Variant::OmnitronU => {
                let last_game = state.persistent_variant_progress.omnitron_u > 1;
                vec![
                    vec![
                        Button::auto("Omnitron", state.persistent_variant_progress.omnitron_u > 0),
                        Button::auto("Cosmic Omnitron", state.persistent_variant_progress.omnitron_u > 1),
                    ],
                    vec![
                        Button::restricted_toggle(
                            if state.persistent_variant_progress.omnitron_u == 0 {
                                "Omnitron-X Not Incapacitated"
                            } else {
                                "Omnitron-X Incapacitated"
                            },
                            !last_game,
                            state.temporary_variant_progress.omnitron_u & 0x1 ^ state.persistent_variant_progress.omnitron_u == 0,
                            "toggle-OmnitronU-omnitron",
                        ),
                        Button::restricted_toggle("Unity Not Incapacitated", !last_game, state.temporary_variant_progress.omnitron_u & 0x2 == 0, "toggle-OmnitronU-unity"),
                    ],
                    vec![
                        Button::restricted_toggle("Equipment", last_game, state.temporary_variant_progress.omnitron_u & 0x4 > 0, "toggle-OmnitronU-equipment"),
                        Button::restricted_toggle("Robot Reclamation", last_game, state.temporary_variant_progress.omnitron_u & 0x8 > 0, "toggle-OmnitronU-reclamation"),
                    ],
                ]
            }
            Variant::SantaGuise => vec![vec![Button::toggle(
                format!("Actions: {}", state.temporary_variant_progress.santa_guise),
                state.temporary_variant_progress.santa_guise >= 25,
                "counter-SantaGuise",
            )]],
            Variant::TheScholarOfTheInfinite => vec![vec![
                Button::toggle(
                    "Incapacitated",
                    state.temporary_variant_progress.the_scholar_of_the_infinite & 0x1 > 0,
                    "toggle-TheScholarOfTheInfinite",
                ),
                Button::toggle(
                    format!("Healing: {}", state.temporary_variant_progress.the_scholar_of_the_infinite >> 1),
                    state.temporary_variant_progress.the_scholar_of_the_infinite >> 1 >= 20,
                    "counter-TheScholarOfTheInfinite",
                ),
            ]],
            Variant::ActionHeroStuntman => vec![
                vec![
                    Button::auto("Classic", state.persistent_variant_progress.action_hero_stuntman > 0),
                    Button::auto("Team", state.persistent_variant_progress.action_hero_stuntman > 1),
                ],
                vec![
                    Button::restricted_toggle(
                        "No Incapacitated",
                        state.persistent_variant_progress.action_hero_stuntman < 2,
                        state.temporary_variant_progress.action_hero_stuntman,
                        "toggle-ActionHeroStuntman-incapacitated",
                    ),
                    Button::restricted_toggle(
                        "Only Mainstay",
                        state.persistent_variant_progress.action_hero_stuntman == 2,
                        state.temporary_variant_progress.action_hero_stuntman,
                        "toggle-ActionHeroStuntman-mainstay",
                    ),
                ],
            ],
            Variant::AkashThriyaSpiritOfTheVoid => {
                vec![
                    Button::buttons(
                        &[("Creeping Mold", "toggle-AkashThriyaSpiritOfTheVoid-0"), ("Healing Pollen", "toggle-AkashThriyaSpiritOfTheVoid-1")],
                        state.temporary_variant_progress.akash_thriya_spirit_of_the_void.0,
                    ),
                    Button::buttons(
                        &[
                            ("Noxious Pod", "toggle-AkashThriyaSpiritOfTheVoid-2"),
                            ("Strangling Roots", "toggle-AkashThriyaSpiritOfTheVoid-3"),
                            ("Vitalized Thorns", "toggle-AkashThriyaSpiritOfTheVoid-4"),
                        ],
                        state.temporary_variant_progress.akash_thriya_spirit_of_the_void.0 >> 2,
                    ),
                    vec![
                        Button::toggle(
                            "Reduced",
                            state.temporary_variant_progress.akash_thriya_spirit_of_the_void.1 & 0x1 > 0,
                            "toggle-AkashThriyaSpiritOfTheVoid-reduced",
                        ),
                        Button::toggle(
                            "Recover",
                            state.temporary_variant_progress.akash_thriya_spirit_of_the_void.1 & 0x2 > 0,
                            "toggle-AkashThriyaSpiritOfTheVoid-recover",
                        ),
                    ],
                    vec![
                        Button::toggle(
                            "Akash'Flora",
                            state.temporary_variant_progress.akash_thriya_spirit_of_the_void.1 & 0x4 > 0,
                            "toggle-AkashThriyaSpiritOfTheVoid-akashflora",
                        ),
                        Button::toggle(
                            "Akash'Flora Destroyed",
                            state.temporary_variant_progress.akash_thriya_spirit_of_the_void.1 & 0x8 == 0,
                            "toggle-AkashThriyaSpiritOfTheVoid-akashfloradestroyed",
                        ),
                    ],
                ]
            }
            Variant::BenchmarkSupplyAndDemand => {
                if state.persistent_variant_progress.benchmark_supply_and_demand {
                    vec![
                        vec![Button::auto("Equipment", true), Button::auto("Devices", true)],
                        vec![Button::unlock("unlock-BenchmarkSupplyAndDemand")],
                    ]
                } else if let CurrentVillains::Team(villains) = &game.villains {
                    let needed = (10
                        - game
                            .current_bases()
                            .filter(|base| [Hero::Benchmark, Hero::Expatriette, Hero::Luminary, Hero::Parse, Hero::Setback].contains(base))
                            .count()
                        - villains
                            .iter()
                            .filter(|(villain, _, _)| {
                                [
                                    TeamVillain::TeamAmbuscade,
                                    TeamVillain::TeamBaronBlade,
                                    TeamVillain::TeamFriction,
                                    TeamVillain::TeamFrightTrain,
                                    TeamVillain::TeamPlagueRat,
                                ]
                                .contains(villain)
                            })
                            .count()) as u8;
                    vec![
                        vec![
                            Button::toggle(
                                format!("Equipment: {}", state.temporary_variant_progress.benchmark_supply_and_demand.0),
                                state.temporary_variant_progress.benchmark_supply_and_demand.0 >= needed * 10,
                                "counter-BenchmarkSupplyAndDemand-equipment",
                            ),
                            Button::toggle(
                                format!("Devices: {}", state.temporary_variant_progress.benchmark_supply_and_demand.1),
                                state.temporary_variant_progress.benchmark_supply_and_demand.1 >= needed * 5,
                                "counter-BenchmarkSupplyAndDemand-devices",
                            ),
                        ],
                        vec![Button::new("Unlock", ButtonClass::Restricted, None)],
                    ]
                } else {
                    vec![
                        vec![Button::new("Equipment: 0", ButtonClass::Restricted, None), Button::new("Devices: 0", ButtonClass::Restricted, None)],
                        vec![Button::new("Unlock", ButtonClass::Restricted, None)],
                    ]
                }
            }
            Variant::HeroicLuminary => vec![
                vec![
                    Button::auto("Baron Blade", state.persistent_variant_progress.heroic_luminary > 0),
                    Button::auto("Freedom Tower", state.persistent_variant_progress.heroic_luminary > 1),
                ],
                Button::restricted_buttons(
                    &[
                        ("Explosive Reconstructor", "toggle-HeroicLuminary-0"),
                        ("Orbital Death-Laser", "toggle-HeroicLuminary-1"),
                        ("Terralunar Translocator", "toggle-HeroicLuminary-2"),
                    ],
                    state.temporary_variant_progress.heroic_luminary,
                    state.persistent_variant_progress.heroic_luminary > 1,
                ),
            ],
            Variant::KnyfeRogueAgent => vec![vec![
                Button::toggle(
                    format!("Agents: {}", state.temporary_variant_progress.knyfe_rogue_agent >> 1),
                    state.temporary_variant_progress.knyfe_rogue_agent >> 1 >= 5,
                    "counter-KnyfeRogueAgent",
                ),
                Button::toggle("Warden Hoefle", state.temporary_variant_progress.knyfe_rogue_agent & 0x1 == 0, "toggle-KnyfeRogueAgent"),
            ]],
            Variant::LaComodoraCurseOfTheBlackSpot => {
                let play = state.temporary_variant_progress.la_comodora_curse_of_the_black_spot & 0x1 > 0;
                let trash = state.temporary_variant_progress.la_comodora_curse_of_the_black_spot & 0x2 > 0;
                let run_aground = state.temporary_variant_progress.la_comodora_curse_of_the_black_spot & 0x4 > 0;
                vec![vec![
                    Button::toggle("Play", play, "toggle-LaComodoraCurseOfTheBlackSpot-play"),
                    Button::restricted_toggle("Trash", play, trash, "toggle-LaComodoraCurseOfTheBlackSpot-trash"),
                    Button::restricted_toggle("Run Aground", trash, run_aground, "toggle-LaComodoraCurseOfTheBlackSpot-runaground"),
                ]]
            }
            Variant::LifelineBloodMage => {
                let mut buttons: Vec<Vec<Button>> = Button::buttons(
                    &[
                        ("Calculated Action", "toggle-LifelineBloodMage-0"),
                        ("Enclave's Tech", "toggle-LifelineBloodMage-1"),
                        ("Harrow", "toggle-LifelineBloodMage-2"),
                        ("Haunting Memories", "toggle-LifelineBloodMage-3"),
                        ("Ley-Line Shift", "toggle-LifelineBloodMage-4"),
                        ("Repair Ley-Line", "toggle-LifelineBloodMage-5"),
                        ("Unleash Energy", "toggle-LifelineBloodMage-6"),
                        ("Unnatural Upheaval", "toggle-LifelineBloodMage-7"),
                    ],
                    state.temporary_variant_progress.lifeline_blood_mage.0,
                )
                .rchunks(3)
                .map(Vec::from)
                .collect();

                buttons.push(vec![
                    Button::toggle(
                        "Blood Countess Bathory",
                        state.temporary_variant_progress.lifeline_blood_mage.1 & 0x1 > 0,
                        "toggle-LifelineBloodMage-bathory",
                    ),
                    Button::toggle(
                        "No Other Incapacitated",
                        state.temporary_variant_progress.lifeline_blood_mage.1 & 0x2 == 0,
                        "toggle-LifelineBloodMage-incapacitated",
                    ),
                ]);

                buttons
            }
            Variant::ParseFugueState => vec![vec![
                Button::toggle("10 Ongoings", state.temporary_variant_progress.parse_fugue_state & 0x1 > 0, "toggle-ParseFugueState-ongoings"),
                Button::toggle("1 HP", state.temporary_variant_progress.parse_fugue_state & 0x2 > 0, "toggle-ParseFugueState-hp"),
            ]],
            Variant::TheAdamantSentinels => vec![vec![
                Button::toggle(
                    "First Incapacitated",
                    state.temporary_variant_progress.adamant_sentinels & 0x1 > 0,
                    "toggle-TheAdamantSentinels-incapacitated",
                ),
                Button::toggle("Signatures", state.temporary_variant_progress.adamant_sentinels & 0x2 > 0, "toggle-TheAdamantSentinels-signatures"),
            ]],
            Variant::TheHuntedNaturalist => vec![
                Button::buttons_auto(&["Crocodile", "Gazelle", "Rhinoceros"], state.persistent_variant_progress.hunted_naturalist),
                vec![
                    Button::neutral_toggle(
                        format!(
                            "Current Form: {}",
                            ["Crocodile", "Gazelle", "Rhinoceros"][(state.temporary_variant_progress.hunted_naturalist >> 1) as usize]
                        ),
                        "switch-TheHuntedNaturalist",
                    ),
                    Button::toggle("Final Blow", state.temporary_variant_progress.hunted_naturalist & 0x1 > 0, "toggle-TheHuntedNaturalist-final"),
                ],
            ],
            Variant::TermiNationBunker => {
                vec![vec![
                    Button::auto("Omnitron", state.persistent_variant_progress.termi_nation_bunker),
                    Button::toggle("<10 HP", state.temporary_variant_progress.termi_nation_bunker, "toggle-TermiNationBunker"),
                ]]
            }
            Variant::TermiNationAbsoluteZero => Button::unlock_single("unlock-TermiNationAbsoluteZero"),
            Variant::TermiNationUnity => Button::unlock_single("unlock-TermiNationUnity"),
            Variant::FreedomSixAbsoluteZero => vec![
                vec![
                    Button::auto("Iron Legacy", state.persistent_variant_progress.freedom_six & 0x1 > 0),
                    Button::restricted_toggle(
                        "Not Incapacitated",
                        state.persistent_variant_progress.freedom_six & 0x1 > 0,
                        state.temporary_variant_progress.freedom_six_absolute_zero & 0x10 == 0,
                        "toggle-FreedomSixAbsoluteZero-incapacitated",
                    ),
                ],
                Button::restricted_buttons(
                    &[("Cryo Chamber", "toggle-FreedomSixAbsoluteZero-0"), ("Focused Apertures", "toggle-FreedomSixAbsoluteZero-1")],
                    state.temporary_variant_progress.freedom_six_absolute_zero,
                    state.persistent_variant_progress.freedom_six & 0x1 > 0,
                ),
                Button::restricted_buttons(
                    &[
                        ("Isothermic Transducer", "toggle-FreedomSixAbsoluteZero-2"),
                        ("Null-Point Calibration Unit", "toggle-FreedomSixAbsoluteZero-3"),
                    ],
                    state.temporary_variant_progress.freedom_six_absolute_zero >> 2,
                    state.persistent_variant_progress.freedom_six & 0x1 > 0,
                ),
            ],
            Variant::FreedomSixBunker => vec![
                vec![Button::auto("Iron Legacy", state.persistent_variant_progress.freedom_six & 0x2 > 0)],
                vec![
                    Button::restricted_toggle(
                        format!("Cards Drawn: {}", state.temporary_variant_progress.freedom_six_bunker >> 1),
                        state.persistent_variant_progress.freedom_six & 0x2 > 0,
                        state.temporary_variant_progress.freedom_six_bunker >> 1 >= 12,
                        "counter-FreedomSixBunker",
                    ),
                    Button::restricted_toggle(
                        "Only Incapacitated",
                        state.persistent_variant_progress.freedom_six & 0x2 > 0,
                        state.temporary_variant_progress.freedom_six_bunker & 0x1 > 0,
                        "toggle-FreedomSixBunker",
                    ),
                ],
            ],
            Variant::FreedomSixTachyon => vec![vec![
                Button::auto("Iron Legacy", state.persistent_variant_progress.freedom_six & 0x4 > 0),
                Button::restricted_toggle(
                    format!("Fleet of Foot: {}", state.temporary_variant_progress.freedom_six_tachyon),
                    state.persistent_variant_progress.freedom_six & 0x4 > 0,
                    state.temporary_variant_progress.freedom_six_tachyon >= 5,
                    "counter-FreedomSixTachyon",
                ),
            ]],
            Variant::FreedomSixTempest => vec![
                vec![
                    Button::auto("Iron Legacy", state.persistent_variant_progress.freedom_six & 0x8 > 0),
                    Button::restricted_toggle(
                        "Only Incapacitated",
                        state.persistent_variant_progress.freedom_six & 0x8 > 0,
                        state.temporary_variant_progress.freedom_six_tempest & 0x1 > 0,
                        "toggle-FreedomSixTempest-incapacitated",
                    ),
                ],
                Button::restricted_buttons(
                    &[
                        ("Aquatic Correspondence", "toggle-FreedomSixTempest-0"),
                        ("Localized Hurricane", "toggle-FreedomSixTempest-1"),
                        ("Reclaim from the Deep", "toggle-FreedomSixTempest-2"),
                    ],
                    state.temporary_variant_progress.freedom_six_tempest >> 1,
                    state.persistent_variant_progress.freedom_six & 0x8 > 0,
                ),
            ],
            Variant::FreedomSixWraith => vec![
                vec![Button::auto("Iron Legacy", state.persistent_variant_progress.freedom_six & 0x10 > 0)],
                Button::restricted_buttons(
                    &[("Final Blow: The Chairman", "toggle-FreedomSixWraith-0"), ("Final Blow: The Operative", "toggle-FreedomSixWraith-1")],
                    state.temporary_variant_progress.freedom_six_wraith,
                    state.persistent_variant_progress.freedom_six & 0x10 > 0,
                ),
            ],
            Variant::FreedomSixUnity => vec![vec![
                Button::auto("Iron Legacy", state.persistent_variant_progress.freedom_six & 0x20 > 0),
                Button::restricted_toggle(
                    "More Golems",
                    state.persistent_variant_progress.freedom_six & 0x20 > 0,
                    state.temporary_variant_progress.freedom_six_unity,
                    "toggle-FreedomSixUnity",
                ),
            ]],
            Variant::DarkWatchExpatriette => vec![vec![Button::toggle("Unload", state.temporary_variant_progress.dark_watch_expatriette, "toggle-DarkWatchExpatriette")]],
            Variant::DarkWatchMisterFixer => Button::unlock_single("unlock-DarkWatchMisterFixer"),
            Variant::DarkWatchNightmist => Button::unlock_single("unlock-DarkWatchNightmist"),
            Variant::DarkWatchSetback => vec![vec![Button::new("Unlock", ButtonClass::Restricted, None)]],
            Variant::DarkWatchHarpy => {
                let avian = state.temporary_variant_progress.dark_watch_harpy & 0x1 > 0;
                let arcana = state.temporary_variant_progress.dark_watch_harpy & 0x2 > 0;
                let incapacitated = state.temporary_variant_progress.dark_watch_harpy & 0x4 > 0;
                let relic = state.temporary_variant_progress.dark_watch_harpy & 0x8 > 0;
                let remain = state.temporary_variant_progress.dark_watch_harpy & 0x10 == 0;
                vec![
                    vec![
                        Button::toggle("Avian", avian, "toggle-DarkWatchHarpy-avian"),
                        Button::restricted_toggle("Arcana", avian, arcana, "toggle-DarkWatchHarpy-arcana"),
                        Button::restricted_toggle("Remain", arcana, remain, "toggle-DarkWatchHarpy-remain"),
                    ],
                    vec![
                        Button::toggle("Relic", relic, "toggle-DarkWatchHarpy-relic"),
                        Button::toggle("Not Incapacitated", incapacitated, "toggle-DarkWatchHarpy-incapacitated"),
                    ],
                ]
            }
            Variant::PrimeWardensArgentAdept => vec![
                vec![Button::auto("Akash'Bhuta", state.persistent_variant_progress.prime_wardens_argent_adept)],
                Button::restricted_buttons(
                    &[
                        ("Akpunku's Drum", "toggle-PrimeWardensArgentAdept-0"),
                        ("Drake's Pipes", "toggle-PrimeWardensArgentAdept-1"),
                        ("Eydisar's Horn", "toggle-PrimeWardensArgentAdept-2"),
                    ],
                    state.temporary_variant_progress.prime_wardens_argent_adept,
                    state.persistent_variant_progress.prime_wardens_argent_adept,
                ),
                Button::restricted_buttons(
                    &[
                        ("Musaragni's Harp", "toggle-PrimeWardensArgentAdept-3"),
                        ("Telamon's Lyra", "toggle-PrimeWardensArgentAdept-4"),
                        ("Xu's Bell", "toggle-PrimeWardensArgentAdept-5"),
                    ],
                    state.temporary_variant_progress.prime_wardens_argent_adept >> 3,
                    state.persistent_variant_progress.prime_wardens_argent_adept,
                ),
            ],
            Variant::PrimeWardensCaptainCosmic => vec![vec![Button::toggle(
                format!("Damage Prevented: {}", state.temporary_variant_progress.prime_wardens_captain_cosmic),
                state.temporary_variant_progress.prime_wardens_captain_cosmic >= 20,
                "counter-PrimeWardensCaptainCosmic",
            )]],
            Variant::PrimeWardensFanatic => vec![vec![
                Button::toggle(
                    format!("Imp Pilferers: {}", state.temporary_variant_progress.prime_wardens_fanatic & 0x3),
                    state.temporary_variant_progress.prime_wardens_fanatic & 0x3 >= 2,
                    "counter-PrimeWardensFanatic-0",
                ),
                Button::toggle(
                    format!("Fiendish Pugilists: {}", state.temporary_variant_progress.prime_wardens_fanatic >> 2 & 0x3),
                    state.temporary_variant_progress.prime_wardens_fanatic >> 2 & 0x3 >= 2,
                    "counter-PrimeWardensFanatic-1",
                ),
                Button::toggle(
                    format!("Relic Spirits: {}", state.temporary_variant_progress.prime_wardens_fanatic >> 4),
                    state.temporary_variant_progress.prime_wardens_fanatic >> 4 >= 2,
                    "counter-PrimeWardensFanatic-2",
                ),
            ]],
            Variant::PrimeWardensHaka => vec![vec![
                Button::toggle("Only Active", state.temporary_variant_progress.prime_wardens_haka & 0x1 > 0, "toggle-PrimeWardensHaka-active"),
                Button::toggle("Maximum HP", state.temporary_variant_progress.prime_wardens_haka & 0x2 == 0, "toggle-PrimeWardensHaka-hp"),
            ]],
            Variant::PrimeWardensTempest => vec![Button::buttons(
                &[
                    ("Ball Lightning", "toggle-PrimeWardensTempest-0"),
                    ("Chain Lightning", "toggle-PrimeWardensTempest-1"),
                    ("Lightning Slash", "toggle-PrimeWardensTempest-2"),
                ],
                state.temporary_variant_progress.prime_wardens_tempest,
            )],
            Variant::XtremePrimeWardensArgentAdept => Button::buttons(
                &[
                    ("Alacritous Subdominant", "toggle-XtremePrimeWardensArgentAdept-0"),
                    ("Cedistic Dissonant", "toggle-XtremePrimeWardensArgentAdept-1"),
                    ("Inspiring Supertonic", "toggle-XtremePrimeWardensArgentAdept-2"),
                    ("Counterpoint Bulwark", "toggle-XtremePrimeWardensArgentAdept-3"),
                    ("Inventive Preparation", "toggle-XtremePrimeWardensArgentAdept-4"),
                    ("Syncopated Onslaught", "toggle-XtremePrimeWardensArgentAdept-5"),
                    ("Rhapsody of Vigor", "toggle-XtremePrimeWardensArgentAdept-6"),
                    ("Sarabande of Destruction", "toggle-XtremePrimeWardensArgentAdept-7"),
                    ("Scherzo of Frost and Flame", "toggle-XtremePrimeWardensArgentAdept-8"),
                ],
                state.temporary_variant_progress.xtreme_prime_wardens_argent_adept,
            )
            .chunks_exact(3)
            .map(Vec::from)
            .collect(),
            Variant::XtremePrimeWardensTempest => vec![vec![
                Button::toggle(
                    "Damage",
                    state.temporary_variant_progress.xtreme_prime_wardens_tempest & 0x1 == 0,
                    "toggle-XtremePrimeWardensTempest-damage",
                ),
                Button::toggle(
                    "Equipment",
                    state.temporary_variant_progress.xtreme_prime_wardens_tempest & 0x2 > 0,
                    "toggle-XtremePrimeWardensTempest-equipment",
                ),
            ]],
            Variant::XtremePrimeWardensCaptainCosmic => vec![vec![
                Button::toggle(
                    format!("Entered: {}", state.temporary_variant_progress.xtreme_prime_wardens_captain_cosmic & 0xF),
                    state.temporary_variant_progress.xtreme_prime_wardens_captain_cosmic & 0xF >= 10,
                    "counter-XtremePrimeWardensCaptainCosmic-entered",
                ),
                Button::toggle(
                    format!("Destroyed: {}", state.temporary_variant_progress.xtreme_prime_wardens_captain_cosmic >> 4),
                    state.temporary_variant_progress.xtreme_prime_wardens_captain_cosmic >> 4 >= 10,
                    "counter-XtremePrimeWardensCaptainCosmic-destroyed",
                ),
            ]],
            Variant::XtremePrimeWardensFanatic => vec![
                vec![Button::toggle(
                    "No Infernal",
                    state.temporary_variant_progress.xtreme_prime_wardens_fanatic >> 6 == 0,
                    "toggle-XtremePrimeWardensFanatic-infernal",
                )],
                Button::buttons(
                    &[
                        ("Blood Countess Bathory", "toggle-XtremePrimeWardensFanatic-0"),
                        ("Dame Katarina", "toggle-XtremePrimeWardensFanatic-1"),
                        ("Dowager Ilona", "toggle-XtremePrimeWardensFanatic-2"),
                    ],
                    state.temporary_variant_progress.xtreme_prime_wardens_fanatic,
                ),
                Button::buttons(
                    &[
                        ("Drudge Ficko", "toggle-XtremePrimeWardensFanatic-3"),
                        ("Matron Erzsi", "toggle-XtremePrimeWardensFanatic-4"),
                        ("Relict Dorotya", "toggle-XtremePrimeWardensFanatic-5"),
                    ],
                    state.temporary_variant_progress.xtreme_prime_wardens_fanatic >> 3,
                ),
            ],
            Variant::XtremePrimeWardensHaka => vec![
                vec![Button::toggle(
                    "No Equipment",
                    state.temporary_variant_progress.xtreme_prime_wardens_haka >> 6 == 0,
                    "toggle-XtremePrimeWardensHaka-equipment",
                )],
                vec![
                    Button::toggle(
                        format!("Villain: {}", state.temporary_variant_progress.xtreme_prime_wardens_haka & 0x7),
                        state.temporary_variant_progress.xtreme_prime_wardens_haka & 0x7 >= 5,
                        "counter-XtremePrimeWardensHaka-villain",
                    ),
                    Button::toggle(
                        format!("Environment: {}", state.temporary_variant_progress.xtreme_prime_wardens_haka >> 3 & 0x7),
                        state.temporary_variant_progress.xtreme_prime_wardens_haka >> 3 & 0x7 >= 5,
                        "counter-XtremePrimeWardensHaka-environment",
                    ),
                ],
            ],
            Variant::FreedomFiveAbsoluteZero => vec![
                Button::freedom_five(state),
                vec![
                    Button::restricted_toggle(
                        format!("Fire: {}", state.temporary_variant_progress.freedom_five_absolute_zero.0),
                        state.persistent_variant_progress.freedom_five > 2,
                        state.temporary_variant_progress.freedom_five_absolute_zero.0 >= 29,
                        "counter-FreedomFiveAbsoluteZero-fire",
                    ),
                    Button::restricted_toggle(
                        format!("Cold: {}", state.temporary_variant_progress.freedom_five_absolute_zero.1),
                        state.persistent_variant_progress.freedom_five > 2,
                        state.temporary_variant_progress.freedom_five_absolute_zero.1 >= 29,
                        "counter-FreedomFiveAbsoluteZero-cold",
                    ),
                    Button::restricted_toggle(
                        "Ongoings",
                        state.persistent_variant_progress.freedom_five > 2,
                        state.temporary_variant_progress.freedom_five_absolute_zero.2,
                        "toggle-FreedomFiveAbsoluteZero-ongoings",
                    ),
                ],
            ],
            Variant::FreedomFiveBunker => {
                let mut buttons = vec![
                    Button::freedom_five(state),
                    vec![
                        Button::restricted_toggle(
                            format!("Recharge: {}", state.temporary_variant_progress.freedom_five_bunker.0 & 0x3),
                            state.persistent_variant_progress.freedom_five > 2,
                            state.temporary_variant_progress.freedom_five_bunker.0 & 0x3 >= 2,
                            "counter-FreedomFiveBunker-0",
                        ),
                        Button::restricted_toggle(
                            format!("Turret: {}", state.temporary_variant_progress.freedom_five_bunker.0 >> 2 & 0x3),
                            state.persistent_variant_progress.freedom_five > 2,
                            state.temporary_variant_progress.freedom_five_bunker.0 >> 2 & 0x3 >= 2,
                            "counter-FreedomFiveBunker-1",
                        ),
                        Button::restricted_toggle(
                            format!("Upgrade: {}", state.temporary_variant_progress.freedom_five_bunker.0 >> 4),
                            state.persistent_variant_progress.freedom_five > 2,
                            state.temporary_variant_progress.freedom_five_bunker.0 >> 4 >= 2,
                            "counter-FreedomFiveBunker-2",
                        ),
                    ],
                ];

                buttons.extend(
                    Button::restricted_buttons(
                        &[
                            ("Cold", "toggle-FreedomFiveBunker-0"),
                            ("Energy", "toggle-FreedomFiveBunker-1"),
                            ("Fire", "toggle-FreedomFiveBunker-2"),
                            ("Infernal", "toggle-FreedomFiveBunker-3"),
                            ("Lightning", "toggle-FreedomFiveBunker-4"),
                            ("Melee", "toggle-FreedomFiveBunker-5"),
                            ("Projectile", "toggle-FreedomFiveBunker-6"),
                            ("Psychic", "toggle-FreedomFiveBunker-7"),
                            ("Radiant", "toggle-FreedomFiveBunker-8"),
                            ("Sonic", "toggle-FreedomFiveBunker-9"),
                            ("Toxic", "toggle-FreedomFiveBunker-10"),
                        ],
                        state.temporary_variant_progress.freedom_five_bunker.1,
                        state.persistent_variant_progress.freedom_five > 2,
                    )
                    .chunks(3)
                    .map(Vec::from),
                );

                buttons
            }
            Variant::FreedomFiveWraith => vec![
                Button::freedom_five(state),
                vec![
                    Button::restricted_toggle(
                        format!("Trust Fund: {}", state.temporary_variant_progress.freedom_five_wraith & 0x3),
                        state.persistent_variant_progress.freedom_five > 2,
                        state.temporary_variant_progress.freedom_five_wraith & 0x3 >= 3,
                        "counter-FreedomFiveWraith-trustfund",
                    ),
                    Button::restricted_toggle(
                        format!("Cards: {}", state.temporary_variant_progress.freedom_five_wraith >> 2 & 0x1F),
                        state.persistent_variant_progress.freedom_five > 2,
                        state.temporary_variant_progress.freedom_five_wraith >> 2 & 0x1F >= 20,
                        "counter-FreedomFiveWraith-cards",
                    ),
                    Button::restricted_toggle(
                        "Smoke Bombs",
                        state.persistent_variant_progress.freedom_five > 2,
                        state.temporary_variant_progress.freedom_five_wraith >> 7 > 0,
                        "toggle-FreedomFiveWraith-smokebombs",
                    ),
                ],
            ],
            Variant::FreedomFiveTachyon => vec![
                Button::freedom_five(state),
                vec![
                    Button::restricted_toggle(
                        "Play",
                        state.persistent_variant_progress.freedom_five > 2,
                        state.temporary_variant_progress.freedom_five_tachyon & 0x1 > 0,
                        "toggle-FreedomFiveTachyon-play",
                    ),
                    Button::restricted_toggle(
                        "Shuffle",
                        state.persistent_variant_progress.freedom_five > 2,
                        state.temporary_variant_progress.freedom_five_tachyon & 0x2 > 0,
                        "toggle-FreedomFiveTachyon-shuffle",
                    ),
                    Button::restricted_toggle(
                        "Incapacitated",
                        state.persistent_variant_progress.freedom_five > 2,
                        state.temporary_variant_progress.freedom_five_tachyon & 0x4 > 0,
                        "toggle-FreedomFiveTachyon-incapacitated",
                    ),
                ],
            ],
            Variant::FreedomFiveLegacy => vec![
                Button::freedom_five(state),
                vec![
                    Button::restricted_toggle(
                        format!("Prevented: {}", state.temporary_variant_progress.freedom_five_legacy.0),
                        state.persistent_variant_progress.freedom_five > 2,
                        state.temporary_variant_progress.freedom_five_legacy.0 > state.temporary_variant_progress.freedom_five_legacy.1 && state.temporary_variant_progress.freedom_five_legacy.0 >= 20,
                        "counter-FreedomFiveLegacy-prevented",
                    ),
                    Button::restricted_toggle(
                        format!("Increased: {}", state.temporary_variant_progress.freedom_five_legacy.1),
                        state.persistent_variant_progress.freedom_five > 2,
                        state.temporary_variant_progress.freedom_five_legacy.0 > state.temporary_variant_progress.freedom_five_legacy.1 && state.temporary_variant_progress.freedom_five_legacy.0 >= 20,
                        "counter-FreedomFiveLegacy-increased",
                    ),
                    Button::restricted_toggle(
                        "Danger Sense",
                        state.persistent_variant_progress.freedom_five > 2,
                        state.temporary_variant_progress.freedom_five_legacy.2,
                        "toggle-FreedomFiveLegacy",
                    ),
                ],
            ],
            Variant::SuperSentaiIdealist => vec![vec![
                Button::toggle(
                    format!("Rounds: {}", state.temporary_variant_progress.super_sentai_idealist),
                    state.temporary_variant_progress.super_sentai_idealist >= 2,
                    "counter-SuperSentaiIdealist",
                ),
                Button::reset("reset-SuperSentaiIdealist"),
                Button::restricted_unlock(state.temporary_variant_progress.super_sentai_idealist >= 2, "unlock-SuperSentaiIdealist"),
            ]],
            Variant::DrMedicoMalpractice => vec![vec![
                Button::toggle(
                    format!("Damage taken: {}", state.temporary_variant_progress.dr_medico_malpractice),
                    state.temporary_variant_progress.dr_medico_malpractice >= 50,
                    "counter-DrMedicoMalpractice",
                ),
                Button::restricted_unlock(state.temporary_variant_progress.dr_medico_malpractice >= 50, "unlock-DrMedicoMalpractice"),
            ]],
            Variant::CosmicInventorWrithe => vec![
                vec![
                    Button::toggle("Equipment", state.temporary_variant_progress.cosmic_inventor_writhe & 0x1 > 0, "toggle-CosmicInventorWrithe-equipment"),
                    Button::toggle("Ongoing", state.temporary_variant_progress.cosmic_inventor_writhe & 0x2 > 0, "toggle-CosmicInventorWrithe-ongoing"),
                ],
                vec![Button::toggle(
                    "More Equipment",
                    state.temporary_variant_progress.cosmic_inventor_writhe & 0x4 == 0,
                    "toggle-CosmicInventorWrithe-more",
                )],
            ],
            Variant::RoadWarriorMainstay => vec![vec![
                Button::toggle(
                    format!("Rounds: {}", state.temporary_variant_progress.road_warrior_mainstay),
                    state.temporary_variant_progress.road_warrior_mainstay >= 3,
                    "counter-RoadWarriorMainstay",
                ),
                Button::reset("reset-RoadWarriorMainstay"),
                Button::restricted_unlock(state.temporary_variant_progress.road_warrior_mainstay >= 3, "unlock-RoadWarriorMainstay"),
            ]],
            Variant::HydraTiamat => vec![
                vec![Button::toggle("The Jaws of Winter", state.temporary_variant_progress.hydra_tiamat & 0x1 > 0, "toggle-HydraTiamat-0")],
                vec![Button::toggle(
                    "The Mouth of the Inferno",
                    state.temporary_variant_progress.hydra_tiamat >> 1 & 0x1 > 0,
                    "toggle-HydraTiamat-1",
                )],
                vec![Button::toggle(
                    "The Eye of the Storm",
                    state.temporary_variant_progress.hydra_tiamat >> 2 & 0x1 > 0,
                    "toggle-HydraTiamat-2",
                )],
            ],
            Variant::FirstResponseCricket => vec![vec![
                Button::toggle("Reduced", state.temporary_variant_progress.first_response_cricket & 0x1 > 0, "toggle-FirstResponseCricket"),
                Button::restricted_toggle(
                    format!("Healing: {}", state.temporary_variant_progress.first_response_cricket >> 1),
                    state.temporary_variant_progress.first_response_cricket & 0x1 > 0,
                    state.temporary_variant_progress.first_response_cricket >> 1 >= 10,
                    "counter-FirstResponseCricket",
                ),
            ]],
            Variant::TheCricketRenegade => vec![vec![
                Button::toggle("Python", state.temporary_variant_progress.the_cricket_renegade & 0x1 > 0, "toggle-TheCricketRenegade-python"),
                Button::toggle("Responder", state.temporary_variant_progress.the_cricket_renegade & 0x2 > 0, "toggle-TheCricketRenegade-responder"),
            ]],
            Variant::TheCricketWastelandRonin => Button::unlock_single("unlock-TheCricketWastelandRonin"),
            Variant::FirstResponseCypher => vec![
                Button::buttons(
                    &[("The Cricket", "toggle-FirstResponseCypher-0"), ("Echelon", "toggle-FirstResponseCypher-1")],
                    state.temporary_variant_progress.first_response_cypher,
                ),
                Button::buttons(
                    &[("Vanish", "toggle-FirstResponseCypher-2"), ("Doc Havoc", "toggle-FirstResponseCypher-3")],
                    state.temporary_variant_progress.first_response_cypher >> 2,
                ),
            ],
            Variant::CypherSwarmingProtocol => vec![
                Button::buttons(
                    &[
                        ("Dermal Aug", "toggle-CypherSwarmingProtocol-0"),
                        ("Fusion Aug", "toggle-CypherSwarmingProtocol-1"),
                        ("Muscle Aug", "toggle-CypherSwarmingProtocol-2"),
                    ],
                    state.temporary_variant_progress.cypher_swarming_protocol,
                ),
                Button::buttons(
                    &[("Retinal Aug", "toggle-CypherSwarmingProtocol-3"), ("Vascular Aug", "toggle-CypherSwarmingProtocol-4")],
                    state.temporary_variant_progress.cypher_swarming_protocol >> 3,
                ),
            ],
            Variant::FirstResponseDocHavoc => vec![
                vec![Button::toggle(
                    format!("Emergencies: {}", state.temporary_variant_progress.first_response_doc_havoc.0),
                    state.temporary_variant_progress.first_response_doc_havoc.0 >= 2,
                    "counter-FirstResponseDocHavoc-emergencies",
                )],
                vec![
                    Button::toggle(
                        format!("Cypher: {}", state.temporary_variant_progress.first_response_doc_havoc.1 & 0xF),
                        state.temporary_variant_progress.first_response_doc_havoc.1 & 0xF >= 10,
                        "counter-FirstResponseDocHavoc-0",
                    ),
                    Button::toggle(
                        format!("The Cricket: {}", state.temporary_variant_progress.first_response_doc_havoc.1 >> 4 & 0xF),
                        state.temporary_variant_progress.first_response_doc_havoc.1 >> 4 & 0xF >= 10,
                        "counter-FirstResponseDocHavoc-1",
                    ),
                ],
                vec![
                    Button::toggle(
                        format!("Echelon: {}", state.temporary_variant_progress.first_response_doc_havoc.1 >> 8 & 0xF),
                        state.temporary_variant_progress.first_response_doc_havoc.1 >> 8 & 0xF >= 10,
                        "counter-FirstResponseDocHavoc-2",
                    ),
                    Button::toggle(
                        format!("Vanish: {}", state.temporary_variant_progress.first_response_doc_havoc.1 >> 12 & 0xF),
                        state.temporary_variant_progress.first_response_doc_havoc.1 >> 12 & 0xF >= 10,
                        "counter-FirstResponseDocHavoc-3",
                    ),
                ],
            ],
            Variant::FirstResponseEchelon => vec![
                vec![Button::toggle(
                    "First Responder",
                    state.temporary_variant_progress.first_response_echelon,
                    "toggle-FirstResponseEchelon",
                )],
                Button::buttons_auto(&["Windmill City", "Superstorm Akela"], state.persistent_variant_progress.first_response_echelon),
                Button::buttons_auto(&["Megalopolis", "Rook City", "Mordengrad"], state.persistent_variant_progress.first_response_echelon >> 2),
            ],
            Variant::GargoyleWastelandRonin => Button::unlock_single("unlock-GargoyleWastelandRonin"),
            Variant::ImpactWastelandRonin => Button::unlock_single("unlock-ImpactWastelandRonin"),
            Variant::NecroLastOfTheForgottenOrder => vec![vec![
                Button::toggle(
                    "Left Behind",
                    state.temporary_variant_progress.necro_last_of_the_forgotten_order || state.persistent_variant_progress.necro_last_of_the_forgotten_order,
                    "toggle-NecroLastOfTheForgottenOrder-leftbehind",
                ),
                Button::restricted_toggle(
                    "Final Blow",
                    state.persistent_variant_progress.necro_last_of_the_forgotten_order,
                    state.temporary_variant_progress.necro_last_of_the_forgotten_order,
                    "toggle-NecroLastOfTheForgottenOrder-finalblow",
                ),
            ]],
            Variant::PyreWastelandRonin => Button::unlock_single("unlock-PyreWastelandRonin"),
            Variant::TheStrangerWastelandRonin => Button::unlock_single("unlock-TheStrangerWastelandRonin"),
            Variant::FirstResponseVanish => vec![
                vec![
                    Button::auto("Defeated", state.persistent_variant_progress.first_response_vanish),
                    Button::restricted_toggle(
                        "Gray Pharma.",
                        !state.persistent_variant_progress.first_response_vanish,
                        state.temporary_variant_progress.first_response_vanish > 0,
                        "toggle-FirstResponseVanish-pharma",
                    ),
                ],
                Button::restricted_buttons(
                    &[("Echelon", "toggle-FirstResponseVanish-0"), ("Doc Havoc", "toggle-FirstResponseVanish-1")],
                    state.temporary_variant_progress.first_response_vanish,
                    state.persistent_variant_progress.first_response_vanish,
                ),
                Button::restricted_buttons(
                    &[("The Cricket", "toggle-FirstResponseVanish-2"), ("Cypher", "toggle-FirstResponseVanish-3")],
                    state.temporary_variant_progress.first_response_vanish >> 2,
                    state.persistent_variant_progress.first_response_vanish,
                ),
            ],
            Variant::OmnitronTechnoTerror => vec![vec![
                Button::toggle(
                    "Vengeful Mad Scientist",
                    state.temporary_variant_progress.omnitron_tech_terror & 0x1 > 0,
                    "toggle-OmnitronTechTerrorVengefulMadScientist",
                ),
                Button::toggle(
                    "Partial Omni-Drone",
                    state.temporary_variant_progress.omnitron_tech_terror & 0x2 > 0,
                    "toggle-OmnitronTechTerrorPartialOmniDrone",
                ),
            ]],
            Variant::BaronBladeBlackHoleGenerator => vec![vec![
                Button::auto("Baron Blade", state.persistent_variant_progress.baron_blade_black_hole_generator),
                Button::restricted_toggle(
                    format!("Agents: {}", state.temporary_variant_progress.baron_blade_black_hole_generator),
                    state.persistent_variant_progress.baron_blade_black_hole_generator,
                    state.temporary_variant_progress.baron_blade_black_hole_generator == 3,
                    "counter-BaronBladeBlackHoleGenerator",
                ),
            ]],
            Variant::AkashBhutaPrimordialCreator => vec![vec![Button::toggle(
                "Volcanic Eruption",
                state.temporary_variant_progress.akash_bhuta_primordial_creator,
                "toggle-AkashBhutaPrimordialCreator",
            )]],
            Variant::GloomweaverRitualOfGnophos => vec![vec![
                Button::toggle(
                    format!("Zombies: {}", state.temporary_variant_progress.gloomweaver_ritual_of_gnophos & 0x3),
                    state.temporary_variant_progress.gloomweaver_ritual_of_gnophos & 0x3 == 3,
                    "counter-GloomweaverRitualOfGnophos",
                ),
                Button::toggle(
                    "Cultist",
                    state.temporary_variant_progress.gloomweaver_ritual_of_gnophos & 0x4 > 0,
                    "toggle-GloomweaverRitualOfGnophos-cultist",
                ),
                Button::toggle("Pin", state.temporary_variant_progress.gloomweaver_ritual_of_gnophos & 0x8 > 0, "toggle-GloomweaverRitualOfGnophos-pin"),
            ]],
            Variant::OmnitronVIHunterKiller => vec![vec![
                Button::auto("Iron Legacy", state.persistent_variant_progress.freedom_six & 0x40 > 0),
                Button::restricted_toggle(
                    format!("Components: {}", state.temporary_variant_progress.omnitron_vi_hunter_killer),
                    state.persistent_variant_progress.freedom_six & 0x40 > 0,
                    state.temporary_variant_progress.omnitron_vi_hunter_killer >= 10,
                    "counter-OmnitronVIHunterKiller",
                ),
            ]],
            Variant::BerserkHaka => vec![Button::buttons(
                &[("Only non-Incapacitated", "toggle-BerserkHaka-0"), ("1 HP", "toggle-BerserkHaka-1")],
                state.temporary_variant_progress.berserk_haka,
            )],
            Variant::CitizenDawnSolarEmpress => vec![vec![
                Button::toggle(
                    "Never Fewer",
                    state.temporary_variant_progress.citizen_dawn_solar_empress & 0x1 == 0,
                    "toggle-CitizenDawnSolarEmpress-citizens",
                ),
                Button::toggle(
                    "Flipped",
                    state.temporary_variant_progress.citizen_dawn_solar_empress & 0x2 > 0,
                    "toggle-CitizenDawnSolarEmpress-flipped",
                ),
            ]],
            Variant::DeadlineAngelOfExtinction => vec![vec![Button::toggle(
                "No removed",
                !state.temporary_variant_progress.deadline_angel_of_extinction,
                "toggle-DeadlineAngelOfExtinction",
            )]],
            Variant::WoundedShapeAnathema => {
                let mut buttons: Vec<_> = Button::buttons(
                    &[
                        ("Bone Cleaver", "toggle-WoundedShapeAnathema-0"),
                        ("Thresher Claw", "toggle-WoundedShapeAnathema-1"),
                        ("Knuckle Dragger", "toggle-WoundedShapeAnathema-2"),
                        ("Whip Tendril", "toggle-WoundedShapeAnathema-3"),
                    ],
                    state.temporary_variant_progress.wounded_shape_anathema,
                )
                .chunks(2)
                .map(Vec::from)
                .collect();

                buttons.extend(
                    Button::buttons(
                        &[
                            ("Heavy Carapace", "toggle-WoundedShapeAnathema-4"),
                            ("Razor Scales", "toggle-WoundedShapeAnathema-5"),
                            ("Metabolic Armor", "toggle-WoundedShapeAnathema-6"),
                            ("Enhanced Senses", "toggle-WoundedShapeAnathema-7"),
                            ("Carapace Helmet", "toggle-WoundedShapeAnathema-8"),
                            ("Reflex Booster", "toggle-WoundedShapeAnathema-9"),
                        ],
                        state.temporary_variant_progress.wounded_shape_anathema >> 4,
                    )
                    .chunks(3)
                    .map(Vec::from),
                );

                buttons
            }
            Variant::WagerMasterOmnipotentAnnoyance => Button::unlock_single("unlock-WagerMasterOmnipotentAnnoyance"),
            Variant::TheThaumaturgyScholar => vec![vec![
                Button::toggle(
                    format!("Rounds: {}", state.temporary_variant_progress.the_thaumaturgy_scholar),
                    state.temporary_variant_progress.the_thaumaturgy_scholar >= 3,
                    "counter-TheThaumaturgyScholar",
                ),
                Button::reset("reset-TheThaumaturgyScholar"),
            ]],
            _ => vec![],
        }
    }
}

impl Button {
    fn new<T>(text: T, class: ButtonClass, id: Option<&'static str>) -> Button
    where
        T: Into<String>,
    {
        Button { text: text.into(), class, id }
    }

    fn auto<T>(text: T, done: bool) -> Button
    where
        T: Into<String>,
    {
        Button::new(text, if done { ButtonClass::Done } else { ButtonClass::Restricted }, None)
    }

    fn restricted_unlock(available: bool, id: &'static str) -> Button {
        if available {
            Button::new("Unlock", ButtonClass::Available, Some(id))
        } else {
            Button::new("Unlock", ButtonClass::Restricted, None)
        }
    }

    fn toggle<T>(text: T, toggle: bool, id: &'static str) -> Button
    where
        T: Into<String>,
    {
        Button::new(text, if toggle { ButtonClass::Done } else { ButtonClass::Available }, Some(id))
    }

    fn neutral_toggle<T>(text: T, id: &'static str) -> Button
    where
        T: Into<String>,
    {
        Button::new(text, ButtonClass::Available, Some(id))
    }

    fn restricted_toggle<T>(text: T, available: bool, toggle: bool, id: &'static str) -> Button
    where
        T: Into<String>,
    {
        if available {
            Button::new(text, if toggle { ButtonClass::Done } else { ButtonClass::Available }, Some(id))
        } else {
            Button::new(text, ButtonClass::Restricted, None)
        }
    }

    fn unlock(id: &'static str) -> Button {
        Button::new("Unlock", ButtonClass::Available, Some(id))
    }

    fn reset(id: &'static str) -> Button {
        Button::new("Reset", ButtonClass::Available, Some(id))
    }

    fn unlock_single(id: &'static str) -> Vec<Vec<Button>> {
        vec![vec![Button::unlock(id)]]
    }

    fn buttons<T, N>(content: &[(T, &'static str)], bitfield: N) -> Vec<Button>
    where
        T: Copy + Into<String>,
        N: Copy + Into<u16>,
    {
        content.iter().zip(0..).map(|((name, id), idx)| Button::toggle(*name, bitfield.into() >> idx & 1 > 0, id)).collect()
    }

    fn restricted_buttons<T, N>(content: &[(T, &'static str)], bitfield: N, available: bool) -> Vec<Button>
    where
        T: Copy + Into<String>,
        N: Copy + Into<u16>,
    {
        content
            .iter()
            .zip(0..)
            .map(|((name, id), idx)| Button::restricted_toggle(*name, available, bitfield.into() >> idx & 1 > 0, id))
            .collect()
    }

    fn buttons_auto<T, N>(content: &[T], bitfield: N) -> Vec<Button>
    where
        T: Copy + Into<String>,
        N: Copy + Into<u16>,
    {
        content.iter().zip(0..).map(|(name, idx)| Button::auto(*name, bitfield.into() >> idx & 1 > 0)).collect()
    }

    fn freedom_five(state: &State) -> Vec<Button> {
        vec![
            Button::auto("Prime Wardens", state.persistent_variant_progress.freedom_five > 0),
            Button::auto("Dark Watch", state.persistent_variant_progress.freedom_five > 1),
            Button::auto("Freedom Five", state.persistent_variant_progress.freedom_five > 2),
        ]
    }

    pub fn class_name(&self) -> &'static str {
        match self.class {
            ButtonClass::Done => "done",
            ButtonClass::Available => "available",
            ButtonClass::Restricted => "restricted",
        }
    }
}
