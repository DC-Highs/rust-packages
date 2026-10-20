use dc_core::StaticFileUrlPlatformPrefix;

use crate::client::impl_downloader;

/// Music track (`.mp3`).
#[derive(Debug, Clone)]
pub struct MusicDownloader {
    pub key_name: String,
    pub platform_prefix: String,
    url: String,
}

impl MusicDownloader {
    pub fn new(key_name: impl Into<String>) -> Self {
        let mut this = Self {
            key_name: key_name.into(),
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
        // The original server path is plain HTTP; kept as is.
        self.url = format!(
            "http://{}-static-s1.socialpointgames.com/static/dragoncity/mobile/sounds/music/{}.mp3",
            self.platform_prefix, self.key_name
        );
    }
}

impl_downloader!(MusicDownloader);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn music_url() {
        assert_eq!(
            MusicDownloader::new("531_dc_party_planning_island").url(),
            "http://dci-static-s1.socialpointgames.com/static/dragoncity/mobile/sounds/music/531_dc_party_planning_island.mp3"
        );
    }
}
