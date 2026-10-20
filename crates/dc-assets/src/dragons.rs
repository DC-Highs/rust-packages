use dc_core::{SpriteQuality, StaticFileUrlPlatformPrefix, TextureCompressionFormat};

use crate::client::impl_downloader;

fn base(platform_prefix: &str) -> String {
    format!("https://{platform_prefix}-static-s1.socialpointgames.com/static/dragoncity")
}

macro_rules! dragon_common_setters {
    () => {
        /// Skin suffix, e.g. `"_1"`.
        pub fn skin(mut self, skin: impl Into<String>) -> Self {
            self.skin = Some(skin.into());
            self.rebuild();
            self
        }

        /// Static server prefix (defaults to iOS, `dci`).
        pub fn platform_prefix(mut self, platform_prefix: impl Into<String>) -> Self {
            self.platform_prefix = platform_prefix.into();
            self.rebuild();
            self
        }
    };
}

/// Dragon sprite (`ui_<image><skin>_<phase><quality>.png`).
#[derive(Debug, Clone)]
pub struct DragonSpriteDownloader {
    pub image_name: String,
    pub phase: u32,
    pub skin: Option<String>,
    pub image_quality: String,
    pub platform_prefix: String,
    url: String,
}

impl DragonSpriteDownloader {
    pub fn new(image_name: impl Into<String>, phase: impl Into<u32>) -> Self {
        let mut this = Self {
            image_name: image_name.into(),
            phase: phase.into(),
            skin: None,
            image_quality: SpriteQuality::default().into(),
            platform_prefix: StaticFileUrlPlatformPrefix::default().into(),
            url: String::new(),
        };
        this.rebuild();
        this
    }

    dragon_common_setters!();

    pub fn quality(mut self, quality: impl Into<String>) -> Self {
        self.image_quality = quality.into();
        self.rebuild();
        self
    }

    fn rebuild(&mut self) {
        self.url = format!(
            "{}/mobile/ui/dragons/ui_{}{}_{}{}.png",
            base(&self.platform_prefix),
            self.image_name,
            self.skin.as_deref().unwrap_or(""),
            self.phase,
            self.image_quality
        );
    }
}

impl_downloader!(DragonSpriteDownloader);

/// Dragon thumbnail (`thumb_<image><skin>_<phase>.png`).
#[derive(Debug, Clone)]
pub struct DragonThumbnailDownloader {
    pub image_name: String,
    pub phase: u32,
    pub skin: Option<String>,
    pub platform_prefix: String,
    url: String,
}

impl DragonThumbnailDownloader {
    pub fn new(image_name: impl Into<String>, phase: impl Into<u32>) -> Self {
        let mut this = Self {
            image_name: image_name.into(),
            phase: phase.into(),
            skin: None,
            platform_prefix: StaticFileUrlPlatformPrefix::default().into(),
            url: String::new(),
        };
        this.rebuild();
        this
    }

    dragon_common_setters!();

    fn rebuild(&mut self) {
        self.url = format!(
            "{}/mobile/ui/dragons/HD/thumb_{}{}_{}.png",
            base(&self.platform_prefix),
            self.image_name,
            self.skin.as_deref().unwrap_or(""),
            self.phase
        );
    }
}

impl_downloader!(DragonThumbnailDownloader);

/// Flash animation (`.swf`).
#[derive(Debug, Clone)]
pub struct DragonFlashAnimationDownloader {
    pub image_name: String,
    pub phase: u32,
    pub skin: Option<String>,
    pub platform_prefix: String,
    url: String,
}

impl DragonFlashAnimationDownloader {
    pub fn new(image_name: impl Into<String>, phase: impl Into<u32>) -> Self {
        let mut this = Self {
            image_name: image_name.into(),
            phase: phase.into(),
            skin: None,
            platform_prefix: StaticFileUrlPlatformPrefix::default().into(),
            url: String::new(),
        };
        this.rebuild();
        this
    }

