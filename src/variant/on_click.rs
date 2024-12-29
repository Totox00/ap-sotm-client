use utils::{flip_bit, flip_bool, incr, reset};

use crate::{
    data::{Hero, TeamVillain, Variant},
    game::CurrentGame,
    interface::CurrentVillains,
    state::State,
};

pub fn on_click(target: &str, state: &mut State, game: &CurrentGame) -> Option<Variant> {
    match target {
        "toggle-SpiteAgentOfGloom" => flip_bool!(spite_agent_of_gloom),
        "toggle-SkinwalkerGloomweaver-side" => flip_bit!(skinwalker_gloomweaver, 0),
        "toggle-SkinwalkerGloomweaver-safehouse" => flip_bit!(skinwalker_gloomweaver, 1),
        "toggle-SkinwalkerGloomweaver-drugs" => flip_bit!(skinwalker_gloomweaver, 2),
        "toggle-SkinwalkerGloomweaver-victims" => flip_bit!(skinwalker_gloomweaver, 3),
        "counter-SkinwalkerGloomweaver-victims" => incr!(skinwalker_gloomweaver, 4, 3, 5),
        "counter-TricksterKismet-hero" => state.temporary_variant_progress.trickster_kismet.0 += 1,
        "counter-TricksterKismet-villain" => state.temporary_variant_progress.trickster_kismet.1 += 1,
        "toggle-HeroicInfinitor-construct" => flip_bit!(heroic_infinitor, 5),
        "toggle-HeroicInfinitor-incapacitated" => flip_bit!(heroic_infinitor, 6),
        "toggle-HeroicInfinitor-0" => flip_bit!(heroic_infinitor, 0),
        "toggle-HeroicInfinitor-1" => flip_bit!(heroic_infinitor, 1),
        "toggle-HeroicInfinitor-2" => flip_bit!(heroic_infinitor, 2),
        "toggle-HeroicInfinitor-3" => flip_bit!(heroic_infinitor, 3),
        "toggle-HeroicInfinitor-4" => flip_bit!(heroic_infinitor, 4),
        "toggle-AmericasGreatestLegacy" => flip_bool!(americas_greatest_legacy),
        "toggle-DarkVisionary" => flip_bool!(dark_visionary),
        "toggle-TheEternalHaka-last" => flip_bit!(eternal_haka, 3),
        "toggle-TheEternalHaka-0" => flip_bit!(eternal_haka, 0),
        "toggle-TheEternalHaka-1" => flip_bit!(eternal_haka, 1),
        "toggle-TheEternalHaka-2" => flip_bit!(eternal_haka, 2),
        "toggle-GIBunker-0" => flip_bit!(gi_bunker, 0),
        "toggle-GIBunker-1" => flip_bit!(gi_bunker, 1),
        "toggle-GIBunker-2" => flip_bit!(gi_bunker, 2),
        "toggle-RaSettingSun-reduced" => flip_bit!(ra_setting_sun, 0),
        "toggle-RaSettingSun-restored" => flip_bit!(ra_setting_sun, 1),
        "counter-RaSettingSun" => incr!(ra_setting_sun, 2, 2, 3),
        "toggle-RedeemerFanatic-undaunted" => flip_bit!(redeemer_fanatic, 0),
        "toggle-RedeemerFanatic-restored" => flip_bit!(redeemer_fanatic, 1),
        "toggle-RedeemerFanatic-prayer" => flip_bit!(redeemer_fanatic, 2),
        "toggle-RedeemerFanatic-absolution" => flip_bit!(redeemer_fanatic, 3),
        "toggle-TheVisionaryUnleashed-0" => flip_bit!(the_visionary_unleashed, 0),
        "toggle-TheVisionaryUnleashed-1" => flip_bit!(the_visionary_unleashed, 1),
        "toggle-TheVisionaryUnleashed-2" => flip_bit!(the_visionary_unleashed, 2),
        "toggle-TheVisionaryUnleashed-visionary" => flip_bit!(the_visionary_unleashed, 3),
        "toggle-TheVisionaryUnleashed-argentadept" => flip_bit!(the_visionary_unleashed, 4),
        "toggle-CaptainCosmicRequital" => flip_bit!(captain_cosmic_requital, 7),
        "counter-CaptainCosmicRequital-manifestations" => incr!(captain_cosmic_requital, 0, 4, 10),
        "counter-CaptainCosmicRequital-constructs" => incr!(captain_cosmic_requital, 4, 3, 5),
        "toggle-ChronoRangerTheBestOfTimes-tachyon" => flip_bit!(chrono_ranger_best_of_times, 0),
        "toggle-ChronoRangerTheBestOfTimes-bounty" => flip_bit!(chrono_ranger_best_of_times, 1),
        "toggle-ChronoRangerTheBestOfTimes-final" => flip_bit!(chrono_ranger_best_of_times, 2),
        "toggle-DarkConductorArgentAdept" => flip_bit!(dark_conductor_argent_adept, 0),
        "counter-DarkConductorArgentAdept" => incr!(dark_conductor_argent_adept, 1, 5, 20),
        "counter-ExtremistSkyScraper" => incr!(sky_scraper_extremist, 0, 2, 3),
        "toggle-OmnitronU-omnitron" => flip_bit!(omnitron_u, 0),
        "toggle-OmnitronU-unity" => flip_bit!(omnitron_u, 1),
        "toggle-OmnitronU-equipment" => flip_bit!(omnitron_u, 2),
        "toggle-OmnitronU-reclamation" => flip_bit!(omnitron_u, 3),
        "counter-SantaGuise" => incr!(santa_guise, 0, 5, 25),
        "toggle-TheScholarOfTheInfinite" => flip_bit!(the_scholar_of_the_infinite, 0),
        "counter-TheScholarOfTheInfinite" => incr!(the_scholar_of_the_infinite, 1, 5, 20),
        "toggle-ActionHeroStuntman-incapacitated" | "toggle-ActionHeroStuntman-mainstay" => flip_bool!(action_hero_stuntman),
        "toggle-AkashThriyaSpiritOfTheVoid-0" => flip_bit!(akash_thriya_spirit_of_the_void.0, 0),
        "toggle-AkashThriyaSpiritOfTheVoid-1" => flip_bit!(akash_thriya_spirit_of_the_void.0, 1),
        "toggle-AkashThriyaSpiritOfTheVoid-2" => flip_bit!(akash_thriya_spirit_of_the_void.0, 2),
        "toggle-AkashThriyaSpiritOfTheVoid-3" => flip_bit!(akash_thriya_spirit_of_the_void.0, 3),
        "toggle-AkashThriyaSpiritOfTheVoid-4" => flip_bit!(akash_thriya_spirit_of_the_void.0, 4),
        "toggle-AkashThriyaSpiritOfTheVoid-reduced" => flip_bit!(akash_thriya_spirit_of_the_void.1, 0),
        "toggle-AkashThriyaSpiritOfTheVoid-recover" => flip_bit!(akash_thriya_spirit_of_the_void.1, 1),
        "toggle-AkashThriyaSpiritOfTheVoid-akashflora" => flip_bit!(akash_thriya_spirit_of_the_void.1, 2),
        "toggle-AkashThriyaSpiritOfTheVoid-akashfloradestroyed" => flip_bit!(akash_thriya_spirit_of_the_void.1, 3),
        "counter-BenchmarkSupplyAndDemand-equipment" => {
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
                if state.temporary_variant_progress.benchmark_supply_and_demand.0 < needed * 10 {
                    state.temporary_variant_progress.benchmark_supply_and_demand.0 += 1;
                }
            }
        }
        "counter-BenchmarkSupplyAndDemand-devices" => {
            if let CurrentVillains::Team(villains) = &game.villains {
                let needed = (10
                    - game
                        .current_bases()
                        .filter(|base| [Hero::Benchmark, Hero::Expatriette, Hero::Luminary, Hero::Parse, Hero::Setback].contains(base))
                        .count()
                    + villains
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
                if state.temporary_variant_progress.benchmark_supply_and_demand.1 < needed * 5 {
                    state.temporary_variant_progress.benchmark_supply_and_demand.1 += 1;
                }
            }
        }
        "toggle-HeroicLuminary-0" => flip_bit!(heroic_luminary, 0),
        "toggle-HeroicLuminary-1" => flip_bit!(heroic_luminary, 1),
        "toggle-HeroicLuminary-2" => flip_bit!(heroic_luminary, 2),
        "counter-KnyfeRogueAgent" => incr!(knyfe_rogue_agent, 1, 3, 5),
        "toggle-KnyfeRogueAgent" => flip_bit!(knyfe_rogue_agent, 0),
        "toggle-LaComodoraCurseOfTheBlackSpot-play" => flip_bit!(la_comodora_curse_of_the_black_spot, 0),
        "toggle-LaComodoraCurseOfTheBlackSpot-trash" => flip_bit!(la_comodora_curse_of_the_black_spot, 1),
        "toggle-LaComodoraCurseOfTheBlackSpot-runaground" => flip_bit!(la_comodora_curse_of_the_black_spot, 2),
        "toggle-LifelineBloodMage-0" => flip_bit!(lifeline_blood_mage.0, 0),
        "toggle-LifelineBloodMage-1" => flip_bit!(lifeline_blood_mage.0, 1),
        "toggle-LifelineBloodMage-2" => flip_bit!(lifeline_blood_mage.0, 2),
        "toggle-LifelineBloodMage-3" => flip_bit!(lifeline_blood_mage.0, 3),
        "toggle-LifelineBloodMage-4" => flip_bit!(lifeline_blood_mage.0, 4),
        "toggle-LifelineBloodMage-5" => flip_bit!(lifeline_blood_mage.0, 5),
        "toggle-LifelineBloodMage-6" => flip_bit!(lifeline_blood_mage.0, 6),
        "toggle-LifelineBloodMage-7" => flip_bit!(lifeline_blood_mage.0, 7),
        "toggle-LifelineBloodMage-bathory" => flip_bit!(lifeline_blood_mage.1, 0),
        "toggle-LifelineBloodMage-incapacitated" => flip_bit!(lifeline_blood_mage.1, 1),
        "toggle-ParseFugueState-ongoings" => flip_bit!(parse_fugue_state, 0),
        "toggle-ParseFugueState-hp" => flip_bit!(parse_fugue_state, 1),
        "toggle-TheAdamantSentinels-incapacitated" => flip_bit!(adamant_sentinels, 0),
        "toggle-TheAdamantSentinels-signatures" => flip_bit!(adamant_sentinels, 1),
        "switch-TheHuntedNaturalist" => {
            let mut next = (state.temporary_variant_progress.hunted_naturalist >> 1) + 1;
            if next > 2 {
                next = 0;
            }
            state.temporary_variant_progress.hunted_naturalist = state.temporary_variant_progress.hunted_naturalist & 0x1 | next << 1;
        }
        "toggle-TheHuntedNaturalist-final" => flip_bit!(hunted_naturalist, 0),
        "toggle-TermiNationBunker" => flip_bool!(termi_nation_bunker),
        "toggle-FreedomSixAbsoluteZero-incapacitated" => flip_bit!(freedom_six_absolute_zero, 4),
        "toggle-FreedomSixAbsoluteZero-0" => flip_bit!(freedom_six_absolute_zero, 0),
        "toggle-FreedomSixAbsoluteZero-1" => flip_bit!(freedom_six_absolute_zero, 1),
        "toggle-FreedomSixAbsoluteZero-2" => flip_bit!(freedom_six_absolute_zero, 2),
        "toggle-FreedomSixAbsoluteZero-3" => flip_bit!(freedom_six_absolute_zero, 3),
        "counter-FreedomSixBunker" => incr!(freedom_six_bunker, 1, 4, 12),
        "toggle-FreedomSixBunker" => flip_bit!(freedom_six_bunker, 0),
        "counter-FreedomSixTachyon" => incr!(freedom_six_tachyon, 0, 3, 5),
        "toggle-FreedomSixTempest-incapacitated" => flip_bit!(freedom_six_tempest, 0),
        "toggle-FreedomSixTempest-0" => flip_bit!(freedom_six_tempest, 1),
        "toggle-FreedomSixTempest-1" => flip_bit!(freedom_six_tempest, 2),
        "toggle-FreedomSixTempest-2" => flip_bit!(freedom_six_tempest, 3),
        "toggle-FreedomSixWraith-0" => flip_bit!(freedom_six_wraith, 0),
        "toggle-FreedomSixWraith-1" => flip_bit!(freedom_six_wraith, 1),
        "toggle-FreedomSixUnity" => flip_bool!(freedom_six_unity),
        "toggle-DarkWatchExpatriette" => flip_bool!(dark_watch_expatriette),
        "toggle-DarkWatchHarpy-avian" => flip_bit!(dark_watch_harpy, 0),
        "toggle-DarkWatchHarpy-arcana" => flip_bit!(dark_watch_harpy, 1),
        "toggle-DarkWatchHarpy-remain" => flip_bit!(dark_watch_harpy, 4),
        "toggle-DarkWatchHarpy-relic" => flip_bit!(dark_watch_harpy, 3),
        "toggle-DarkWatchHarpy-incapacitated" => flip_bit!(dark_watch_harpy, 2),
        "toggle-PrimeWardensArgentAdept-0" => flip_bit!(prime_wardens_argent_adept, 0),
        "toggle-PrimeWardensArgentAdept-1" => flip_bit!(prime_wardens_argent_adept, 1),
        "toggle-PrimeWardensArgentAdept-2" => flip_bit!(prime_wardens_argent_adept, 2),
        "toggle-PrimeWardensArgentAdept-3" => flip_bit!(prime_wardens_argent_adept, 3),
        "toggle-PrimeWardensArgentAdept-4" => flip_bit!(prime_wardens_argent_adept, 4),
        "toggle-PrimeWardensArgentAdept-5" => flip_bit!(prime_wardens_argent_adept, 5),
        "counter-PrimeWardensCaptainCosmic" => incr!(prime_wardens_captain_cosmic, 0, 5, 20),
        "counter-PrimeWardensFanatic-0" => incr!(prime_wardens_fanatic, 0, 2, 2),
        "counter-PrimeWardensFanatic-1" => incr!(prime_wardens_fanatic, 2, 2, 2),
        "counter-PrimeWardensFanatic-2" => incr!(prime_wardens_fanatic, 4, 2, 2),
        "toggle-PrimeWardensHaka-active" => flip_bit!(prime_wardens_haka, 0),
        "toggle-PrimeWardensHaka-hp" => flip_bit!(prime_wardens_haka, 1),
        "toggle-PrimeWardensTempest-0" => flip_bit!(prime_wardens_tempest, 0),
        "toggle-PrimeWardensTempest-1" => flip_bit!(prime_wardens_tempest, 1),
        "toggle-PrimeWardensTempest-2" => flip_bit!(prime_wardens_tempest, 2),
        "toggle-XtremePrimeWardensArgentAdept-0" => flip_bit!(xtreme_prime_wardens_argent_adept, 0),
        "toggle-XtremePrimeWardensArgentAdept-1" => flip_bit!(xtreme_prime_wardens_argent_adept, 1),
        "toggle-XtremePrimeWardensArgentAdept-2" => flip_bit!(xtreme_prime_wardens_argent_adept, 2),
        "toggle-XtremePrimeWardensArgentAdept-3" => flip_bit!(xtreme_prime_wardens_argent_adept, 3),
        "toggle-XtremePrimeWardensArgentAdept-4" => flip_bit!(xtreme_prime_wardens_argent_adept, 4),
        "toggle-XtremePrimeWardensArgentAdept-5" => flip_bit!(xtreme_prime_wardens_argent_adept, 5),
        "toggle-XtremePrimeWardensArgentAdept-6" => flip_bit!(xtreme_prime_wardens_argent_adept, 6),
        "toggle-XtremePrimeWardensArgentAdept-7" => flip_bit!(xtreme_prime_wardens_argent_adept, 7),
        "toggle-XtremePrimeWardensArgentAdept-8" => flip_bit!(xtreme_prime_wardens_argent_adept, 8),
        "toggle-XtremePrimeWardensTempest-damage" => flip_bit!(xtreme_prime_wardens_tempest, 0),
        "toggle-XtremePrimeWardensTempest-equipment" => flip_bit!(xtreme_prime_wardens_tempest, 1),
        "counter-XtremePrimeWardensCaptainCosmic-entered" => incr!(xtreme_prime_wardens_captain_cosmic, 0, 4, 10),
        "counter-XtremePrimeWardensCaptainCosmic-destroyed" => incr!(xtreme_prime_wardens_captain_cosmic, 4, 4, 10),
        "toggle-XtremePrimeWardensFanatic-infernal" => flip_bit!(xtreme_prime_wardens_fanatic, 5),
        "toggle-XtremePrimeWardensFanatic-0" => flip_bit!(xtreme_prime_wardens_fanatic, 0),
        "toggle-XtremePrimeWardensFanatic-1" => flip_bit!(xtreme_prime_wardens_fanatic, 1),
        "toggle-XtremePrimeWardensFanatic-2" => flip_bit!(xtreme_prime_wardens_fanatic, 2),
        "toggle-XtremePrimeWardensFanatic-3" => flip_bit!(xtreme_prime_wardens_fanatic, 3),
        "toggle-XtremePrimeWardensFanatic-4" => flip_bit!(xtreme_prime_wardens_fanatic, 4),
        "toggle-XtremePrimeWardensFanatic-5" => flip_bit!(xtreme_prime_wardens_fanatic, 5),
        "toggle-XtremePrimeWardensHaka-equipment" => flip_bit!(xtreme_prime_wardens_haka, 6),
        "counter-XtremePrimeWardensHaka-villain" => incr!(xtreme_prime_wardens_haka, 0, 3, 5),
        "counter-XtremePrimeWardensHaka-environment" => incr!(xtreme_prime_wardens_haka, 3, 3, 5),
        "counter-FreedomFiveAbsoluteZero-fire" => incr!(freedom_five_absolute_zero.0, 0, 5, 29),
        "counter-FreedomFiveAbsoluteZero-cold" => incr!(freedom_five_absolute_zero.1, 0, 5, 29),
        "toggle-FreedomFiveAbsoluteZero-ongoings" => flip_bool!(freedom_five_absolute_zero.2),
        "counter-FreedomFiveBunker-0" => incr!(freedom_five_bunker.0, 0, 2, 2),
        "counter-FreedomFiveBunker-1" => incr!(freedom_five_bunker.0, 2, 2, 2),
        "counter-FreedomFiveBunker-2" => incr!(freedom_five_bunker.0, 4, 2, 2),
        "toggle-FreedomFiveBunker-0" => flip_bit!(freedom_five_bunker.1, 0),
        "toggle-FreedomFiveBunker-1" => flip_bit!(freedom_five_bunker.1, 1),
        "toggle-FreedomFiveBunker-2" => flip_bit!(freedom_five_bunker.1, 2),
        "toggle-FreedomFiveBunker-3" => flip_bit!(freedom_five_bunker.1, 3),
        "toggle-FreedomFiveBunker-4" => flip_bit!(freedom_five_bunker.1, 4),
        "toggle-FreedomFiveBunker-5" => flip_bit!(freedom_five_bunker.1, 5),
        "toggle-FreedomFiveBunker-6" => flip_bit!(freedom_five_bunker.1, 6),
        "toggle-FreedomFiveBunker-7" => flip_bit!(freedom_five_bunker.1, 7),
        "toggle-FreedomFiveBunker-8" => flip_bit!(freedom_five_bunker.1, 8),
        "toggle-FreedomFiveBunker-9" => flip_bit!(freedom_five_bunker.1, 9),
        "toggle-FreedomFiveBunker-10" => flip_bit!(freedom_five_bunker.1, 10),
        "counter-FreedomFiveWraith-trustfund" => incr!(freedom_five_wraith, 0, 2, 3),
        "counter-FreedomFiveWraith-cards" => incr!(freedom_five_wraith, 2, 5, 20),
        "toggle-FreedomFiveWraith-smokebombs" => flip_bit!(freedom_five_wraith, 7),
        "toggle-FreedomFiveTachyon-play" => flip_bit!(freedom_five_tachyon, 0),
        "toggle-FreedomFiveTachyon-shuffle" => flip_bit!(freedom_five_tachyon, 1),
        "toggle-FreedomFiveTachyon-incapacitated" => flip_bit!(freedom_five_tachyon, 2),
        "counter-FreedomFiveLegacy-prevented" => state.temporary_variant_progress.freedom_five_legacy.0 += 1,
        "counter-FreedomFiveLegacy-increased" => state.temporary_variant_progress.freedom_five_legacy.1 += 1,
        "toggle-FreedomFiveLegacy" => flip_bool!(freedom_five_legacy.2),
        "counter-SuperSentaiIdealist" => incr!(super_sentai_idealist, 0, 2, 2),
        "reset-SuperSentaiIdealist" => reset!(super_sentai_idealist, 0, 2),
        "counter-DrMedicoMalpractice" => incr!(dr_medico_malpractice, 0, 6, 50),
        "toggle-CosmicInventorWrithe-equipment" => flip_bit!(cosmic_inventor_writhe, 0),
        "toggle-CosmicInventorWrithe-ongoing" => flip_bit!(cosmic_inventor_writhe, 1),
        "toggle-CosmicInventorWrithe-more" => flip_bit!(cosmic_inventor_writhe, 2),
        "counter-RoadWarriorMainstay" => incr!(road_warrior_mainstay, 0, 2, 3),
        "reset-RoadWarriorMainstay" => reset!(road_warrior_mainstay, 0, 2),
        _ => (),
    };

    match target {
        "toggle-GIBunker-0" | "toggle-GIBunker-1" | "toggle-GIBunker-2" => multi_click_unlock(state.temporary_variant_progress.gi_bunker == 0x7, Variant::GIBunker),
        "counter-SantaGuise" => multi_click_unlock(state.temporary_variant_progress.santa_guise == 25, Variant::SantaGuise),
        "toggle-HeroicLuminary-0" | "toggle-HeroicLuminary-1" | "toggle-HeroicLuminary-2" => multi_click_unlock(state.temporary_variant_progress.heroic_luminary == 0x7, Variant::HeroicLuminary),
        "counter-KnyfeRogueAgent" => multi_click_unlock(state.temporary_variant_progress.knyfe_rogue_agent == 0xA, Variant::KnyfeRogueAgent),
        "counter-PrimeWardensCaptainCosmic" => multi_click_unlock(state.temporary_variant_progress.prime_wardens_captain_cosmic == 20, Variant::PrimeWardensCaptainCosmic),
        "counter-PrimeWardensFanatic-0" | "counter-PrimeWardensFanatic-1" | "counter-PrimeWardensFanatic-2" => {
            multi_click_unlock(state.temporary_variant_progress.prime_wardens_fanatic == 0x2A, Variant::PrimeWardensFanatic)
        }
        "toggle-PrimeWardensTempest-0" | "toggle-PrimeWardensTempest-1" | "toggle-PrimeWardensTempest-2" => {
            multi_click_unlock(state.temporary_variant_progress.prime_wardens_tempest == 0x7, Variant::PrimeWardensTempest)
        }
        _ => None,
    }
}

fn multi_click_unlock(condition: bool, variant: Variant) -> Option<Variant> {
    if condition {
        Some(variant)
    } else {
        None
    }
}
