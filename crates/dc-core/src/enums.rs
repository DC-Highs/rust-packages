use std::fmt;


/// Error returned when parsing one of the string enums of this crate fails.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid {enum_name} value: {value:?}")]
pub struct ParseEnumError {
    pub enum_name: &'static str,
    pub value: String,
}

#[macro_export]
macro_rules! string_enum {
    (
        $(#[$meta:meta])*
        $name:ident {
            $( $(#[$vmeta:meta])* $variant:ident => $value:literal ),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        pub enum $name {
            $( $(#[$vmeta])* #[serde(rename = $value)] $variant ),+
        }

        impl $name {
            /// Every variant, in declaration order.
            pub const ALL: &'static [$name] = &[$( $name::$variant ),+];

            /// The wire value used by the game servers.
            pub const fn as_str(&self) -> &'static str {
                match self { $( $name::$variant => $value ),+ }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> String {
                value.as_str().to_owned()
            }
        }

        impl std::str::FromStr for $name {
            type Err = $crate::enums::ParseEnumError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $( $value => Ok($name::$variant), )+
                    _ => Err($crate::enums::ParseEnumError { enum_name: stringify!($name), value: s.to_owned() }),
                }
            }
        }
    };
}

macro_rules! default_variant {
    ($name:ident, $variant:ident) => {
        impl Default for $name {
            fn default() -> Self {
                $name::$variant
            }
        }
    };
}

string_enum! {
    /// Sections of the game configuration that can be requested/filtered.
    ConfigFilter {
        Chests => "chests",
        Items => "items",
        Levels => "levels",
        BreedOposite => "breed_oposite",
        TreasureItems => "treasure_items",
        SanctuaryBreeding => "sanctuary_breeding",
        HelpViews => "help_views",
        DeityWorldDragons => "deity_world_dragons",
        ProgressiveIsland => "progressive_island",
        News => "news",
        Dragonarium => "dragonarium",
        GridIsland => "grid_island",
        DragonBook => "dragon_book",
        DragonTournaments => "dragon_tournaments",
        FogIsland => "fog_island",
        HeroicRaces => "heroic_races",
        SpecialBreeding => "special_breeding",
        MazeIsland => "maze_island",
        TowerIsland => "tower_island",
        DragonCollections => "dragon_collections",
        EventIsland => "event_island",
        Skills => "skills",
        TreeOfLife => "tree_of_life",
        TreeOfLifeRecall => "tree_of_life_recall",
        TreeOfLifePowerup => "tree_of_life_powerup",
        HabitatTokens => "habitat_tokens",
        MaxDragonLevels => "max_dragon_levels",
        Collectibles => "collectibles",
        PuzzleIsland => "puzzle_island",
        VipPass => "vip_pass",
        DragonSkins => "dragon_skins",
        DragonWars => "dragon_wars",
        FarmCrops => "farm_crops",
        Breeding => "breeding",
        LiveopsChallenges => "liveops_challenges",
        AssetVersioning => "asset_versioning",
        BattlePass => "battle_pass",
        DragonMastery => "dragon_mastery",
        RunnerIsland => "runner_island",
        Perks => "perks",
    }
}

string_enum! {
    /// Languages supported by the game configuration and localization files.
    ConfigLanguage {
        English => "en",
        Portuguese => "pt",
        BrazilianPortuguese => "br",
        Spanish => "es",
        French => "fr",
        German => "de",
        Italian => "it",
        Japanese => "ja",
        Russian => "ru",
        Korean => "ko",
        Turkish => "tr",
    }
}
default_variant!(ConfigLanguage, English);

string_enum! {
    /// Platforms the game configuration can be requested for.
    ConfigPlatform {
        IOS => "ios",
        Android => "android",
        Windows => "windows",
        Amazon => "amazon",
    }
}
default_variant!(ConfigPlatform, IOS);

string_enum! {
    /// Sub-domain prefix of the static file servers (`{prefix}-static-s1.socialpointgames.com`).
    StaticFileUrlPlatformPrefix {
        IOS => "dci",
        Android => "dca",
        Windows => "dcw",
        Amazon => "dcaz",
    }
}
default_variant!(StaticFileUrlPlatformPrefix, IOS);

string_enum! {
    /// Texture compression format of the packaged animations/islands.
    TextureCompressionFormat {
        DXT5 => "dxt5",
        PVR => "pvr",
        ETC1 => "etc1",
        ATITC => "atitc",
    }
}
default_variant!(TextureCompressionFormat, DXT5);

string_enum! {
    /// Sprite quality suffix used in the file names of the static servers.
    ///
    /// Shared by dragons, buildings, decorations, habitats and chests.
    SpriteQuality {
        Normal => "",
        Large => "@2x",
    }
}
default_variant!(SpriteQuality, Normal);

pub type DragonSpriteQuality = SpriteQuality;
pub type BuildingSpriteQuality = SpriteQuality;
pub type DecorationSpriteQuality = SpriteQuality;
pub type HabitatSpriteQuality = SpriteQuality;
pub type ChestSpriteQuality = SpriteQuality;

string_enum! {
    /// Kinds of event islands.
    IslandType {
        FogIslands => "fog_islands",
        GridIslands => "grid_islands",
        MazeIslands => "maze_islands",
        HeroicRaces => "heroic_races",
        TowerIslands => "tower_islands",
        PuzzleIslands => "puzzle_islands",
        RunnerIslands => "runner_islands",
        ProgressiveIslands => "progressive_islands",
    }
}

string_enum! {
    /// Dragon elements.
    DragonElement {
        Terra => "terra",
        Flame => "flame",
        Sea => "sea",
        Nature => "nature",
        Electric => "electric",
        Ice => "ice",
        Metal => "metal",
        Dark => "dark",
        Light => "light",
        War => "war",
        Pure => "pure",
        Legend => "legend",
        Primal => "primal",
        Wind => "wind",
        Time => "time",
        Happy => "happy",
        Chaos => "chaos",
        Magic => "magic",
        Soul => "soul",
        Beauty => "beauty",
        Dream => "dream",
        Physical => "physical",
    }
}

impl DragonElement {
    /// Short code used by the game configuration (e.g. `"el"` for electric).
    pub const fn acronym(&self) -> &'static str {
        crate::settings::elements::element_info(*self).acronym
    }

    /// Resolves an element from its configuration acronym.
    pub fn from_acronym(acronym: &str) -> Option<DragonElement> {
        DragonElement::ALL
            .iter()
            .copied()
            .find(|element| element.acronym() == acronym)
    }

    pub const fn strengths(&self) -> &'static [DragonElement] {
        crate::settings::elements::element_info(*self).strengths
    }

    pub const fn weaknesses(&self) -> &'static [DragonElement] {
        crate::settings::elements::element_info(*self).weaknesses
    }
}

