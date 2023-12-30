use std::str::FromStr;

use asr::Process;
use asr::settings::Gui;
use asr::watcher::Pair;
use serde::{Deserialize, Serialize};
use ugly_widget::radio_button::{RadioButtonOptions, options_str};
use ugly_widget::store::StoreWidget;

use super::auto_splitter_settings::Settings;
use super::hollow_knight_memory::*;

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SplitterAction {
    #[default]
    Pass,
    Split,
    Skip,
    Reset,
}

impl SplitterAction {
    pub fn or_else<F: FnOnce() -> SplitterAction>(self, f: F) -> SplitterAction {
        match self {
            SplitterAction::Pass => f(),
            a => a,
        }
    }
}

fn should_split(b: bool) -> SplitterAction {
    if b {
        SplitterAction::Split
    } else {
        SplitterAction::Pass
    }
}

fn should_skip(b: bool) -> SplitterAction {
    if b {
        SplitterAction::Skip
    } else {
        SplitterAction::Pass
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Gui, Ord, PartialEq, PartialOrd, RadioButtonOptions, Serialize)]
pub enum Split {
    // region: Start, End, and Menu
    /// Start New Game (Start)
    /// 
    /// Splits when starting a new save file, including Normal, Steel Soul, and Godseeker mode
    StartNewGame,
    /// Start Any Game (Start)
    /// 
    /// Splits when entering a new or existing save file
    StartAnyGame,
    /// Rando Wake (Event)
    /// 
    /// Splits when gaining control after waking up in Rando
    RandoWake,
    /// Credits Roll (Event)
    /// 
    /// Splits on any credits rolling
    #[default]
    EndingSplit,
    /// The Hollow Knight (Ending)
    /// 
    /// Splits on The Hollow Knight ending
    EndingA,
    /// Sealed Siblings (Ending)
    /// 
    /// Splits on Sealed Siblings ending
    EndingB,
    /// Dream No More (Ending)
    /// 
    /// Splits on Dream No More ending
    EndingC,
    /// Embrace the Void (Ending)
    /// 
    /// Splits on Embrace the Void ending
    EndingD,
    /// Delicate Flower (Ending)
    /// 
    /// Splits on Delicate Flower ending
    EndingE,
    /// Main Menu (Menu)
    /// 
    /// Splits on the main menu
    Menu,
    /// Any Bench (Bench)
    /// 
    /// Splits when sitting on a bench
    BenchAny,
    /// Death (Event)
    /// 
    /// Splits when player HP is 0
    PlayerDeath,
    /// Any Transition (Transition)
    /// 
    /// Splits when the knight enters a transition (only one will split per transition)
    AnyTransition,
    /// Transition excluding Save State (Transition)
    /// 
    /// Splits when the knight enters a transition (excludes save states and Sly's basement)
    TransitionAfterSaveState,
    // endregion: Start, End, and Menu

    // region: Dreamers
    /// Lurien the Watcher (Dreamer)
    /// 
    /// Splits when you see the mask for Lurien
    Lurien,
    /// Monomon the Teacher (Dreamer)
    /// 
    /// Splits when you see the mask for Monomon
    Monomon,
    /// Herrah the Beast (Dreamer)
    /// 
    /// Splits when you see the mask for Herrah
    Hegemol,
    /// First Dreamer (Dreamer)
    /// 
    /// Splits when you see the mask for the first dreamer killed
    Dreamer1,
    /// Second Dreamer (Dreamer)
    /// 
    /// Splits when you see the mask for the second dreamer killed
    Dreamer2,
    /// Third Dreamer (Dreamer)
    /// 
    /// Splits when you see the mask for the third dreamer killed
    Dreamer3,
    /// Main Menu w/ 3 Dreamers (Menu)
    /// 
    /// Splits on transition to the main menu after 3 Dreamers acquired
    MenuDreamer3,
    /// Lurien (Old Dreamer Timing)
    /// 
    /// Matches the old legacy split. Splits when Lurien is registered as defeated (After killing Watcher Knight)
    LurienDreamer,
    /// Monomon (Old Dreamer Timing)
    /// 
    /// Matches the old legacy split. Splits when Monomon is registered as defeated (After killing Uumuu)
    MonomonDreamer,
    /// Herrah (Old Dreamer Timing)
    /// 
    /// Matches the old legacy split. Splits when Herrah is registered as defeated (In Spider Area)
    HegemolDreamer,
    // endregion: Dreamers

    // region: Mr Mushroom
    /// Mr. Mushroom 1 (Spot)
    /// 
    /// Splits when talking to Mister Mushroom in Fungal Wastes
    MrMushroom1,
    /// Mr. Mushroom 2 (Spot)
    /// 
    /// Splits when talking to Mister Mushroom in Kingdom's Edge
    MrMushroom2,
    /// Mr. Mushroom 3 (Spot)
    /// 
    /// Splits when talking to Mister Mushroom in Deepnest
    MrMushroom3,
    /// Mr. Mushroom 4 (Spot)
    /// 
    /// Splits when talking to Mister Mushroom in Mato's Hut
    MrMushroom4,
    /// Mr. Mushroom 5 (Spot)
    /// 
    /// Splits when talking to Mister Mushroom in Ancient Basin
    MrMushroom5,
    /// Mr. Mushroom 6 (Spot)
    /// 
    /// Splits when talking to Mister Mushroom by Overgrown Mound
    MrMushroom6,
    /// Mr. Mushroom 7 (Spot)
    /// 
    /// Splits when talking to Mister Mushroom in King's Pass
    MrMushroom7,
    // endregion: Mr Mushroom

    // region: Spell Levels
    /// Vengeful Spirit (Skill)
    /// 
    /// Splits when obtaining Vengeful Spirit
    VengefulSpirit,
    /// Has Vengeful Spirit (Transition)
    /// 
    /// Splits on transition after Vengeful Spirit acquired
    TransVS,
    /// Shade Soul (Skill)
    /// 
    /// Splits when obtaining Shade Soul
    ShadeSoul,
    /// Has Shade Soul (Transition)
    /// 
    /// Splits on transition after Shade Soul acquired
    TransShadeSoul,
    MenuShadeSoul,
    /// Desolate Dive (Skill)
    /// 
    /// Splits when obtaining Desolate Dive
    DesolateDive,
    /// Descending Dark (Skill)
    /// 
    /// Splits when obtaining Descending Dark
    DescendingDark,
    /// Has Descending Dark (Transition)
    /// 
    /// Splits on transition after Descending Dark acquired
    TransDescendingDark,
    /// Howling Wraiths (Skill)
    /// 
    /// Splits when obtaining Howling Wraiths
    HowlingWraiths,
    /// Abyss Shriek (Skill)
    /// 
    /// Splits when obtaining Abyss Shriek
    AbyssShriek,
    // endregion: Spell Levels

    // region: Movement Abilities
    /// Mothwing Cloak (Skill)
    /// 
    /// Splits when obtaining Mothwing Cloak
    MothwingCloak,
    /// Main Menu w/ Mothwing Cloak (Menu)
    /// 
    /// Splits on transition to the main menu after Mothwing Cloak acquired
    MenuCloak,
    /// Shade Cloak (Skill)
    /// 
    /// Splits when obtaining Shade Cloak
    ShadeCloak,
    /// Mantis Claw (Skill)
    /// 
    /// Splits when obtaining Mantis Claw
    MantisClaw,
    /// Has Claw (Transition)
    /// 
    /// Splits on transition after Mantis Claw acquired
    TransClaw,
    /// Main Menu w/ Claw (Menu)
    /// 
    /// Splits on transition to the main menu after Mantis Claw acquired
    MenuClaw,
    /// Monarch Wings (Skill)
    /// 
    /// Splits when obtaining Monarch Wings
    MonarchWings,
    MenuWings,
    /// Crystal Heart (Skill)
    /// 
    /// Splits when obtaining Crystal Heart
    CrystalHeart,
    /// Isma's Tear (Skill)
    /// 
    /// Splits when obtaining Isma's Tear
    IsmasTear,
    /// Has Isma's Tear (Transition)
    /// 
    /// Splits on transition after Isma's Tear acquired
    TransTear,
    /// Main Menu w/ Isma's Tear (Menu)
    /// 
    /// Splits on transition to the main menu after Isma's Tear acquired
    MenuIsmasTear,
    // endregion: Movement Abilities

    // region: Nail Arts
    /// Cyclone Slash (Skill)
    /// 
    /// Splits when obtaining Cyclone Slash
    CycloneSlash,
    /// Dash Slash (Skill)
    /// 
    /// Splits when obtaining Dash Slash
    DashSlash,
    /// Great Slash (Skill)
    /// 
    /// Splits when obtaining Great Slash
    GreatSlash,
    // endregion: Nail Arts

    // region: Dream Nail Levels
    /// Dream Nail (Skill)
    /// 
    /// Splits when obtaining Dream Nail
    DreamNail,
    /// Main Menu w/ Dream Nail (Menu)
    /// 
    /// Splits on transition to the main menu after Dream Nail acquired
    MenuDreamNail,
    /// Dream Gate (Skill)
    /// 
    /// Splits when obtaining Dream Gate
    DreamGate,
    /// Main Menu w/ Dream Gate (Menu)
    /// 
    /// Splits on transition to the main menu after Dream Gate acquired
    MenuDreamGate,
    /// Dream Nail - Awoken (Skill)
    /// 
    /// Splits when Awkening the Dream Nail
    DreamNail2,
    // endregion: Dream Nail Levels

    // region: Keys
    /// City Crest (Item)
    /// 
    /// Splits when obtaining the City Crest
    CityKey,
    /// Lumafly Lantern (Item)
    /// 
    /// Splits when obtaining the Lumafly Lantern
    LumaflyLantern,
    /// Shop Lumafly Lantern (Transition)
    /// 
    /// Splits on transition after Lantern has been acquired
    LumaflyLanternTransition,
    /// Simple Key - First (Item)
    /// 
    /// Splits when obtaining the first Simple Key
    SimpleKey,
    /// Simple Key (Obtain)
    /// 
    /// Splits when obtaining a Simple Key
    OnObtainSimpleKey,
    /// Shopkeeper's Key (Item)
    /// 
    /// Splits when obtaining the Shopkeeper's Key
    SlyKey,
    /// Elegant Key (Item)
    /// 
    /// Splits when obtaining the Elegant Key
    ElegantKey,
    /// Love Key (Item)
    /// 
    /// Splits when obtaining the Love Key
    LoveKey,
    /// Pale Lurker Key (Item)
    /// 
    /// Splits when obtaining the Simple Key from the Pale Lurker
    PaleLurkerKey,
    /// Sly Simple Key (Item)
    /// 
    /// Splits when buying the simple key from Sly
    SlySimpleKey,
    /// King's Brand (Item)
    /// 
    /// Splits when obtaining the King's Brand
    KingsBrand,
    /// Tram Pass (Item)
    /// 
    /// Splits when obtaining the Tram Pass
    TramPass,
    // endregion: Keys

    // region: Nail and Pale Ore
    /// Pale Ore (Obtain)
    /// 
    /// Splits when obtaining a Pale Ore
    OnObtainPaleOre,
    /// Pale Ore 1 (Ore)
    /// 
    /// Splits after obtaining the first pale ore.
    Ore1,
    /// Pale Ore 2 (Ore)
    /// 
    /// Splits after obtaining the second pale ore.
    Ore2,
    /// Pale Ore 3 (Ore)
    /// 
    /// Splits after obtaining the third pale ore.
    Ore3,
    /// Pale Ore 4 (Ore)
    /// 
    /// Splits after obtaining the fourth pale ore.
    Ore4,
    /// Pale Ore 5 (Ore)
    /// 
    /// Splits after obtaining the fifth pale ore.
    Ore5,
    /// Pale Ore 6 (Ore)
    /// 
    /// Splits after obtaining the sixth pale ore.
    Ore6,
    /// Nail 1 (Upgrade)
    /// 
    /// Splits upon upgrading to the Sharpened Nail
    NailUpgrade1,
    /// Nail 2 (Upgrade)
    /// 
    /// Splits upon upgrading to the Channeled Nail
    NailUpgrade2,
    /// Nail 3 (Upgrade)
    /// 
    /// Splits upon upgrading to the Coiled Nail
    NailUpgrade3,
    /// Nail 4 (Upgrade)
    /// 
    /// Splits upon upgrading to the Pure Nail
    NailUpgrade4,
    // endregion: Nail and Pale Ore

    // region: Masks and Mask Shards
    /// Mask Shard (Obtain)
    /// 
    /// Splits when obtaining a Mask Shard or upgrade for complete Mask
    OnObtainMaskShard,
    /// Mask Shard 1 (Fragment)
    /// 
    /// Splits when getting 1st Mask Shard
    MaskFragment1,
    /// Mask Shard 2 (Fragment)
    /// 
    /// Splits when getting 2nd Mask Shard
    MaskFragment2,
    /// Mask Shard 3 (Fragment)
    /// 
    /// Splits when getting 3rd Mask Shard
    MaskFragment3,
    /// Mask Upgrade 4 (Upgrade)
    /// 
    /// Splits when getting 1 extra Mask (6 base HP)
    Mask1,
    /// Mask Shard 5 (Fragment)
    /// 
    /// Splits when getting 5th Mask Shard
    MaskFragment5,
    /// Mask Shard 6 (Fragment)
    /// 
    /// Splits when getting 6th Mask Shard
    MaskFragment6,
    /// Mask Shard 7 (Fragment)
    /// 
    /// Splits when getting 7th Mask Shard
    MaskFragment7,
    /// Mask Upgrade 8 (Upgrade)
    /// 
    /// Splits when getting 2 extra Masks (7 base HP)
    Mask2,
    /// Mask Shard 9 (Fragment)
    /// 
    /// Splits when getting 9th Mask Shard
    MaskFragment9,
    /// Mask Shard 10 (Fragment)
    /// 
    /// Splits when getting 10th Mask Shard
    MaskFragment10,
    /// Mask Shard 11 (Fragment)
    /// 
    /// Splits when getting 11th Mask Shard
    MaskFragment11,
    /// Mask Upgrade 12 (Upgrade)
    /// 
    /// Splits when getting 3 extra Masks (8 base HP)
    Mask3,
    /// Mask Shard 13 (Fragment)
    /// 
    /// Splits when getting 13th Mask Shard
    MaskFragment13,
    /// Mask Shard 14 (Fragment)
    /// 
    /// Splits when getting 14th Mask Shard
    MaskFragment14,
    /// Mask Shard 15 (Fragment)
    /// 
    /// Splits when getting 15th Mask Shard
    MaskFragment15,
    /// Mask Upgrade 16 (Upgrade)
    /// 
    /// Splits when getting 4 extra Masks (9 base HP)
    Mask4,
    /// Brooding Mawlek Mask Shard (Obtain)
    /// 
    /// Splits when getting the Mask Shard from Brooding Mawlek
    MaskShardMawlek,
    /// Grub Reward Mask Shard (Obtain)
    /// 
    /// Splits when getting the Mask Shard given by Grubfather
    MaskShardGrubfather,
    /// Goam Mask Shard (Obtain)
    /// 
    /// Splits when getting the Goam Mask Shard in Forgotten Crossroads
    MaskShardGoam,
    /// Queen's Station Mask Shard (Obtain)
    /// 
    /// Splits when getting the Mask Shard in Queen's Station
    MaskShardQueensStation,
    /// Bretta Mask Shard (Obtain)
    /// 
    /// Splits when getting the Mask Shard in Bretta's hut in Dirtmouth
    MaskShardBretta,
    /// Stone Sanctuary Mask Shard (Obtain)
    /// 
    /// Splits when getting the Mask Shard in Stone Sanctuary
    MaskShardStoneSanctuary,
    /// Waterways Mask Shard (Obtain)
    /// 
    /// Splits when getting the Mask Shard in Royal Wayerways
    MaskShardWaterways,
    /// Fungal Core Mask Shard (Obtain)
    /// 
    /// Splits when getting the Mask Shard below Fungal Core
    MaskShardFungalCore,
    /// Enraged Guardian Mask Shard (Obtain)
    /// 
    /// Splits when getting the Mask Shard from Enraged Guardian
    MaskShardEnragedGuardian,
    /// Hive Mask Shard (Obtain)
    /// 
    /// Splits when getting the Mask Shard in the Hive
    MaskShardHive,
    /// Seer Mask Shard (Obtain)
    /// 
    /// Splits when getting the Mask Shard from Seer
    MaskShardSeer,
    /// Grey Mourner Mask Shard (Obtain)
    /// 
    /// Splits when getting the Mask Shard from Grey Mourner
    MaskShardFlower,
    // endregion: Masks and Mask Shards

    // region: Vessels and Vessel Fragments
    /// Vessel Fragment (Obtain)
    /// 
    /// Splits when obtaining a Vessel Fragment or on upgrade for full Soul Vessel
    OnObtainVesselFragment,
    /// Vessel Fragment 1 (Fragment)
    /// 
    /// Splits when getting 1st Soul Vessel Fragment
    VesselFragment1,
    /// Vessel Fragment 2 (Fragment)
    /// 
    /// Splits when getting 2nd Soul Vessel Fragment
    VesselFragment2,
    /// Soul Vessel 1 (Upgrade)
    /// 
    /// Splits when upgrading to 1 Soul Vessel (3 Soul Vessel Fragments)
    Vessel1,
    /// Vessel Fragment 4 (Fragment)
    /// 
    /// Splits when getting 4th Soul Vessel Fragment
    VesselFragment4,
    /// Vessel Fragment 5 (Fragment)
    /// 
    /// Splits when getting 5th Soul Vessel Fragment
    VesselFragment5,
    /// Soul Vessel 2 (Upgrade)
    /// 
    /// Splits when upgrading to 2 Soul Vessels (6 Soul Vessel Fragments)
    Vessel2,
    /// Vessel Fragment 7 (Fragment)
    /// 
    /// Splits when getting 7th Soul Vessel Fragment
    VesselFragment7,
    /// Vessel Fragment 8 (Fragment)
    /// 
    /// Splits when getting 8th Soul Vessel Fragment
    VesselFragment8,
    /// Soul Vessel 3 (Upgrade)
    /// 
    /// Splits when upgrading to 3 Soul Vessels (9 Soul Vessel Fragments)
    Vessel3,
    /// Greenpath Vessel Fragment (Obtain)
    /// 
    /// Splits when getting Vessel Fragment in Greenpath
    VesselFragGreenpath,
    /// Crossroads Vessel Fragment (Obtain)
    /// 
    /// Splits when getting the Vessel Fragment in Forgotten Crossroads
    VesselFragCrossroadsLift,
    /// King's Station Vessel Fragment (Obtain)
    /// 
    /// Splits when getting the Vessel Fragment after the arena above King's Station
    VesselFragKingsStation,
    /// Deepnest Vessel Fragment (Obtain)
    /// 
    /// Splits when getting the Vessel Fragment in Deepnest
    VesselFragGarpedes,
    /// Stag Nest Vessel Fragment (Obtain)
    /// 
    /// Splits when getting the Vessel Fragment in Stag Nest
    VesselFragStagNest,
    /// Seer Vessel Fragment (Obtain)
    /// 
    /// Splits when getting the Vessel Fragment from Seer
    VesselFragSeer,
    /// Basin Fountain Vessel Fragment (Obtain)
    /// 
    /// Splits when getting the Vessel Fragment from the fountain in Ancient Basin
    VesselFragFountain,
    // endregion: Vessels and Vessel Fragments

    // region: Charm Notches
    /// Shrumal Ogres (Charm Notch)
    /// 
    /// Splits when obtaining the charm notch after defeating the Shrumal Ogres
    NotchShrumalOgres,
    /// Salubra 1 (Charm Notch)
    /// 
    /// Splits when obtaining the first charm notch from Salubra
    NotchSalubra1,
    /// Salubra 2 (Charm Notch)
    /// 
    /// Splits when obtaining the second charm notch from Salubra
    NotchSalubra2,
    /// Salubra 3 (Charm Notch)
    /// 
    /// Splits when obtaining the third charm notch from Salubra
    NotchSalubra3,
    /// Salubra 4 (Charm Notch)
    /// 
    /// Splits when obtaining the fourth charm notch from Salubra
    NotchSalubra4,
    /// Fog Canyon (Charm Notch)
    /// 
    /// Splits when obtaining the charm notch in Fog Canyon
    NotchFogCanyon,
    /// Grimm (Charm Notch)
    /// 
    /// Splits when obtaining the charm notch after Grimm
    NotchGrimm,
    /// Charm Notch (Obtain)
    /// 
    /// Splits when obtaining a new Charm Slot
    OnObtainCharmNotch,
    // endregion: Charm Notches

