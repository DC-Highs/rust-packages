//! Buildings, decorations and habitats: sprites (`ui_<image><quality>.png`) and thumbnails (`<image>.jpg`).

use dc_core::{SpriteQuality, StaticFileUrlPlatformPrefix};

use crate::client::impl_downloader;

macro_rules! sprite_downloader {
    ($(#[$meta:meta])* $name:ident, $dir:literal) => {
        $(#[$meta])*
        #[derive(Debug, Clone)]
        pub struct $name {
            pub image_name: String,
            pub image_quality: SpriteQuality,
            pub platform_prefix: String,
            url: String,
        }

        impl $name {
            pub fn new(image_name: impl Into<String>) -> Self {
                let mut this = Self {
                    image_name: image_name.into(),
                    image_quality: SpriteQuality::default(),
                    platform_prefix: StaticFileUrlPlatformPrefix::default().into(),
                    url: String::new(),
                };
                this.rebuild();
                this
            }

            pub fn quality(mut self, quality: SpriteQuality) -> Self {
                self.image_quality = quality;
                self.rebuild();
                self
            }

            /// Static server prefix (defaults to iOS, `dci`).
            pub fn platform_prefix(mut self, platform_prefix: impl Into<String>) -> Self {
                self.platform_prefix = platform_prefix.into();
                self.rebuild();
                self
            }

            fn rebuild(&mut self) {
                self.url = format!(
                    "https://{}-static-s1.socialpointgames.com/static/dragoncity/mobile/ui/{}/ui_{}{}.png",
                    self.platform_prefix, $dir, self.image_name, self.image_quality
                );
            }
        }

        impl_downloader!($name);
    };
}

macro_rules! thumbnail_downloader {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone)]
        pub struct $name {
            pub image_name: String,
            pub platform_prefix: String,
            url: String,
        }

        impl $name {
            pub fn new(image_name: impl Into<String>) -> Self {
                let mut this = Self {
                    image_name: image_name.into(),
                    platform_prefix: StaticFileUrlPlatformPrefix::default().into(),
                    url: String::new(),
                };
                this.rebuild();
                this
            }

            /// Static server prefix (defaults to iOS, `dci`).
            pub fn platform_prefix(mut self, platform_prefix: impl Into<String>) -> Self {
                self.platform_prefix = platform_prefix.into();
                self.rebuild();
                self
            }

            fn rebuild(&mut self) {
                self.url = format!(
                    "https://{}-static-s1.socialpointgames.com/static/dragoncity/assets/thumbs/{}.jpg",
                    self.platform_prefix, self.image_name
                );
            }
        }

        impl_downloader!($name);
    };
}

sprite_downloader!(
    /// Building sprite.
    BuildingSpriteDownloader,
    "buildings"
);
sprite_downloader!(
    /// Decoration sprite.
    DecorationSpriteDownloader,
    "decorations"
);
sprite_downloader!(
    /// Habitat sprite.
    HabitatSpriteDownloader,
    "habitats"
);

thumbnail_downloader!(
    /// Building thumbnail (`.jpg`).
    BuildingThumbnailDownloader
);
thumbnail_downloader!(
    /// Decoration thumbnail (`.jpg`).
    DecorationThumbnailDownloader
);
thumbnail_downloader!(
    /// Habitat thumbnail (`.jpg`).
    HabitatThumbnailDownloader
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn building_urls() {
        let sprite = BuildingSpriteDownloader::new("10552_hatchery6reskinart_building")
            .quality(SpriteQuality::Large)
            .platform_prefix("dca");
        assert_eq!(
            sprite.url(),
            "https://dca-static-s1.socialpointgames.com/static/dragoncity/mobile/ui/buildings/ui_10552_hatchery6reskinart_building@2x.png"
        );

        assert_eq!(
            BuildingThumbnailDownloader::new("10552_hatchery6reskinart_building").url(),
            "https://dci-static-s1.socialpointgames.com/static/dragoncity/assets/thumbs/10552_hatchery6reskinart_building.jpg"
        );
    }

    #[test]
    fn decoration_and_habitat_dirs() {
        assert!(DecorationSpriteDownloader::new("x").url().contains("/ui/decorations/ui_x.png"));
        assert!(HabitatSpriteDownloader::new("x").url().contains("/ui/habitats/ui_x.png"));
    }
}