/// Growth phase of a dragon. Its numeric value is used in asset file names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum DragonPhase {
    Egg = 0,
    Baby = 1,
    Young = 2,
    Adult = 3,
}

impl From<DragonPhase> for u32 {
    fn from(phase: DragonPhase) -> u32 {
        phase as u32
    }
}

impl fmt::Display for DragonPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

/// Dragon category / tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum DragonCategory {
    Basic = 1,
    ProBasic = 2,
    PreAdvanced = 3,
    Advanced = 4,
    Elite = 5,
    Amazing = 6,
    Charming = 7,
    Graceful = 8,
    Champion = 9,
    Mythical = 10,
    Heroic = 11,
}

impl DragonCategory {
    pub const ALL: &'static [DragonCategory] = &[
        DragonCategory::Basic,
        DragonCategory::ProBasic,
        DragonCategory::PreAdvanced,
        DragonCategory::Advanced,
        DragonCategory::Elite,
        DragonCategory::Amazing,
        DragonCategory::Charming,
        DragonCategory::Graceful,
        DragonCategory::Champion,
        DragonCategory::Mythical,
        DragonCategory::Heroic,
    ];

    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub const fn from_u8(value: u8) -> Option<DragonCategory> {
        match value {
            1 => Some(DragonCategory::Basic),
            2 => Some(DragonCategory::ProBasic),
            3 => Some(DragonCategory::PreAdvanced),
            4 => Some(DragonCategory::Advanced),
            5 => Some(DragonCategory::Elite),
            6 => Some(DragonCategory::Amazing),
            7 => Some(DragonCategory::Charming),
            8 => Some(DragonCategory::Graceful),
            9 => Some(DragonCategory::Champion),
            10 => Some(DragonCategory::Mythical),
            11 => Some(DragonCategory::Heroic),
            _ => None,
        }
    }
}

impl fmt::Display for DragonCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Dragon battle ranks (Bronze III up to Platinum I).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum DragonRank {
    BronzeIII = 1,
    BronzeII = 2,
    BronzeI = 3,
    SilverIII = 4,
    SilverII = 5,
    SilverI = 6,
    GoldIII = 7,
    GoldII = 8,
    GoldI = 9,
    PlatinumIII = 10,
    PlatinumII = 11,
    PlatinumI = 12,
}

impl DragonRank {
    pub const ALL: &'static [DragonRank] = &[
        DragonRank::BronzeIII,
        DragonRank::BronzeII,
        DragonRank::BronzeI,
        DragonRank::SilverIII,
        DragonRank::SilverII,
        DragonRank::SilverI,
        DragonRank::GoldIII,
        DragonRank::GoldII,
        DragonRank::GoldI,
        DragonRank::PlatinumIII,
        DragonRank::PlatinumII,
        DragonRank::PlatinumI,
    ];

    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub const fn from_u8(value: u8) -> Option<DragonRank> {
        match value {
            1 => Some(DragonRank::BronzeIII),
            2 => Some(DragonRank::BronzeII),
            3 => Some(DragonRank::BronzeI),
            4 => Some(DragonRank::SilverIII),
            5 => Some(DragonRank::SilverII),
            6 => Some(DragonRank::SilverI),
            7 => Some(DragonRank::GoldIII),
            8 => Some(DragonRank::GoldII),
            9 => Some(DragonRank::GoldI),
            10 => Some(DragonRank::PlatinumIII),
            11 => Some(DragonRank::PlatinumII),
            12 => Some(DragonRank::PlatinumI),
            _ => None,
        }
    }

    pub const fn name_key(&self) -> &'static str {
        crate::settings::dragons::rank_name_key(*self)
    }
}

impl fmt::Display for DragonRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

string_enum! {
    /// Dragon rarity (Common through Heroic).
    DragonRarity {
        Common => "C",
        Rare => "R",
        VeryRare => "V",
        Epic => "E",
        Legendary => "L",
        Mythic => "M",
        Heroic => "H",
    }
}
