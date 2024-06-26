use crate::{
    data::{Environment, Hero, TeamVillain, Variant, Villain},
    state::Items,
};

#[allow(clippy::too_many_lines)]
pub fn can_unlock(variant: Variant, items: &Items) -> bool {
    match variant {
        Variant::AmericasGreatestLegacy => items.has_villain(Villain::Ambuscade) && items.has_environment(Environment::SilverGulch1883),
        Variant::AmericasNewestLegacy => any_baron_blade(items) && items.has_hero(Hero::Legacy) && items.has_environment(Environment::WagnerMarsBase),
        Variant::DarkVisionary => items.has_villain(Villain::Gloomweaver) && items.has_hero(Hero::Visionary),
        Variant::TheEternalHaka => items.has_hero(Hero::Haka) && items.has_environment(Environment::TheFinalWasteland),
        Variant::GIBunker => items.has_hero(Hero::Bunker),
        Variant::RaHorusOfTwoHorizons => items.has_villain(Villain::TheEnnead) && items.has_hero(Hero::Ra),
        Variant::RaSettingSun => items.has_villain(Villain::TheEnnead) && items.has_hero_variant(Variant::RaHorusOfTwoHorizons) && items.has_environment(Environment::TombOfAnubis),
        Variant::RedeemerFanatic => items.has_villain(Villain::Apostate) && items.has_hero(Hero::Fanatic),
        Variant::RookCityWraith => items.has_hero(Hero::Wraith),
        Variant::TheSuperScientificTachyon => items.has_hero(Hero::Tachyon),
        Variant::TheVisionaryUnleashed => items.has_environment(Environment::TheEnclaveOfTheEndlings) && items.has_base_hero(Hero::ArgentAdept) && items.has_hero_variant(Variant::DarkVisionary),
        Variant::CaptainCosmicRequital => (items.has_villain(Villain::Infinitor) || items.has_villain(Villain::HeroicInfinitor)) && items.has_base_hero(Hero::CaptainCosmic),
        Variant::ChronoRangerTheBestOfTimes => {
            items.has_villain(Villain::Ambuscade) && items.has_environment(Environment::WagnerMarsBase) && items.has_hero(Hero::Tachyon) && items.has_hero(Hero::ChronoRanger)
        }
        Variant::DarkConductorArgentAdept => items.has_hero(Hero::ArgentAdept),
        Variant::ExtremistSkyScraper => any_baron_blade(items) && items.has_base_hero(Hero::SkyScraper),
        Variant::OmnitronU => items.has_villain(Villain::Omnitron) && items.has_villain(Villain::OmnitronII) && items.has_base_hero(Hero::OmnitronX) && items.has_hero(Hero::Unity),
        Variant::SantaGuise => items.has_hero(Hero::Guise),
        Variant::TheScholarOfTheInfinite => {
            (items.has_villain(Villain::Gloomweaver) || items.has_villain(Villain::SkinwalkerGloomweaver) || items.has_villain(Villain::Apostate)) && items.has_hero(Hero::TheScholar)
        }
        Variant::ActionHeroStuntman => {
            items.has_villain(Villain::Ambuscade)
                && items.has_villain(Villain::TheChairman)
                && items.has_team_villain(TeamVillain::Ambuscade)
                && items.team_villains.count_ones() >= 3
                && items.has_environment(Environment::PikeIndustrialComplex)
                && items.has_hero(Hero::TheSentinels)
        }
        Variant::AkashThriyaSpiritOfTheVoid => items.has_environment(Environment::NexusOfTheVoid) && items.has_hero(Hero::AkashThriya),
        Variant::BenchmarkSupplyAndDemand => {
            items.has_team_villain(TeamVillain::Ambuscade)
                && items.has_team_villain(TeamVillain::BaronBlade)
                && items.has_team_villain(TeamVillain::Friction)
                && items.has_team_villain(TeamVillain::FrightTrain)
                && items.has_team_villain(TeamVillain::PlagueRat)
                && items.has_hero(Hero::Benchmark)
                && items.has_hero(Hero::Expatriette)
                && items.has_hero(Hero::Luminary)
                && items.has_hero(Hero::Parse)
                && items.has_hero(Hero::Setback)
        }
        Variant::HeroicLuminary => {
            any_baron_blade(items)
                && items.has_environment(Environment::RealmOfDiscord)
                && items.has_environment(Environment::FreedomTower)
                && items.has_environment(Environment::Megalopolis)
                && items.has_hero(Hero::Luminary)
                && items.has_hero(Hero::Legacy)
                && items.has_hero(Hero::Bunker)
                && items.has_hero(Hero::AbsoluteZero)
                && items.has_hero(Hero::Tachyon)
                && items.has_hero(Hero::Wraith)
        }
        Variant::KnyfeRogueAgent => items.has_base_hero(Hero::Knyfe) && items.has_environment(Environment::TheBlock),
        Variant::LaComodoraCurseOfTheBlackSpot => items.has_hero(Hero::LaComodora) && items.has_environment(Environment::TimeCataclysm),
        Variant::LifelineBloodMage => items.has_hero(Hero::Lifeline) && items.has_environment(Environment::TheCourtOfBlood),
        Variant::ParseFugueState => items.has_base_hero(Hero::Parse) && items.has_villain(Villain::Progeny),
        Variant::TheAdamantSentinels => items.has_base_hero(Hero::TheSentinels),
        Variant::TheHuntedNaturalist => items.has_base_hero(Hero::TheNaturalist),
        Variant::TermiNationBunker => {
            items.has_villain(Villain::Omnitron) && items.has_villain(Villain::OmnitronII) && items.has_environment(Environment::OmnitronIV) && items.has_base_hero(Hero::Bunker)
        }
        Variant::TermiNationAbsoluteZero => items.has_base_hero(Hero::AbsoluteZero),
        Variant::TermiNationUnity => items.has_base_hero(Hero::Unity),
        Variant::FreedomSixAbsoluteZero => items.has_villain(Villain::IronLegacy) && items.has_base_hero(Hero::AbsoluteZero),
        Variant::FreedomSixBunker => items.has_villain(Villain::IronLegacy) && items.has_base_hero(Hero::Bunker),
        Variant::FreedomSixTachyon => items.has_villain(Villain::IronLegacy) && items.has_base_hero(Hero::Tachyon),
        Variant::FreedomSixTempest => items.has_villain(Villain::IronLegacy) && items.has_base_hero(Hero::Tempest),
        Variant::FreedomSixWraith => items.has_villain(Villain::IronLegacy) && items.has_base_hero(Hero::Wraith) && items.has_villain(Villain::TheChairman),
        Variant::FreedomSixUnity => items.has_villain(Villain::IronLegacy) && items.has_base_hero(Hero::Unity),
        Variant::DarkWatchExpatriette => any_baron_blade(items) && items.has_environment(Environment::RookCity) && items.has_hero(Hero::Expatriette),
        Variant::DarkWatchMisterFixer => items.has_villain(Villain::TheChairman) && items.has_hero(Hero::MisterFixer),
        Variant::DarkWatchNightmist => items.has_hero(Hero::Nightmist) && items.has_environment(Environment::RealmOfDiscord) && items.has_hero(Hero::Expatriette),
        Variant::DarkWatchSetback => {
            items.has_villain(Villain::TheChairman)
                && items.has_environment(Environment::RookCity)
                && items.has_hero_variant(Variant::DarkWatchExpatriette)
                && items.has_hero_variant(Variant::DarkWatchMisterFixer)
                && items.has_hero_variant(Variant::DarkWatchNightmist)
                && items.has_base_hero(Hero::Setback)
        }
        Variant::DarkWatchHarpy => {
            (items.has_villain(Villain::Gloomweaver) || items.has_villain(Villain::SkinwalkerGloomweaver)) && items.has_hero(Hero::TheHarpy) && items.has_environment(Environment::RealmOfDiscord)
        }
        Variant::PrimeWardensArgentAdept => {
            items.has_villain(Villain::AkashBhuta)
                && items.has_base_hero(Hero::ArgentAdept)
                && items.has_base_hero(Hero::CaptainCosmic)
                && items.has_base_hero(Hero::Haka)
                && items.has_base_hero(Hero::Tempest)
                && items.has_hero_variant(Variant::RedeemerFanatic)
        }
        Variant::PrimeWardensCaptainCosmic => {
            items.has_hero_variant(Variant::PrimeWardensArgentAdept) && items.has_base_hero(Hero::CaptainCosmic) && items.has_environment(Environment::DokThorathCapital)
        }
        Variant::PrimeWardensFanatic => {
            items.has_hero_variant(Variant::PrimeWardensArgentAdept) && (items.has_base_hero(Hero::Fanatic) || items.has_hero_variant(Variant::RedeemerFanatic)) && items.has_villain(Villain::Apostate)
        }
        Variant::PrimeWardensHaka => items.has_hero_variant(Variant::PrimeWardensArgentAdept) && items.has_base_hero(Hero::Haka) && items.has_villain(Villain::Ambuscade),
        Variant::PrimeWardensTempest => items.has_hero_variant(Variant::PrimeWardensArgentAdept) && items.has_base_hero(Hero::Tempest),
        Variant::XtremePrimeWardensArgentAdept => items.has_base_hero(Hero::ArgentAdept) && items.has_environment(Environment::InsulaPrimalis),
        Variant::XtremePrimeWardensTempest => items.has_base_hero(Hero::Tempest) && items.has_environment(Environment::TheEnclaveOfTheEndlings),
        Variant::XtremePrimeWardensCaptainCosmic => items.has_base_hero(Hero::CaptainCosmic) && items.has_environment(Environment::DokThorathCapital),
        Variant::XtremePrimeWardensFanatic => items.has_base_hero(Hero::Fanatic) && items.has_environment(Environment::TheCourtOfBlood),
        Variant::XtremePrimeWardensHaka => items.has_base_hero(Hero::Haka) && items.has_environment(Environment::Magmaria),
        Variant::FreedomFiveAbsoluteZero | Variant::FreedomFiveBunker | Variant::FreedomFiveWraith | Variant::FreedomFiveTachyon | Variant::FreedomFiveLegacy => freedom_five_reqs(items),
        Variant::SuperSentaiIdealist => items.has_hero(Hero::TheIdealist),
        Variant::DrMedicoMalpractice => items.has_hero(Hero::DoctorMedico) && items.has_team_villain(TeamVillain::Ambuscade) && items.team_villains.count_ones() >= 3,
        Variant::CosmicInventorWrithe => items.has_hero(Hero::Writhe),
        Variant::RoadWarriorMainstay => items.has_hero(Hero::Mainstay),
        Variant::MadBomberBaronBlade => items.has_villain(Villain::BaronBlade) && items.has_villain(Villain::CitizenDawn),
        Variant::OmnitronII => items.has_villain(Villain::Omnitron) && items.has_villain(Villain::GrandWarlordVoss),
        Variant::SpiteAgentOfGloom => items.has_villain(Villain::Spite) && items.has_villain(Villain::Gloomweaver),
        Variant::SkinwalkerGloomweaver => {
            items.has_villain(Villain::Spite) && items.has_villain(Villain::Gloomweaver) && items.has_villain(Villain::SpiteAgentOfGloom) && items.has_environment(Environment::RookCity)
        }
        Variant::TricksterKismet => {
            items.has_villain(Villain::Kismet) && items.has_environment(Environment::TheBlock) && items.has_hero(Hero::Knyfe) && items.has_hero(Hero::ArgentAdept) && items.has_hero(Hero::Fanatic)
        }
        Variant::HeroicInfinitor => (items.has_villain(Villain::Infinitor) || items.has_villain(Villain::HeroicInfinitor)) && items.has_hero(Hero::CaptainCosmic),
        _ => false,
    }
}