    dragon_common_setters!();

    fn rebuild(&mut self) {
        self.url = format!(
            "{}/assets/sprites/{}{}_{}.swf",
            base(&self.platform_prefix),
            self.image_name,
            self.skin.as_deref().unwrap_or(""),
            self.phase
        );
    }
}

impl_downloader!(DragonFlashAnimationDownloader);

/// Spine animation package (`.zip`).
#[derive(Debug, Clone)]
pub struct DragonSpineAnimationDownloader {
    pub image_name: String,
    pub phase: u32,
    pub skin: Option<String>,
    pub texture_compression_format: String,
    pub platform_prefix: String,
    pub use_new_url_format: bool,
    url: String,
}

impl DragonSpineAnimationDownloader {
    pub fn new(image_name: impl Into<String>, phase: impl Into<u32>) -> Self {
        let mut this = Self {
            image_name: image_name.into(),
            phase: phase.into(),
            skin: None,
            texture_compression_format: TextureCompressionFormat::default().into(),
            platform_prefix: StaticFileUrlPlatformPrefix::default().into(),
            use_new_url_format: true,
            url: String::new(),
        };
        this.rebuild();
        this
    }

    dragon_common_setters!();

    pub fn texture_compression_format(mut self, format: impl Into<String>) -> Self {
        self.texture_compression_format = format.into();
        self.rebuild();
        self
    }

    /// `true` (default) uses the `_HD_tweened_` package, `false` the legacy `basic_..._spine-3-8-59` one.
    pub fn use_new_url_format(mut self, use_new_url_format: bool) -> Self {
        self.use_new_url_format = use_new_url_format;
        self.rebuild();
        self
    }

    fn rebuild(&mut self) {
        let name = format!(
            "{}{}_{}",
            self.image_name,
            self.skin.as_deref().unwrap_or(""),
            self.phase
        );
        let dir = format!("{}/mobile/engine/version_1_1/dragons/{name}", base(&self.platform_prefix));

        self.url = if self.use_new_url_format {
            format!("{dir}/{name}_HD_tweened_{}.zip", self.texture_compression_format)
        } else {
            format!("{dir}/basic_{name}_HD_spine-3-8-59_{}.zip", self.texture_compression_format)
        };
    }
}

impl_downloader!(DragonSpineAnimationDownloader);

#[cfg(test)]
mod tests {
    use super::*;
    use dc_core::DragonPhase;

    #[test]
    fn sprite_url_matches_the_typescript_package() {
        let sprite = DragonSpriteDownloader::new("1000_dragon_nature", DragonPhase::Adult)
            .skin("_1")
            .quality(SpriteQuality::Large);

        assert_eq!(
            sprite.url(),
            "https://dci-static-s1.socialpointgames.com/static/dragoncity/mobile/ui/dragons/ui_1000_dragon_nature_1_3@2x.png"
        );
    }

    #[test]
    fn spine_urls() {
        let new = DragonSpineAnimationDownloader::new("1000_dragon_nature", DragonPhase::Baby);
        assert!(new.url().ends_with("1000_dragon_nature_1/1000_dragon_nature_1_HD_tweened_dxt5.zip"));

        let old = new.use_new_url_format(false).platform_prefix("dca");
        assert!(old.url().starts_with("https://dca-static-s1."));
        assert!(old.url().ends_with("basic_1000_dragon_nature_1_HD_spine-3-8-59_dxt5.zip"));
    }

    #[test]
    fn thumbnail_and_flash_urls() {
        assert!(DragonThumbnailDownloader::new("a", DragonPhase::Egg)
            .url()
            .ends_with("/mobile/ui/dragons/HD/thumb_a_0.png"));
        assert!(DragonFlashAnimationDownloader::new("a", 2u32)
            .url()
            .ends_with("/assets/sprites/a_2.swf"));
    }
}
