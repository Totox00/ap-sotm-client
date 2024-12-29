use crate::{
    data::{Environment, Hero, TeamVillain, Variant, Villain},
    game::CurrentGame,
    interface::{CurrentVillains, SelectedHero},
    state::State,
};

pub mod button;
pub mod game_end;
pub mod on_click;
pub mod state;

impl Variant {
    pub fn is_available(&self, state: &State, game: &CurrentGame) -> bool {
        match self {
            Variant::MadBomberBaronBlade => {
                (game.is_classic(Villain::BaronBlade) && state.checked_locations.has_unchecked_villain(Villain::BaronBlade, 0))
                    || ((game.is_classic(Villain::CitizenDawn) || game.is_classic(Villain::CitizenDawnSolarEmpress)) && !state.checked_locations.has_unchecked_villain(Villain::BaronBlade, 0))
            }
            Variant::OmnitronII => {
                (game.is_classic(Villain::Omnitron) && state.checked_locations.has_unchecked_villain(Villain::Omnitron, 0))
                    || ((game.is_classic(Villain::GrandWarlordVoss) || game.is_classic(Villain::GrandWarlordVossGeneBoundGeneral))
                        && !state.checked_locations.has_unchecked_villain(Villain::Omnitron, 0))
            }
            Variant::SpiteAgentOfGloom => {
                (game.is_classic(Villain::Spite) && state.checked_locations.has_unchecked_villain(Villain::Spite, 0))
                    || (game.is_classic(Villain::Gloomweaver) && !state.checked_locations.has_unchecked_villain(Villain::Spite, 0))
            }
            Variant::SkinwalkerGloomweaver => {
                (game.is_classic(Villain::Spite) && state.checked_locations.has_unchecked_villain(Villain::Spite, 0))
                    || (game.is_classic(Villain::Gloomweaver) && state.checked_locations.has_unchecked_villain(Villain::Gloomweaver, 0))
                    || (game.is_classic(Villain::SpiteAgentOfGloom)
                        && !state.checked_locations.has_unchecked_villain(Villain::Spite, 0)
                        && !state.checked_locations.has_unchecked_villain(Villain::Gloomweaver, 0)
                        && game.environment(Environment::RookCity))
            }
            Variant::TricksterKismet => game.is_classic(Villain::Kismet) && game.environment(Environment::TheBlock) && game.has_all_any_variants(&[Hero::Knyfe, Hero::ArgentAdept, Hero::Fanatic]),
            Variant::HeroicInfinitor => game.is_classic(Villain::Infinitor) && game.first_hero_any_variant(Hero::CaptainCosmic),
            Variant::AmericasGreatestLegacy => game.any_ambuscade() && game.environment(Environment::SilverGulch1883) && !game.has_hero_all_variants(Hero::Legacy),
            Variant::AmericasNewestLegacy => game.any_baron_blade() && game.environment(Environment::WagnerMarsBase) && game.has_hero_all_variants(Hero::Legacy),
            Variant::DarkVisionary => game.is_classic(Villain::Gloomweaver) && game.has_hero_all_variants(Hero::Visionary),
            Variant::TheEternalHaka => game.has_hero_all_variants(Hero::Haka) && (!state.persistent_variant_progress.eternal_haka || game.environment(Environment::TheFinalWasteland)),
            Variant::GIBunker => game.has_hero_all_variants(Hero::Bunker),
            Variant::RaHorusOfTwoHorizons => (game.is_classic(Villain::TheEnnead) || game.is_classic(Villain::TheEnneadWrathOfTheGods)) && game.has_hero_all_variants(Hero::Ra),
            Variant::RaSettingSun => {
                (game.is_classic(Villain::TheEnnead) || game.is_classic(Villain::TheEnneadWrathOfTheGods))
                    && game.environment(Environment::TombOfAnubis)
                    && game.first_variant(Variant::RaHorusOfTwoHorizons)
                    && !game.has_hero_all_variants(Hero::Fanatic)
            }
            Variant::RedeemerFanatic => (game.is_classic(Villain::Apostate) || game.is_classic(Villain::ApostateDarkAngel)) && game.has_hero_all_variants(Hero::Fanatic),
            Variant::RookCityWraith => game.has_hero_all_variants(Hero::Wraith),
            Variant::TheSuperScientificTachyon => game.has_hero_all_variants(Hero::Tachyon),
            Variant::TheVisionaryUnleashed => game.environment(Environment::TheEnclaveOfTheEndlings) && game.has_hero(Hero::ArgentAdept) && game.has_variant(Variant::DarkVisionary),
            Variant::CaptainCosmicRequital => (game.is_classic(Villain::Infinitor) || game.is_classic(Villain::HeroicInfinitor)) && game.has_hero(Hero::CaptainCosmic),
            Variant::ChronoRangerTheBestOfTimes => game.any_ambuscade() && game.environment(Environment::WagnerMarsBase) && game.has_all_any_variants(&[Hero::Tachyon, Hero::ChronoRanger]),
            Variant::DarkConductorArgentAdept => game.has_hero_all_variants(Hero::ArgentAdept),
            Variant::ExtremistSkyScraper => game.any_baron_blade() && game.has_hero(Hero::SkyScraper),
            Variant::OmnitronU => {
                (state.persistent_variant_progress.omnitron_u < 2
                    && game.is_classic(if state.persistent_variant_progress.omnitron_u == 0 {
                        Villain::Omnitron
                    } else {
                        Villain::OmnitronII
                    })
                    && game.has_all_any_variants(&[Hero::OmnitronX, Hero::Unity]))
                    || (state.persistent_variant_progress.omnitron_u == 2
                        && game.has_hero_all_variants(Hero::Unity)
                        && !game.has_hero_all_variants(Hero::Tachyon)
                        && !game.has_hero_all_variants(Hero::OmnitronX))
            }
            Variant::SantaGuise => game.has_hero_all_variants(Hero::Guise),
            Variant::TheScholarOfTheInfinite => {
                (game.is_classic(Villain::Gloomweaver)
                    || game.is_classic(Villain::GloomweaverRitualOfGnophos)
                    || game.is_classic(Villain::SkinwalkerGloomweaver)
                    || game.is_classic(Villain::Apostate)
                    || game.is_classic(Villain::ApostateDarkAngel))
                    && game.has_hero_all_variants(Hero::TheScholar)
            }
            Variant::ActionHeroStuntman => match state.persistent_variant_progress.action_hero_stuntman {
                0 => game.is_classic(Villain::Ambuscade) || game.is_classic(Villain::AmbuscadeThrillOfTheHunt),
                1 => game.has_team(TeamVillain::TeamAmbuscade),
                2 => {
                    (game.is_classic(Villain::TheChairman) || game.is_classic(Villain::TheChairmanKingpin))
                        && game.environment(Environment::PikeIndustrialComplex)
                        && game.has_hero_all_variants(Hero::TheSentinels)
                }
                _ => false,
            },
            Variant::AkashThriyaSpiritOfTheVoid => game.environment(Environment::NexusOfTheVoid) && game.has_hero_all_variants(Hero::AkashThriya),
            Variant::BenchmarkSupplyAndDemand => state.persistent_variant_progress.benchmark_supply_and_demand || matches!(game.villains, CurrentVillains::Team(_)),
            Variant::HeroicLuminary => match state.persistent_variant_progress.heroic_luminary {
                0 => game.any_baron_blade() && game.environment(Environment::RealmOfDiscord) && !game.has_hero_all_variants(Hero::Luminary),
                1 => game.environment(Environment::FreedomTower) && game.has_all_any_variants(&[Hero::AbsoluteZero, Hero::Bunker, Hero::Wraith, Hero::Tachyon, Hero::Legacy]),
                2 => game.environment(Environment::Megalopolis) && game.has_hero_all_variants(Hero::Luminary),
                _ => false,
            },
            Variant::KnyfeRogueAgent => game.environment(Environment::TheBlock) && game.has_hero(Hero::Knyfe),
            Variant::LaComodoraCurseOfTheBlackSpot => game.environment(Environment::TimeCataclysm) && game.has_hero_all_variants(Hero::LaComodora),
            Variant::LifelineBloodMage => game.environment(Environment::TheCourtOfBlood) && game.has_hero_all_variants(Hero::Lifeline),
            Variant::ParseFugueState => (game.is_classic(Villain::Progeny) || game.is_classic(Villain::ProgenyTheMessenger)) && game.has_hero(Hero::Parse),
            Variant::TheAdamantSentinels => game.has_hero(Hero::TheSentinels),
            Variant::TheHuntedNaturalist => game.has_hero(Hero::TheNaturalist),
            Variant::TermiNationBunker => {
                game.environment(Environment::OmnitronIV)
                    && if state.persistent_variant_progress.termi_nation_bunker {
                        game.is_classic(Villain::OmnitronII) && game.last_hero(Hero::Bunker)
                    } else {
                        game.is_classic(Villain::Omnitron) && game.first_hero(Hero::Bunker)
                    }
            }
            Variant::TermiNationAbsoluteZero => game.has_hero(Hero::AbsoluteZero),
            Variant::TermiNationUnity => game.has_hero(Hero::Unity),
            Variant::FreedomSixAbsoluteZero => {
                game.has_hero(Hero::AbsoluteZero)
                    && ((state.persistent_variant_progress.freedom_six >> 0 & 1 > 0 && !game.has_hero_all_variants(Hero::Legacy))
                        || game.is_classic(Villain::IronLegacy)
                        || game.is_classic(Villain::IronLegacyJusticeForAll))
            }
            Variant::FreedomSixBunker => {
                game.has_hero(Hero::Bunker)
                    && ((state.persistent_variant_progress.freedom_six >> 1 & 1 > 0 && !game.has_hero_all_variants(Hero::Legacy))
                        || game.is_classic(Villain::IronLegacy)
                        || game.is_classic(Villain::IronLegacyJusticeForAll))
            }
            Variant::FreedomSixTachyon => {
                game.has_hero(Hero::Tachyon)
                    && ((state.persistent_variant_progress.freedom_six >> 2 & 1 > 0 && !game.has_hero_all_variants(Hero::Legacy))
                        || game.is_classic(Villain::IronLegacy)
                        || game.is_classic(Villain::IronLegacyJusticeForAll))
            }
            Variant::FreedomSixTempest => {
                game.has_hero(Hero::Tempest)
                    && ((state.persistent_variant_progress.freedom_six >> 3 & 1 > 0 && !game.has_hero_all_variants(Hero::Legacy))
                        || game.is_classic(Villain::IronLegacy)
                        || game.is_classic(Villain::IronLegacyJusticeForAll))
            }
            Variant::FreedomSixWraith => {
                game.first_hero_any_variant(Hero::Wraith)
                    && ((state.persistent_variant_progress.freedom_six >> 4 & 1 > 0
                        && (game.is_classic(Villain::TheChairman) || game.is_classic(Villain::TheChairmanKingpin))
                        && !game.has_hero_all_variants(Hero::Legacy))
                        || game.is_classic(Villain::IronLegacy)
                        || game.is_classic(Villain::IronLegacyJusticeForAll))
            }
            Variant::FreedomSixUnity => {
                game.has_hero(Hero::Unity)
                    && ((state.persistent_variant_progress.freedom_six >> 5 & 1 > 0 && !game.has_hero_all_variants(Hero::Legacy))
                        || game.is_classic(Villain::IronLegacy)
                        || game.is_classic(Villain::IronLegacyJusticeForAll))
            }
            Variant::DarkWatchExpatriette => game.any_baron_blade() && game.environment(Environment::RookCity) && game.has_hero_all_variants(Hero::Expatriette),
            Variant::DarkWatchMisterFixer => {
                (game.is_classic(Villain::TheChairman) || game.is_classic(Villain::TheChairmanKingpin) || game.has_team(TeamVillain::TeamTheOperative))
                    && game.environment(Environment::RookCity)
                    && game.has_hero_all_variants(Hero::MisterFixer)
            }
            Variant::DarkWatchNightmist => game.environment(Environment::RealmOfDiscord) && game.has_all_any_variants(&[Hero::Expatriette, Hero::Nightmist]),
            Variant::DarkWatchSetback => {
                (game.is_classic(Villain::TheChairman) || game.is_classic(Villain::TheChairmanKingpin)) && game.environment(Environment::RookCity) && {
                    let mut count = 0;
                    for (hero, _) in &game.heroes {
                        match hero {
                            SelectedHero::Hero(Hero::Setback)
                            | SelectedHero::Variant(Variant::DarkWatchExpatriette)
                            | SelectedHero::Variant(Variant::DarkWatchMisterFixer)
                            | SelectedHero::Variant(Variant::DarkWatchNightmist) => count += 1,
                            _ => return false,
                        }
                    }
                    count == 4
                }
            }
            Variant::DarkWatchHarpy => {
                (match &game.villains {
                    CurrentVillains::Classic((villain, diff, _)) => diff & 1 > 0 && [Villain::Gloomweaver, Villain::GloomweaverRitualOfGnophos].contains(&villain),
                    _ => false,
                } && game.environment(Environment::RealmOfDiscord)
                    && game.has_hero_all_variants(Hero::TheHarpy))
            }
            Variant::PrimeWardensArgentAdept => {
                (game.is_classic(Villain::AkashBhuta) || game.is_classic(Villain::AkashBhutaPrimordialCreator))
                    && ((!state.persistent_variant_progress.prime_wardens_argent_adept && game.has_hero(Hero::ArgentAdept))
                        || (game.first_hero(Hero::ArgentAdept) && {
                            let mut count = 0;
                            for (hero, _) in game.heroes.iter().skip(1) {
                                match hero {
                                    SelectedHero::Hero(Hero::Haka) | SelectedHero::Hero(Hero::CaptainCosmic) | SelectedHero::Hero(Hero::Tempest) | SelectedHero::Variant(Variant::RedeemerFanatic) => {
                                        count += 1
                                    }
                                    _ => return false,
                                }
                            }
                            count == 4
                        }))
            }
            Variant::PrimeWardensCaptainCosmic => game.environment(Environment::DokThorathCapital) && game.has_hero(Hero::CaptainCosmic),
            Variant::PrimeWardensFanatic => {
                (game.is_classic(Villain::Apostate) || game.is_classic(Villain::ApostateDarkAngel))
                    && game.heroes.iter().any(|(hero, _)| hero.is_hero(Hero::Fanatic) || hero.is_variant(Variant::RedeemerFanatic))
            }
            Variant::PrimeWardensHaka => game.any_ambuscade() && game.has_hero(Hero::Haka),
            Variant::PrimeWardensTempest => game.has_hero(Hero::Tempest),
            Variant::XtremePrimeWardensArgentAdept => game.environment(Environment::InsulaPrimalis) && game.has_hero(Hero::ArgentAdept),
            Variant::XtremePrimeWardensTempest => game.environment(Environment::DokThorathCapital) && game.has_hero(Hero::Tempest),
            Variant::XtremePrimeWardensCaptainCosmic => game.environment(Environment::TheEnclaveOfTheEndlings) && game.has_hero(Hero::CaptainCosmic),
            Variant::XtremePrimeWardensFanatic => game.environment(Environment::TheCourtOfBlood) && game.has_hero(Hero::Fanatic),
            Variant::XtremePrimeWardensHaka => game.environment(Environment::Magmaria) && game.has_hero(Hero::Haka),
            Variant::FreedomFiveAbsoluteZero => game.freedom_five(state.persistent_variant_progress.freedom_five),
            Variant::FreedomFiveBunker => game.freedom_five(state.persistent_variant_progress.freedom_five),
            Variant::FreedomFiveWraith => game.freedom_five(state.persistent_variant_progress.freedom_five),
            Variant::FreedomFiveTachyon => game.freedom_five(state.persistent_variant_progress.freedom_five),
            Variant::FreedomFiveLegacy => game.freedom_five(state.persistent_variant_progress.freedom_five),
            Variant::SuperSentaiIdealist => game.has_hero_all_variants(Hero::TheIdealist),
            Variant::DrMedicoMalpractice => game.has_hero_all_variants(Hero::DoctorMedico) && game.has_team(TeamVillain::TeamAmbuscade),
            Variant::CosmicInventorWrithe => game.has_hero_all_variants(Hero::Writhe),
            Variant::RoadWarriorMainstay => game.has_hero_all_variants(Hero::Mainstay),
            _ => false,
        }
    }
}