fn any_baron_blade(items: &Items) -> bool {
    items.has_villain(Villain::BaronBlade) || items.has_villain(Villain::MadBomberBaronBlade)
}

fn freedom_five_reqs(items: &Items) -> bool {
    items.has_villain(Villain::Progeny)
        && items.has_environment(Environment::RookCity)
        && items.has_environment(Environment::Megalopolis)
        && items.has_hero_variant(Variant::PrimeWardensArgentAdept)
        && items.has_hero_variant(Variant::PrimeWardensCaptainCosmic)
        && items.has_hero_variant(Variant::PrimeWardensFanatic)
        && items.has_hero_variant(Variant::PrimeWardensHaka)
        && items.has_hero_variant(Variant::PrimeWardensTempest)
        && items.has_hero_variant(Variant::DarkWatchExpatriette)
        && items.has_hero_variant(Variant::DarkWatchMisterFixer)
        && items.has_hero_variant(Variant::DarkWatchNightmist)
        && items.has_hero_variant(Variant::DarkWatchSetback)
        && items.has_hero_variant(Variant::DarkWatchHarpy)
        && items.has_base_hero(Hero::Legacy)
        && items.has_base_hero(Hero::Bunker)
        && items.has_base_hero(Hero::AbsoluteZero)
        && items.has_base_hero(Hero::Tachyon)
        && items.has_base_hero(Hero::Wraith)
}