    // region: Charms
    /// Gathering Swarm (Charm)
    /// 
    /// Splits when obtaining the Gathering Swarm charm
    GatheringSwarm,
    /// Wayward Compass (Charm)
    /// 
    /// Splits when obtaining Wayward Compass charm
    WaywardCompass,
    /// Grubsong (Charm)
    /// 
    /// Splits when obtaining the Grubsong charm
    Grubsong,
    /// Stalwart Shell (Charm)
    /// 
    /// Splits when obtaining Stalwart Shell charm
    StalwartShell,
    /// Baldur Shell (Charm)
    /// 
    /// Splits when obtaining the Baldur Shell charm
    BaldurShell,
    /// Fury of the Fallen (Charm)
    /// 
    /// Splits when obtaining the Fury of the Fallen charm
    FuryOfTheFallen,
    /// Quick Focus (Charm)
    /// 
    /// Splits when obtaining the Quick Focus charm
    QuickFocus,
    /// Lifeblood Heart (Charm)
    /// 
    /// Splits when obtaining the Lifeblood Heart charm
    LifebloodHeart,
    /// Lifeblood Core (Charm)
    /// 
    /// Splits when obtaining the Lifeblood Core charm
    LifebloodCore,
    /// Defenders Crest (Charm)
    /// 
    /// Splits when obtaining the Defenders Crest charm
    DefendersCrest,
    /// Flukenest (Charm)
    /// 
    /// Splits when obtaining the Flukenest charm
    Flukenest,
    /// Thorns of Agony (Charm)
    /// 
    /// Splits when obtaining Thorns of Agony charm
    ThornsOfAgony,
    /// Mark of Pride (Charm)
    /// 
    /// Splits when obtaining the Mark of Pride charm
    MarkOfPride,
    /// Steady Body (Charm)
    /// 
    /// Splits when obtaining the Steady Body charm
    SteadyBody,
    /// Heavy Blow (Charm)
    /// 
    /// Splits when obtaining the Heavy Blow charm
    HeavyBlow,
    /// Sharp Shadow (Charm)
    /// 
    /// Splits when obtaining Sharp Shadow charm
    SharpShadow,
    /// Spore Shroom (Charm)
    /// 
    /// Splits when obtaining the Spore Shroom charm
    SporeShroom,
    /// Longnail (Charm)
    /// 
    /// Splits when obtaining the Longnail charm
    Longnail,
    /// Shaman Stone (Charm)
    /// 
    /// Splits when obtaining Shaman Stone charm
    ShamanStone,
    /// Soul Catcher (Charm)
    /// 
    /// Splits when obtaining the Soul Catcher charm
    SoulCatcher,
    /// Soul Eater (Charm)
    /// 
    /// Splits when obtaining the Soul Eater charm
    SoulEater,
    /// Glowing Womb (Charm)
    /// 
    /// Splits when obtaining the Glowing Womb charm
    GlowingWomb,
    /// Nailmaster's Glory (Charm)
    /// 
    /// Splits when obtaining the Nailmaster's Glory charm
    NailmastersGlory,
    /// Joni's Blessing (Charm)
    /// 
    /// Splits when obtaining the Joni's Blessing charm
    JonisBlessing,
    /// Shape of Unn (Charm)
    /// 
    /// Splits when obtaining Shape of Unn charm
    ShapeOfUnn,
    /// Hiveblood (Charm)
    /// 
    /// Splits when obtaining the Hiveblood charm
    Hiveblood,
    /// Dream Wielder (Charm)
    /// 
    /// Splits when obtaining the Dream Wielder charm
    DreamWielder,
    /// Dashmaster (Charm)
    /// 
    /// Splits when obtaining the Dashmaster charm
    Dashmaster,
    /// Main Menu w/ Dashmaster (Menu)
    /// 
    /// Splits on transition to the main menu after Dashmaster acquired
    MenuDashmaster,
    /// Quick Slash (Charm)
    /// 
    /// Splits when obtaining the Quick Slash charm
    QuickSlash,
    /// Spell Twister (Charm)
    /// 
    /// Splits when obtaining the Spell Twister charm
    SpellTwister,
    /// Deep Focus (Charm)
    /// 
    /// Splits when obtaining the Deep Focus charm
    DeepFocus,
    /// Grubberfly's Elegy (Charm)
    /// 
    /// Splits when obtaining the Grubberfly's Elegy charm
    GrubberflysElegy,
    /// Sprintmaster (Charm)
    /// 
    /// Splits when obtaining the Sprintmaster charm
    Sprintmaster,
    /// Dreamshield (Charm)
    /// 
    /// Splits when obtaining the Dreamshield charm
    Dreamshield,
    /// Weaversong (Charm)
    /// 
    /// Splits when obtaining the Weaversong charm
    Weaversong,
    // Fragile / Unbreakable Charms
    /// Fragile Heart (Charm)
    /// 
    /// Splits when obtaining the Fragile Heart charm
    FragileHeart,
    /// Unbreakable Heart (Charm)
    /// 
    /// Splits when obtaining the Unbreakable Heart charm
    UnbreakableHeart,
    /// Fragile Greed (Charm)
    /// 
    /// Splits when obtaining the Fragile Greed charm
    FragileGreed,
    /// Unbreakable Greed (Charm)
    /// 
    /// Splits when obtaining the Unbreakable Greed charm
    UnbreakableGreed,
    /// Fragile Strength (Charm)
    /// 
    /// Splits when obtaining the Fragile Strength charm
    FragileStrength,
    /// Unbreakable Strength (Charm)
    /// 
    /// Splits when obtaining the Unbreakable Strength charm
    UnbreakableStrength,
    /// All Breakables (Event)
    /// 
    /// Splits when all 3 fragile charms are broken
    AllBreakables,
    /// All Unbreakables (Charm)
    /// 
    /// Splits when all 3 unbreakable charms are obtained
    AllUnbreakables,
    // Grimmchild / Carefree Melody
    /// Grimmchild (Charm)
    /// 
    /// Splits when obtaining the Grimmchild charm
    Grimmchild,
    /// Grimmchild Lvl 2 (Charm)
    /// 
    /// Splits when upgrading Grimmchild to level 2
    Grimmchild2,
    /// Grimmchild Lvl 3 (Charm)
    /// 
    /// Splits when upgrading Grimmchild to level 3
    Grimmchild3,
    /// Grimmchild Lvl 4 (Charm)
    /// 
    /// Splits when upgrading Grimmchild to level 4
    Grimmchild4,
    /// Carefree Melody (Charm)
    /// 
    /// Splits when obtaining the Carefree Melody charm
    CarefreeMelody,
    /// Grimm Flame 1 (Flame)
    /// 
    /// Splits after obtaining the first flame.
    Flame1,
    /// Grimm Flame 2 (Flame)
    /// 
    /// Splits after obtaining the second flame.
    Flame2,
    /// Grimm Flame 3 (Flame)
    /// 
    /// Splits after obtaining the third flame.
    Flame3,
    /// Brumm Flame (NPC)
    /// 
    /// Splits when collecting Brumm's flame in Deepnest
    BrummFlame,
    // Kingsoul / VoidHeart
    /// White Fragment - Queen's (Charm)
    /// 
    /// Splits on picking up the left White Fragment from the White Lady
    WhiteFragmentLeft,
    /// White Fragment - King's (Charm)
    /// 
    /// Splits on picking up the right White Fragment from the Pale King
    WhiteFragmentRight,
    /// White Fragment (Obtain)
    /// 
    /// Splits when obtaining any White Fragment, or Void Heart
    OnObtainWhiteFragment,
    /// Kingsoul (Charm)
    /// 
    /// Splits when obtaining the completed Kingsoul charm
    Kingsoul,
    /// Void Heart (Charm)
    /// 
    /// Splits when changing the Kingsoul to the Void Heart charm
    VoidHeart,
    /// Main Menu w/ Void Heart (Menu)
    /// 
    /// Splits on transition to the main menu after Void Heart acquired
    MenuVoidHeart,
    // endregion: Charms

    // region: Stags
    /// Riding Stag (Event)
    /// 
    /// Splits while riding the stag
    RidingStag,
    /// Stag Position Updated (Event)
    /// 
    /// Splits when the stag is called
    StagMoved,
    /// Forgotten Crossroads (Stag Station)
    /// 
    /// Splits when opening the Forgotten Crossroads Stag Station
    CrossroadsStation,
    /// Greenpath (Stag Station)
    /// 
    /// Splits when obtaining Greenpath Stag Station
    GreenpathStation,
    /// Queen's Station (Stag Station)
    /// 
    /// Splits when obtaining Queen's Station Stag Station
    QueensStationStation,
    /// City Storerooms (Stag Station)
    /// 
    /// Splits when obtaining City Storerooms Stag Station
    StoreroomsStation,
    /// King's Station (Stag Station)
    /// 
    /// Splits when obtaining King's Station Stag Station
    KingsStationStation,
    /// Resting Grounds (Stag Station)
    /// 
    /// Splits when obtaining Resting Grounds Stag Station
    RestingGroundsStation,
    /// Hidden Station (Stag Station)
    /// 
    /// Splits when obtaining to Hidden Station Stag Station
    HiddenStationStation,
    /// Distant Village (Stag Station)
    /// 
    /// Splits when obtaining Distant Village Stag Station
    DeepnestStation,
    /// Queen's Gardens (Stag Station)
    /// 
    /// Splits when obtaining Queen's Gardens Stag Station
    QueensGardensStation,
    /// Stagnest (Stag Station)
    /// 
    /// Splits when traveling to Stagnest (Requires Ordered Splits)
    StagnestStation,
    // endregion: Stags

    // region: Relics
    /// Wanderer's Journal (Obtain)
    /// 
    /// Splits when obtaining a Wanderer's Journal
    OnObtainWanderersJournal,
    /// All Seals (Item)
    /// 
    /// Splits when 17 Hallownest Seals have been collected
    AllSeals,
    /// Hallownest Seal (Obtain)
    /// 
    /// Splits when obtaining a Hallownest Seal
    OnObtainHallownestSeal,
    /// Soul Sanctum Hallownest Seal (Relic)
    /// 
    /// Splits when the Hallownest Seal in Soul Sanctum is collected
    SoulSanctumSeal,
    /// King's Idol (Obtain)
    /// 
    /// Splits when obtaining a King's Idol
    OnObtainKingsIdol,
    /// Glade Idol (Item)
    /// 
    /// Splits when picking up the King's Idol in the Spirits' Glade
    GladeIdol,
    /// Dung Defender Idol (Item)
    /// 
    /// Splits when picking up Dung Defender idol as the first idol
    DungDefenderIdol,
    /// Arcane Egg 8 (Obtain)
    /// 
    /// Splits when obtaining 8 Arcane Eggs
    ArcaneEgg8,
    /// Arcane Egg (Obtain)
    /// 
    /// Splits when obtaining an Arcane Egg
    OnObtainArcaneEgg,
    /// Rancid Egg (Obtain)
    /// 
    /// Splits when obtaining a Rancid Egg
    OnObtainRancidEgg,
    // endregion: Relics

    // region: Grubs and Mimics
    /// Rescued Grub 1 (Grub)
    /// 
    /// Splits when rescuing grub #1
    Grub1,
    /// Rescued Grub 2 (Grub)
    /// 
    /// Splits when rescuing grub #2
    Grub2,
    /// Rescued Grub 3 (Grub)
    /// 
    /// Splits when rescuing grub #3
    Grub3,
    /// Rescued Grub 4 (Grub)
    /// 
    /// Splits when rescuing grub #4
    Grub4,
    /// Rescued Grub 5 (Grub)
    /// 
    /// Splits when rescuing grub #5
    Grub5,
    /// Rescued Grub 6 (Grub)
    /// 
    /// Splits when rescuing grub #6
    Grub6,
    /// Rescued Grub 7 (Grub)
    /// 
    /// Splits when rescuing grub #7
    Grub7,
    /// Rescued Grub 8 (Grub)
    /// 
    /// Splits when rescuing grub #8
    Grub8,
    /// Rescued Grub 9 (Grub)
    /// 
    /// Splits when rescuing grub #9
    Grub9,
    /// Rescued Grub 10 (Grub)
    /// 
    /// Splits when rescuing grub #10
    Grub10,
    /// Rescued Grub 11 (Grub)
    /// 
    /// Splits when rescuing grub #11
    Grub11,
    /// Rescued Grub 12 (Grub)
    /// 
    /// Splits when rescuing grub #12
    Grub12,
    /// Rescued Grub 13 (Grub)
    /// 
    /// Splits when rescuing grub #13
    Grub13,
    /// Rescued Grub 14 (Grub)
    /// 
    /// Splits when rescuing grub #14
    Grub14,
    /// Rescued Grub 15 (Grub)
    /// 
    /// Splits when rescuing grub #15
    Grub15,
    /// Rescued Grub 16 (Grub)
    /// 
    /// Splits when rescuing grub #16
    Grub16,
    /// Rescued Grub 17 (Grub)
    /// 
    /// Splits when rescuing grub #17
    Grub17,
    /// Rescued Grub 18 (Grub)
    /// 
    /// Splits when rescuing grub #18
    Grub18,
    /// Rescued Grub 19 (Grub)
    /// 
    /// Splits when rescuing grub #19
    Grub19,
    /// Rescued Grub 20 (Grub)
    /// 
    /// Splits when rescuing grub #20
    Grub20,
    /// Rescued Grub 21 (Grub)
    /// 
    /// Splits when rescuing grub #21
    Grub21,
    /// Rescued Grub 22 (Grub)
    /// 
    /// Splits when rescuing grub #22
    Grub22,
    /// Rescued Grub 23 (Grub)
    /// 
    /// Splits when rescuing grub #23
    Grub23,
    /// Rescued Grub 24 (Grub)
    /// 
    /// Splits when rescuing grub #24
    Grub24,
    /// Rescued Grub 25 (Grub)
    /// 
    /// Splits when rescuing grub #25
    Grub25,
    /// Rescued Grub 26 (Grub)
    /// 
    /// Splits when rescuing grub #26
    Grub26,
    /// Rescued Grub 27 (Grub)
    /// 
    /// Splits when rescuing grub #27
    Grub27,
    /// Rescued Grub 28 (Grub)
    /// 
    /// Splits when rescuing grub #28
    Grub28,
    /// Rescued Grub 29 (Grub)
    /// 
    /// Splits when rescuing grub #29
    Grub29,
    /// Rescued Grub 30 (Grub)
    /// 
    /// Splits when rescuing grub #30
    Grub30,
    /// Rescued Grub 31 (Grub)
    /// 
    /// Splits when rescuing grub #31
    Grub31,
    /// Rescued Grub 32 (Grub)
    /// 
    /// Splits when rescuing grub #32
    Grub32,
    /// Rescued Grub 33 (Grub)
    /// 
    /// Splits when rescuing grub #33
    Grub33,
    /// Rescued Grub 34 (Grub)
    /// 
    /// Splits when rescuing grub #34
    Grub34,
    /// Rescued Grub 35 (Grub)
    /// 
    /// Splits when rescuing grub #35
    Grub35,
    /// Rescued Grub 36 (Grub)
    /// 
    /// Splits when rescuing grub #36
    Grub36,
    /// Rescued Grub 37 (Grub)
    /// 
    /// Splits when rescuing grub #37
    Grub37,
    /// Rescued Grub 38 (Grub)
    /// 
    /// Splits when rescuing grub #38
    Grub38,
    /// Rescued Grub 39 (Grub)
    /// 
    /// Splits when rescuing grub #39
    Grub39,
    /// Rescued Grub 40 (Grub)
    /// 
    /// Splits when rescuing grub #40
    Grub40,
    /// Rescued Grub 41 (Grub)
    /// 
    /// Splits when rescuing grub #41
    Grub41,
    /// Rescued Grub 42 (Grub)
    /// 
    /// Splits when rescuing grub #42
    Grub42,
    /// Rescued Grub 43 (Grub)
    /// 
    /// Splits when rescuing grub #43
    Grub43,
    /// Rescued Grub 44 (Grub)
    /// 
    /// Splits when rescuing grub #44
    Grub44,
    /// Rescued Grub 45 (Grub)
    /// 
    /// Splits when rescuing grub #45
    Grub45,
    /// Rescued Grub 46 (Grub)
    /// 
    /// Splits when rescuing grub #46
    Grub46,
    /// Rescued Any Grub (Grub)
    /// 
    /// Splits when rescuing any grub
    OnObtainGrub,
    /// Rescued Grub Basin Dive (Grub)
    /// 
    /// Splits when rescuing the grub in Abyss_17
    GrubBasinDive,
    /// Rescued Grub Basin Wings (Grub)
    /// 
    /// Splits when rescuing the grub in Abyss_19
    GrubBasinWings,
    /// Rescued Grub City Below Love Tower (Grub)
    /// 
    /// Splits when rescuing the grub in Ruins2_07
    GrubCityBelowLoveTower,
    /// Rescued Grub City Below Sanctum (Grub)
    /// 
    /// Splits when rescuing the grub in Ruins1_05
    GrubCityBelowSanctum,
    /// Rescued Grub City Guard House (Grub)
    /// 
    /// Splits when rescuing the grub in Ruins_House_01
    GrubCityGuardHouse,
    /// Rescued Grub City Sanctum (Grub)
    /// 
    /// Splits when rescuing the grub in Ruins1_32
    GrubCitySanctum,
    /// Rescued Grub City Spire (Grub)
    /// 
    /// Splits when rescuing the grub in Ruins2_03
    GrubCitySpire,
    /// Rescued Grub Cliffs Baldur Shell (Grub)
    /// 
    /// Splits when rescuing the grub in Fungus1_28
    GrubCliffsBaldurShell,
    /// Rescued Grub Crossroads Acid (Grub)
    /// 
    /// Splits when rescuing the grub in Crossroads_35
    GrubCrossroadsAcid,
    /// Rescued Grub Crossroads Guarded (Grub)
    /// 
    /// Splits when rescuing the grub in Crossroads_48
    GrubCrossroadsGuarded,
    /// Rescued Grub Crossroads Spikes (Grub)
    /// 
    /// Splits when rescuing the grub in Crossroads_31
    GrubCrossroadsSpikes,
    /// Rescued Grub Crossroads Vengefly (Grub)
    /// 
    /// Splits when rescuing the grub in Crossroads_05
    GrubCrossroadsVengefly,
    /// Rescued Grub Crossroads Wall (Grub)
    /// 
    /// Splits when rescuing the grub in Crossroads_03
    GrubCrossroadsWall,
    /// Rescued Grub Crystal Peak Bottom Lever (Grub)
    /// 
    /// Splits when rescuing the grub in Mines_04
    GrubCrystalPeaksBottomLever,
    /// Rescued Grub Crystal Peak Crown (Grub)
    /// 
    /// Splits when rescuing the grub in Mines_24
    GrubCrystalPeaksCrown,
    /// Rescued Grub Crystal Peak Crushers (Grub)
    /// 
    /// Splits when rescuing the grub in Mines_19
    GrubCrystalPeaksCrushers,
    /// Rescued Grub Crystal Peak Crystal Heart (Grub)
    /// 
    /// Splits when rescuing the grub in Mines_31
    GrubCrystalPeaksCrystalHeart,
    /// Rescued Grub Crystal Peak Mimic (Grub)
    /// 
    /// Splits when rescuing the grub in Mines_16
    GrubCrystalPeaksMimics,
    /// Rescued Grub Crystal Peak Mound (Grub)
    /// 
    /// Splits when rescuing the grub in Mines_35
    GrubCrystalPeaksMound,
    /// Rescued Grub Crystal Peak Spikes (Grub)
    /// 
    /// Splits when rescuing the grub in Mines_03
    GrubCrystalPeaksSpikes,
    /// Rescued Grub Deepnest Beast's Den (Grub)
    /// 
    /// Splits when rescuing the grub in Deepnest_Spider_Town
    GrubDeepnestBeastsDen,
    /// Rescued Grub Deepnest Dark (Grub)
    /// 
    /// Splits when rescuing the grub in Deepnest_39
    GrubDeepnestDark,
    /// Rescued Grub Deepnest Mimics (Grub)
    /// 
    /// Splits when rescuing the grub in Deepnest_36
    GrubDeepnestMimics,
    /// Rescued Grub Deepnest Nosk (Grub)
    /// 
    /// Splits when rescuing the grub in Deepnest_31
    GrubDeepnestNosk,
    /// Rescued Grub Deepnest Spikes (Grub)
    /// 
    /// Splits when rescuing the grub in Deepnest_03
    GrubDeepnestSpikes,
    /// Rescued Grub Fog Canyon Archives (Grub)
    /// 
    /// Splits when rescuing the grub in Fungus3_47
    GrubFogCanyonArchives,
    /// Rescued Grub Fungal Bouncy (Grub)
    /// 
    /// Splits when rescuing the grub in Fungus2_18
    GrubFungalBouncy,
    /// Rescued Grub Fungal Spore Shroom (Grub)
    /// 
    /// Splits when rescuing the grub in Fungus2_20
    GrubFungalSporeShroom,
    /// Rescued Grub Greenpath Cornifer (Grub)
    /// 
    /// Splits when rescuing the grub in Fungus1_06
    GrubGreenpathCornifer,
    /// Rescued Grub Greenpath Hunter (Grub)
    /// 
    /// Splits when rescuing the grub in Fungus1_07
    GrubGreenpathHunter,
    /// Rescued Grub Greenpath Moss Knight (Grub)
    /// 
    /// Splits when rescuing the grub in Fungus1_21
    GrubGreenpathMossKnight,
    /// Rescued Grub Greenpath Vessel Fragment (Grub)
    /// 
    /// Splits when rescuing the grub in Fungus1_13
    GrubGreenpathVesselFragment,
    /// Rescued Grub Hive External (Grub)
    /// 
    /// Splits when rescuing the grub in Hive_03
    GrubHiveExternal,
    /// Rescued Grub Hive Internal (Grub)
    /// 
    /// Splits when rescuing the grub in Hive_04
    GrubHiveInternal,
    /// Rescued Grub Kingdom's Edge Center (Grub)
    /// 
    /// Splits when rescuing the grub in Deepnest_East_11
    GrubKingdomsEdgeCenter,
    /// Rescued Grub Kingdom's Edge Oro (Grub)
    /// 
    /// Splits when rescuing the grub in Deepnest_East_14
    GrubKingdomsEdgeOro,
    /// Rescued Grub Queen's Gardens Below Stag (Grub)
    /// 
    /// Splits when rescuing the grub in Fungus3_10
    GrubQueensGardensBelowStag,
    /// Rescued Grub Queen's Gardens Upper (Grub)
    /// 
    /// Splits when rescuing the grub in Fungus3_22
    GrubQueensGardensUpper,
    /// Rescued Grub Queen's Gardens White Lady (Grub)
    /// 
    /// Splits when rescuing the grub in Fungus3_48
    GrubQueensGardensWhiteLady,
    /// Rescued Grub Resting Grounds Crypts (Grub)
    /// 
    /// Splits when rescuing the grub in RestingGrounds_10
    GrubRestingGroundsCrypts,
    /// Rescued Grub Waterways Center (Grub)
    /// 
    /// Splits when rescuing the grub in Waterways_04
    GrubWaterwaysCenter,
    /// Rescued Grub Waterways Hwurmps (Grub)
    /// 
    /// Splits when rescuing the grub in Waterways_14
    GrubWaterwaysHwurmps,
    /// Rescued Grub Waterways Isma (Grub)
    /// 
    /// Splits when rescuing the grub in Waterways_13
    GrubWaterwaysIsma,
    /// Mimic 1 (Killed)
    /// 
    /// Splits when rescuing mimic #1
    Mimic1,
    /// Mimic 2 (Killed)
    /// 
    /// Splits when rescuing mimic #2
    Mimic2,
    /// Mimic 3 (Killed)
    /// 
    /// Splits when rescuing mimic #3
    Mimic3,
    /// Mimic 4 (Killed)
    /// 
    /// Splits when rescuing mimic #4
    Mimic4,
    /// Mimic 5 (Killed)
    /// 
    /// Splits when rescuing mimic #5
    Mimic5,
    // endregion: Grubs and Mimics

