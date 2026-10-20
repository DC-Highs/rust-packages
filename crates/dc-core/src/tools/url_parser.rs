use regex::Regex;
use std::sync::LazyLock;

use crate::enums::{DragonPhase, DragonSpriteQuality, StaticFileUrlPlatformPrefix};

static PREFIX_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)https?://([a-z]+)-static").unwrap());
static IMAGE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"/(?:basic_|thumb_|ui_)?(\d+)_([\w_]+)").unwrap());
static ID_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"/(?:basic_|thumb_|ui_)?(\d+)_").unwrap());
static PHASE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\d+)@2x\.(?:png|swf)|(\d+)\.(?:png|swf)|(\d+)_HD_tweened_dxt5\.zip|(\d+)_HD_spine-3-8-59_dxt5\.zip").unwrap()
});
static SKIN_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"_skin\d+").unwrap());
static QUALITY_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@\d+x").unwrap());
static DIGITS_UNDERSCORE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"_\d+").unwrap());

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DragonParsedSprite {
    pub platform_prefix: Option<StaticFileUrlPlatformPrefix>,
    pub id: Option<u64>,
    pub image_name: Option<String>,
    pub phase: Option<DragonPhase>,
    pub skin: Option<String>,
    pub image_quality: DragonSpriteQuality,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DragonParsedThumbnail {
    pub platform_prefix: Option<StaticFileUrlPlatformPrefix>,
    pub id: Option<u64>,
    pub image_name: Option<String>,
    pub phase: Option<DragonPhase>,
    pub skin: Option<String>,
}

pub struct DragonStaticFileUrlParser;

impl DragonStaticFileUrlParser {
    pub fn get_platform_prefix(url: &str) -> Option<StaticFileUrlPlatformPrefix> {
        let captures = PREFIX_RE.captures(url)?;
        let prefix = captures.get(1)?.as_str().to_lowercase();
        prefix.parse::<StaticFileUrlPlatformPrefix>().ok()
    }

    pub fn get_image_name(url: &str) -> Option<String> {
        let captures = IMAGE_RE.captures(url)?;
        let id = captures.get(1)?.as_str();
        let rest = captures.get(2)?.as_str();
        let raw = format!("{id}_{rest}");

        let without_digits = DIGITS_UNDERSCORE_RE.replace(&raw, "").to_string();
        let without_skin = SKIN_RE.replace(&without_digits, "").to_string();

        Some(without_skin)
    }

    pub fn get_id(url: &str) -> Option<u64> {
        let captures = ID_RE.captures(url)?;
        captures.get(1)?.as_str().parse::<u64>().ok()
    }

    pub fn get_phase(url: &str) -> Option<DragonPhase> {
        let captures = PHASE_RE.captures(url)?;
        let phase_str = captures
            .get(1)
            .or_else(|| captures.get(2))
            .or_else(|| captures.get(3))
            .or_else(|| captures.get(4))?
            .as_str();

        let num: u8 = phase_str.parse().ok()?;
        match num {
            0 => Some(DragonPhase::Egg),
            1 => Some(DragonPhase::Baby),
            2 => Some(DragonPhase::Young),
            3 => Some(DragonPhase::Adult),
            _ => None,
        }
    }

    pub fn get_skin(url: &str) -> Option<String> {
        let captures = SKIN_RE.captures(url)?;
        Some(captures.get(0)?.as_str().to_string())
    }

    pub fn get_image_quality(url: &str) -> DragonSpriteQuality {
        if QUALITY_RE.is_match(url) {
            DragonSpriteQuality::Large
        } else {
            DragonSpriteQuality::Normal
        }
    }

    pub fn parse_from_sprite(url: &str) -> DragonParsedSprite {
        DragonParsedSprite {
            platform_prefix: Self::get_platform_prefix(url),
            id: Self::get_id(url),
            image_name: Self::get_image_name(url),
            phase: Self::get_phase(url),
            skin: Self::get_skin(url),
            image_quality: Self::get_image_quality(url),
        }
    }

    pub fn parse_from_thumbnail(url: &str) -> DragonParsedThumbnail {
        DragonParsedThumbnail {
            platform_prefix: Self::get_platform_prefix(url),
            id: Self::get_id(url),
            image_name: Self::get_image_name(url),
            phase: Self::get_phase(url),
            skin: Self::get_skin(url),
        }
    }

    pub fn parse_from_flash_animation(url: &str) -> DragonParsedThumbnail {
        Self::parse_from_thumbnail(url)
    }

    pub fn parse_from_spine_animation(url: &str) -> DragonParsedThumbnail {
        Self::parse_from_thumbnail(url)
    }
}
