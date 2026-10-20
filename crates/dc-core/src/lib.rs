//! Shared enums and constants for the DC Highs Dragon City crates.
//!
//! This is a *subset* of the TypeScript `@dchighs/dc-core` package: only what
//! `dc-assets`, `dc-config`, `dc-config-mapper` and `dc-localization` need.
//!
//! This is a fan-made library and is not affiliated with SocialPoint.

use std::fmt;
use std::str::FromStr;

/// Error returned when parsing one of the string enums of this crate fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseEnumError {
    pub enum_name: &'static str,
    pub value: String,
}

impl fmt::Display for ParseEnumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {} value: {:?}", self.enum_name, self.value)
    }
}

impl std::error::Error for ParseEnumError {}

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

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

        impl FromStr for $name {
            type Err = ParseEnumError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $( $value => Ok($name::$variant), )+
                    _ => Err(ParseEnumError { enum_name: stringify!($name), value: s.to_owned() }),
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
        match self {
            DragonElement::Terra => "e",
            DragonElement::Flame => "f",
            DragonElement::Sea => "w",
            DragonElement::Nature => "p",
            DragonElement::Electric => "el",
            DragonElement::Ice => "i",
            DragonElement::Metal => "m",
            DragonElement::Dark => "d",
            DragonElement::Light => "li",
            DragonElement::War => "wr",
            DragonElement::Pure => "pu",
            DragonElement::Legend => "l",
            DragonElement::Primal => "pr",
            DragonElement::Wind => "wd",
            DragonElement::Time => "ti",
            DragonElement::Happy => "hp",
            DragonElement::Chaos => "ch",
            DragonElement::Magic => "mg",
            DragonElement::Soul => "so",
            DragonElement::Beauty => "bt",
            DragonElement::Dream => "dr",
            DragonElement::Physical => "ph",
        }
    }

    /// Resolves an element from its configuration acronym.
    pub fn from_acronym(acronym: &str) -> Option<DragonElement> {
        DragonElement::ALL
            .iter()
            .copied()
            .find(|element| element.acronym() == acronym)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_roundtrip() {
        assert_eq!("tr".parse::<ConfigLanguage>().unwrap(), ConfigLanguage::Turkish);
        assert_eq!(ConfigFilter::DeityWorldDragons.as_str(), "deity_world_dragons");
        assert!("nope".parse::<ConfigFilter>().is_err());
        assert_eq!(SpriteQuality::Large.as_str(), "@2x");
    }

    #[test]
    fn element_acronyms_are_unique_and_reversible() {
        for element in DragonElement::ALL {
            assert_eq!(DragonElement::from_acronym(element.acronym()), Some(*element));
        }
    }

    #[test]
    fn defaults_match_the_typescript_package() {
        assert_eq!(ConfigLanguage::default().as_str(), "en");
        assert_eq!(ConfigPlatform::default().as_str(), "ios");
        assert_eq!(StaticFileUrlPlatformPrefix::default().as_str(), "dci");
        assert_eq!(TextureCompressionFormat::default().as_str(), "dxt5");
    }
}
