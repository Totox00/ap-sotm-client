use std::hash::Hash;

use generate_data::generate_data;
use num_derive::FromPrimitive;
use strum::EnumIter;

generate_data!(
    (BaronBlade, Villain, "Baron Blade"),
    (MadBomberBaronBlade, Villain, "Mad Bomber Baron Blade"),
    (CitizenDawn, Villain, "Citizen Dawn"),
    (GrandWarlordVoss, Villain, "Grand Warlord Voss"),
    (Omnitron, Villain, "Omnitron"),
    (OmnitronII, Villain, "Omnitron II"),
    (Ambuscade, Villain, "Ambuscade"),
    (TheChairman, Villain, "The Chairman"),
    (TheMatriarch, Villain, "The Matriarch"),
    (PlagueRat, Villain, "Plague Rat"),
    (Spite, Villain, "Spite"),
    (SpiteAgentOfGloom, Villain, "Spite: Agent of Gloom"),
    (AkashBhuta, Villain, "Akash'Bhuta"),
    (Apostate, Villain, "Apostate"),
    (TheEnnead, Villain, "The Ennead"),
    (Gloomweaver, Villain, "Gloomweaver"),
    (SkinwalkerGloomweaver, Villain, "Skinwalker Gloomweaver"),
    (MissInformation, Villain, "Miss Information"),
    (LaCapitan, Villain, "La Capitán"),
    (TheDreamer, Villain, "The Dreamer"),
    (IronLegacy, Villain, "Iron Legacy"),
    (Kismet, Villain, "Kismet"),
    (TricksterKismet, Villain, "Trickster Kismet"),
    (Deadline, Villain, "Deadline"),
    (Infinitor, Villain, "Infinitor"),
    (HeroicInfinitor, Villain, "Heroic Infinitor"),
    (KaargraWarfang, Villain, "Kaargra Warfang"),
    (Progeny, Villain, "Progeny"),
    (WagerMaster, Villain, "Wager Master"),
    (Chokepoint, Villain, "Chokepoint"),
    (Anathema, Villain, "Anathema"),
    (EvolvedAnathema, Villain, "Evolved Anathema"),
    (Celadroch, Villain, "Celadroch"),
    (Dendron, Villain, "Dendron"),
    (WindcolorDendron, Villain, "Windcolor Dendron"),
    (Dynamo, Villain, "Dynamo"),
    (Gray, Villain, "Gray"),
    (TheInfernalChoir, Villain, "The Infernal Choir"),
    (Menagerie, Villain, "Menagerie"),
    (TheMistressOfFate, Villain, "The Mistress of Fate"),
    (Mythos, Villain, "Mythos"),
    (Oriphel, Villain, "Oriphel"),
    (Outlander, Villain, "Outlander"),
    (Phase, Villain, "Phase"),
    (TheRam, Villain, "The Ram"),
    (Ram1929, Villain, "1929 Ram"),
    (ScreamMachine, Villain, "Scream Machine"),
    (SwarmEater, Villain, "Swarm Eater"),
    (HivemindSwarmEater, Villain, "Hivemind Swarm Eater"),
    (Tiamat, Villain, "Tiamat"),
    (HydraTiamat, Villain, "Hydra Tiamat"),
    (Tiamat2199, Villain, "Tiamat 2199"),
    (Vector, Villain, "Vector"),
    (BaronBlade, TeamVillain, "Baron Blade (Team)"),
    (Ermine, TeamVillain, "Ermine"),
    (Friction, TeamVillain, "Friction"),
    (FrightTrain, TeamVillain, "Fright Train"),
    (Proletariat, TeamVillain, "Proletariat"),
    (Ambuscade, TeamVillain, "Ambuscade (Team)"),
    (Biomancer, TeamVillain, "Biomancer"),
    (Bugbear, TeamVillain, "Bugbear"),
    (LaCapitan, TeamVillain, "La Capitan (Team)"),
    (CitizensHammerAndAnvil, TeamVillain, "Citizens Hammer And Anvil"),
    (Greazer, TeamVillain, "Greazer"),
    (MissInformation, TeamVillain, "Miss Information (Team)"),
    (TheOperative, TeamVillain, "The Operative"),
    (PlagueRat, TeamVillain, "Plague Rat (Team)"),
    (SergeantSteel, TeamVillain, "Sergeant Steel"),
    (AbsoluteZero, Hero, "Absolute Zero"),
    (Bunker, Hero, "Bunker"),
    (Fanatic, Hero, "Fanatic"),
    (Haka, Hero, "Haka"),
    (Legacy, Hero, "Legacy"),
    (Ra, Hero, "Ra"),
    (Tachyon, Hero, "Tachyon"),
    (Tempest, Hero, "Tempest"),
    (Visionary, Hero, "Visionary"),
    (Wraith, Hero, "Wraith"),
    (Unity, Hero, "Unity"),
    (Expatriette, Hero, "Expatriette"),
    (MisterFixer, Hero, "Mister Fixer"),
    (ArgentAdept, Hero, "Argent Adept"),
    (Nightmist, Hero, "Nightmist"),
    (TheScholar, Hero, "The Scholar"),
    (ChronoRanger, Hero, "Chrono Ranger"),
    (OmnitronX, Hero, "Omnitron-X"),
    (CaptainCosmic, Hero, "Captain Cosmic"),
    (SkyScraper, Hero, "Sky-Scraper"),
    (Guise, Hero, "Guise"),
    (Knyfe, Hero, "K.N.Y.F.E."),
    (TheNaturalist, Hero, "The Naturalist"),
    (Parse, Hero, "Parse"),
    (TheSentinels, Hero, "The Sentiels"),
    (Setback, Hero, "Setback"),
    (Benchmark, Hero, "Benchmark"),
    (Stuntman, Hero, "Stuntman"),
    (DoctorMedico, Hero, "Doctor Medico"),
    (TheIdealist, Hero, "The Idealist"),
    (Mainstay, Hero, "Mainstay"),
    (Writhe, Hero, "Writhe"),
    (AkashThriya, Hero, "Akash'Thriya"),
    (LaComodora, Hero, "La Comodora"),
    (TheHarpy, Hero, "The Harpy"),
    (Lifeline, Hero, "Lifeline"),
    (Luminary, Hero, "Luminary"),
    (Baccarat, Hero, "Baccarat"),
    (TheCricket, Hero, "The Cricket"),
    (Cypher, Hero, "Cypher"),
    (DocHavoc, Hero, "Doc Havoc"),
    (Drift, Hero, "Drift"),
    (Echelon, Hero, "Echelon"),
    (Gargoyle, Hero, "Gargoyle"),
    (Gyrosaur, Hero, "Gyrosaur"),
    (Impact, Hero, "Impact"),
    (TheKnight, Hero, "The Knight"),
    (LadyOfTheWood, Hero, "Lady of the Wood"),
    (MagnificentMara, Hero, "Magnificent Mara"),
    (Malichae, Hero, "Malichae"),
    (Necro, Hero, "Necro"),
    (Pyre, Hero, "Pyre"),
    (Quicksilver, Hero, "Quicksilver"),
    (Starlight, Hero, "Starlight"),
    (TheStranger, Hero, "The Stranger"),
    (TangoOne, Hero, "Tango One"),
    (Terminus, Hero, "Terminus"),
    (Titan, Hero, "Titan"),
    (Vanish, Hero, "Vanish"),
    (InsulaPrimalis, Environment, "Insula Primalis"),
    (Megalopolis, Environment, "Megalopolis"),
    (RuinsOfAtlantis, Environment, "Ruins of Atlantis"),
    (WagnerMarsBase, Environment, "Wagner Mars Base"),
    (SilverGulch1883, Environment, "Silver Gulch 1883"),
    (PikeIndustrialComplex, Environment, "Pike Industrial Complex"),
    (RookCity, Environment, "Rook City"),
    (RealmOfDiscord, Environment, "Realm of Discord"),
    (TombOfAnubis, Environment, "Tomb of Anubis"),
    (TheFinalWasteland, Environment, "The Final Wasteland"),
    (TheBlock, Environment, "The Block"),
    (TimeCataclysm, Environment, "Time Cataclysm"),
    (DokThorathCapital, Environment, "Dok Thorath Capital"),
    (TheEnclaveOfTheEndlings, Environment, "The Enclave of the Endlings"),
    (OmnitronIV, Environment, "Omnitron IV"),
    (FreedomTower, Environment, "Freedom Tower"),
    (MobileDefensePlatform, Environment, "Mobile Defense Platform"),
    (TheCourtOfBlood, Environment, "The Court of Blood"),
    (
        MadameMittermeiersFantasticalFestivalOfConundrumsAndCuriosities,
        Environment,
        "Madame Mittermeier's Fantastical Festival of Conundrums and Curiosities"
    ),
    (Magmaria, Environment, "Magmaria"),
    (TheTempleOfZhuLong, Environment, "The Temple of Zhu Long"),
    (TheCelestialTribunal, Environment, "The Celestial Tribunal"),
    (ChampionStudios, Environment, "Champion Studios"),
    (FortAdamant, Environment, "Fort Adamant"),
    (MaerynianRefuge, Environment, "Maerynian Refuge"),
    (Mordengrad, Environment, "Mordengrad"),
    (NexusOfTheVoid, Environment, "Nexus of the Void"),
    (BlackwoodForest, Environment, "Blackwood Forest"),
    (CatchwaterHarbor1929, Environment, "Catchwater Harbor 1929"),
    (TheChasmOfAThousandNights, Environment, "The Chasm of A Thousand Nights"),
    (TheCybersphere, Environment, "The Cybersphere"),
    (DungeonsOfTerror, Environment, "Dungeons of Terror"),
    (FSCContinuanceWanderer, Environment, "F.S.C. Continuance Wanderer"),
    (HalberdERC, Environment, "Halberd E.R.C."),
    (NightloreCitadel, Environment, "Nightlore Citadel"),
    (Northspar, Environment, "Northspar"),
    (OblaskCrater, Environment, "Oblask Crater"),
    (StSimeonsCatacombs, Environment, "St. Simeon's Catacombs"),
    (SuperstormAkela, Environment, "Superstorm Akela"),
    (Vault5, Environment, "Vault 5"),
    (TheWanderingIsle, Environment, "The Wandering Isle"),
    (WindmillCity, Environment, "Windmill City"),
    (AmericasGreatestLegacy, Variant, Legacy, "America's Greatest Legacy"),
    (AmericasNewestLegacy, Variant, Legacy, "America's Newest Legacy"),
    (DarkVisionary, Variant, Visionary, "Dark Visionary"),
    (TheEternalHaka, Variant, Haka, "The Eternal Haka"),
    (GIBunker, Variant, Bunker, "G.I. Bunker"),
    (RaHorusOfTwoHorizons, Variant, Ra, "Ra: Horus of Two Horizons"),
    (RaSettingSun, Variant, Ra, "Ra: Setting Sun"),
    (RedeemerFanatic, Variant, Fanatic, "Redeemer Fanatic"),
    (RookCityWraith, Variant, Wraith, "Rook City Wraith"),
    (TheSuperScientificTachyon, Variant, Tachyon, "The Super-Scientific Tachyon"),
    (TheVisionaryUnleashed, Variant, Visionary, "The Visionary Unleashed"),
    (CaptainCosmicRequital, Variant, CaptainCosmic, "Captain Cosmic Requital"),
    (ChronoRangerTheBestOfTimes, Variant, ChronoRanger, "Chrono-Ranger the Best of Times"),
    (DarkConductorArgentAdept, Variant, ArgentAdept, "Dark Conductor Argent Adept"),
    (ExtremistSkyScraper, Variant, SkyScraper, "Extremist Sky-Scraper"),
    (OmnitronU, Variant, OmnitronX, "Omnitron-U"),
    (SantaGuise, Variant, Guise, "Santa Guise"),
    (TheScholarOfTheInfinite, Variant, TheScholar, "The Scholar of the Infinite"),
    (ActionHeroStuntman, Variant, Stuntman, "Action Hero Stuntman"),
    (AkashThriyaSpiritOfTheVoid, Variant, AkashThriya, "Akash'Thriya: Spirit of the Void"),
    (BenchmarkSupplyAndDemand, Variant, Benchmark, "Benchmark Supply & Demand"),
    (HeroicLuminary, Variant, Luminary, "Heroic Luminary"),
    (KnyfeRogueAgent, Variant, Knyfe, "K.N.Y.F.E. Rogue Agent"),
    (LaComodoraCurseOfTheBlackSpot, Variant, LaComodora, "La Comodora: Curse of the Black Spot"),
    (LifelineBloodMage, Variant, Lifeline, "Lifeline Blood Mage"),
    (ParseFugueState, Variant, Parse, "Parse: Fugue State"),
    (TheAdamantSentinels, Variant, TheSentinels, "The Adamant Sentinels"),
    (TheHuntedNaturalist, Variant, TheNaturalist, "The Hunted Naturalist"),
    (TermiNationBunker, Variant, Bunker, "Termi-Nation Bunker"),
    (TermiNationAbsoluteZero, Variant, AbsoluteZero, "Termi-Nation Absolute Zero"),
    (TermiNationUnity, Variant, Unity, "Termi-Nation Unity"),
    (FreedomSixAbsoluteZero, Variant, AbsoluteZero, "Freedom Six Absolute Zero"),
    (FreedomSixBunker, Variant, Bunker, "Freedom Six Bunker"),
    (FreedomSixTachyon, Variant, Tachyon, "Freedom Six Tachyon"),
    (FreedomSixTempest, Variant, Tempest, "Freedom Six Tempest"),
    (FreedomSixWraith, Variant, Wraith, "Freedom Six Wraith"),
    (FreedomSixUnity, Variant, Unity, "Freedom Six Unity"),
    (DarkWatchExpatriette, Variant, Expatriette, "Dark Watch Expatriette"),
    (DarkWatchMisterFixer, Variant, MisterFixer, "Dark Watch Mister Fixer"),
    (DarkWatchNightmist, Variant, Nightmist, "Dark Watch Nightmist"),
    (DarkWatchSetback, Variant, Setback, "Dark Watch Setback"),
    (DarkWatchHarpy, Variant, TheHarpy, "Dark Watch Harpy"),
    (PrimeWardensArgentAdept, Variant, ArgentAdept, "Prime Wardens Argent Adept"),
    (PrimeWardensCaptainCosmic, Variant, CaptainCosmic, "Prime Wardens Captain Cosmic"),
    (PrimeWardensFanatic, Variant, Fanatic, "Prime Wardens Fanatic"),
    (PrimeWardensHaka, Variant, Haka, "Prime Wardens Haka"),
    (PrimeWardensTempest, Variant, Tempest, "Prime Wardens Tempest"),
    (XtremePrimeWardensArgentAdept, Variant, ArgentAdept, "Xtreme Prime Wardens Argent Adept"),
    (XtremePrimeWardensTempest, Variant, Tempest, "Xtreme Prime Wardens Tempest"),
    (XtremePrimeWardensCaptainCosmic, Variant, CaptainCosmic, "Xtreme Prime Wardens Captain Cosmic"),
    (XtremePrimeWardensFanatic, Variant, Fanatic, "Xtreme Prime Wardens Fanatic"),
    (XtremePrimeWardensHaka, Variant, Haka, "Xtreme Prime Wardens Haka"),
    (FreedomFiveAbsoluteZero, Variant, AbsoluteZero, "Freedom Five Absolute Zero"),
    (FreedomFiveBunker, Variant, Bunker, "Freedom Five Bunker"),
    (FreedomFiveWraith, Variant, Wraith, "Freedom Five Wraith"),
    (FreedomFiveTachyon, Variant, Tachyon, "Freedom Five Tachyon"),
    (FreedomFiveLegacy, Variant, Legacy, "Freedom Five Legacy"),
    (SuperSentaiIdealist, Variant, TheIdealist, "Super Sentai Idealist"),
    (DrMedicoMalpractice, Variant, DoctorMedico, "Dr. Medico Malpractice"),
    (CosmicInventorWrithe, Variant, Writhe, "Cosmic Inventor Writhe"),
    (RoadWarriorMainstay, Variant, Mainstay, "Road Warrior Mainstay"),
    (CompletionistGuise, Variant, Guise, "Completionist Guise"),
    (MadBomberBaronBlade, Variant, Villain, "Mad Bomber Baron Blade"),
    (OmnitronII, Variant, Villain, "Omnitron II"),
    (SpiteAgentOfGloom, Variant, Villain, "Spite Agent of Gloom"),
    (SkinwalkerGloomweaver, Variant, Villain, "Skinwalker Gloomweaver"),
    (TricksterKismet, Variant, Villain, "Trickster Kismet"),
    (HeroicInfinitor, Variant, Villain, "Heroic Infinitor"),
    (BaccaratAceOfSwords, Variant, Baccarat, "Baccarat Ace of Swords"),
    (BaccaratAceOfSorrows, Variant, Baccarat, "Baccarat Ace of Sorrows"),
    (Baccarat1929, Variant, Baccarat, "Baccarat 1929"),
    (FirstResponseCricket, Variant, TheCricket, "First Response Cricket"),
    (TheCricketRenegade, Variant, TheCricket, "The Cricket Renegade"),
    (TheCricketWastelandRonin, Variant, TheCricket, "The Cricket Wasteland Ronin"),
    (FirstResponseCypher, Variant, Cypher, "First Response Cypher"),
    (CypherSwarmingProtocol, Variant, Cypher, "Cypher Swarming Protocol"),
    (FirstResponseDocHavoc, Variant, DocHavoc, "First Response Doc Havoc"),
    (DocHavoc2199, Variant, DocHavoc, "Doc Havoc 2199"),
    (DriftThroughTheBreach, Variant, Drift, "Drift Through the Breach"),
    (Drift1929And2199, Variant, Drift, "Drift 1929 & 2199"),
    (Drift1609, Variant, Drift, "Drift 1609"),
    (Drift1789, Variant, Drift, "Drift 1789"),
    (TestSubjectDrift, Variant, Drift, "Test Subject Drift"),
    (FirstResponseEchelon, Variant, Echelon, "First Response Echelon"),
    (Echelon2199, Variant, Echelon, "Echelon 2199"),
    (GargoyleWastelandRonin, Variant, Gargoyle, "Gargoyle Wasteland Ronin"),
    (Gargoyle2199, Variant, Gargoyle, "Gargoyle 2199"),
    (GargoyleDragonRanger, Variant, Gargoyle, "Gargoyle Dragon Ranger"),
    (GargoyleInfiltrator, Variant, Gargoyle, "Gargoyle Infiltrator"),
    (GyrosaurSpeedDemon, Variant, Gyrosaur, "Gyrosaur Speed Demon"),
    (GyrosaurRenegade, Variant, Gyrosaur, "Gyrosaur Renegade"),
    (CaptainGyrosaur, Variant, Gyrosaur, "Captain Gyrosaur"),
    (ImpactRenegade, Variant, Impact, "Impact Renegade"),
    (ImpactWastelandRonin, Variant, Impact, "Impact Wasteland Ronin"),
    (TheFairKnight, Variant, TheKnight, "The Fair Knight"),
    (TheBerserkerKnight, Variant, TheKnight, "The Berserker Knight"),
    (TheKnight1929, Variant, TheKnight, "The Knight 1929"),
    (TheKnightsWastelandRonin, Variant, TheKnight, "The Knights Wasteland Ronin"),
    (LadyOfTheWoodSeasonOfChange, Variant, LadyOfTheWood, "Lady of the Wood Season of Change"),
    (MinistryOfStrategicScienceLadyOfTheWood, Variant, LadyOfTheWood, "Ministry of Strategic Science Lady of the Wood"),
    (LadyOfTheWood2199, Variant, LadyOfTheWood, "Lady of the Wood 2199"),
    (MinistryOfStrategicScienceMagnificentMara, Variant, MagnificentMara, "Ministry of Strategic Science Magnificent Mara"),
    (MagnificentMara1929, Variant, MagnificentMara, "Magnificent Mara 1929"),
    (ShardmasterMalichae, Variant, Malichae, "Shardmaster Malichae"),
    (MinistryOfStrategicScienceMalichae, Variant, Malichae, "Ministry of Strategic Science Malichae"),
    (NecroWardenOfChaos, Variant, Necro, "Necro Warden of Chaos"),
    (Necro1929, Variant, Necro, "Necro 1929"),
    (NecroLastOfTheForgottenOrder, Variant, Necro, "Necro Last of the Forgotten Order"),
    (TheUnstablePyre, Variant, Pyre, "The Unstable Pyre"),
    (PyreWastelandRonin, Variant, Pyre, "Pyre Wasteland Ronin"),
    (PyreExpeditionOblask, Variant, Pyre, "Pyre Expedition Oblask"),
    (TheUncannyQuicksilver, Variant, Quicksilver, "The Uncanny Quicksilver"),
    (QuicksilverRenegade, Variant, Quicksilver, "Quicksilver Renegade"),
    (HarbingerQuicksilver, Variant, Quicksilver, "Harbinger Quicksilver"),
    (StarlightGenesis, Variant, Starlight, "Starlight Genesis"),
    (NightloreCouncilStarlight, Variant, Starlight, "Nightlore Council Starlight"),
    (StarlightArea51, Variant, Starlight, "Starlight Area-51"),
    (TheRunecarvedStranger, Variant, TheStranger, "The Runecarved Stranger"),
    (TheStranger1929, Variant, TheStranger, "The Stranger 1929"),
    (TheStrangerWastelandRonin, Variant, TheStranger, "The Stranger Wasteland Ronin"),
    (TheStrangerInTheCorn, Variant, TheStranger, "The Stranger in the Corn"),
    (TangoOneGhostOps, Variant, TangoOne, "Tango One Ghost Ops"),
    (TangoOne1929, Variant, TangoOne, "Tango One 1929"),
    (TangoOneCreedOfTheSniper, Variant, TangoOne, "Tango One Creed of the Sniper"),
    (MinistryOfStrategicScienceTerminus, Variant, Terminus, "Ministry of Strategic Science Terminus"),
    (Terminus2199, Variant, Terminus, "Terminus 2199"),
    (MinistryOfStrategicScienceTitan, Variant, Titan, "Ministry of Strategic Science Titan"),
    (Titan2199, Variant, Titan, "Titan 2199"),
    (TitanOni, Variant, Titan, "Titan Oni"),
    (FirstResponseVanish, Variant, Vanish, "First Response Vanish"),
    (Vanish1929, Variant, Vanish, "Vanish 1929"),
    (VanishTombOfThieves, Variant, Vanish, "Vanish Tomb of Thieves"),
    (UrbanWarfareExpatriette, Variant, Expatriette, "Urban Warfare Expatriette"),
    (SiegeBreakerBunker, Variant, Bunker, "Siege Breaker Bunker"),
    (NitroBoostAbsoluteZero, Variant, AbsoluteZero, "Nitro Boost Absolute Zero"),
    (EnlightenedMisterFixer, Variant, MisterFixer, "Enlightened Mister Fixer"),
    (NorthernWindMisterFixer, Variant, MisterFixer, "Northern Wind Mister Fixer"),
    (OmnitronXI, Variant, OmnitronX, "Omnitron-XI")
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item {
    Hero(Hero),
    Variant(Variant),
    Villain(Villain),
    TeamVillain(TeamVillain),
    Environment(Environment),
    Scion,
    Filler,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Location {
    Variant(Variant),
    Villain((Villain, u8)),
    TeamVillain((TeamVillain, u8)),
    Environment(Environment),
}

impl Variant {
    pub fn as_desc(&self) -> &str {
        match self {
            Variant::AmericasGreatestLegacy => "Defeat Ambuscade in Silver Gulch with no hero character cards over 9 HP and Legacy not on your team.",
            Variant::AmericasNewestLegacy => "Baron Blade himself deals damage that incapacitates Legacy in Wagner Mars Base.",
            Variant::DarkVisionary => "Defeat GloomWeaver on his \"Demon-God_Incarnate\" side using The Visionary. Telekinetic Cocoon must have been destroyed at the start of The Visionary's turn",
            Variant::TheEternalHaka => "Win a game where Haka is the only non-incapacitated hero. Then, defeat any villain in The Final Wasteland, playing each \"Haka_of\" card at least once each. Again, Haka must be the only non-incapacitated hero.",
            Variant::GIBunker => "Play all three mode cards in the same game.",
            Variant::RaHorusOfTwoHorizons => "A member of The Ennead deals damage that incapacitates Ra. The Staff of Ra, Flame Barrier, Flesh of the Sun God, Living Conflagration, Solar Flare, and Wrathful Gaze must be in play.",
            Variant::RaSettingSun => "Defeat the Ennead in the Tomb of Anubis with Ra: Horus of Two Horizons as the first hero. Fanatic may not be on the team. During the game, Ra must be reduced to 1_HP, then restored to maximum HP. Ra must deal the final blow to at least 3_members of the Ennead.",
            Variant::RedeemerFanatic => "Defeat Apostate with damage dealt by Fanatic using the power on \"Absolution.\" Undaunted must be in play. Prayer of Desperation must have been played at some time after Fanatic was restored by Aegis of Resurrection.",
            Variant::RookCityWraith => "Use the Infrared Eyepiece power twice in the same turn.",
            Variant::TheSuperScientificTachyon => "Put five or more Burst cards into the trash in a single turn.",
            Variant::TheVisionaryUnleashed => "Complete a game in the Enclave of the Endlings with the regular Argent Adept and Dark Visionary on the team. The Argent Adept must use the effects of at least one melody, one harmony, and one rhythm on Dark Visionary. Dark Visionary must be incapacitated, and the Argent Adept may not be incapacitated.",
            Variant::CaptainCosmicRequital => "Defeat Infinitor with regular Captain Cosmic on the team. 10_Manifestations and 5_Constructs must be destroyed. Captain Cosmic must deal the final blow to Infinitor.",
            Variant::ChronoRangerTheBestOfTimes => "Win a game against Ambuscade in the Wagner Mars Base with any versions of Tachyon and Chrono-Ranger on the team. Tachyon must not be incapacitated. Chrono-Ranger must not play any bounties on targets other than Ambuscade. Neither Tachyon nor Chrono-Ranger may deal the final blow to Ambuscade.",
            Variant::DarkConductorArgentAdept => "Win a game with any version of the Argent Adept on the team. Every time the Argent Adept uses a power, perform text, or accompany text that can benefit himself, he must benefit himself. The Argent Adept must deal at least 20_damage over the course of the game.",
            Variant::ExtremistSkyScraper => "Win a game against Baron Blade with regular Sky-Scraper on the team. Three separate times in the game, Sky-Scraper must be Tiny, Normal, and Huge over the course of a single turn.",
            Variant::OmnitronU => "Defeat regular Omnitron with Omnitron-X and Unity on the team; neither of them may be incapacitated. Then, defeat Cosmic Omnitron with Omnitron-X and Unity on the team; Omnitron-X must be incapacitated, but Unity may not be incapacitated. Then, defeat any villain with Unity and without Tachyon or Omnitron-X on the team. Construction Pylon, Modular Workbench, Scrap Metal, Supply Crate, and Volatile Parts must be in play. Robot Reclamation must be played at least once.",
            Variant::SantaGuise => "In a game with Guise on the team, allow a single player to take 25_actions outside their turn. Valid actions are playing a card, using a power, or drawing a card. The actions must be caused by hero cards, powers, or incapacitated abilities.",
            Variant::TheScholarOfTheInfinite => "Defeat either GloomWeaver or Apostate with The Scholar incapacitated. The Scholar must have healed hero targets for at least 20_HP in total.",
            Variant::ActionHeroStuntman => "First, defeat Ambuscade_(classic_mode) without any heroes getting incapacitated. Then, defeat Ambuscade_(team_mode) without any heroes getting incapacitated. Then, defeat The Chairman in Pike Industrial Complex with The Sentinels on the team. At the end of that game, all heroes except Mainstay must be incapacitated.",
            Variant::AkashThriyaSpiritOfTheVoid => "Win a game in Nexus of the Void with Akash'Thriya on the team. Each unique primordial seed must enter the environment deck. Akash'Thriya must be reduced to 9_or less HP, then recover back to full HP. Akash'Flora must be in play at the end of the game without ever having left play.",
            Variant::BenchmarkSupplyAndDemand => "First, win a team mode game wherein you play at least (100_-_10*X) equipment cards and destroy at least (50_-_5*X) devices, where X is the number of RevoCorp related decks in the game. Then, play an equipment card. The list of RevoCorp related decks is: Ambuscade, Baron Blade, Benchmark, Expatriette, Friction, Fright Train, Luminary, Parse, Plague Rat, and Setback.",
            Variant::HeroicLuminary => "First, defeat Baron Blade in the Realm of Discord without Luminary on the team. Then, win a game in Freedom Tower with the Freedom Five (any variants). Then, in a game in Megalopolis with Luminary on the team, use the power on each Doomsday Device while at least 15_cards are in Luminary's trash.",
            Variant::KnyfeRogueAgent => "In a game with regular K.N.Y.F.E. in The Block, have K.N.Y.F.E. destroy 5_Agents other than Warden Hoefle. K.N.Y.F.E. may not destroy Warden Hoefle.",
            Variant::LaComodoraCurseOfTheBlackSpot => "Win a game in Time Cataclysm with La Comodora on the team. Each of La Comodora's unique equipment cards must be in play at one time. After that, each of her unique equipment cards must be in her trash at once. La Comodora must deal the final blow with at least 10_damage using Run Aground.",
            Variant::LifelineBloodMage => "Win a game in the Court of Blood with Lifeline on the team. Play each of Lifeline's unique one-shots while both copies of Cosmic Immolation are in play. Lifeline must be incapacitated by Blood Countess Bathory. No other heroes may be incapacitated.",
            Variant::ParseFugueState => "Defeat Progeny with regular Parse on the team. At the end of the game, Parse must have at least 10 ongoing cards in play, and must be at exactly 1_HP.",
            Variant::TheAdamantSentinels => "Lose a game with regular The Sentinels incapacitated at the end. The Sentinels must be incapacitated before any other heroes are incapacitated. All four Signature cards along with Sentinel Tactics must have been in play at the same time during the game.",
            Variant::TheHuntedNaturalist => "Win three games where regular The Naturalist deals the final blow to the villain. Each different villain must be defeated while The Naturalist is in a different form.",
            Variant::TermiNationBunker => "Defeat Omnitron in Omnitron-IV with regular Bunker as the first hero. Then, defeat Cosmic Omnitron in Omnitron-IV with regular Bunker as the last hero. At the end of both games, Bunker must be active and have less than 10_HP.",
            Variant::TermiNationAbsoluteZero => "In a single non-hero turn, regular Absolute Zero must take over 10_damage, regain over 10_HP, and deal over 10_damage to a single non-hero target (in_one_shot). These three things can happen in any sequence, but all must occur on the same turn.",
            Variant::TermiNationUnity => "In a game with regular Unity on the team, more than 5_times_(H) hero targets must be in play for a complete round (from the start of the villain turn through the end of the environment turn). Only hero targets that have HP printed on their card count.",
            Variant::FreedomSixAbsoluteZero => "Lose a game to Iron Legacy with regular Absolute Zero on the team. Then, defeat any villain with regular Absolute Zero and without a Legacy on the team. Each of Absolute Zero's equipment cards must be destroyed at least once, and he may not be incapacitated.",
            Variant::FreedomSixBunker => "Lose a game to Iron Legacy with regular Bunker on the team. Then, defeat any villain with regular Bunker and without a Legacy on the team. Bunker must draw at least 12_cards, and he must be the only incapacitated hero.",
            Variant::FreedomSixTachyon => "Lose a game to Iron Legacy with regular Tachyon on the team. Then, defeat any villain with regular Tachyon and without a Legacy on the team. Fleet of Foot must be played at least 5_times.",
            Variant::FreedomSixTempest => "Lose a game to Iron Legacy with regular Tempest on the team. Then, defeat any villain with regular Tempest and without a Legacy on the team. Aquatic Correspondence, Localized Hurricane, and Reclaim from the Deep must be played at least once each. Tempest must be the only incapacitated hero.",
            Variant::FreedomSixWraith => "Lose a game to Iron Legacy with regular The Wraith on the team. Then, defeat The Chairman with The Wraith as the first hero and without a Legacy on the team. The Wraith must deal the final blows to The Chairman and The Operative.",
            Variant::FreedomSixUnity => "Lose a game to Iron Legacy with regular Unity on the team. Then, defeat any villain with regular Unity and without a Legacy on the team. Unity must end the game with more mechanical golems in play than her HP.",
            Variant::DarkWatchExpatriette => "Defeat Baron Blade in Rook City with Expatriette using the card \"Unload\" to fire guns at least 3_times at Baron Blade to destroy him.",
            Variant::DarkWatchMisterFixer => "The Operative herself deals damage that incapacitates Mr. Fixer in Rook City. Bloody Knuckles, any one Style, and no Tool must be in play.",
            Variant::DarkWatchNightmist => "In the Realm of Discord, deal damage to both hero and villain targets with an Oblivion play that revealed two 4_spell_icons. Expatriette must be on the team and Master of Magic must be in play",
            Variant::DarkWatchSetback => "Defeat The Chairman in Rook City with the 4_hero team: Setback, Dark Watch Expatriette, Dark Watch Mr. Fixer, and Dark Watch NightMist (in any order).",
            Variant::DarkWatchHarpy => "Defeat Advanced GloomWeaver in the Realm of Discord with The Harpy on the team. Flip all control tokens to avian, then all to arcana. After that, they must remain all arcana until the end of the game. The Harpy must destroy at least one villain relic, and may not be incapacitated.",
            Variant::PrimeWardensArgentAdept => "Lose a game to Akash'bhuta with the regular Argent Adept on the team. Then, defeat Akash'bhuta with the regular Argent Adept as the first hero; the rest of the team must be regular Haka, regular Captain Cosmic, regular Tempest, and Redeemer Fanatic (in_any_order). Each of the Argent Adept's instruments must enter play during the game.",
            Variant::PrimeWardensCaptainCosmic => "Prime Wardens Argent Adept must be unlocked. Then, in a game in Dok'Thorath Capital, standard Captain Cosmic must prevent a total of 20_damage that would be dealt to other heroes or refugees. The prevention must be accomplished with Energy Bracers or by redirecting damage from Abject Refugees to Captain Cosmic.",
            Variant::PrimeWardensFanatic => "Prime Wardens Argent Adept must be unlocked. Then, in a game against Apostate, regular or Redeemer Fanatic must destroy 2_Imp Pilferers, 2_Fiendish Pugilists, and 2_Relic Spirits.",
            Variant::PrimeWardensHaka => "Prime Wardens Argent Adept must be unlocked. Then, defeat Ambuscade with regular Haka on the team. At the end of the game, Haka must be the only active hero, and be at maximum HP.",
            Variant::PrimeWardensTempest => "Prime Wardens Argent Adept must be unlocked. Then, with regular Tempest in a game, play Ball Lightning, Chain Lightning, and Lightning Slash at any time when Electrical Storm, Grievous Hail Storm, Localized Hurricane, and Vicious Cyclone are all in play.",
            Variant::XtremePrimeWardensArgentAdept => "Win a game in Insula Primalis with the regular Argent Adept on the team. The Argent Adept must use his Vocalize power to activate each of his unique Melody, Harmony, and Rhythm cards at least once each.",
            Variant::XtremePrimeWardensTempest => "Win a game in the Enclave of the Endlings with regular Captain Cosmic on the team. At least 10_Construct cards must enter play. By using his own card effects, damage from himself, or damage from Constructs, Captain Cosmic must destroy at least 10_Construct cards.",
            Variant::XtremePrimeWardensCaptainCosmic => "Win a game in Dok'Thorath Capital with regular Tempest on the team. Tempest must deal at least 1_damage on each of Tempest's turns. At the end of the game, both Gene-Bound Shackles and Elemental Subwave Inducer must be in play.",
            Variant::XtremePrimeWardensFanatic => "Win a game in the Court of Blood with regular Fanatic on the team. Fanatic must destroy each unique Vampire at least once. Fanatic must not be dealt any infernal damage.",
            Variant::XtremePrimeWardensHaka => "Win a game in Magmaria with regular Haka on the team. Haka may never have any equipment cards in his play area. Haka must destroy at least 5_villain targets and at least 5_environment targets.",
            Variant::FreedomFiveAbsoluteZero => "First, the Prime Wardens (all variant members) must be defeated by Progeny anywhere other than Rook City or Megalopolis. Then, Dark Watch (all variant members) must by defeated by Progeny in Rook City. Then, the Freedom Five (non-variant members) must be defeated by Progeny in Rook City. Then, the Freedom Five (non-variant team members) must defeat Progeny in Megalopolis. During that game, Absolute Zero must be dealt over 29_fire damage and must deal over 29_cold damage. Absolute Zero must have at least 6_Ongoing cards in play at one time.",
            Variant::FreedomFiveBunker => "First, the Prime Wardens (all variant members) must be defeated by Progeny anywhere other than Rook City or Megalopolis. Then, Dark Watch (all variant members) must by defeated by Progeny in Rook City. Then, the Freedom Five (non-variant members) must be defeated by Progeny in Rook City. Then, the Freedom Five (non-variant team members) must defeat Progeny in Megalopolis. During that game, Bunker must discard each unique Mode card at least twice. Bunker must deal at least 4_different types of damage to Progeny.",
            Variant::FreedomFiveWraith => "First, the Prime Wardens (all variant members) must be defeated by Progeny anywhere other than Rook City or Megalopolis. Then, Dark Watch (all variant members) must by defeated by Progeny in Rook City. Then, the Freedom Five (non-variant members) must be defeated by Progeny in Rook City. Then, the Freedom Five (non-variant team members) must defeat Progeny in Megalopolis. During that game, Trust Fund must be played at least 3_times. The Wraith must draw at least 20_cards and play Smoke Bombs at least once.",
            Variant::FreedomFiveTachyon => "First, the Prime Wardens (all variant members) must be defeated by Progeny anywhere other than Rook City or Megalopolis. Then, Dark Watch (all variant members) must by defeated by Progeny in Rook City. Then, the Freedom Five (non-variant members) must be defeated by Progeny in Rook City. Then, the Freedom Five (non-variant team members) must defeat Progeny in Megalopolis. During that game, Tachyon must play at least 10_cards in one turn. Tachyon must run out of cards in her deck and thus shuffle her trash into her deck at least once. Tachyon must be incapacitated.",
            Variant::FreedomFiveLegacy => "First, the Prime Wardens (all variant members) must be defeated by Progeny anywhere other than Rook City or Megalopolis. Then, Dark Watch (all variant members) must by defeated by Progeny in Rook City. Then, the Freedom Five (non-variant members) must be defeated by Progeny in Rook City. Then, the Freedom Five (non-variant team members) must defeat Progeny in Megalopolis. During that game, Legacy must prevent at least 20_points of damage and must prevent more damage than he increases. Danger Sense must be played during the game.",
            Variant::SuperSentaiIdealist => "First, 5_Concepts must be in play with at least 2_cards under each of them for at least 2_rounds. Then, in the same game, 5_Concepts must be in play and no Concept may have a card under it.",
            Variant::DrMedicoMalpractice => "Complete a game with Void Guard Writhe on the team. Void Guard Writhe must play at least 1_equipment card and at least 1_ongoing card. Whenever Void Guard Writhe has an ongoing card in his play area, he must have more equipment cards than ongoing cards in his play area.",
            Variant::CosmicInventorWrithe => "First, Void Guard Dr. Medico must be dealt 50_damage in a single game. Then, Void Guard Dr. Medico must destroy Re-Volt.",
            Variant::RoadWarriorMainstay => "First, all 3_of Void Guard Mainstay's equipment cards must enter and remain in play for at least 3_rounds. Then, in the same game, Void Guard Mainstay must destroy all 3_of his equipment cards in one turn.",
            Variant::MadBomberBaronBlade => "Defeat standard Baron Blade. Then, while fighting Citizen Dawn, destroy Citizens Blood, Sweat, and Tears in the same round.",
            Variant::OmnitronII => "Defeat standard Omnitron. Then, while fighting Grand Warlord Voss, destroy two spaceships in the same round.",
            Variant::SpiteAgentOfGloom => "Defeat standard Spite. Then, defeat GloomWeaver by having 3_villain relics in the villain trash. At least two of the following must be in play: Cursed Acolyte, Profane Zealot, Chosen Disciple, or Ophidia the Deceiver. No Voodoo Pin may be in play.",
            Variant::SkinwalkerGloomweaver => "Defeat standard Spite and GloomWeaver. Then, do one of the following:\\1)_Defeat Spite: Agent of Gloom in Rook City on his \"Broken Vessel\" side with zero cards under the Safe House and at least five Victims in play. OR\\2)_Be defeated by Spite: Agent of Gloom in Rook City on his \"Agent_of_Gloom\" side with no more than two Drug cardsface up. Spite must have destroyed at least five Victims.",
            Variant::TricksterKismet => "Lose a game against regular Kismet in The Block, with at least K.N.Y.F.E., The Argent Adept, and Fanatic on the team. Over the course of the game, the Talisman must spend more full turns in hero play areas than full turns in the villain play area.",
            Variant::HeroicInfinitor => "Defeat Infinitor with damage dealt by a Construct. Captain Cosmic must be the first hero and no heroes may be incapacitated. Each different Manifestation must enter play at least once.",
            _ => "",
        }
    }
}

impl Hash for Item {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Item::Hero(v) => *v as usize,
            Item::Variant(v) => *v as usize + Hero::variant_count(),
            Item::Villain(v) => *v as usize + Hero::variant_count() + Variant::variant_count(),
            Item::TeamVillain(v) => *v as usize + Hero::variant_count() + Variant::variant_count() + Villain::variant_count(),
            Item::Environment(v) => *v as usize + Hero::variant_count() + Variant::variant_count() + Villain::variant_count() + TeamVillain::variant_count(),
            Item::Scion => Hero::variant_count() + Variant::variant_count() + Villain::variant_count() + TeamVillain::variant_count() + Environment::variant_count(),
            Item::Filler => Hero::variant_count() + Variant::variant_count() + Villain::variant_count() + TeamVillain::variant_count() + Environment::variant_count() + 1,
        }
        .hash(state);
    }
}

impl Hash for Location {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Location::Variant(v) => *v as usize,
            Location::Villain((v, d)) => (*v as usize) * 4 + *d as usize + Variant::variant_count(),
            Location::TeamVillain((v, d)) => (*v as usize) * 4 + *d as usize + Variant::variant_count() + Villain::variant_count() * 4,
            Location::Environment(v) => *v as usize + Variant::variant_count() + Villain::variant_count() * 4 + TeamVillain::variant_count() * 4,
        }
        .hash(state);
    }
}
