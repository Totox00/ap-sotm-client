use crate::{
    data::{Environment, Hero, TeamVillain, Variant},
    game::CurrentGame,
    interface::CurrentVillains,
    state::State,
};

impl Variant {
    pub fn game_end(&self, state: &mut State, game: &CurrentGame, victory: bool, push: &mut bool) -> bool {
        match self {
            Variant::SpiteAgentOfGloom => victory && state.temporary_variant_progress.spite_agent_of_gloom,
            Variant::SkinwalkerGloomweaver => {
                (victory && state.temporary_variant_progress.skinwalker_gloomweaver & 0xB == 0xB) || (!victory && state.temporary_variant_progress.skinwalker_gloomweaver & 0x55 == 0x50)
            }
            Variant::TricksterKismet => !victory && state.temporary_variant_progress.trickster_kismet.0 > state.temporary_variant_progress.trickster_kismet.1,
            Variant::HeroicInfinitor => victory && state.temporary_variant_progress.heroic_infinitor == 0x3F,
            Variant::AmericasGreatestLegacy => state.temporary_variant_progress.americas_greatest_legacy,
            Variant::DarkVisionary => state.temporary_variant_progress.dark_visionary,
            Variant::TheEternalHaka => {
                if state.persistent_variant_progress.eternal_haka {
                    victory && state.temporary_variant_progress.eternal_haka == 0xF
                } else if victory && state.temporary_variant_progress.eternal_haka & 0x8 > 0 {
                    *push = true;
                    state.persistent_variant_progress.eternal_haka = true;
                    false
                } else {
                    false
                }
            }
            Variant::RaSettingSun => victory && state.temporary_variant_progress.ra_setting_sun == 0xF,
            Variant::RedeemerFanatic => victory && state.temporary_variant_progress.redeemer_fanatic == 0xF,
            Variant::TheVisionaryUnleashed => state.temporary_variant_progress.the_visionary_unleashed == 0xF,
            Variant::CaptainCosmicRequital => victory && state.temporary_variant_progress.captain_cosmic_requital == 0xDA,
            Variant::ChronoRangerTheBestOfTimes => victory && state.temporary_variant_progress.chrono_ranger_best_of_times == 0x4,
            Variant::DarkConductorArgentAdept => victory && state.temporary_variant_progress.dark_conductor_argent_adept == 20 << 1,
            Variant::ExtremistSkyScraper => victory && state.temporary_variant_progress.sky_scraper_extremist == 3,
            Variant::OmnitronU => match state.persistent_variant_progress.omnitron_u {
                0 => {
                    if victory && state.temporary_variant_progress.omnitron_u & 0x3 == 0x0 {
                        *push = true;
                        state.persistent_variant_progress.omnitron_u = 1;
                    }
                    false
                }
                1 => {
                    if victory && state.temporary_variant_progress.omnitron_u & 0x3 == 0x1 {
                        *push = true;
                        state.persistent_variant_progress.omnitron_u = 2;
                    }
                    false
                }
                2 => victory && state.temporary_variant_progress.omnitron_u & 0xC == 0xC,
                _ => unreachable!(),
            },
            Variant::TheScholarOfTheInfinite => victory && state.temporary_variant_progress.the_scholar_of_the_infinite == 0x29,
            Variant::ActionHeroStuntman => {
                if state.persistent_variant_progress.action_hero_stuntman < 2 {
                    if victory && state.temporary_variant_progress.action_hero_stuntman {
                        *push = true;
                        state.persistent_variant_progress.action_hero_stuntman += 1;
                    }
                    false
                } else {
                    victory && state.temporary_variant_progress.action_hero_stuntman
                }
            }
            Variant::AkashThriyaSpiritOfTheVoid => {
                victory && state.temporary_variant_progress.akash_thriya_spirit_of_the_void.0 == 0x1F && state.temporary_variant_progress.akash_thriya_spirit_of_the_void.1 == 0x7
            }
            Variant::BenchmarkSupplyAndDemand => {
                if !state.persistent_variant_progress.benchmark_supply_and_demand {
                    if let CurrentVillains::Team(villains) = &game.villains {
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

                        if victory && state.temporary_variant_progress.benchmark_supply_and_demand.0 >= needed * 10 && state.temporary_variant_progress.benchmark_supply_and_demand.1 >= needed * 5 {
                            *push = true;
                            state.persistent_variant_progress.benchmark_supply_and_demand = true;
                        }
                    }
                }
                false
            }
            Variant::HeroicLuminary => {
                if victory && state.persistent_variant_progress.heroic_luminary < 2 {
                    *push = true;
                    state.persistent_variant_progress.heroic_luminary += 1;
                }
                false
            }
            Variant::LaComodoraCurseOfTheBlackSpot => victory && state.temporary_variant_progress.la_comodora_curse_of_the_black_spot == 0x7,
            Variant::LifelineBloodMage => victory && state.temporary_variant_progress.lifeline_blood_mage.0 == 0xFF && state.temporary_variant_progress.lifeline_blood_mage.1 == 0x1,
            Variant::ParseFugueState => victory && state.temporary_variant_progress.parse_fugue_state == 0x3,
            Variant::TheAdamantSentinels => !victory && state.temporary_variant_progress.adamant_sentinels == 0x3,
            Variant::TheHuntedNaturalist => {
                if victory {
                    if state.temporary_variant_progress.hunted_naturalist & 0x1 > 0
                        && state.persistent_variant_progress.hunted_naturalist >> (state.temporary_variant_progress.hunted_naturalist >> 1) & 0x1 == 0
                    {
                        *push = true;
                        state.persistent_variant_progress.hunted_naturalist |= 1 << (state.temporary_variant_progress.hunted_naturalist >> 1);
                    }
                    state.persistent_variant_progress.hunted_naturalist == 0x7
                } else {
                    false
                }
            }
            Variant::TermiNationBunker => {
                if victory && state.temporary_variant_progress.termi_nation_bunker {
                    if state.persistent_variant_progress.termi_nation_bunker {
                        return true;
                    } else {
                        *push = true;
                        state.persistent_variant_progress.termi_nation_bunker = true;
                    }
                }
                false
            }
            Variant::FreedomSixAbsoluteZero => {
                if state.persistent_variant_progress.freedom_six & 0x1 > 0 {
                    victory && state.temporary_variant_progress.freedom_six_absolute_zero == 0xF
                } else {
                    if !victory && state.persistent_variant_progress.freedom_six & 0x1 == 0 {
                        *push = true;
                        state.persistent_variant_progress.freedom_six |= 0x1;
                    }
                    false
                }
            }
            Variant::FreedomSixBunker => {
                if state.persistent_variant_progress.freedom_six & 0x2 > 0 {
                    victory && state.temporary_variant_progress.freedom_six_bunker == 0x19
                } else {
                    if !victory && state.persistent_variant_progress.freedom_six & 0x2 == 0 {
                        *push = true;
                        state.persistent_variant_progress.freedom_six |= 0x2;
                    }
                    false
                }
            }
            Variant::FreedomSixTachyon => {
                if state.persistent_variant_progress.freedom_six & 0x4 > 0 {
                    victory && state.temporary_variant_progress.freedom_six_tachyon == 0x5
                } else {
                    if !victory && state.persistent_variant_progress.freedom_six & 0x4 == 0 {
                        *push = true;
                        state.persistent_variant_progress.freedom_six |= 0x4;
                    }
                    false
                }
            }
            Variant::FreedomSixTempest => {
                if state.persistent_variant_progress.freedom_six & 0x8 > 0 {
                    victory && state.temporary_variant_progress.freedom_six_tempest == 0xF
                } else {
                    if !victory && state.persistent_variant_progress.freedom_six & 0x8 == 0 {
                        *push = true;
                        state.persistent_variant_progress.freedom_six |= 0x8;
                    }
                    false
                }
            }
            Variant::FreedomSixWraith => {
                if state.persistent_variant_progress.freedom_six & 0x10 > 0 {
                    victory && state.temporary_variant_progress.freedom_six_wraith == 0x3
                } else {
                    if !victory && state.persistent_variant_progress.freedom_six & 0x10 == 0 {
                        *push = true;
                        state.persistent_variant_progress.freedom_six |= 0x10;
                    }
                    false
                }
            }
            Variant::FreedomSixUnity => {
                if state.persistent_variant_progress.freedom_six & 0x20 > 0 {
                    victory && state.temporary_variant_progress.freedom_six_unity
                } else {
                    if !victory && state.persistent_variant_progress.freedom_six & 0x20 == 0 {
                        *push = true;
                        state.persistent_variant_progress.freedom_six |= 0x20;
                    }
                    false
                }
            }
            Variant::DarkWatchExpatriette => victory && state.temporary_variant_progress.dark_watch_expatriette,
            Variant::DarkWatchSetback => victory,
            Variant::DarkWatchHarpy => victory && state.temporary_variant_progress.dark_watch_harpy == 0xF,
            Variant::PrimeWardensArgentAdept => {
                if state.persistent_variant_progress.prime_wardens_argent_adept {
                    victory && state.temporary_variant_progress.prime_wardens_argent_adept == 0x3F
                } else if !victory {
                    *push = true;
                    state.persistent_variant_progress.prime_wardens_argent_adept = !victory;
                    false
                } else {
                    false
                }
            }
            Variant::PrimeWardensHaka => victory && state.temporary_variant_progress.prime_wardens_haka == 0x1,
            Variant::XtremePrimeWardensArgentAdept => victory && state.temporary_variant_progress.xtreme_prime_wardens_argent_adept == 0x1FF,
            Variant::XtremePrimeWardensTempest => victory && state.temporary_variant_progress.xtreme_prime_wardens_tempest == 0x2,
            Variant::XtremePrimeWardensCaptainCosmic => victory && state.temporary_variant_progress.xtreme_prime_wardens_captain_cosmic == 0xAA,
            Variant::XtremePrimeWardensFanatic => victory && state.temporary_variant_progress.xtreme_prime_wardens_fanatic == 0x3F,
            Variant::XtremePrimeWardensHaka => victory && state.temporary_variant_progress.xtreme_prime_wardens_haka == 0x2D,
            Variant::FreedomFiveAbsoluteZero => freedom_five(victory, state, state.temporary_variant_progress.freedom_five_absolute_zero == (29, 29, true), push),
            Variant::FreedomFiveBunker => freedom_five(
                victory,
                state,
                state.temporary_variant_progress.freedom_five_bunker.0 == 0x2A && state.temporary_variant_progress.freedom_five_bunker.1.count_ones() >= 4,
                push,
            ),
            Variant::FreedomFiveWraith => freedom_five(victory, state, state.temporary_variant_progress.freedom_five_wraith == 0xD3, push),
            Variant::FreedomFiveTachyon => freedom_five(victory, state, state.temporary_variant_progress.freedom_five_tachyon == 0x7, push),
            Variant::FreedomFiveLegacy => freedom_five(
                victory,
                state,
                state.temporary_variant_progress.freedom_five_legacy.0 > state.temporary_variant_progress.freedom_five_legacy.1
                    && state.temporary_variant_progress.freedom_five_legacy.0 >= 20
                    && state.temporary_variant_progress.freedom_five_legacy.2,
                push,
            ),
            Variant::CosmicInventorWrithe => state.temporary_variant_progress.cosmic_inventor_writhe == 0x3,
            Variant::HydraTiamat => state.temporary_variant_progress.hydra_tiamat == 0x7,
            Variant::FirstResponseCricket => victory && state.temporary_variant_progress.first_response_cricket == 0x15,
            Variant::TheCricketRenegade => !victory && state.temporary_variant_progress.the_cricket_renegade == 0x3,
            Variant::FirstResponseCypher => victory && state.temporary_variant_progress.first_response_cypher == 0xF,
            Variant::FirstResponseDocHavoc => victory && state.temporary_variant_progress.first_response_doc_havoc == (2, 0xAAAA),
            Variant::FirstResponseEchelon => {
                if victory && state.temporary_variant_progress.first_response_echelon {
                    state.persistent_variant_progress.first_response_echelon |= match game.environment {
                        Some(Environment::WindmillCity) => 0x1,
                        Some(Environment::SuperstormAkela) => 0x2,
                        Some(Environment::Megalopolis) => 0x4,
                        Some(Environment::RookCity) => 0x8,
                        Some(Environment::Mordengrad) => 0x10,
                        _ => 0,
                    };
                }

                state.persistent_variant_progress.first_response_echelon == 0x1F
            }
            Variant::NecroLastOfTheForgottenOrder => {
                if state.persistent_variant_progress.necro_last_of_the_forgotten_order {
                    victory && state.temporary_variant_progress.necro_last_of_the_forgotten_order
                } else {
                    if !victory && state.temporary_variant_progress.necro_last_of_the_forgotten_order {
                        state.persistent_variant_progress.necro_last_of_the_forgotten_order = true;
                    }
                    false
                }
            }
            Variant::FirstResponseVanish => {
                if state.persistent_variant_progress.first_response_vanish {
                    victory && state.temporary_variant_progress.first_response_vanish == 0xF
                } else {
                    if !victory && state.temporary_variant_progress.first_response_vanish > 0 {
                        state.persistent_variant_progress.first_response_vanish = true;
                    }
                    false
                }
            }
            _ => false,
        }
    }
}

fn freedom_five(victory: bool, state: &mut State, unlock: bool, push: &mut bool) -> bool {
    if state.temporary_variant_progress.freedom_five_prereq_advanced {
        false
    } else if state.persistent_variant_progress.freedom_five < 3 {
        state.temporary_variant_progress.freedom_five_prereq_advanced = true;
        if !victory {
            *push = true;
            state.persistent_variant_progress.freedom_five += 1;
        }
        false
    } else {
        victory && unlock
    }
}