    // region: Essence, Trees, and Ghosts
    /// 100 Essence (Essence)
    /// 
    /// Splits upon obtaining 100 Essence
    Essence100,
    /// 200 Essence (Essence)
    /// 
    /// Splits upon obtaining 200 Essence
    Essence200,
    /// 300 Essence (Essence)
    /// 
    /// Splits upon obtaining 300 Essence
    Essence300,
    /// 400 Essence (Essence)
    /// 
    /// Splits upon obtaining 400 Essence
    Essence400,
    /// 500 Essence (Essence)
    /// 
    /// Splits upon obtaining 500 Essence
    Essence500,
    /// 600 Essence (Essence)
    /// 
    /// Splits upon obtaining 600 Essence
    Essence600,
    /// 700 Essence (Essence)
    /// 
    /// Splits upon obtaining 700 Essence
    Essence700,
    /// 800 Essence (Essence)
    /// 
    /// Splits upon obtaining 800 Essence
    Essence800,
    /// 900 Essence (Essence)
    /// 
    /// Splits upon obtaining 900 Essence
    Essence900,
    /// 1000 Essence (Essence)
    /// 
    /// Splits upon obtaining 1000 Essence
    Essence1000,
    /// 1100 Essence (Essence)
    /// 
    /// Splits upon obtaining 1100 Essence
    Essence1100,
    /// 1200 Essence (Essence)
    /// 
    /// Splits upon obtaining 1200 Essence
    Essence1200,
    /// 1300 Essence (Essence)
    /// 
    /// Splits upon obtaining 1300 Essence
    Essence1300,
    /// 1400 Essence (Essence)
    /// 
    /// Splits upon obtaining 1400 Essence
    Essence1400,
    /// 1500 Essence (Essence)
    /// 
    /// Splits upon obtaining 1500 Essence
    Essence1500,
    /// 1600 Essence (Essence)
    /// 
    /// Splits upon obtaining 1600 Essence
    Essence1600,
    /// 1700 Essence (Essence)
    /// 
    /// Splits upon obtaining 1700 Essence
    Essence1700,
    /// 1800 Essence (Essence)
    /// 
    /// Splits upon obtaining 1800 Essence
    Essence1800,
    /// 1900 Essence (Essence)
    /// 
    /// Splits upon obtaining 1900 Essence
    Essence1900,
    /// 2000 Essence (Essence)
    /// 
    /// Splits upon obtaining 2000 Essence
    Essence2000,
    /// 2100 Essence (Essence)
    /// 
    /// Splits upon obtaining 2100 Essence
    Essence2100,
    /// 2200 Essence (Essence)
    /// 
    /// Splits upon obtaining 2200 Essence
    Essence2200,
    /// 2300 Essence (Essence)
    /// 
    /// Splits upon obtaining 2300 Essence
    Essence2300,
    /// 2400 Essence (Essence)
    /// 
    /// Splits upon obtaining 2400 Essence
    Essence2400,
    /// Dream Nail Marissa (Obtain)
    /// 
    /// Splits when obtaining the essence from Marissa
    OnObtainGhostMarissa,
    /// Dream Nail Caelif and Fera (Obtain)
    /// 
    /// Splits when obtaining the essence from Caelif and Fera Orthop
    OnObtainGhostCaelifFera,
    /// Dream Nail Poggy (Obtain)
    /// 
    /// Splits when obtaining the essence from Poggy Thorax
    OnObtainGhostPoggy,
    /// Dream Nail Gravedigger (Obtain)
    /// 
    /// Splits when obtaining the essence from Gravedigger
    OnObtainGhostGravedigger,
    /// Dream Nail Joni (Obtain)
    /// 
    /// Splits when obtaining the essence from Blue Child Joni
    OnObtainGhostJoni,
    // TODO: resolve possible confounding essence sources for Cloth, Vespa, and Revek
    // endregion: Essence, Trees, and Ghosts

