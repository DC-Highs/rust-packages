use dc_core::{IslandType, SpriteQuality};

use crate::client::impl_downloader;
use crate::{AssetSource, Error};

/// Island content package (`.zip`).
#[derive(Debug, Clone)]
pub struct IslandPackageDownloader {
    url: String,
}

impl IslandPackageDownloader {
    /// From an island type (e.g. [`IslandType::HeroicRaces`]) and the package file name.
    pub fn new(island_type: impl AsRef<str>, file_name: impl Into<String>) -> Self {
        let island_type = island_type.as_ref();
        let mut file_name: String = file_name.into();

        if island_type == IslandType::GridIslands.as_str() && !file_name.ends_with("_optim.zip") {
            file_name = file_name.replacen(".zip", "_optim.zip", 1);
        }

        let directory = if island_type == IslandType::HeroicRaces.as_str() {
            "heroicraces_islands"
        } else {
            island_type
        };

        Self {
            url: format!(
                "https://www.socialpointgames.com/static/dragoncity/mobile/ui/{directory}/HD/dxt5/{file_name}"
            ),
        }
    }

    /// From a configuration `zip_file` path such as
    /// `/static/dragoncity/mobile/<island_type>/<file>.zip` (island type is the 4th and
    /// file name the 5th `/`-separated segment).
    pub fn from_path(path: &str) -> Result<Self, Error> {
        let mut segments = path.split('/');
        let island_type = segments.nth(3);
        let file_name = segments.next();

        match (island_type, file_name) {
            (Some(island_type), Some(file_name)) if !island_type.is_empty() && !file_name.is_empty() => {
                Ok(Self::new(island_type, file_name))
            }
            _ => Err(Error::InvalidArguments(format!(
                "cannot extract island type and file name from path {path:?}"
            ))),
        }
    }
}

impl_downloader!(IslandPackageDownloader);

/// Chest sprite. Tries `ui_<image><quality>.png` first and falls back to `ui_basic_<image><quality>.png`.
#[derive(Debug, Clone)]
pub struct ChestSpriteDownloader {
    pub image_name: String,
    pub image_quality: SpriteQuality,
    basic_url: String,
    url: String,
}

impl ChestSpriteDownloader {
    pub fn new(image_name: impl Into<String>) -> Self {
        let mut this = Self {
            image_name: image_name.into(),
            image_quality: SpriteQuality::default(),
            basic_url: String::new(),
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

    /// The fallback URL used when the main one is not available.
    pub fn basic_url(&self) -> &str {
        &self.basic_url
    }

    fn rebuild(&mut self) {
        let base = "https://dci-static-s1.socialpointgames.com/static/dragoncity/mobile/ui/chests";
        self.basic_url = format!("{base}/ui_basic_{}{}.png", self.image_name, self.image_quality);
        self.url = format!("{base}/ui_{}{}.png", self.image_name, self.image_quality);
    }
}

// Not `impl_downloader!`: the chest has a fallback URL.
impl AssetSource for ChestSpriteDownloader {
    fn urls(&self) -> Vec<&str> {
        vec![self.url.as_str(), self.basic_url.as_str()]
    }
}

macro_rules! chest_methods {
    () => {
        impl ChestSpriteDownloader {
            pub fn url(&self) -> &str {
                &self.url
            }

            pub async fn fetch(&self) -> Result<::bytes::Bytes, Error> {
                self.fetch_with(crate::default_client()).await
            }

            pub async fn fetch_with(&self, client: &::reqwest::Client) -> Result<::bytes::Bytes, Error> {
                crate::client::fetch_first(client, &self.urls()).await
            }

            pub async fn download(
                &self,
                path: impl AsRef<::std::path::Path>,
            ) -> Result<::std::path::PathBuf, Error> {
                self.download_with(crate::default_client(), path).await
            }

            pub async fn download_with(
                &self,
                client: &::reqwest::Client,
                path: impl AsRef<::std::path::Path>,
            ) -> Result<::std::path::PathBuf, Error> {
                crate::client::download_first(client, &self.urls(), path.as_ref()).await
            }
        }
    };
}
chest_methods!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn island_package_urls() {
        assert_eq!(
            IslandPackageDownloader::new("heroic_races", "a.zip").url(),
            "https://www.socialpointgames.com/static/dragoncity/mobile/ui/heroicraces_islands/HD/dxt5/a.zip"
        );
        assert!(IslandPackageDownloader::new("grid_islands", "a.zip").url().ends_with("/a_optim.zip"));
        assert!(IslandPackageDownloader::new("grid_islands", "a_optim.zip").url().ends_with("/a_optim.zip"));
        assert!(IslandPackageDownloader::new("maze_islands", "a.zip").url().contains("/maze_islands/"));
    }

    #[test]
    fn island_package_from_path() {
        let d = IslandPackageDownloader::from_path("/static/dragoncity/maze_islands/file.zip");
        // segments: "", "static", "dragoncity", "maze_islands", "file.zip"
        assert!(d.unwrap().url().ends_with("/maze_islands/HD/dxt5/file.zip"));
        assert!(IslandPackageDownloader::from_path("short").is_err());
    }

    #[test]
    fn chest_has_a_fallback() {
        let chest = ChestSpriteDownloader::new("abc").quality(SpriteQuality::Large);
        assert!(chest.url().ends_with("/chests/ui_abc@2x.png"));
        assert!(chest.basic_url().ends_with("/chests/ui_basic_abc@2x.png"));
        assert_eq!(chest.urls().len(), 2);
    }
}