    // region: Dirtmouth
    /// King's Pass (Transition)
    /// 
    /// Splits when entering Dirtmouth from King's Pass
    KingsPass,
    /// Dirtmouth (Transition)
    /// 
    /// Splits on any transition into Dirtmouth Town
    EnterDirtmouth,
    /// Dirtmouth (Area)
    /// 
    /// Splits when entering Dirtmouth text first appears
    Dirtmouth,
    SlyShopExit,
    /// Elderbug Flower Quest (NPC)
    /// 
    /// Splits when giving the flower to the Elderbug
    ElderbugFlower,
    /// Cornifer at Home (Transition)
    /// 
    /// Splits when entering Iselda's hut while Cornifer is sleeping
    CorniferAtHome,
    /// Enter Troupe Master Grimm (Transition)
    /// 
    /// Splits when entering Grimm tent with requirements to trigger Troupe Master Grimm boss
    EnterTMG,
    /// Troupe Master Grimm (Boss)
    /// 
    /// Splits when killing Troupe Master Grimm
    TroupeMasterGrimm,
    /// NKG Dream (Transition)
    /// 
    /// Splits on transition into Nightmare King Grimm dream
    EnterNKG,
    /// Nightmare King Grimm (Boss)
    /// 
    /// Splits when killing Nightmare King Grimm
    NightmareKingGrimm,
    /// Grey Prince Zote (Boss)
    /// 
    /// Splits when killing Grey Prince
    GreyPrince,
    /// Grey Prince Zote (Essence)
    /// 
    /// Splits when getting Grey Prince Zote essence
    GreyPrinceEssence,
    // endregion: Dirtmouth
    // region: Crossroads
    /// Forgotten Crossroads (Area)
    /// 
    /// Splits when entering Forgotten Crossroads text first appears
    ForgottenCrossroads,
    /// Infected Crossroads (Area)
    /// 
    /// Splits when entering Infected Crossroads text first appears
    InfectedCrossroads,
    /// Menderbug (Killed)
    /// 
    /// Splits when killing Menderbug
    MenderBug,
    /// Enter Brooding Mawlek (Transition)
    /// 
    /// Splits when entering the Brooding Mawlek arena transition in Forgotten Crossroads
    EnterBroodingMawlek,
    /// Brooding Mawlek (Boss)
    /// 
    /// Splits when killing Brooding Mawlek
    BroodingMawlek,
    /// Ancestral Mound (Transition)
    /// 
    /// Splits on transition into Ancestral Mound
    AncestralMound,
    /// Gruz Mother (Boss)
    /// 
    /// Splits when killing Gruz Mother
    GruzMother,
    /// Sly Rescued (NPC)
    /// 
    /// Splits when saving Sly
    SlyRescued,
    /// False Knight (Boss)
    /// 
    /// Splits when killing False Knight
    FalseKnight,
    /// Failed Champion (Boss)
    /// 
    /// Splits when killing Failed Champion
    FailedKnight,
    /// Failed Champion (Essence)
    /// 
    /// Splits when getting Failed Champion essence
    FailedChampionEssence,
    /// Salubra's Blessing (Item)
    /// 
    /// Splits when obtaining Salubra's Blessing
    SalubrasBlessing,
    /// Salubra Exit (Transition)
    /// 
    /// Splits on the transition out of Salubra's Hut
    SalubraExit,
    EnterHollowKnight,
    /// Chains Broken - Hollow Knight (Event)
    /// 
    /// Splits at the end of the first Hollow Knight scream after the chains are broken
    UnchainedHollowKnight,
    /// Segment Practice - THK (Boss)
    /// 
    /// Splits when killing The Hollow Knight
    HollowKnightBoss,
    /// Radiance Dream Entry (Event)
    /// 
    /// Splits upon entering the Radiance dream
    // TODO: Skips upon killing the Hollow Knight (requires ordered splits)
    HollowKnightDreamnail,
    /// Segment Practice - Radiance (Boss)
    /// 
    /// Splits when killing The Radiance
    RadianceBoss,
    // endregion: Crossroads
    // region: Greenpath
    /// Greenpath (Transition)
    /// 
    /// Splits when entering Greenpath
    EnterGreenpath,
    /// Greenpath (Area)
    /// 
    /// Splits when entering Greenpath text first appears
    Greenpath,
    /// Moss Knight (Mini Boss)
    /// 
    /// Splits when killing Moss Knight
    MossKnight,
    /// Zote Rescued - Vengefly King (Mini Boss)
    /// 
    /// Splits when rescuing Zote from the Vengefly King
    Zote1,
    /// Vengefly King Killed (Transition)
    /// 
    /// Splits on transition after Vengefly King in Greenpath killed
    VengeflyKingTrans,
    /// Enter Hornet 1 (Transition)
    /// 
    /// Splits when entering Hornet boss arena transition in Greenpath
    EnterHornet1,
    /// Hornet 1 (Boss)
    /// 
    /// Splits when killing Hornet Protector in Greenpath
    Hornet1,
    /// Aluba (Killed)
    /// 
    /// Splits when killing an Aluba
    Aluba,
    /// Hunter's Mark (Item)
    /// 
    /// Splits when obtaining the Hunter's Mark
    HuntersMark,
    /// No Eyes (Boss)
    /// 
    /// Splits when killing No Eyes
    NoEyes,
    /// No Eyes (Essence)
    /// 
    /// Splits when absorbing essence from No Eyes
    NoEyesEssence,
    /// Massive Moss Charger (Boss)
    /// 
    /// Splits when killing Massive Moss Charger
    MegaMossCharger,
    /// Massive Moss Charger Killed (Transition)
    /// 
    /// Splits on transition after Massive Moss Charger is killed
    MegaMossChargerTrans,
    /// Happy Couple (Event)
    /// 
    /// Splits when talking to Nailsmith in Sheo's hut for the first time
    HappyCouplePlayerDataEvent,
    // endregion: Greenpath
    // region: Fungal
    /// Fungal Wastes Entry (Transition)
    /// 
    /// Splits on transition to Fungal Wastes
    /// 
    /// (Room below Crossroads, right of Queen's Station, left of Waterways or Spore Shroom room)
    FungalWastesEntry,
    /// Fungal Wastes (Area)
    /// 
    /// Splits when entering Fungal Wastes text first appears
    FungalWastes,
    /// Elder Hu (Boss)
    /// 
    /// Splits when killing Elder Hu
    ElderHu,
    /// Elder Hu (Essence)
    /// 
    /// Splits when absorbing essence from Elder Hu
    ElderHuEssence,
    /// Elder Hu Killed (Transition)
    /// 
    /// Splits on the transition after killing Elder Hu
    ElderHuTrans,
    MenuMantisJournal,
    /// Bretta Rescued (NPC)
    /// 
    /// Splits when saving Bretta
    BrettaRescued,
    /// Mantis Lords (Boss)
    /// 
    /// Splits when killing Mantis Lords
    MantisLords,
    // endregion: Fungal
    // region: Cliffs
    /// Gorb (Boss)
    /// 
    /// Splits when killing Gorb
    Gorb,
    /// Gorb (Essence)
    /// 
    /// Splits when absorbing essence from Gorb
    GorbEssence,
    /// Nightmare Lantern Lit (Event)
    /// 
    /// Splits when initially lighting the Nightmare Lantern
    NightmareLantern,
    /// Nightmare Lantern Destroyed (Event)
    /// 
    /// Splits when destroying the Nightmare Lantern
    NightmareLanternDestroyed,
    // endregion: Cliffs
    // region: Resting Grounds
    /// Blue Lake (Transition)
    /// 
    /// Splits on transition to Blue Lake from either side
    BlueLake,
    /// Enter Any Dream (Transition)
    /// 
    /// Splits when entering any dream world
    EnterAnyDream,
    DreamNailExit,
    /// Resting Grounds (Area)
    /// 
    /// Splits when entering Resting Grounds text first appears
    RestingGrounds,
    /// Resting Grounds Stag (Bench)
    /// 
    /// Splits when sitting on the bench at Resting Grounds Stag
    BenchRGStag,
    /// Xero (Boss)
    /// 
    /// Splits when killing Xero
    Xero,
    /// Xero (Essence)
    /// 
    /// Splits when absorbing essence from Xero
    XeroEssence,
    /// Spirit Glade Door (Event)
    /// 
    /// Splits when the Seer opens the Spirits' Glade after bringing back 200 essence
    SpiritGladeOpen,
    /// Seer Departs (Event)
    /// 
    /// Splits when the Seer Departs after bringing back 2400 essence
    SeerDeparts,
    /// Catacombs Entry (Transition)
    /// 
    /// Splits on entry to the catacombs below Resting Grounds
    CatacombsEntry,
    /// Met Grey Mourner (NPC)
    /// 
    /// Splits when talking to Grey Mourner for the first time
    MetGreyMourner,
    /// Mourner w/ Seer Ascended (NPC)
    /// 
    /// Splits when both talked to Grey Mourner and Seer has ascended
    GreyMournerSeerAscended,
    // endregion: Resting Grounds
    // region: City
    /// City Gate (Event)
    /// 
    /// Splits when using the City Crest to open the gate
    CityGateOpen,
    /// City Gate w/ Mantis Lords defeated (Event)
    /// 
    /// To make sure you don't forget Mantis Lords
    CityGateAndMantisLords,
    /// City of Tears (Area)
    /// 
    /// Splits when entering City of Tears text first appears
    CityOfTears,
    /// Gorgeous Husk (Killed)
    /// 
    /// Splits when killing Gorgeous Husk
    GorgeousHusk,
    /// Gorgeous Husk Killed (Transition)
    /// 
    /// Splits on transition after Gorgeous Husk defeated
    TransGorgeousHusk,
    /// Main Menu w/ Ghusk (Menu)
    /// 
    /// Splits on transition to the main menu after Gorgeous Husk defeated
    MenuGorgeousHusk,
    /// Rafters (Transition)
    /// 
    /// Splits on any transition into the City Rafters room
    EnterRafters,
    /// Lemm Shop (NPC)
    /// 
    /// Splits when talking to Lemm in the shop for the first time
    Lemm2,
    /// Sanctum Bench (Toll)
    /// 
    /// Splits when buying City/Sanctum toll bench by Cornifer's location
    TollBenchCity,
    /// Soul Sanctum (Transition)
    /// 
    /// Splits when entering Soul Sanctum
    EnterSanctum,
    /// Soul Sanctum w/ Shade Soul (Transition)
    /// 
    /// Splits when entering Soul Sanctum after obtaining Shade Soul
    EnterSanctumWithShadeSoul,
    /// Enter Soul Master (Transition)
    /// 
    /// Splits when entering Soul Master boss arena transition
    EnterSoulMaster,
    /// Soul Master Encountered (Boss)
    /// 
    /// Splits when Soul Master is activated the first time as the gate closes
    SoulMasterEncountered,
    /// Soul Master - Fake Spell Pickup (Boss)
    /// 
    /// Splits when triggering Soul Master phase 2 the first time
    SoulMasterPhase1,
    /// Soul Master (Boss)
    /// 
    /// Splits when killing Soul Master
    SoulMaster,
    /// Soul Tyrant (Boss)
    /// 
    /// Splits when killing Soul Tyrant
    SoulTyrant,
    /// Soul Tyrant (Essence)
    /// 
    /// Splits when getting Soul Tyrant essence
    SoulTyrantEssence,
    MenuStoreroomsSimpleKey,
    EnterBlackKnight,
    /// Chandelier - Watcher Knights (Event)
    /// 
    /// Splits when dropping the chandelier on one of the Watcher Knights
    WatcherChandelier,
    /// Watcher Knight (Boss)
    /// 
    /// Splits when killing Watcher Knights
    BlackKnight,
    /// Watcher Knight Killed (Transition)
    /// 
    /// Splits on the transition after killing Watcher Knights
    BlackKnightTrans,
    /// Tower of Love (Transition)
    /// 
    /// Splits when entering the Tower of Love
    EnterLoveTower,
    /// Collector (Boss)
    /// 
    /// Splits when killing Collector
    Collector,
    /// Collector Defeated (Transition)
    /// 
    /// Splits on transition after defeating the Collector
    TransCollector,
    /// Nailsmith Killed (Event)
    /// 
    /// Splits when Nailsmith is killed
    NailsmithKilled,
    // endregion: City
    // region: Peak
    /// Crystal Peak Entry (Transition)
    /// 
    /// Splits on transition to the room where the dive and toll entrances meet, or the room right of Dirtmouth
    CrystalPeakEntry,
    /// Crystal Peak (Area)
    /// 
    /// Splits when entering Crystal Peak text first appears
    CrystalPeak,
    /// Husk Miner (Killed)
    /// 
    /// Splits when killing a Husk Miner
    HuskMiner,
    MenuSlyKey,
    /// Crystal Guardian (Boss)
    /// 
    /// Splits when killing the Crystal Guardian
    CrystalGuardian1,
    /// Enraged Guardian (Boss)
    /// 
    /// Splits when killing the Enraged Guardian
    CrystalGuardian2,
    /// Hallownest's Crown (Transition)
    /// 
    /// Splits on transition into the room with the Whispering Root at the base of Hallownest's Crown
    EnterCrown,
    /// Crystal Mound Exit (Transition)
    /// 
    /// Splits on transition from Crystal Mound
    CrystalMoundExit,
    /// Crystal Peak Lift Opened (Event)
    /// 
    /// Splits when opening the lever for the lift between Dirtmouth and Crystal Peak
    MineLiftOpened,
    // endregion: Peak
    // region: Waterways
    /// Waterways Manhole (Toll)
    /// 
    /// Splits when opening the Waterways Manhole
    WaterwaysManhole,
    /// Waterways (Transition)
    /// 
    /// Splits on transition to Waterways
    WaterwaysEntry,
    /// Royal Waterways (Area)
    /// 
    /// Splits when entering Royal Waterways text first appears
    RoyalWaterways,
    /// Dung Defender (Boss)
    /// 
    /// Splits when killing Dung Defender
    DungDefender,
    DungDefenderExit,
    /// White Defender (Boss)
    /// 
    /// Splits when killing White Defender
    WhiteDefender,
    /// White Defender (Essence)
    /// 
    /// Splits when getting White Defender essence
    WhiteDefenderEssence,
    /// Met Emilitia (Event)
    /// 
    /// Splits when talking to Emilitia for the first time
    MetEmilitia,
    /// Flukemarm (Boss)
    /// 
    /// Splits when killing Flukemarm
    Flukemarm,
    /// Junk Pit (Transition)
    /// 
    /// Splits on transition into Junk Pit
    EnterJunkPit,
    // endregion: Waterways
    // region: Basin
    /// Ancient Basin (Transition)
    /// 
    /// Splits on transition to Ancient Basin
    BasinEntry,
    /// Ancient Basin (Area)
    /// 
    /// Splits when entering Ancient Basin text first appears
    Abyss,
    /// Saved Cloth (Event)
    /// 
    /// Splits when saving Cloth in Ancient Basin
    SavedCloth,
    /// Basin Bench (Toll)
    /// 
    /// Splits when buying Ancient Basin toll bench
    TollBenchBasin,
    Abyss19from18,
    /// Broken Vessel (Boss)
    /// 
    /// Splits when killing Broken Vessel
    BrokenVessel,
    /// Broken Vessel (Transition)
    /// 
    /// Splits on any non-death transition after defeating Broken Vessel
    BrokenVesselTrans,
    /// Lost Kin (Boss)
    /// 
    /// Splits when killing Lost Kin
    LostKin,
    /// Lost Kin (Essence)
    /// 
    /// Splits when getting Lost Kin essence
    LostKinEssence,
    // endregion: Basin
    // region: White Palace
    /// White Palace Entry (Transition)
    /// 
    /// Splits when entering the first White Palace scene
    WhitePalaceEntry,
    /// White Palace - Lower Entry (Room)
    /// 
    /// Splits on transition to White_Palace_01
    WhitePalaceLowerEntry,
    /// White Palace (Area)
    /// 
    /// Splits when entering White Palace text for the first time
    WhitePalace,
    /// White Palace - Lower Orb (Room)
    /// 
    /// Splits on transition to White_Palace_02
    WhitePalaceLowerOrb,
    /// White Palace - Lower Orb (Lever)
    /// 
    /// Splits when lighting the orb in White Palace lowest floor
    WhitePalaceOrb1,
    /// White Palace - Atrium (Room)
    /// 
    /// Splits on any transition to White_Palace_03_Hub
    WhitePalaceAtrium,
    /// White Palace - Left Entry (Room)
    /// 
    /// Splits on transition to White_Palace_04
    WhitePalaceLeftEntry,
    /// White Palace - Left Midpoint (Room)
    /// 
    /// Splits on transition between White_Palace_04 and _14
    WhitePalaceLeftWingMid,
    /// White Palace - Left Orb (Lever)
    /// 
    /// Splits when lighting the orb in White Palace left wing
    WhitePalaceOrb3,
    /// White Palace - Right Side Entry (Room)
    /// 
    /// Splits on transition between White_Palace_03_Hub and _15
    WhitePalaceRightEntry,
    /// White Palace - Right Side Climb (Room)
    /// 
    /// Splits on transition between White_Palace_05 and _16
    WhitePalaceRightClimb,
    /// White Palace - Right Side Saw Squeeze (Room)
    /// 
    /// Splits on transition between White_Palace_16 and _05
    WhitePalaceRightSqueeze,
    /// White Palace - Right Side Exit (Room)
    /// 
    /// Splits on transition between White_Palace_05 and _15
    WhitePalaceRightDone,
    /// White Palace - Right Orb (Lever)
    /// 
    /// Splits when lighting the orb in White Palace right wing
    WhitePalaceOrb2,
    /// White Palace - Top Entry (Room)
    /// 
    /// Splits on transition between White_Palace_03_Hub and _06
    WhitePalaceTopEntry,
    /// Path of Pain Room 1 (Room)
    /// 
    /// Splits on transition to the first room in PoP (entry to PoP)
    PathOfPainEntry,
    /// Path of Pain Room 2 (Room)
    /// 
    /// Splits on transition to the second room in PoP
    PathOfPainTransition1,
    /// Path of Pain Room 3 (Room)
    /// 
    /// Splits on transition to the third room in PoP
    PathOfPainTransition2,
    /// Path of Pain Room 4 (Room)
    /// 
    /// Splits on transition to the fourth room in PoP (Final room)
    PathOfPainTransition3,
    /// Path of Pain (Completed)
    /// 
    /// Splits when completing the Path of Pain in White Palace
    PathOfPain,
    /// White Palace - Top Cursed Cycle (Room)
    /// 
    /// Splits on transition between White_Palace_06 and _07
    WhitePalaceTopClimb,
    /// White Palace - Top Lever (Room)
    /// 
    /// Splits on transition between White_Palace_07 and _12
    WhitePalaceTopLeverRoom,
    /// White Palace - Top Final Platforming (Room)
    /// 
    /// Splits on transition between White_Palace_12 and _13
    WhitePalaceTopLastPlats,
    /// White Palace - Workshop (Area)
    /// 
    /// Splits when visiting the secret room in White Palace
    WhitePalaceSecretRoom,
    /// White Palace - Throne Room (Room)
    /// 
    /// Splits on transition between White_Palace_13 and _09
    WhitePalaceThroneRoom,
    // endregion: White Palace
    // region: Kingdom's Edge
    /// Kingdom's Edge (Transition)
    /// 
    /// Splits on transition to Kingdom's Edge from King's Station
    // Question: should this be any entrance to Kingdom's Edge,
    //           or just the King's Station entrance?
    //           Maybe the room off the side of the RG elevator shouldn't count,
    //           but what about the Tram entrance?
    KingdomsEdgeEntry,
    /// Kingdom's Edge (Area)
    /// 
    /// Splits when entering Kingdom's Edge text first appears
    KingdomsEdge,
    /// Hive (Transition)
    /// 
    /// Splits on transition to Hive
    HiveEntry,
    /// Hive (Area)
    /// 
    /// Splits when entering Hive text first appears
    Hive,
    /// Enter Hive Knight (Transition)
    /// 
    /// Splits when entering Hive Knight boss arena transition
    EnterHiveKnight,
    /// Hive Knight (Boss)
    /// 
    /// Splits when killing Hive Knight
    HiveKnight,
    /// Great Hopper (Killed)
    /// 
    /// Splits when killing a Great Hopper
    GreatHopper,
    /// Enter Hornet 2 (Transition)
    /// 
    /// Splits when entering Hornet boss arena transition in Kingdom's Edge
    EnterHornet2,
    /// Hornet 2 (Boss)
    /// 
    /// Splits when killing Hornet Sentinel in Kingdom's Edge
    Hornet2,
    /// Markoth (Boss)
    /// 
    /// Splits when killing Markoth
    Markoth,
    /// Markoth (Essence)
    /// 
    /// Splits when absorbing essence from Markoth
    MarkothEssence,
    // endregion: Kingdom's Edge
    // region: Colosseum
    /// Little Fool (NPC)
    /// 
    /// Splits when talking to the Little Fool for the first time
    LittleFool,
    /// Colosseum Unlocked 1 (Trial)
    /// 
    /// Splits when the knight unlocks the Trial of the Warrior at Little Fool
    ColosseumBronzeUnlocked,
    /// Colosseum Entrance 1 (Transition)
    /// 
    /// Splits on the transition into the Trial of the Warrior
    ColosseumBronzeEntry,
    /// Colosseum (Area)
    /// 
    /// Splits when entering Colosseum text first appears
    Colosseum,
    /// Zote Defeated - Colosseum (Mini Boss)
    /// 
    /// Splits when defeating Zote in the Colosseum
    ZoteKilled,
    /// Colosseum Fight 1 (Trial)
    /// 
    /// Splits when beating the Trial of the Warrior
    ColosseumBronze,
    /// Colosseum Exit 1 (Transition)
    /// 
    /// Splits on the transition out of the trial, or in the load-in after quitout
    ColosseumBronzeExit,
    /// Colosseum Unlocked 2 (Trial)
    /// 
    /// Splits when the knight unlocks the Trial of the Conqueror at Little Fool
    ColosseumSilverUnlocked,
    /// Colosseum Entrance 2 (Transition)
    /// 
    /// Splits on the transition into the Trial of the Conqueror
    ColosseumSilverEntry,
    /// Colosseum Fight 2 (Trial)
    /// 
    /// Splits when beating the Trial of the Conqueror
    ColosseumSilver,
    /// Colosseum Exit 2 (Transition)
    /// 
    /// Splits on the transition out of the trial, or in the load-in after quitout
    ColosseumSilverExit,
    /// Colosseum Unlocked 3 (Trial)
    /// 
    /// Splits when the knight unlocks the Trial of the Fool at Little Fool
    ColosseumGoldUnlocked,
    /// Colosseum Entrance 3 (Transition)
    /// 
    /// Splits on the transition into the Trial of the Warrior
    ColosseumGoldEntry,
    /// God Tamer (Boss)
    /// 
    /// Splits when killing the God Tamer
    GodTamer,
    /// Colosseum Fight 3 (Trial)
    /// 
    /// Splits when beating the Trial of the Warrior
    ColosseumGold,
    /// Colosseum Exit 3 (Transition)
    /// 
    /// Splits on the transition out of the trial, or in the load-in after quitout
    ColosseumGoldExit,
    // endregion: Colosseum
    // region: Fog Canyon
    /// Fog Canyon (Transition)
    /// 
    /// Splits on transition to Fog Canyon
    FogCanyonEntry,
    /// Fog Canyon (Area)
    /// 
    /// Splits when entering Fog Canyon text first appears
    FogCanyon,
    /// Teachers Archive (Area)
    /// 
    /// Splits when entering Teachers Archive for the first time
    TeachersArchive,
    /// Uumuu Encountered (Boss)
    /// 
    /// Splits Uumuu is activated the first time as the gate closes
    UumuuEncountered,
    /// Uumuu (Boss)
    /// 
    /// Splits when killing Uumuu
    Uumuu,
    // endregion: Fog Canyon
    // region: Queen's Gardens
    /// Queen's Garden Entry (Transition)
    /// 
    /// Splits on transition to QG scene following QGA or above Deepnest
    QueensGardensEntry,
    /// Queen's Gardens (Area)
    /// 
    /// Splits when entering Queen's Gardens text first appears
    QueensGardens,
    /// Queen's Garden Bench (Toll)
    /// 
    /// Splits when buying Queen's Garden toll bench
    TollBenchQG,
    /// Queen's Garden - Post-Upper Arena (Transition)
    /// 
    /// Splits on transition to room after upper arena in QG
    QueensGardensPostArenaTransition,
    /// Flower Quest (Event)
    /// 
    /// Splits when placing the flower at the grave of the Traitors' Child
    FlowerQuest,
    /// Queen's Garden - Frogs (Transition)
    /// 
    /// Splits on transition to QG frogs scene
    QueensGardensFrogsTrans,
    /// Marmu (Boss)
    /// 
    /// Splits when killing Marmu
    Marmu,
    /// Marmu (Essence)
    /// 
    /// Splits when absorbing essence from Marmu
    MarmuEssence,
    /// Traitor Lord (Boss)
    /// 
    /// Splits when killing Traitor Lord
    TraitorLord,
    // endregion: Queen's Gardens
    // region: Deepnest
    /// Deepnest (Transition)
    /// 
    /// Splits on transition into Deepnest
    EnterDeepnest,
    /// Deepnest (Area)
    /// 
    /// Splits when entering Deepnest text first appears
    Deepnest,
    /// Deepnest Spa (Area)
    /// 
    /// Splits when entering the Deepnest Spa area with bench
    DeepnestSpa,
    /// Zote Rescued - Deepnest (Mini Boss)
    /// 
    /// Splits when rescuing Zote in Deepnest
    Zote2,
    /// Tram Deepnest (Tram)
    /// 
    /// Splits when unlocking the tram in Deepnest
    TramDeepnest,
    /// Nosk (Transition)
    /// 
    /// Splits when entering Nosk boss arena transition
    EnterNosk,
    /// Nosk (Boss)
    /// 
    /// Splits when killing Nosk
    Nosk,
    /// Galien (Boss)
    /// 
    /// Splits when killing Galien
    Galien,
    /// Galien (Essence)
    /// 
    /// Splits when absorbing essence from Galien
    GalienEssence,
    /// Trap Bench (Event)
    /// 
    /// Splits when getting the trap bench in Beasts Den
    BeastsDenTrapBench,
    // endregion: Deepnest
    // region: Godhome
    /// God Tuner (Item)
    /// 
    /// Splits when obtaining the God Tuner
    GodTuner,
    /// Godhome (Transition)
    /// 
    /// Splits on transition to Godhome
    EnterGodhome,
    /// Godhome (Area)
    /// 
    /// Splits when entering Godhome text first appears
    Godhome,
    /// Eternal Ordeal Unlocked (Event)
    /// 
    /// Splits when breaking the wall to the Zote statue in Godhome
    EternalOrdealUnlocked,
    /// Eternal Ordeal Achieved (Event)
    /// 
    /// Splits when achieving the ordeal (57th Zote killed)
    EternalOrdealAchieved,
    /// Pantheon 1-4 (Transition)
    /// 
    /// Splits on entry to any of Pantheon 1 - 4
    Pantheon1to4Entry,
    /// Vengefly King (Pantheon)
    /// 
    /// Splits after killing Vengefly King in Pantheon 1 or Pantheon 5
    VengeflyKingP,
    /// Gruz Mother (Pantheon)
    /// 
    /// Splits after killing Gruz Mother in Pantheon 1 or Pantheon 5
    GruzMotherP,
    /// False Knight (Pantheon)
    /// 
    /// Splits after killing False Knight in Pantheon 1 or Pantheon 5
    FalseKnightP,
    /// Massive Moss Charger (Pantheon)
    /// 
    /// Splits after killing Massive Moss Charger in Pantheon 1 or Pantheon 5
    MassiveMossChargerP,
    /// Hornet 1 (Pantheon)
    /// 
    /// Splits after killing Hornet Protector in Pantheon 1 or Pantheon 5
    Hornet1P,
    /// Godhome Bench (Transition)
    /// 
    /// Splits when leaving a Godhome Bench room
    GodhomeBench,
    /// Gorb (Pantheon)
    /// 
    /// Splits after killing Gorb in Pantheon 1 or Pantheon 5
    GorbP,
    /// Dung Defender (Pantheon)
    /// 
    /// Splits after killing Dung Defender in Pantheon 1 or Pantheon 5
    DungDefenderP,
    /// Soul Warrior (Pantheon)
    /// 
    /// Splits after killing Soul Warrior in Pantheon 1 or Pantheon 5
    SoulWarriorP,
    /// Brooding Mawlek (Pantheon)
    /// 
    /// Splits after killing Brooding Mawlek in Pantheon 1 or Pantheon 5
    BroodingMawlekP,
    /// Godhome Lore Room (Transition)
    /// 
    /// Splits when leaving a Godhome lore room
    GodhomeLoreRoom,
    /// Oro & Mato Nail Bros (Boss)
    /// 
    /// Splits when defeating Brothers Oro & Mato
    MatoOroNailBros,
    /// Oro & Mato Nail Bros (Pantheon)
    /// 
    /// Splits after killing Brothers Oro & Mato in Pantheon 1 or Pantheon 5
    OroMatoNailBrosP,
    /// Pantheon 1 (Trial)
    /// 
    /// Splits when beating the Pantheon of the Master
    Pantheon1,
    /// Xero (Pantheon)
    /// 
    /// Splits after killing Xero in Pantheon 2 or Pantheon 5
    XeroP,
    /// Crystal Guardian (Pantheon)
    /// 
    /// Splits after killing Crystal Guardian in Pantheon 2 or Pantheon 5
    CrystalGuardianP,
    /// Soul Master (Pantheon)
    /// 
    /// Splits after killing Soul Master in Pantheon 2 or Pantheon 5
    SoulMasterP,
    /// Oblobbles (Pantheon)
    /// 
    /// Splits after killing Oblobbles in Pantheon 2 or Pantheon 5
    OblobblesP,
    /// Mantis Lords (Pantheon)
    /// 
    /// Splits after killing Mantis Lords in Pantheon 2 or Sisters of Battle Pantheon 5
    MantisLordsP,
    /// Marmu (Pantheon)
    /// 
    /// Splits after killing Marmu in Pantheon 2 or Pantheon 5
    MarmuP,
    /// Nosk (Pantheon)
    /// 
    /// Splits after killing Nosk in Pantheon 2
    NoskP,
    /// Flukemarm (Pantheon)
    /// 
    /// Splits after killing Flukemarm in Pantheon 2 or Pantheon 5
    FlukemarmP,
    /// Broken Vessel (Pantheon)
    /// 
    /// Splits after killing Broken Vessel in Pantheon 2 or Pantheon 5
    BrokenVesselP,
    /// Paintmaster Sheo (Boss)
    /// 
    /// Splits when killing Paintmaster Sheo
    SheoPaintmaster,
    /// Paintmaster Sheo (Pantheon)
    /// 
    /// Splits after killing Paintmaster Sheo in Pantheon 2 or Pantheon 5
    SheoPaintmasterP,
    /// Pantheon 2 (Trial)
    /// 
    /// Splits when beating the Pantheon of the Artist
    Pantheon2,
    /// Hive Knight (Pantheon)
    /// 
    /// Splits after killing Hive Knight in Pantheon 3 or Pantheon 5
    HiveKnightP,
    /// Elder Hu (Pantheon)
    /// 
    /// Splits after killing Elder Hu in Pantheon 3 or Pantheon 5
    ElderHuP,
    /// Collector (Pantheon)
    /// 
    /// Splits after killing The Collector in Pantheon 3 or Pantheon 5
    CollectorP,
    /// God Tamer (Pantheon)
    /// 
    /// Splits after killing God Tamer in Pantheon 3 or Pantheon 5
    GodTamerP,
    /// Troupe Master Grimm (Pantheon)
    /// 
    /// Splits after killing Troupe Master Grimm in Pantheon 3 or Pantheon 5
    TroupeMasterGrimmP,
    /// Galien (Pantheon)
    /// 
    /// Splits after killing Galien in Pantheon 3 or Pantheon 5
    GalienP,
    /// Grey Prince Zote (Pantheon)
    /// 
    /// Splits after killing Grey Prince Zote in Pantheon 3 or Pantheon 5
    GreyPrinceZoteP,
    /// Uumuu (Pantheon)
    /// 
    /// Splits after killing Uumuu in Pantheon 3 or Pantheon 5
    UumuuP,
    /// Hornet 2 (Pantheon)
    /// 
    /// Splits after killing Hornet Sentinel in Pantheon 3 or Pantheon 5
    Hornet2P,
    /// Great Nailsage Sly (Boss)
    /// 
    /// Splits when killing Great Nailsage Sly
    SlyNailsage,
    /// Great Nailsage Sly (Pantheon)
    /// 
    /// Splits after killing Great Nailsage Sly in Pantheon 3 or Pantheon 5
    SlyP,
    /// Pantheon 3 (Trial)
    /// 
    /// Splits when beating the Pantheon of the Sage
    Pantheon3,
    /// Enraged Guardian (Pantheon)
    /// 
    /// Splits after killing Enraged Guardian in Pantheon 4 or Pantheon 5
    EnragedGuardianP,
    /// Lost Kin (Pantheon)
    /// 
    /// Splits after killing Lost Kin in Pantheon 4 or Pantheon 5
    LostKinP,
    /// No Eyes (Pantheon)
    /// 
    /// Splits after killing No Eyes in Pantheon 4 or Pantheon 5
    NoEyesP,
    /// Traitor Lord (Pantheon)
    /// 
    /// Splits after killing Traitor Lord in Pantheon 4 or Pantheon 5
    TraitorLordP,
    /// White Defender (Pantheon)
    /// 
    /// Splits after killing White Defender in Pantheon 4 or Pantheon 5
    WhiteDefenderP,
    /// Failed Champion (Pantheon)
    /// 
    /// Splits after killing Failed Champion in Pantheon 4 or Pantheon 5
    FailedChampionP,
    /// Markoth (Pantheon)
    /// 
    /// Splits after killing Markoth in Pantheon 4 or Pantheon 5
    MarkothP,
    /// Watcher Knights (Pantheon)
    /// 
    /// Splits after killing Watcher Knights in Pantheon 4 or Pantheon 5
    WatcherKnightsP,
    /// Soul Tyrant (Pantheon)
    /// 
    /// Splits after killing Soul Tyrant in Pantheon 4 or Pantheon 5
    SoulTyrantP,
    /// Pure Vessel (Boss)
    /// 
    /// Splits when killing Pure Vessel
    PureVessel,
    /// Pure Vessel (Pantheon)
    /// 
    /// Splits after killing Pure Vessel in Pantheon 4 or Pantheon 5
    PureVesselP,
    /// Pantheon 4 (Trial)
    /// 
    /// Splits when beating the Pantheon of the Knight
    Pantheon4,
    /// Pantheon 5 (Transition)
    /// 
    /// Splits on entry to Pantheon 5
    Pantheon5Entry,
    /// Winged Nosk (Pantheon)
    /// 
    /// Splits after killing Winged Nosk in Pantheon 5
    NoskHornetP,
    /// Nightmare King Grimm (Pantheon)
    /// 
    /// Splits after killing Nightmare King Grimm in Pantheon 5
    NightmareKingGrimmP,
    /// Absolute Radiance (Pantheon)
    /// 
    /// Splits after killing Absolute Radiance in Pantheon 5
    RadianceP,
    /// Pantheon 5 (Trial)
    /// 
    /// Splits when beating the Pantheon of Hallownest
    Pantheon5,
    // endregion: Godhome
}

impl StoreWidget for Split {
    fn insert_into(&self, settings_map: &asr::settings::Map, key: &str) -> bool {
        let new_s = options_str(self);
        if settings_map.get(key).is_some_and(|old_v| old_v.get_string().is_some_and(|old_s| old_s == new_s)) {
            return false;
        }
        settings_map.insert(key, new_s);
        true
    }
}

impl ToString for Split {
    fn to_string(&self) -> String {
        serde_json::to_value(self).unwrap_or_default().as_str().unwrap_or_default().to_string()
    }
}

impl FromStr for Split {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Split, serde_json::Error> {
        serde_json::value::from_value(serde_json::Value::String(s.to_string()))
    }
}

impl Split {
    pub fn from_settings_str<S: Settings>(s: S) -> Option<Split> {
        Split::from_str(&s.as_string()?).ok()
    }
    pub fn from_settings_split<S: Settings>(s: S) -> Option<Split> {
        Split::from_settings_str(s.dict_get("Split").unwrap_or(s))
    }
}

pub fn transition_splits(s: &Split, p: &Pair<&str>, prc: &Process, g: &GameManagerFinder, pds: &mut PlayerDataStore) -> SplitterAction {
    match s {
        // region: Start, End, and Menu
        Split::EndingSplit => should_split(p.current.starts_with("Cinematic_Ending")),
        Split::EndingA => should_split(p.current == "Cinematic_Ending_A"),
        Split::EndingB => should_split(p.current == "Cinematic_Ending_B"),
        Split::EndingC => should_split(p.current == "Cinematic_Ending_C"),
        Split::EndingD => should_split(p.current == "Cinematic_Ending_D"),
        Split::EndingE => should_split(p.current == "Cinematic_Ending_E"),
        Split::Menu => should_split(is_menu(p.current)),
        Split::AnyTransition => should_split(p.current != p.old && !(p.old.is_empty() || p.current.is_empty() || is_menu(p.old))),
        Split::TransitionAfterSaveState => should_split(p.current != p.old
                                                        && !(p.old.is_empty()
                                                            || p.current.is_empty()
                                                            || is_menu(p.old)
                                                            || is_debug_save_state_scene(p.old)
                                                            || is_debug_save_state_scene(p.current))),
        // endregion: Start, End, and Menu

        // region: Dreamers
        /*
        // Old scene-transition based dreamer splits from when I only knew how to read the scene name
        Split::Lurien => should_split(p.old == "Dream_Guardian_Lurien" && p.current == "Cutscene_Boss_Door"),
        Split::Monomon => should_split(p.old == "Dream_Guardian_Monomon" && p.current == "Cutscene_Boss_Door"),
        Split::Hegemol => should_split(p.old == "Dream_Guardian_Hegemol" && p.current == "Cutscene_Boss_Door"),
        */
        Split::MenuDreamer3 => should_split(3 <= pds.guardians_defeated(prc, g) && is_menu(p.current)),
        // endregion: Dreamers

        // region: Dirtmouth
        Split::KingsPass => should_split(p.old == "Tutorial_01" && p.current == "Town"),
        Split::EnterDirtmouth => should_split(p.current == "Town" && p.current != p.old),
        Split::SlyShopExit => should_split(p.old == "Room_shop" && p.current != p.old),
        Split::LumaflyLanternTransition => should_split(pds.has_lantern(prc, g) && !p.current.starts_with("Room_shop")),
        Split::CorniferAtHome => should_split(pds.cornifer_at_home(prc, g) && p.old.starts_with("Town") && p.current.starts_with("Room_mapper")),
        // TODO: should EnterTMG check that Grimmchild is actually equipped?
        Split::EnterTMG => should_split(p.current.starts_with("Grimm_Main_Tent")
                                        && p.current != p.old
                                        && g.grimm_child_level(prc).is_some_and(|l| l == 2)
                                        && g.flames_collected(prc).is_some_and(|f| 3 <= f)),
        Split::EnterNKG => should_split(p.old.starts_with("Grimm_Main_Tent") && p.current.starts_with("Grimm_Nightmare")),
        // endregion: Dirtmouth
        // region: Crossroads
        Split::EnterBroodingMawlek => should_split(p.current == "Crossroads_09" && p.current != p.old),
        Split::AncestralMound => should_split(p.current == "Crossroads_ShamanTemple" && p.current != p.old),
        Split::TransVS => should_split(1 <= pds.get_fireball_level(prc, g) && p.current != p.old),
        Split::SalubraExit => should_split(p.old == "Room_Charm_Shop" && p.current != p.old),
        Split::EnterHollowKnight => should_split(p.current == "Room_Final_Boss_Core" && p.current != p.old),
        Split::HollowKnightDreamnail => should_split(p.current.starts_with("Dream_Final") && p.current != p.old),
        // endregion: Crossroads
        // region: Greenpath
        Split::EnterGreenpath => should_split(p.current.starts_with("Fungus1_01") && !p.old.starts_with("Fungus1_01")),
        Split::VengeflyKingTrans => should_split(pds.zote_rescued_buzzer(prc, g) && p.current != p.old),
        Split::EnterHornet1 => should_split(p.current.starts_with("Fungus1_04") && p.current != p.old),
        Split::MenuCloak => should_split(pds.has_dash(prc, g) && is_menu(p.current)),
        Split::MegaMossChargerTrans => should_split(pds.mega_moss_charger_defeated(prc, g) && p.current != p.old),
        // endregion: Greenpath
        // region: Fungal
        Split::FungalWastesEntry => should_split(starts_with_any(p.current, FUNGAL_WASTES_ENTRY_SCENES) && p.current != p.old),
        Split::ElderHuTrans => should_split(pds.killed_ghost_hu(prc, g) && p.current != p.old),
        Split::MenuDashmaster => should_split(pds.got_charm_31(prc, g) && is_menu(p.current)),
        Split::TransClaw => should_split(pds.has_wall_jump(prc, g) && p.current != p.old),
        Split::MenuClaw => should_split(pds.has_wall_jump(prc, g) && is_menu(p.current)),
        Split::MenuMantisJournal => should_split(is_menu(p.current) && p.old == "Fungus2_17"),
        // endregion: Fungal
        // TODO: should there be a HowlingCliffsEntry or EnterHowlingCliffs transition split?
        //       and what scenes should it be based on?
        //       should the room with Baldur Shell count,
        //       or only the rooms that the area text can appear in?
        // region: Resting Grounds
        Split::BlueLake => should_split(p.current.starts_with("Crossroads_50") && !p.old.starts_with("Crossroads_50")), // blue lake is Crossroads_50
        Split::EnterAnyDream => should_split(p.current.starts_with("Dream_") && p.current != p.old),
        Split::DreamNailExit => should_split(p.old == "Dream_Nailcollection" && p.current == "RestingGrounds_07"),
        Split::MenuDreamNail => should_split(pds.has_dream_nail(prc, g) && is_menu(p.current)),
        Split::MenuDreamGate => should_split(pds.has_dream_gate(prc, g) && is_menu(p.current)),
        Split::CatacombsEntry => should_split(p.current.starts_with("RestingGrounds_10") && !p.old.starts_with("RestingGrounds_10")),
        // endregion: Resting Grounds
        // region: City
        Split::TransGorgeousHusk => should_split(pds.killed_gorgeous_husk(prc, g) && p.current != p.old),
        Split::MenuGorgeousHusk => should_split(pds.killed_gorgeous_husk(prc, g) && is_menu(p.current)),
        Split::EnterRafters => should_split(p.current == "Ruins1_03" && p.current != p.old),
        Split::EnterSanctum => should_split(p.current.starts_with("Ruins1_23") && !p.old.starts_with("Ruins1_23")),
        Split::EnterSanctumWithShadeSoul => should_split(2 <= pds.get_fireball_level(prc, g) && p.current.starts_with("Ruins1_23") && !p.old.starts_with("Ruins1_23")),
        Split::EnterSoulMaster => should_split(p.current.starts_with("Ruins1_24") && p.current != p.old),
        Split::MenuStoreroomsSimpleKey => should_split(is_menu(p.current) && p.old == "Ruins1_17"),
        Split::TransShadeSoul => should_split(2 <= pds.get_fireball_level(prc, g) && p.current != p.old),
        Split::MenuShadeSoul => should_split(2 <= pds.get_fireball_level(prc, g) && is_menu(p.current)),
        Split::EnterBlackKnight => should_split(p.current == "Ruins2_03" && p.current != p.old),
        Split::BlackKnightTrans => should_split(pds.killed_black_knight(prc, g) && p.current != p.old),
        Split::EnterLoveTower => should_split(p.current.starts_with("Ruins2_11") && p.current != p.old),
        Split::TransCollector => should_split(pds.collector_defeated(prc, g) && p.current != p.old),
        // endregion: City
        // region: Peak
        Split::CrystalPeakEntry => should_split(starts_with_any(p.current, CRYSTAL_PEAK_ENTRY_SCENES) && p.current != p.old),
        Split::MenuSlyKey => should_split(is_menu(p.current) && p.old == "Mines_11"),
        Split::EnterCrown => should_split(p.current == "Mines_23" && p.current != p.old),
        Split::TransDescendingDark => should_split(2 <= pds.get_quake_level(prc, g) && p.current != p.old),
        Split::CrystalMoundExit => should_split(p.old.starts_with("Mines_35") && p.current != p.old),
        // endregion: Peak
        // region: Waterways
        Split::WaterwaysEntry => should_split(starts_with_any(p.current, WATERWAYS_ENTRY_SCENES) && p.current != p.old),
        Split::DungDefenderExit => should_split(p.old == "Waterways_05" && p.current == "Abyss_01"),
        Split::TransTear => should_split(pds.has_acid_armour(prc, g) && p.current != p.old),
        Split::MenuIsmasTear => should_split(pds.has_acid_armour(prc, g) && is_menu(p.current)),
        Split::EnterJunkPit => should_split(p.current == "GG_Waterways" && p.current != p.old),
        // endregion: Waterways
        // region: Basin
        Split::BasinEntry => should_split(p.current.starts_with("Abyss_04") && p.current != p.old),
        Split::Abyss19from18 => should_split(p.old == "Abyss_18" && p.current == "Abyss_19"),
        Split::BrokenVesselTrans => should_split(pds.killed_infected_knight(prc, g) && g.get_health(prc).is_some_and(|h| 0 < h)),
        Split::MenuWings => should_split(pds.has_double_jump(prc, g) && is_menu(p.current)),
        Split::MenuVoidHeart => should_split(pds.got_shade_charm(prc, g) && is_menu(p.current)),
        // endregion: Basin
        // region: White Palace
        Split::WhitePalaceEntry => should_split(p.current.starts_with("White_Palace_11") && p.current != p.old),
        Split::WhitePalaceLowerEntry => should_split(p.current.starts_with("White_Palace_01") && p.current != p.old),
        Split::WhitePalaceLowerOrb => should_split(p.current.starts_with("White_Palace_02") && p.current != p.old),
        Split::WhitePalaceAtrium => should_split(p.current.starts_with("White_Palace_03_hub") && p.current != p.old),
        Split::WhitePalaceLeftEntry => should_split(p.current.starts_with("White_Palace_04") && p.current != p.old),
        Split::WhitePalaceLeftWingMid => should_split(p.old.starts_with("White_Palace_04") && p.current.starts_with("White_Palace_14")),
        Split::WhitePalaceRightEntry => should_split(p.current.starts_with("White_Palace_15") && p.current != p.old),
        Split::WhitePalaceRightClimb => should_split(p.old.starts_with("White_Palace_05") && p.current.starts_with("White_Palace_16")),
        Split::WhitePalaceRightSqueeze => should_split(p.old.starts_with("White_Palace_16") && p.current.starts_with("White_Palace_05")),
        Split::WhitePalaceRightDone => should_split(p.old.starts_with("White_Palace_05") && p.current.starts_with("White_Palace_15")),
        Split::WhitePalaceTopEntry => should_split(p.old.starts_with("White_Palace_03_hub") && p.current.starts_with("White_Palace_06")),
        Split::PathOfPainEntry => should_split(p.current.starts_with("White_Palace_18") && p.old.starts_with("White_Palace_06")),
        Split::PathOfPainTransition1 => should_split(p.current.starts_with("White_Palace_17") && p.old.starts_with("White_Palace_18")),
        Split::PathOfPainTransition2 => should_split(p.current.starts_with("White_Palace_19") && p.old.starts_with("White_Palace_17")),
        Split::PathOfPainTransition3 => should_split(p.current.starts_with("White_Palace_20") && p.old.starts_with("White_Palace_19")),
        Split::WhitePalaceTopClimb => should_split(p.old.starts_with("White_Palace_06") && p.current.starts_with("White_Palace_07")),
        Split::WhitePalaceTopLeverRoom => should_split(p.old.starts_with("White_Palace_07") && p.current.starts_with("White_Palace_12")),
        Split::WhitePalaceTopLastPlats => should_split(p.old.starts_with("White_Palace_12") && p.current.starts_with("White_Palace_13")),
        Split::WhitePalaceThroneRoom => should_split(p.old.starts_with("White_Palace_13") && p.current.starts_with("White_Palace_09")),
        // endregion: White Palace
        // region: Kingdom's Edge
        // Deepnest_East_03 is the KE room with Cornifer, acid, and raining fools,
        // where the King's Station and Tram entrances meet
        Split::KingdomsEdgeEntry => should_split(p.current.starts_with("Deepnest_East_03") && p.current != p.old),
        Split::HiveEntry => should_split(p.current.starts_with("Hive_01") && p.current != p.old),
        Split::EnterHiveKnight => should_split(p.current.starts_with("Hive_05") && p.current != p.old),
        Split::EnterHornet2 => should_split(p.current.starts_with("Deepnest_East_Hornet") && p.current != p.old),
        // endregion: Kingdom's Edge
        // region: Colosseum
        Split::ColosseumBronzeEntry => should_split(p.old == "Room_Colosseum_01" && p.current == "Room_Colosseum_Bronze"),
        Split::ColosseumBronzeExit => should_split(pds.colosseum_bronze_completed(prc, g) && !p.current.starts_with("Room_Colosseum_Bronze")),
        Split::ColosseumSilverEntry => should_split(p.old == "Room_Colosseum_01" && p.current == "Room_Colosseum_Silver"),
        Split::ColosseumSilverExit => should_split(pds.colosseum_bronze_completed(prc, g) && !p.current.starts_with("Room_Colosseum_Silver")),
        Split::ColosseumGoldEntry => should_split(p.old == "Room_Colosseum_01" && p.current == "Room_Colosseum_Gold"),
        Split::ColosseumGoldExit => should_split(pds.colosseum_bronze_completed(prc, g) && !p.current.starts_with("Room_Colosseum_Gold")),
        // endregion: Colosseum
        // region: Fog Canyon
        Split::FogCanyonEntry => should_split(starts_with_any(p.current, FOG_CANYON_ENTRY_SCENES) && p.current != p.old),
        Split::TeachersArchive => should_split(p.current.starts_with("Fungus3_archive") && !p.old.starts_with("Fungus3_archive")),
        // endregion: Fog Canyon
        // region: Queen's Gardens
        Split::QueensGardensEntry => should_split(starts_with_any(p.current, QUEENS_GARDENS_ENTRY_SCENES) && p.current != p.old),
        Split::QueensGardensPostArenaTransition => should_split(p.current.starts_with("Fungus3_13") && p.current != p.old),
        // Fungus1_23 is the first frogs room in QG, even though QG usually uses Fungus3, and GP usually uses Fungus1
        Split::QueensGardensFrogsTrans => should_split(p.current.starts_with("Fungus1_23") && p.current != p.old),
        // endregion: Queen's Gardens
        // region: Deepnest
        Split::EnterDeepnest => should_split(starts_with_any(p.current, DEEPNEST_ENTRY_SCENES) && p.current != p.old),
        Split::EnterNosk => should_split(p.current.starts_with("Deepnest_32") && p.current != p.old),
        // endregion: Deepnest
        // region: Godhome
        Split::EnterGodhome => should_split(p.current.starts_with("GG_Atrium") && p.current != p.old),
        Split::Pantheon1to4Entry => should_split(p.current.starts_with("GG_Boss_Door_Entrance") && p.current != p.old),
        Split::VengeflyKingP => should_split(p.old.starts_with("GG_Vengefly") && p.current.starts_with("GG_Gruz_Mother")),
        Split::GruzMotherP => should_split(p.old.starts_with("GG_Gruz_Mother") && p.current.starts_with("GG_False_Knight")),
        Split::FalseKnightP => should_split(p.old.starts_with("GG_False_Knight") && p.current.starts_with("GG_Mega_Moss_Charger")),
        Split::MassiveMossChargerP => should_split(p.old.starts_with("GG_Mega_Moss_Charger") && p.current.starts_with("GG_Hornet_1")),
        Split::Hornet1P => should_split(p.old.starts_with("GG_Hornet_1") && starts_with_any(p.current, &["GG_Spa", "GG_Engine"])),
        Split::GodhomeBench => should_split(p.old.starts_with("GG_Spa") && p.current != p.old),
        Split::GorbP => should_split(p.old.starts_with("GG_Ghost_Gorb") && p.current.starts_with("GG_Dung_Defender")),
        Split::DungDefenderP => should_split(p.old.starts_with("GG_Dung_Defender") && p.current.starts_with("GG_Mage_Knight")),
        Split::SoulWarriorP => should_split(p.old.starts_with("GG_Mage_Knight") && p.current.starts_with("GG_Brooding_Mawlek")),
        Split::BroodingMawlekP => should_split(p.old.starts_with("GG_Brooding_Mawlek") && starts_with_any(p.current, &["GG_Engine", "GG_Nailmasters"])),
        Split::GodhomeLoreRoom => should_split(starts_with_any(p.old, GODHOME_LORE_SCENES) && p.current != p.old),
        Split::OroMatoNailBrosP => should_split(p.old.starts_with("GG_Nailmasters") && starts_with_any(p.current, &["GG_End_Sequence", "GG_Spa"])),
        Split::XeroP => should_split(p.old.starts_with("GG_Ghost_Xero") && p.current.starts_with("GG_Crystal_Guardian")),
        Split::CrystalGuardianP => should_split(p.old.starts_with("GG_Crystal_Guardian") && p.current.starts_with("GG_Soul_Master")),
        Split::SoulMasterP => should_split(p.old.starts_with("GG_Soul_Master") && p.current.starts_with("GG_Oblobbles")),
        Split::OblobblesP => should_split(p.old.starts_with("GG_Oblobbles") && p.current.starts_with("GG_Mantis_Lords")),
        Split::MantisLordsP => should_split(p.old.starts_with("GG_Mantis_Lords") && p.current.starts_with("GG_Spa")),
        Split::MarmuP => should_split(p.old.starts_with("GG_Ghost_Marmu") && starts_with_any(p.current, &["GG_Nosk", "GG_Flukemarm"])),
        Split::NoskP => should_split(p.old.starts_with("GG_Nosk") && p.current.starts_with("GG_Flukemarm")),
        Split::FlukemarmP => should_split(p.old.starts_with("GG_Flukemarm") && p.current.starts_with("GG_Broken_Vessel")),
        Split::BrokenVesselP => should_split(p.old.starts_with("GG_Broken_Vessel") && starts_with_any(p.current, &["GG_Engine", "GG_Ghost_Galien"])),
        Split::SheoPaintmasterP => should_split(p.old.starts_with("GG_Painter") && starts_with_any(p.current, &["GG_End_Sequence", "GG_Spa"])),
        Split::HiveKnightP => should_split(p.old.starts_with("GG_Hive_Knight") && p.current.starts_with("GG_Ghost_Hu")),
        Split::ElderHuP => should_split(p.old.starts_with("GG_Ghost_Hu") && p.current.starts_with("GG_Collector")),
        Split::CollectorP => should_split(p.old.starts_with("GG_Collector") && p.current.starts_with("GG_God_Tamer")),
        Split::GodTamerP => should_split(p.old.starts_with("GG_God_Tamer") && p.current.starts_with("GG_Grimm")),
        Split::TroupeMasterGrimmP => should_split(p.old.starts_with("GG_Grimm") && p.current.starts_with("GG_Spa")),
        Split::GalienP => should_split(p.old.starts_with("GG_Ghost_Galien") && starts_with_any(p.current, &["GG_Grey_Prince_Zote", "GG_Painter", "GG_Uumuu"])),
        Split::GreyPrinceZoteP => should_split(p.old.starts_with("GG_Grey_Prince_Zote") && starts_with_any(p.current, &["GG_Uumuu", "GG_Failed_Champion"])),
        Split::UumuuP => should_split(p.old.starts_with("GG_Uumuu") && starts_with_any(p.current, &["GG_Hornet_2", "GG_Nosk_Hornet"])),
        Split::Hornet2P => should_split(p.old.starts_with("GG_Hornet_2") && starts_with_any(p.current, &["GG_Engine", "GG_Spa"])),
        Split::SlyP => should_split(p.old.starts_with("GG_Sly") && starts_with_any(p.current, &["GG_End_Sequence", "GG_Hornet_2"])),
        Split::EnragedGuardianP => should_split(p.old.starts_with("GG_Crystal_Guardian_2") && p.current.starts_with("GG_Lost_Kin")),
        Split::LostKinP => should_split(p.old.starts_with("GG_Lost_Kin") && p.current.starts_with("GG_Ghost_No_Eyes")),
        Split::NoEyesP => should_split(p.old.starts_with("GG_Ghost_No_Eyes") && p.current.starts_with("GG_Traitor_Lord")),
        Split::TraitorLordP => should_split(p.old.starts_with("GG_Traitor_Lord") && p.current.starts_with("GG_White_Defender")),
        Split::WhiteDefenderP => should_split(p.old.starts_with("GG_White_Defender") && p.current.starts_with("GG_Spa")),
        Split::FailedChampionP => should_split(p.old.starts_with("GG_Failed_Champion") && starts_with_any(p.current, &["GG_Ghost_Markoth", "GG_Grimm_Nightmare"])),
        Split::MarkothP => should_split(p.old.starts_with("GG_Ghost_Markoth") && starts_with_any(p.current, &["GG_Watcher_Knights", "GG_Grey_Prince_Zote", "GG_Failed_Champion"])),
        Split::WatcherKnightsP => should_split(p.old.starts_with("GG_Watcher_Knights") && starts_with_any(p.current, &["GG_Soul_Tyrant", "GG_Uumuu"])),
        Split::SoulTyrantP => should_split(p.old.starts_with("GG_Soul_Tyrant") && starts_with_any(p.current, &["GG_Engine_Prime", "GG_Ghost_Markoth"])),
        // Pure Vessel (Pantheon) can transition from PV to either GG_Door_5_Finale for first P4 cutscene, GG_End_Sequence for subsequent P4s, or GG_Radiance in P5
        Split::PureVesselP => should_split(p.old.starts_with("GG_Hollow_Knight") && starts_with_any(p.current, &["GG_End_Sequence", "GG_Radiance", "GG_Door_5_Finale"])),
        Split::Pantheon5Entry => should_split(p.current.starts_with("GG_Vengefly_V") && p.old.starts_with("GG_Atrium_Roof")),
        Split::NoskHornetP => should_split(p.old.starts_with("GG_Nosk_Hornet") && p.current.starts_with("GG_Sly")),
        Split::NightmareKingGrimmP => should_split(p.old.starts_with("GG_Grimm_Nightmare") && p.current.starts_with("GG_Spa")),
        // Absolute Radiance (Pantheon) can transition from AbsRad to either Cinematic_Ending_D for void ending or Cinematic_Ending_E for flower ending
        Split::RadianceP => should_split(p.old.starts_with("GG_Radiance") && p.current.starts_with("Cinematic_Ending")),
        // endregion: Godhome
        // else
        _ => should_split(false)
    }
}

pub fn transition_once_splits(s: &Split, p: &Pair<&str>, prc: &Process, g: &GameManagerFinder, _pds: &mut PlayerDataStore) -> SplitterAction {
    match s {
        // region: Start
        Split::StartNewGame => {
            should_split(starting_kings_pass(p, prc, g)
                         || (is_menu(p.old) && p.current == GG_ENTRANCE_CUTSCENE))
        },
        Split::StartAnyGame => {
            should_split(starting_kings_pass(p, prc, g)
                         || (is_menu(p.old) && (p.current == GG_ENTRANCE_CUTSCENE || is_play_scene(p.current))))
        }
        // endregion: Start
        // else
        _ => should_split(false)
    }
}

pub fn continuous_splits(s: &Split, p: &Process, g: &GameManagerFinder, pds: &mut PlayerDataStore) -> SplitterAction {
    match s {
        Split::RandoWake => should_split(g.disable_pause(p).is_some_and(|d| !d)
                                         && g.get_game_state(p).is_some_and(|s| s == GAME_STATE_PLAYING)
                                         && g.get_scene_name(p).is_some_and(|s| !is_menu(&s))),
        Split::BenchAny => should_split(g.at_bench(p).is_some_and(|b| b)),
        Split::PlayerDeath => should_split(g.get_health(p).is_some_and(|h| h == 0)),
        // region: Dreamers
        Split::Lurien => should_split(g.mask_broken_lurien(p).is_some_and(|b| b)),
        Split::Monomon => should_split(g.mask_broken_monomon(p).is_some_and(|b| b)),
        Split::Hegemol => should_split(g.mask_broken_hegemol(p).is_some_and(|b| b)),
        Split::Dreamer1 => should_split(g.guardians_defeated(p).is_some_and(|d| 1 <= d)),
        Split::Dreamer2 => should_split(g.guardians_defeated(p).is_some_and(|d| 2 <= d)),
        Split::Dreamer3 => should_split(g.guardians_defeated(p).is_some_and(|d| 3 <= d)),
        Split::MenuDreamer3 => { pds.guardians_defeated(p, g); should_split(false) },
        // Old Dreamer Timings, mark deprecated or whatever
        Split::LurienDreamer => should_split(g.lurien_defeated(p).is_some_and(|d| d)),
        Split::MonomonDreamer => should_split(g.monomon_defeated(p).is_some_and(|d| d)),
        Split::HegemolDreamer => should_split(g.hegemol_defeated(p).is_some_and(|d| d)),
        // endregion: Dreamers
        // region: Mr Mushroom
        Split::MrMushroom1 => should_split(g.mr_mushroom_state(p).is_some_and(|s| 2 <= s)),
        Split::MrMushroom2 => should_split(g.mr_mushroom_state(p).is_some_and(|s| 3 <= s)),
        Split::MrMushroom3 => should_split(g.mr_mushroom_state(p).is_some_and(|s| 4 <= s)),
        Split::MrMushroom4 => should_split(g.mr_mushroom_state(p).is_some_and(|s| 5 <= s)),
        Split::MrMushroom5 => should_split(g.mr_mushroom_state(p).is_some_and(|s| 6 <= s)),
        Split::MrMushroom6 => should_split(g.mr_mushroom_state(p).is_some_and(|s| 7 <= s)),
        Split::MrMushroom7 => should_split(g.mr_mushroom_state(p).is_some_and(|s| 8 <= s)),
        // endregion: Mr Mushroom
        // region: Spell Levels
        Split::VengefulSpirit => should_split(g.get_fireball_level(p).is_some_and(|l| 1 <= l)),
        Split::TransVS => { pds.get_fireball_level(p, g); should_split(false) },
        Split::ShadeSoul => should_split(g.get_fireball_level(p).is_some_and(|l| 2 <= l)),
        Split::TransShadeSoul => { pds.get_fireball_level(p, g); should_split(false) },
        Split::MenuShadeSoul => { pds.get_fireball_level(p, g); should_split(false) },
        Split::DesolateDive => should_split(g.get_quake_level(p).is_some_and(|l| 1 <= l)),
        Split::DescendingDark => should_split(g.get_quake_level(p).is_some_and(|l| 2 <= l)),
        Split::TransDescendingDark => { pds.get_quake_level(p, g); should_split(false) },
        Split::HowlingWraiths => should_split(g.get_scream_level(p).is_some_and(|l| 1 <= l)),
        Split::AbyssShriek => should_split(g.get_scream_level(p).is_some_and(|l| 2 <= l)),
        // endregion: Spell Levels
        // region: Movement Abilities
        Split::MothwingCloak => should_split(g.has_dash(p).is_some_and(|d| d)),
        Split::MenuCloak => { pds.has_dash(p, g); should_split(false) },
        Split::ShadeCloak => should_split(g.has_shadow_dash(p).is_some_and(|s| s)),
        Split::MantisClaw => should_split(g.has_wall_jump(p).is_some_and(|w| w)),
        Split::TransClaw => { pds.has_wall_jump(p, g); should_split(false) },
        Split::MenuClaw => { pds.has_wall_jump(p, g); should_split(false) },
        Split::MonarchWings => should_split(g.has_double_jump(p).is_some_and(|w| w)),
        Split::MenuWings => { pds.has_double_jump(p, g); should_split(false) },
        Split::CrystalHeart => should_split(g.has_super_dash(p).is_some_and(|s| s)),
        Split::IsmasTear => should_split(g.has_acid_armour(p).is_some_and(|a| a)),
        Split::TransTear => { pds.has_acid_armour(p, g); should_split(false) },
        Split::MenuIsmasTear => { pds.has_acid_armour(p, g); should_split(false) },
        // endregion: Movement Abilities
        // region: Nail Arts
        Split::CycloneSlash => should_split(g.has_cyclone(p).is_some_and(|s| s)),
        // hasUpwardSlash: secretly means Dash Slash, from Oro
        Split::DashSlash => should_split(g.has_upward_slash(p).is_some_and(|s| s)),
        // hasDashSlash: secretly means Great Slash, from Sheo
        Split::GreatSlash => should_split(g.has_dash_slash(p).is_some_and(|s| s)),
        // endregion: Nail Arts
        // region: Dream Nail Levels
        Split::DreamNail => should_split(g.has_dream_nail(p).is_some_and(|d| d)),
        Split::MenuDreamNail => { pds.has_dream_nail(p, g); should_split(false) },
        Split::DreamGate => should_split(g.has_dream_gate(p).is_some_and(|d| d)),
        Split::MenuDreamGate => { pds.has_dream_gate(p, g); should_split(false) },
        Split::DreamNail2 => should_split(g.dream_nail_upgraded(p).is_some_and(|d| d)),
        // endregion: Dream Nail Levels
        // region: Keys
        Split::CityKey => should_split(g.has_city_key(p).is_some_and(|k| k)),
        Split::LumaflyLantern => should_split(g.has_lantern(p).is_some_and(|l| l)),
        Split::LumaflyLanternTransition => { pds.has_lantern(p, g); should_split(false) },
        Split::SimpleKey => should_split(g.simple_keys(p).is_some_and(|k| 1 <= k)),
        Split::OnObtainSimpleKey => should_split(pds.incremented_simple_keys(p, g)),
        Split::SlyKey => should_split(g.has_sly_key(p).is_some_and(|k| k)),
        Split::ElegantKey => should_split(g.has_white_key(p).is_some_and(|k| k)),
        Split::LoveKey => should_split(g.has_love_key(p).is_some_and(|k| k)),
        Split::PaleLurkerKey => should_split(g.got_lurker_key(p).is_some_and(|k| k)),
        Split::SlySimpleKey => should_split(g.sly_simple_key(p).is_some_and(|k| k)),
        Split::KingsBrand => should_split(g.has_kings_brand(p).is_some_and(|k| k)),
        Split::TramPass => should_split(g.has_tram_pass(p).is_some_and(|k| k)),
        // endregion: Keys
        // region: Nail and Pale Ore
        Split::NailUpgrade1 => should_split(g.nail_smith_upgrades(p).is_some_and(|n| 1 <= n)),
        Split::NailUpgrade2 => should_split(g.nail_smith_upgrades(p).is_some_and(|n| 2 <= n)),
        Split::NailUpgrade3 => should_split(g.nail_smith_upgrades(p).is_some_and(|n| 3 <= n)),
        Split::NailUpgrade4 => should_split(g.nail_smith_upgrades(p).is_some_and(|n| 4 <= n)),
        Split::OnObtainPaleOre => should_split(pds.incremented_ore(p, g)),
        Split::Ore1 => should_split(g.ore_gross(p).is_some_and(|o| 1 <= o)),
        Split::Ore2 => should_split(g.ore_gross(p).is_some_and(|o| 2 <= o)),
        Split::Ore3 => should_split(g.ore_gross(p).is_some_and(|o| 3 <= o)),
        Split::Ore4 => should_split(g.ore_gross(p).is_some_and(|o| 4 <= o)),
        Split::Ore5 => should_split(g.ore_gross(p).is_some_and(|o| 5 <= o)),
        Split::Ore6 => should_split(g.ore_gross(p).is_some_and(|o| 6 <= o)),
        // endregion: Nail and Pale Ore
        // region: Masks and Mask Shards
        Split::OnObtainMaskShard => should_split(pds.obtained_mask_shard(p, g)),
        Split::MaskFragment1 => should_split(g.max_health_base(p).is_some_and(|h| h == 5) && g.heart_pieces(p).is_some_and(|p| p == 1)),
        Split::MaskFragment2 => should_split(g.max_health_base(p).is_some_and(|h| h == 5) && g.heart_pieces(p).is_some_and(|p| p == 2)),
        Split::MaskFragment3 => should_split(g.max_health_base(p).is_some_and(|h| h == 5) && g.heart_pieces(p).is_some_and(|p| p == 3)),
        Split::Mask1 => should_split(g.max_health_base(p).is_some_and(|h| h == 6)),
        Split::MaskFragment5 => should_split(g.heart_pieces(p).is_some_and(|s| s == 5 || (g.max_health_base(p).is_some_and(|h| h == 6) && s == 1))),
        Split::MaskFragment6 => should_split(g.heart_pieces(p).is_some_and(|s| s == 6 || (g.max_health_base(p).is_some_and(|h| h == 6) && s == 2))),
        Split::MaskFragment7 => should_split(g.heart_pieces(p).is_some_and(|s| s == 7 || (g.max_health_base(p).is_some_and(|h| h == 6) && s == 3))),
        Split::Mask2 => should_split(g.max_health_base(p).is_some_and(|h| h == 7)),
        Split::MaskFragment9  => should_split(g.heart_pieces(p).is_some_and(|s| s ==  9 || (g.max_health_base(p).is_some_and(|h| h == 7) && s == 1))),
        Split::MaskFragment10 => should_split(g.heart_pieces(p).is_some_and(|s| s == 10 || (g.max_health_base(p).is_some_and(|h| h == 7) && s == 2))),
        Split::MaskFragment11 => should_split(g.heart_pieces(p).is_some_and(|s| s == 11 || (g.max_health_base(p).is_some_and(|h| h == 7) && s == 3))),
        Split::Mask3 => should_split(g.max_health_base(p).is_some_and(|h| h == 8)),
        Split::MaskFragment13 => should_split(g.heart_pieces(p).is_some_and(|s| s == 13 || (g.max_health_base(p).is_some_and(|h| h == 8) && s == 1))),
        Split::MaskFragment14 => should_split(g.heart_pieces(p).is_some_and(|s| s == 14 || (g.max_health_base(p).is_some_and(|h| h == 8) && s == 2))),
        Split::MaskFragment15 => should_split(g.heart_pieces(p).is_some_and(|s| s == 15 || (g.max_health_base(p).is_some_and(|h| h == 8) && s == 3))),
        Split::Mask4 => should_split(g.max_health_base(p).is_some_and(|h| h == 9)),
        Split::MaskShardMawlek => should_split(g.get_scene_name(p).is_some_and(|s| s == "Crossroads_09") && pds.obtained_mask_shard(p, g)),
        Split::MaskShardGrubfather => should_split(g.get_scene_name(p).is_some_and(|s| s == "Crossroads_38") && pds.obtained_mask_shard(p, g)),
        Split::MaskShardBretta => should_split(g.get_scene_name(p).is_some_and(|s| s == "Room_Bretta") && pds.obtained_mask_shard(p, g)),
        Split::MaskShardQueensStation => should_split(g.get_scene_name(p).is_some_and(|s| s == "Fungus2_01") && pds.obtained_mask_shard(p, g)),
        Split::MaskShardEnragedGuardian => should_split(g.get_scene_name(p).is_some_and(|s| s == "Mines_32") && pds.obtained_mask_shard(p, g)),
        Split::MaskShardSeer => should_split(g.get_scene_name(p).is_some_and(|s| s == "RestingGrounds_07") && pds.obtained_mask_shard(p, g)),
        Split::MaskShardGoam => should_split(g.get_scene_name(p).is_some_and(|s| s == "Crossroads_13") && pds.obtained_mask_shard(p, g)),
        Split::MaskShardStoneSanctuary => should_split(g.get_scene_name(p).is_some_and(|s| s == "Fungus1_36") && pds.obtained_mask_shard(p, g)),
        Split::MaskShardWaterways => should_split(g.get_scene_name(p).is_some_and(|s| s == "Waterways_04b") && pds.obtained_mask_shard(p, g)),
        Split::MaskShardFungalCore => should_split(g.get_scene_name(p).is_some_and(|s| s == "Fungus2_25") && pds.obtained_mask_shard(p, g)),
        Split::MaskShardHive => should_split(g.get_scene_name(p).is_some_and(|s| s == "Hive_04") && pds.obtained_mask_shard(p, g)),
        Split::MaskShardFlower => should_split(g.get_scene_name(p).is_some_and(|s| s == "Room_Mansion") && pds.obtained_mask_shard(p, g)),
        // endregion: Masks and Mask Shards
        // region: Vessels and Vessel Fragments
        Split::OnObtainVesselFragment => should_split(pds.obtained_vessel_fragment(p, g)),
        Split::VesselFragment1 => should_split(g.mp_reserve_max(p).is_some_and(|mp| mp == 0) && g.vessel_fragments(p).is_some_and(|f| f == 1)),
        Split::VesselFragment2 => should_split(g.mp_reserve_max(p).is_some_and(|mp| mp == 0) && g.vessel_fragments(p).is_some_and(|f| f == 2)),
        Split::Vessel1 => should_split(g.mp_reserve_max(p).is_some_and(|mp| mp == 33)),
        Split::VesselFragment4 => should_split(g.vessel_fragments(p).is_some_and(|f| f == 4 || (g.mp_reserve_max(p).is_some_and(|mp| mp == 33) && f == 1))),
        Split::VesselFragment5 => should_split(g.vessel_fragments(p).is_some_and(|f| f == 5 || (g.mp_reserve_max(p).is_some_and(|mp| mp == 33) && f == 2))),
        Split::Vessel2 => should_split(g.mp_reserve_max(p).is_some_and(|mp| mp == 66)),
        Split::VesselFragment7 => should_split(g.vessel_fragments(p).is_some_and(|f| f == 7 || (g.mp_reserve_max(p).is_some_and(|mp| mp == 66) && f == 1))),
        Split::VesselFragment8 => should_split(g.vessel_fragments(p).is_some_and(|f| f == 8 || (g.mp_reserve_max(p).is_some_and(|mp| mp == 66) && f == 2))),
        Split::Vessel3 => should_split(g.mp_reserve_max(p).is_some_and(|mp| mp == 99)),
        Split::VesselFragGreenpath => should_split(g.get_scene_name(p).is_some_and(|s| s == "Fungus1_13") && pds.obtained_vessel_fragment(p, g)),
        Split::VesselFragCrossroadsLift => should_split(g.get_scene_name(p).is_some_and(|s| s == "Crossroads_37") && pds.obtained_vessel_fragment(p, g)),
        Split::VesselFragKingsStation => should_split(g.get_scene_name(p).is_some_and(|s| s == "Ruins2_09") && pds.obtained_vessel_fragment(p, g)),
        Split::VesselFragGarpedes => should_split(g.get_scene_name(p).is_some_and(|s| s == "Deepnest_38") && pds.obtained_vessel_fragment(p, g)),
        Split::VesselFragStagNest => should_split(g.get_scene_name(p).is_some_and(|s| s == "Cliffs_03") && pds.obtained_vessel_fragment(p, g)),
        Split::VesselFragSeer => should_split(g.get_scene_name(p).is_some_and(|s| s == "RestingGrounds_07") && pds.obtained_vessel_fragment(p, g)),
        Split::VesselFragFountain => should_split(g.get_scene_name(p).is_some_and(|s| s == "Abyss_04") && pds.obtained_vessel_fragment(p, g)),
        // endregion: Vessels and Vessel Fragments
        // region: Charm Notches
        Split::NotchShrumalOgres => should_split(g.notch_shroom_ogres(p).is_some_and(|n| n)),
        Split::NotchSalubra1 => should_split(g.salubra_notch1(p).is_some_and(|n| n)),
        Split::NotchSalubra2 => should_split(g.salubra_notch2(p).is_some_and(|n| n)),
        Split::NotchSalubra3 => should_split(g.salubra_notch3(p).is_some_and(|n| n)),
        Split::NotchSalubra4 => should_split(g.salubra_notch4(p).is_some_and(|n| n)),
        Split::NotchFogCanyon => should_split(g.notch_fog_canyon(p).is_some_and(|n| n)),
        Split::NotchGrimm => should_split(g.got_grimm_notch(p).is_some_and(|n| n)),
        Split::OnObtainCharmNotch => should_split(pds.incremented_charm_slots(p, g)),
        // endregion: Charm Notches
        // region: Charms
        Split::GatheringSwarm => should_split(g.got_charm_1(p).is_some_and(|c| c)),
        Split::WaywardCompass => should_split(g.got_charm_2(p).is_some_and(|c| c)),
        Split::Grubsong => should_split(g.got_charm_3(p).is_some_and(|c| c)),
        Split::StalwartShell => should_split(g.got_charm_4(p).is_some_and(|c| c)),
        Split::BaldurShell => should_split(g.got_charm_5(p).is_some_and(|c| c)),
        Split::FuryOfTheFallen => should_split(g.got_charm_6(p).is_some_and(|c| c)),
        Split::QuickFocus => should_split(g.got_charm_7(p).is_some_and(|c| c)),
        Split::LifebloodHeart => should_split(g.got_charm_8(p).is_some_and(|c| c)),
        Split::LifebloodCore => should_split(g.got_charm_9(p).is_some_and(|c| c)),
        Split::DefendersCrest => should_split(g.got_charm_10(p).is_some_and(|c| c)),
        Split::Flukenest => should_split(g.got_charm_11(p).is_some_and(|c| c)),
        Split::ThornsOfAgony => should_split(g.got_charm_12(p).is_some_and(|c| c)),
        Split::MarkOfPride => should_split(g.got_charm_13(p).is_some_and(|c| c)),
        Split::SteadyBody => should_split(g.got_charm_14(p).is_some_and(|c| c)),
        Split::HeavyBlow => should_split(g.got_charm_15(p).is_some_and(|c| c)),
        Split::SharpShadow => should_split(g.got_charm_16(p).is_some_and(|c| c)),
        Split::SporeShroom => should_split(g.got_charm_17(p).is_some_and(|c| c)),
        Split::Longnail => should_split(g.got_charm_18(p).is_some_and(|c| c)),
        Split::ShamanStone => should_split(g.got_charm_19(p).is_some_and(|c| c)),
        Split::SoulCatcher => should_split(g.got_charm_20(p).is_some_and(|c| c)),
        Split::SoulEater => should_split(g.got_charm_21(p).is_some_and(|c| c)),
        Split::GlowingWomb => should_split(g.got_charm_22(p).is_some_and(|c| c)),
        Split::NailmastersGlory => should_split(g.got_charm_26(p).is_some_and(|c| c)),
        Split::JonisBlessing => should_split(g.got_charm_27(p).is_some_and(|c| c)),
        Split::ShapeOfUnn => should_split(g.got_charm_28(p).is_some_and(|c| c)),
        Split::Hiveblood => should_split(g.got_charm_29(p).is_some_and(|c| c)),
        Split::DreamWielder => should_split(g.got_charm_30(p).is_some_and(|c| c)),
        Split::Dashmaster => should_split(g.got_charm_31(p).is_some_and(|c| c)),
        Split::MenuDashmaster => { pds.got_charm_31(p, g); should_split(false) },
        Split::QuickSlash => should_split(g.got_charm_32(p).is_some_and(|c| c)),
        Split::SpellTwister => should_split(g.got_charm_33(p).is_some_and(|c| c)),
        Split::DeepFocus => should_split(g.got_charm_34(p).is_some_and(|c| c)),
        Split::GrubberflysElegy => should_split(g.got_charm_35(p).is_some_and(|c| c)),
        Split::Sprintmaster => should_split(g.got_charm_37(p).is_some_and(|c| c)),
        Split::Dreamshield => should_split(g.got_charm_38(p).is_some_and(|c| c)),
        Split::Weaversong => should_split(g.got_charm_39(p).is_some_and(|c| c)),
        // Fragile / Unbreakable Charms
        Split::FragileHeart => should_split(g.got_charm_23(p).is_some_and(|c| c)),
        Split::UnbreakableHeart => should_split(g.fragile_health_unbreakable(p).is_some_and(|c| c)),
        Split::FragileGreed => should_split(g.got_charm_24(p).is_some_and(|c| c)),
        Split::UnbreakableGreed => should_split(g.fragile_greed_unbreakable(p).is_some_and(|c| c)),
        Split::FragileStrength => should_split(g.got_charm_25(p).is_some_and(|c| c)),
        Split::UnbreakableStrength => should_split(g.fragile_strength_unbreakable(p).is_some_and(|c| c)),
        Split::AllBreakables => should_split(g.broken_charm_23(p).is_some_and(|b| b)
                                             && g.broken_charm_24(p).is_some_and(|b| b)
                                             && g.broken_charm_25(p).is_some_and(|b| b)),
        Split::AllUnbreakables => should_split(g.fragile_greed_unbreakable(p).is_some_and(|u| u)
                                               && g.fragile_health_unbreakable(p).is_some_and(|u| u)
                                               && g.fragile_strength_unbreakable(p).is_some_and(|u| u)),
        // Grimmchild / Carefree Melody
        Split::Grimmchild => should_split(g.got_charm_40(p).is_some_and(|c| c) && g.grimm_child_level(p).is_some_and(|l| l <= 4)),
        Split::Grimmchild2 => should_split(g.grimm_child_level(p).is_some_and(|l| 2 <= l && l <= 4)),
        Split::Grimmchild3 => should_split(g.grimm_child_level(p).is_some_and(|l| 3 <= l && l <= 4)),
        Split::Grimmchild4 => should_split(g.grimm_child_level(p).is_some_and(|l| l == 4)),
        Split::CarefreeMelody => should_split(g.got_charm_40(p).is_some_and(|c| c) && g.grimm_child_level(p).is_some_and(|l| l == 5)),
        Split::Flame1 => should_split(g.flames_collected(p).is_some_and(|f| 1 <= f)),
        Split::Flame2 => should_split(g.flames_collected(p).is_some_and(|f| 2 <= f)),
        Split::Flame3 => should_split(g.flames_collected(p).is_some_and(|f| 3 <= f)),
        Split::BrummFlame => should_split(g.got_brumms_flame(p).is_some_and(|f| f)),
        // Kingsoul / VoidHeart
        Split::WhiteFragmentLeft => should_split(g.got_queen_fragment(p).is_some_and(|c| c)),
        Split::WhiteFragmentRight => should_split(g.got_king_fragment(p).is_some_and(|c| c)),
        Split::OnObtainWhiteFragment => should_split(pds.increased_royal_charm_state(p, g)),
        Split::Kingsoul => should_split(g.charm_cost_36(p).is_some_and(|c| c == 5) && g.royal_charm_state(p).is_some_and(|s| s == 3)),
        Split::VoidHeart => should_split(g.got_shade_charm(p).is_some_and(|c| c)),
        Split::MenuVoidHeart => { pds.got_shade_charm(p, g); should_split(false) },
        // endregion: Charms
        // region: Stags
        Split::RidingStag => should_split(pds.changed_travelling_true(p, g)),
        Split::StagMoved => should_split(pds.changed_stag_position(p, g)),
        Split::CrossroadsStation => should_split(g.opened_crossroads(p).is_some_and(|o| o)),
        Split::GreenpathStation => should_split(g.opened_greenpath(p).is_some_and(|o| o)),
        Split::QueensStationStation => should_split(g.opened_fungal_wastes(p).is_some_and(|o| o)),
        Split::StoreroomsStation => should_split(g.opened_ruins1(p).is_some_and(|o| o)),
        Split::KingsStationStation => should_split(g.opened_ruins2(p).is_some_and(|o| o)),
        Split::RestingGroundsStation => should_split(g.opened_resting_grounds(p).is_some_and(|o| o)),
        Split::HiddenStationStation => should_split(g.opened_hidden_station(p).is_some_and(|o| o)),
        Split::DeepnestStation => should_split(g.opened_deepnest(p).is_some_and(|o| o)),
        Split::QueensGardensStation => should_split(g.opened_royal_gardens(p).is_some_and(|o| o)),
        Split::StagnestStation => should_split(g.get_next_scene_name(p).is_some_and(|n| n == "Cliffs_03")
                                               && g.travelling(p).is_some_and(|t| t)
                                               && g.opened_stag_nest(p).is_some_and(|o| o)),
        // endregion: Stags
        // region: Relics
        Split::OnObtainWanderersJournal => should_split(pds.incremented_trinket1(p, g)),
        Split::AllSeals => should_split(17 <= g.trinket2(p).unwrap_or_default() + g.sold_trinket2(p).unwrap_or_default()),
        Split::OnObtainHallownestSeal => should_split(pds.incremented_trinket2(p, g)),
        Split::SoulSanctumSeal => should_split(pds.incremented_trinket2(p, g) && g.get_scene_name(p).is_some_and(|s| s.starts_with("Ruins1_32"))),
        Split::OnObtainKingsIdol => should_split(pds.incremented_trinket3(p, g)),
        Split::GladeIdol => should_split(pds.incremented_trinket3(p, g) && g.get_scene_name(p).is_some_and(|s| s.starts_with("RestingGrounds_08"))),
        Split::DungDefenderIdol => should_split(pds.incremented_trinket3(p, g) && g.get_scene_name(p).is_some_and(|s| s.starts_with("Waterways_15"))),
        Split::ArcaneEgg8 => should_split(8 <= g.trinket4(p).unwrap_or_default() + g.sold_trinket4(p).unwrap_or_default()),
        Split::OnObtainArcaneEgg => should_split(pds.incremented_trinket4(p, g)),
        Split::OnObtainRancidEgg => should_split(pds.incremented_rancid_eggs(p, g)),
        // endregion: Relics
        // region: Grubs and Mimics
        Split::Grub1 => should_split(g.grubs_collected(p).is_some_and(|g| g == 1)),
        Split::Grub2 => should_split(g.grubs_collected(p).is_some_and(|g| g == 2)),
        Split::Grub3 => should_split(g.grubs_collected(p).is_some_and(|g| g == 3)),
        Split::Grub4 => should_split(g.grubs_collected(p).is_some_and(|g| g == 4)),
        Split::Grub5 => should_split(g.grubs_collected(p).is_some_and(|g| g == 5)),
        Split::Grub6 => should_split(g.grubs_collected(p).is_some_and(|g| g == 6)),
        Split::Grub7 => should_split(g.grubs_collected(p).is_some_and(|g| g == 7)),
        Split::Grub8 => should_split(g.grubs_collected(p).is_some_and(|g| g == 8)),
        Split::Grub9 => should_split(g.grubs_collected(p).is_some_and(|g| g == 9)),
        Split::Grub10 => should_split(g.grubs_collected(p).is_some_and(|g| g == 10)),
        Split::Grub11 => should_split(g.grubs_collected(p).is_some_and(|g| g == 11)),
        Split::Grub12 => should_split(g.grubs_collected(p).is_some_and(|g| g == 12)),
        Split::Grub13 => should_split(g.grubs_collected(p).is_some_and(|g| g == 13)),
        Split::Grub14 => should_split(g.grubs_collected(p).is_some_and(|g| g == 14)),
        Split::Grub15 => should_split(g.grubs_collected(p).is_some_and(|g| g == 15)),
        Split::Grub16 => should_split(g.grubs_collected(p).is_some_and(|g| g == 16)),
        Split::Grub17 => should_split(g.grubs_collected(p).is_some_and(|g| g == 17)),
        Split::Grub18 => should_split(g.grubs_collected(p).is_some_and(|g| g == 18)),
        Split::Grub19 => should_split(g.grubs_collected(p).is_some_and(|g| g == 19)),
        Split::Grub20 => should_split(g.grubs_collected(p).is_some_and(|g| g == 20)),
        Split::Grub21 => should_split(g.grubs_collected(p).is_some_and(|g| g == 21)),
        Split::Grub22 => should_split(g.grubs_collected(p).is_some_and(|g| g == 22)),
        Split::Grub23 => should_split(g.grubs_collected(p).is_some_and(|g| g == 23)),
        Split::Grub24 => should_split(g.grubs_collected(p).is_some_and(|g| g == 24)),
        Split::Grub25 => should_split(g.grubs_collected(p).is_some_and(|g| g == 25)),
        Split::Grub26 => should_split(g.grubs_collected(p).is_some_and(|g| g == 26)),
        Split::Grub27 => should_split(g.grubs_collected(p).is_some_and(|g| g == 27)),
        Split::Grub28 => should_split(g.grubs_collected(p).is_some_and(|g| g == 28)),
        Split::Grub29 => should_split(g.grubs_collected(p).is_some_and(|g| g == 29)),
        Split::Grub30 => should_split(g.grubs_collected(p).is_some_and(|g| g == 30)),
        Split::Grub31 => should_split(g.grubs_collected(p).is_some_and(|g| g == 31)),
        Split::Grub32 => should_split(g.grubs_collected(p).is_some_and(|g| g == 32)),
        Split::Grub33 => should_split(g.grubs_collected(p).is_some_and(|g| g == 33)),
        Split::Grub34 => should_split(g.grubs_collected(p).is_some_and(|g| g == 34)),
        Split::Grub35 => should_split(g.grubs_collected(p).is_some_and(|g| g == 35)),
        Split::Grub36 => should_split(g.grubs_collected(p).is_some_and(|g| g == 36)),
        Split::Grub37 => should_split(g.grubs_collected(p).is_some_and(|g| g == 37)),
        Split::Grub38 => should_split(g.grubs_collected(p).is_some_and(|g| g == 38)),
        Split::Grub39 => should_split(g.grubs_collected(p).is_some_and(|g| g == 39)),
        Split::Grub40 => should_split(g.grubs_collected(p).is_some_and(|g| g == 40)),
        Split::Grub41 => should_split(g.grubs_collected(p).is_some_and(|g| g == 41)),
        Split::Grub42 => should_split(g.grubs_collected(p).is_some_and(|g| g == 42)),
        Split::Grub43 => should_split(g.grubs_collected(p).is_some_and(|g| g == 43)),
        Split::Grub44 => should_split(g.grubs_collected(p).is_some_and(|g| g == 44)),
        Split::Grub45 => should_split(g.grubs_collected(p).is_some_and(|g| g == 45)),
        Split::Grub46 => should_split(g.grubs_collected(p).is_some_and(|g| g == 46)),
        Split::OnObtainGrub => should_split(pds.incremented_grubs_collected(p, g)),
        Split::GrubBasinDive => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Abyss_17")),
        Split::GrubBasinWings => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Abyss_19")),
        Split::GrubCityBelowLoveTower => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Ruins2_07")),
        Split::GrubCityBelowSanctum => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Ruins1_05")),
        Split::GrubCityGuardHouse => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Ruins_House_01")),
        Split::GrubCitySanctum => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Ruins1_32")),
        Split::GrubCitySpire => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Ruins2_03")),
        Split::GrubCliffsBaldurShell => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Fungus1_28")),
        Split::GrubCrossroadsAcid => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Crossroads_35")),
        Split::GrubCrossroadsGuarded => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Crossroads_48")),
        Split::GrubCrossroadsSpikes => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Crossroads_31")),
        Split::GrubCrossroadsVengefly => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Crossroads_05")),
        Split::GrubCrossroadsWall => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Crossroads_03")),
        Split::GrubCrystalPeaksBottomLever => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Mines_04")),
        Split::GrubCrystalPeaksCrown => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Mines_24")),
        Split::GrubCrystalPeaksCrushers => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Mines_19")),
        Split::GrubCrystalPeaksCrystalHeart => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Mines_31")),
        Split::GrubCrystalPeaksMimics => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Mines_16")),
        Split::GrubCrystalPeaksMound => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Mines_35")),
        Split::GrubCrystalPeaksSpikes => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Mines_03")),
        Split::GrubDeepnestBeastsDen => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Deepnest_Spider_Town")),
        Split::GrubDeepnestDark => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Deepnest_39")),
        Split::GrubDeepnestMimics => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Deepnest_36")),
        Split::GrubDeepnestNosk => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Deepnest_31")),
        Split::GrubDeepnestSpikes => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Deepnest_03")),
        Split::GrubFogCanyonArchives => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Fungus3_47")),
        Split::GrubFungalBouncy => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Fungus2_18")),
        Split::GrubFungalSporeShroom => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Fungus2_20")),
        Split::GrubGreenpathCornifer => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Fungus1_06")),
        Split::GrubGreenpathHunter => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Fungus1_07")),
        Split::GrubGreenpathMossKnight => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Fungus1_21")),
        Split::GrubGreenpathVesselFragment => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Fungus1_13")),
        Split::GrubHiveExternal => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Hive_03")),
        Split::GrubHiveInternal => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Hive_04")),
        Split::GrubKingdomsEdgeCenter => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Deepnest_East_11")),
        Split::GrubKingdomsEdgeOro => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Deepnest_East_14")),
        Split::GrubQueensGardensBelowStag => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Fungus3_10")),
        Split::GrubQueensGardensUpper => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Fungus3_22")),
        Split::GrubQueensGardensWhiteLady => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Fungus3_48")),
        Split::GrubRestingGroundsCrypts => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "RestingGrounds_10")),
        Split::GrubWaterwaysCenter => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Waterways_04")),
        Split::GrubWaterwaysHwurmps => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Waterways_14")),
        Split::GrubWaterwaysIsma => should_split(pds.incremented_grubs_collected(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Waterways_13")),
        Split::Mimic1 => should_split(g.kills_grub_mimic(p).is_some_and(|k| k == 4)),
        Split::Mimic2 => should_split(g.kills_grub_mimic(p).is_some_and(|k| k == 3)),
        Split::Mimic3 => should_split(g.kills_grub_mimic(p).is_some_and(|k| k == 2)),
        Split::Mimic4 => should_split(g.kills_grub_mimic(p).is_some_and(|k| k == 1)),
        Split::Mimic5 => should_split(g.kills_grub_mimic(p).is_some_and(|k| k == 0)),
        // endregion: Grubs and Mimics
        // region: Essence, Trees, and Ghosts
        Split::Essence100 => should_split(g.dream_orbs(p).is_some_and(|o| 100 <= o)),
        Split::Essence200 => should_split(g.dream_orbs(p).is_some_and(|o| 200 <= o)),
        Split::Essence300 => should_split(g.dream_orbs(p).is_some_and(|o| 300 <= o)),
        Split::Essence400 => should_split(g.dream_orbs(p).is_some_and(|o| 400 <= o)),
        Split::Essence500 => should_split(g.dream_orbs(p).is_some_and(|o| 500 <= o)),
        Split::Essence600 => should_split(g.dream_orbs(p).is_some_and(|o| 600 <= o)),
        Split::Essence700 => should_split(g.dream_orbs(p).is_some_and(|o| 700 <= o)),
        Split::Essence800 => should_split(g.dream_orbs(p).is_some_and(|o| 800 <= o)),
        Split::Essence900 => should_split(g.dream_orbs(p).is_some_and(|o| 900 <= o)),
        Split::Essence1000 => should_split(g.dream_orbs(p).is_some_and(|o| 1000 <= o)),
        Split::Essence1100 => should_split(g.dream_orbs(p).is_some_and(|o| 1100 <= o)),
        Split::Essence1200 => should_split(g.dream_orbs(p).is_some_and(|o| 1200 <= o)),
        Split::Essence1300 => should_split(g.dream_orbs(p).is_some_and(|o| 1300 <= o)),
        Split::Essence1400 => should_split(g.dream_orbs(p).is_some_and(|o| 1400 <= o)),
        Split::Essence1500 => should_split(g.dream_orbs(p).is_some_and(|o| 1500 <= o)),
        Split::Essence1600 => should_split(g.dream_orbs(p).is_some_and(|o| 1600 <= o)),
        Split::Essence1700 => should_split(g.dream_orbs(p).is_some_and(|o| 1700 <= o)),
        Split::Essence1800 => should_split(g.dream_orbs(p).is_some_and(|o| 1800 <= o)),
        Split::Essence1900 => should_split(g.dream_orbs(p).is_some_and(|o| 1900 <= o)),
        Split::Essence2000 => should_split(g.dream_orbs(p).is_some_and(|o| 2000 <= o)),
        Split::Essence2100 => should_split(g.dream_orbs(p).is_some_and(|o| 2100 <= o)),
        Split::Essence2200 => should_split(g.dream_orbs(p).is_some_and(|o| 2200 <= o)),
        Split::Essence2300 => should_split(g.dream_orbs(p).is_some_and(|o| 2300 <= o)),
        Split::Essence2400 => should_split(g.dream_orbs(p).is_some_and(|o| 2400 <= o)),
        Split::OnObtainGhostMarissa => should_split(pds.incremented_dream_orbs(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Ruins_Bathhouse")),
        Split::OnObtainGhostCaelifFera => should_split(pds.incremented_dream_orbs(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Fungus1_24")),
        Split::OnObtainGhostPoggy => should_split(pds.incremented_dream_orbs(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Ruins_Elevator")),
        Split::OnObtainGhostGravedigger => should_split(pds.incremented_dream_orbs(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Town")),
        Split::OnObtainGhostJoni => should_split(pds.incremented_dream_orbs(p, g) && g.get_scene_name(p).is_some_and(|s| s == "Cliffs_05")),
        // TODO: resolve possible confounding essence sources for Cloth, Vespa, and Revek
        // endregion: Essence, Trees, and Ghosts
        // region: Dirtmouth
        Split::Dirtmouth => should_split(g.visited_dirtmouth(p).is_some_and(|v| v)),
        Split::ElderbugFlower => should_split(g.elderbug_gave_flower(p).is_some_and(|g| g)),
        Split::TroupeMasterGrimm => should_split(g.killed_grimm(p).is_some_and(|k| k)),
        Split::NightmareKingGrimm => should_split(g.killed_nightmare_grimm(p).is_some_and(|k| k)),
        Split::GreyPrince => should_split(g.killed_grey_prince(p).is_some_and(|k| k)),
        Split::GreyPrinceEssence => should_split(g.grey_prince_orbs_collected(p).is_some_and(|o| o)),
        // endregion: Dirtmouth
        // region: Crossroads
        Split::ForgottenCrossroads => should_split(g.visited_crossroads(p).is_some_and(|v| v) && g.get_scene_name(p).is_some_and(|s| s.starts_with("Crossroads_"))),
        Split::InfectedCrossroads => should_split(g.crossroads_infected(p).is_some_and(|i| i) && g.visited_crossroads(p).is_some_and(|v| v)),
        Split::MenderBug => should_split(g.killed_mender_bug(p).is_some_and(|k| k)),
        Split::BroodingMawlek => should_split(g.killed_mawlek(p).is_some_and(|k| k)),
        Split::GruzMother => should_split(g.killed_big_fly(p).is_some_and(|f| f)),
        Split::SlyRescued => should_split(g.sly_rescued(p).is_some_and(|s| s)),
        Split::FalseKnight => should_split(g.killed_false_knight(p).is_some_and(|k| k)),
        Split::FailedKnight => should_split(g.false_knight_dream_defeated(p).is_some_and(|k| k)),
        Split::FailedChampionEssence => should_split(g.false_knight_orbs_collected(p).is_some_and(|o| o)),
        Split::SalubrasBlessing => should_split(g.salubra_blessing(p).is_some_and(|b| b)),
        Split::UnchainedHollowKnight => should_split(g.unchained_hollow_knight(p).is_some_and(|u| u)),
        Split::HollowKnightBoss => should_split(g.killed_hollow_knight(p).is_some_and(|k| k)),
        Split::RadianceBoss => should_split(g.killed_final_boss(p).is_some_and(|k| k)),
        // endregion: Crossroads
        // region: Greenpath
        Split::Greenpath => should_split(g.visited_greenpath(p).is_some_and(|v| v)),
        Split::MossKnight => should_split(g.killed_moss_knight(p).is_some_and(|k| k)),
        Split::Zote1 => should_split(g.zote_rescued_buzzer(p).is_some_and(|z| z)),
        Split::VengeflyKingTrans => { pds.zote_rescued_buzzer(p, g); should_split(false) },
        Split::Hornet1 => should_split(g.killed_hornet(p).is_some_and(|k| k)),
        Split::Aluba => should_split(g.killed_lazy_flyer(p).is_some_and(|k| k)),
        Split::HuntersMark => should_split(g.killed_hunter_mark(p).is_some_and(|k| k)),
        Split::NoEyes => should_split(g.killed_ghost_no_eyes(p).is_some_and(|k| k)),
        Split::NoEyesEssence => should_split(g.no_eyes_defeated(p).is_some_and(|d| d == 2)),
        Split::MegaMossCharger => should_split(g.mega_moss_charger_defeated(p).is_some_and(|k| k)),
        Split::MegaMossChargerTrans => { pds.mega_moss_charger_defeated(p, g); should_split(false) },
        Split::HappyCouplePlayerDataEvent => should_split(g.nailsmith_convo_art(p).is_some_and(|c| c)),
        // endregion: Greenpath
        // region: Fungal
        Split::FungalWastes => should_split(g.visited_fungus(p).is_some_and(|v| v)),
        Split::ElderHu => should_split(g.killed_ghost_hu(p).is_some_and(|k| k)),
        Split::ElderHuEssence => should_split(g.elder_hu_defeated(p).is_some_and(|d| d == 2)),
        Split::ElderHuTrans => { pds.killed_ghost_hu(p, g); should_split(false) },
        Split::BrettaRescued => should_split(g.bretta_rescued(p).is_some_and(|b| b)),
        Split::MantisLords => should_split(g.defeated_mantis_lords(p).is_some_and(|k| k)),
        // endregion: Fungal
        // region: Cliffs
        // TODO: is there a Howling Cliffs Area Text split? should there be?
        //       or would it be better to just have a transition split instead?
        Split::Gorb => should_split(g.killed_ghost_aladar(p).is_some_and(|k| k)),
        Split::GorbEssence => should_split(g.aladar_slug_defeated(p).is_some_and(|d| d == 2)),
        Split::NightmareLantern => should_split(g.nightmare_lantern_lit(p).is_some_and(|l| l)),
        Split::NightmareLanternDestroyed => should_split(g.destroyed_nightmare_lantern(p).is_some_and(|l| l)),
        // endregion: Cliffs
        // region: Resting Grounds
        Split::RestingGrounds => should_split(g.visited_resting_grounds(p).is_some_and(|v| v)),
        Split::BenchRGStag => should_split(g.at_bench(p).is_some_and(|b| b) && g.get_scene_name(p).is_some_and(|s| s == "RestingGrounds_09")),
        Split::Xero => should_split(g.killed_ghost_xero(p).is_some_and(|k| k)),
        Split::XeroEssence => should_split(g.xero_defeated(p).is_some_and(|d| d == 2)),
        Split::SpiritGladeOpen => should_split(g.glade_door_opened(p).is_some_and(|o| o)),
        Split::SeerDeparts => should_split(g.moth_departed(p).is_some_and(|d| d)),
        Split::MetGreyMourner => should_split(g.met_xun(p).is_some_and(|m| m)),
        Split::GreyMournerSeerAscended => should_split(g.met_xun(p).is_some_and(|m| m) && g.moth_departed(p).is_some_and(|d| d)),
        // endregion: Resting Grounds
        // region: City
        Split::CityGateOpen => should_split(g.opened_city_gate(p).is_some_and(|o| o)),
        Split::CityGateAndMantisLords => should_split(g.opened_city_gate(p).is_some_and(|o| o) && g.defeated_mantis_lords(p).is_some_and(|k| k)),
        Split::CityOfTears => should_split(g.visited_ruins(p).is_some_and(|v| v)),
        Split::GorgeousHusk => should_split(pds.killed_gorgeous_husk(p, g)),
        Split::TransGorgeousHusk => { pds.killed_gorgeous_husk(p, g); should_split(false) },
        Split::MenuGorgeousHusk => { pds.killed_gorgeous_husk(p, g); should_split(false) },
        Split::Lemm2 => should_split(g.met_relic_dealer_shop(p).is_some_and(|m| m)),
        Split::TollBenchCity => should_split(g.toll_bench_city(p).is_some_and(|b| b)),
        Split::SoulMasterEncountered => should_split(g.mage_lord_encountered(p).is_some_and(|b| b)),
        Split::SoulMasterPhase1 => should_split(g.mage_lord_encountered_2(p).is_some_and(|b| b)),
        Split::SoulMaster => should_split(g.killed_mage_lord(p).is_some_and(|k| k)),
        Split::SoulTyrant => should_split(g.mage_lord_dream_defeated(p).is_some_and(|k| k)),
        Split::SoulTyrantEssence => should_split(g.mage_lord_orbs_collected(p).is_some_and(|o| o)),
        Split::WatcherChandelier => should_split(g.watcher_chandelier(p).is_some_and(|c| c)),
        Split::BlackKnight => should_split(g.killed_black_knight(p).is_some_and(|k| k)),
        Split::BlackKnightTrans => { pds.killed_black_knight(p, g); should_split(false) },
        Split::Collector => should_split(g.collector_defeated(p).is_some_and(|k| k)),
        Split::TransCollector => { pds.collector_defeated(p, g); should_split(false) },
        Split::NailsmithKilled => should_split(g.nailsmith_killed(p).is_some_and(|k| k)),
        // endregion: City
        // region: Peak
        Split::CrystalPeak => should_split(g.visited_mines(p).is_some_and(|v| v)),
        Split::HuskMiner => should_split(pds.decremented_kills_zombie_miner(p, g)),
        Split::CrystalGuardian1 => should_split(g.defeated_mega_beam_miner(p).is_some_and(|k| k)),
        Split::CrystalGuardian2 => should_split(g.kills_mega_beam_miner(p).is_some_and(|k| k == 0)),
        Split::MineLiftOpened => should_split(g.mine_lift_opened(p).is_some_and(|o| o)),
        // endregion: Peak
        // region: Waterways
        Split::WaterwaysManhole => should_split(g.opened_waterways_manhole(p).is_some_and(|o| o)),
        Split::RoyalWaterways => should_split(g.visited_waterways(p).is_some_and(|v| v)),
        Split::DungDefender => should_split(g.killed_dung_defender(p).is_some_and(|k| k)),
        Split::WhiteDefender => should_split(g.killed_white_defender(p).is_some_and(|k| k)),
        Split::WhiteDefenderEssence => should_split(g.white_defender_orbs_collected(p).is_some_and(|o| o)),
        Split::MetEmilitia => should_split(g.met_emilitia(p).is_some_and(|m| m)),
        Split::Flukemarm => should_split(g.killed_fluke_mother(p).is_some_and(|k| k)),
        // endregion: Waterways
        // region: Basin
        Split::Abyss => should_split(g.visited_abyss(p).is_some_and(|v| v)),
        Split::SavedCloth => should_split(g.saved_cloth(p).is_some_and(|s| s)),
        Split::TollBenchBasin => should_split(g.toll_bench_abyss(p).is_some_and(|b| b)),
        Split::BrokenVessel => should_split(g.killed_infected_knight(p).is_some_and(|k| k)),
        Split::BrokenVesselTrans => { pds.killed_infected_knight(p, g); should_split(false) },
        Split::LostKin => should_split(g.infected_knight_dream_defeated(p).is_some_and(|k| k)),
        Split::LostKinEssence => should_split(g.infected_knight_orbs_collected(p).is_some_and(|o| o)),
        // TODO: should there be a split for the actual Abyss Area Text?
        // endregion: Basin
        // region: White Palace
        Split::WhitePalace => should_split(g.visited_white_palace(p).is_some_and(|v| v)),
        Split::WhitePalaceOrb1 => should_split(g.white_palace_orb_1(p).is_some_and(|o| o)),
        Split::WhitePalaceOrb3 => should_split(g.white_palace_orb_3(p).is_some_and(|o| o)),
        Split::WhitePalaceOrb2 => should_split(g.white_palace_orb_2(p).is_some_and(|o| o)),
        Split::PathOfPain => should_split(g.new_data_binding_seal(p).is_some_and(|n| n)),
        Split::WhitePalaceSecretRoom => should_split(g.white_palace_secret_room_visited(p).is_some_and(|v| v)),
        // endregion: White Palace
        // region: Kingdom's Edge
        Split::KingdomsEdge => should_split(g.visited_outskirts(p).is_some_and(|v| v)),
        Split::Hive => should_split(g.visited_hive(p).is_some_and(|v| v)),
        Split::HiveKnight => should_split(g.killed_hive_knight(p).is_some_and(|k| k)),
        Split::GreatHopper => should_split(g.killed_giant_hopper(p).is_some_and(|k| k)),
        Split::Hornet2 => should_split(g.hornet_outskirts_defeated(p).is_some_and(|k| k)),
        Split::Markoth => should_split(g.killed_ghost_markoth(p).is_some_and(|k| k)),
        Split::MarkothEssence => should_split(g.markoth_defeated(p).is_some_and(|d| d == 2)),
        // endregion: Kingdom's Edge
        // region: Colosseum
        Split::LittleFool => should_split(g.little_fool_met(p).is_some_and(|m| m)),
        Split::ColosseumBronzeUnlocked => should_split(g.colosseum_bronze_opened(p).is_some_and(|o| o)),
        Split::Colosseum => should_split(g.seen_colosseum_title(p).is_some_and(|s| s)),
        Split::ZoteKilled => should_split(g.killed_zote(p).is_some_and(|k| k)),
        Split::ColosseumBronze => should_split(g.colosseum_bronze_completed(p).is_some_and(|c| c)),
        Split::ColosseumBronzeExit => { pds.colosseum_bronze_completed(p, g); should_split(false) },
        Split::ColosseumSilverUnlocked => should_split(g.colosseum_silver_opened(p).is_some_and(|o| o)),
        Split::ColosseumSilver => should_split(g.colosseum_silver_completed(p).is_some_and(|c| c)),
        Split::ColosseumSilverExit => { pds.colosseum_silver_completed(p, g); should_split(false) },
        Split::ColosseumGoldUnlocked => should_split(g.colosseum_gold_opened(p).is_some_and(|o| o)),
        Split::GodTamer => should_split(g.killed_lobster_lancer(p).is_some_and(|k| k)),
        Split::ColosseumGold => should_split(g.colosseum_gold_completed(p).is_some_and(|c| c)),
        Split::ColosseumGoldExit => { pds.colosseum_gold_completed(p, g); should_split(false) },
        // endregion: Colosseum
        // region: Fog Canyon
        Split::FogCanyon => should_split(g.visited_fog_canyon(p).is_some_and(|v| v)),
        Split::UumuuEncountered => should_split(g.encountered_mega_jelly(p).is_some_and(|b| b)),
        Split::Uumuu => should_split(g.killed_mega_jellyfish(p).is_some_and(|k| k)),
        // endregion: Fog Canyon
        // region: Queen's Gardens
        Split::QueensGardens => should_split(g.visited_royal_gardens(p).is_some_and(|v| v)),
        Split::TollBenchQG => should_split(g.toll_bench_queens_gardens(p).is_some_and(|b| b)),
        Split::FlowerQuest => should_split(g.xun_flower_given(p).is_some_and(|g| g)),
        Split::Marmu => should_split(g.killed_ghost_marmu(p).is_some_and(|k| k)),
        Split::MarmuEssence => should_split(g.mum_caterpillar_defeated(p).is_some_and(|d| d == 2)),
        Split::TraitorLord => should_split(g.killed_traitor_lord(p).is_some_and(|k| k)),
        // endregion: Queen's Gardens
        // region: Deepnest
        Split::Deepnest => should_split(g.visited_deepnest(p).is_some_and(|v| v)),
        Split::DeepnestSpa => should_split(g.visited_deepnest_spa(p).is_some_and(|v| v)),
        Split::Zote2 => should_split(g.zote_rescued_deepnest(p).is_some_and(|z| z)),
        Split::TramDeepnest => should_split(g.opened_tram_lower(p).is_some_and(|o| o)),
        Split::Nosk => should_split(g.killed_mimic_spider(p).is_some_and(|k| k)),
        Split::Galien => should_split(g.killed_ghost_galien(p).is_some_and(|k| k)),
        Split::GalienEssence => should_split(g.galien_defeated(p).is_some_and(|d| d == 2)),
        Split::BeastsDenTrapBench => should_split(g.spider_capture(p).is_some_and(|c| c)),
        // endregion: Deepnest
        // region: Godhome
        Split::GodTuner => should_split(g.has_godfinder(p).is_some_and(|g| g)),
        Split::Godhome => should_split(g.visited_godhome(p).is_some_and(|v| v)),
        Split::EternalOrdealUnlocked => should_split(g.zote_statue_wall_broken(p).is_some_and(|b| b)),
        Split::EternalOrdealAchieved => should_split(g.ordeal_achieved(p).is_some_and(|a| a)),
        Split::MatoOroNailBros => should_split(g.killed_nail_bros(p).is_some_and(|k| k)),
        Split::Pantheon1 => should_split(g.boss_door_state_tier1(p).is_some_and(|c| c.completed)),
        Split::SheoPaintmaster => should_split(g.killed_paintmaster(p).is_some_and(|k| k)),
        Split::Pantheon2 => should_split(g.boss_door_state_tier2(p).is_some_and(|c| c.completed)),
        Split::SlyNailsage => should_split(g.killed_nailsage(p).is_some_and(|k| k)),
        Split::Pantheon3 => should_split(g.boss_door_state_tier3(p).is_some_and(|c| c.completed)),
        Split::PureVessel => should_split(g.killed_hollow_knight_prime(p).is_some_and(|k| k)),
        Split::Pantheon4 => should_split(g.boss_door_state_tier4(p).is_some_and(|c| c.completed)),
        Split::Pantheon5 => should_split(g.boss_door_state_tier5(p).is_some_and(|c| c.completed)),
        // endregion: Godhome
        // else
        _ => should_split(false)
    }
}

pub fn splits(s: &Split, prc: &Process, g: &GameManagerFinder, trans_now: bool, ss: &mut SceneStore, pds: &mut PlayerDataStore) -> SplitterAction {
    #[cfg(debug_assertions)]
    pds.get_game_state(prc, g);
    let a1 = continuous_splits(s, prc, g, pds).or_else(|| {
        let pair = ss.pair();
        let a2 = if !ss.split_this_transition {
            transition_once_splits(s, &pair, prc, g, pds)
        } else {
            SplitterAction::Pass
        };
        a2.or_else(|| {
            if trans_now { transition_splits(s, &pair, prc, g, pds) } else { SplitterAction::Pass }
        })
    });
    if a1 != SplitterAction::Pass { ss.split_this_transition = true; }
    a1
}

fn starting_kings_pass(p: &Pair<&str>, prc: &Process, g: &GameManagerFinder) -> bool {
    OPENING_SCENES.contains(&p.old)
    && p.current == "Tutorial_01"
    && g.get_game_state(prc).is_some_and(|gs| {
        gs == GAME_STATE_ENTERING_LEVEL
    })
}

pub fn default_splits() -> Vec<Split> {
    vec![Split::StartNewGame,
         Split::EndingSplit]
}

pub fn auto_reset_safe(s: &[Split]) -> bool {
    s.first() == Some(&Split::StartNewGame)
    && !s[1..].contains(&Split::StartNewGame)
    && !s[0..(s.len()-1)].contains(&Split::EndingSplit)
}
