use std::fmt;

use dc_core::{ConfigFilter, ConfigLanguage, ConfigPlatform};
use reqwest::Client;
use serde_json::Map;

use crate::dto::{GameConfig, GameData, GameDataConfig};
use crate::Error;

/// Builds a [`Config`] from data you already have.
#[derive(Debug, Clone)]
pub struct ConfigOptions {
    pub language: ConfigLanguage,
    pub platform: ConfigPlatform,
    pub filter: Option<Vec<ConfigFilter>>,
    pub data: GameConfig,
}

/// Fetch the configuration from the authenticated endpoint.
#[derive(Clone)]
pub struct FetchOptions {
    pub url: String,
    pub user_id: String,
    pub auth_token: String,
    pub language: Option<ConfigLanguage>,
    pub platform: Option<ConfigPlatform>,
    pub filter: Option<Vec<ConfigFilter>>,
}

impl FetchOptions {
    pub fn new(
        url: impl Into<String>,
        user_id: impl Into<String>,
        auth_token: impl Into<String>,
    ) -> Self {
        Self {
            url: url.into(),
            user_id: user_id.into(),
            auth_token: auth_token.into(),
            language: None,
            platform: None,
            filter: None,
        }
    }

    pub fn language(mut self, language: ConfigLanguage) -> Self {
        self.language = Some(language);
        self
    }

    pub fn platform(mut self, platform: ConfigPlatform) -> Self {
        self.platform = Some(platform);
        self
    }

    pub fn filter(mut self, filter: impl IntoIterator<Item = ConfigFilter>) -> Self {
        self.filter = Some(filter.into_iter().collect());
        self
    }
}

// The auth token must never end up in logs.
impl fmt::Debug for FetchOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FetchOptions")
            .field("url", &self.url)
            .field("user_id", &self.user_id)
            .field("auth_token", &"<redacted>")
            .field("language", &self.language)
            .field("platform", &self.platform)
            .field("filter", &self.filter)
            .finish()
    }
}

pub type CreateOptions = FetchOptions;

/// Fetch a raw (public, unauthenticated) configuration file.
#[derive(Debug, Clone)]
pub struct FetchRawOptions {
    pub url: String,
    pub filter: Option<Vec<ConfigFilter>>,
}

impl FetchRawOptions {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            filter: None,
        }
    }

    pub fn filter(mut self, filter: impl IntoIterator<Item = ConfigFilter>) -> Self {
        self.filter = Some(filter.into_iter().collect());
        self
    }
}

#[derive(Debug, Clone)]
pub struct CreateRawOptions {
    pub url: String,
    pub filter: Option<Vec<ConfigFilter>>,
    pub language: Option<ConfigLanguage>,
    pub platform: Option<ConfigPlatform>,
}

impl CreateRawOptions {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            filter: None,
            language: None,
            platform: None,
        }
    }

    pub fn filter(mut self, filter: impl IntoIterator<Item = ConfigFilter>) -> Self {
        self.filter = Some(filter.into_iter().collect());
        self
    }

    pub fn language(mut self, language: ConfigLanguage) -> Self {
        self.language = Some(language);
        self
    }

    pub fn platform(mut self, platform: ConfigPlatform) -> Self {
        self.platform = Some(platform);
        self
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub platform: ConfigPlatform,
    pub language: ConfigLanguage,
    pub filter: Option<Vec<ConfigFilter>>,
    pub data: GameConfig,
}

impl Config {
    pub fn new(options: ConfigOptions) -> Self {
        Self {
            platform: options.platform,
            language: options.language,
            filter: options.filter,
            data: options.data,
        }
    }

    /// Fetches the configuration from the authenticated endpoint and wraps it.
    pub async fn create(options: CreateOptions) -> Result<Self, Error> {
        Self::create_with(&Client::new(), options).await
    }

    pub async fn create_with(client: &Client, options: CreateOptions) -> Result<Self, Error> {
        let data = Self::fetch_with(client, &options).await?;

        Ok(Self {
            data,
            filter: options.filter,
            language: options.language.unwrap_or_default(),
            platform: options.platform.unwrap_or_default(),
        })
    }

    /// Fetches a raw configuration file from a URL and wraps it.
    pub async fn create_raw(options: CreateRawOptions) -> Result<Self, Error> {
        Self::create_raw_with(&Client::new(), options).await
    }

    pub async fn create_raw_with(client: &Client, options: CreateRawOptions) -> Result<Self, Error> {
        let data = Self::fetch_raw_with(
            client,
            &FetchRawOptions {
                url: options.url,
                filter: options.filter.clone(),
            },
        )
        .await?;

        Ok(Self {
            data,
            filter: options.filter,
            language: options.language.unwrap_or_default(),
            platform: options.platform.unwrap_or_default(),
        })
    }

    /// Fetches the configuration from the authenticated endpoint (`POST url?authToken=..&userId=..`).
    pub async fn fetch(options: &FetchOptions) -> Result<GameConfig, Error> {
        Self::fetch_with(&Client::new(), options).await
    }

    pub async fn fetch_with(client: &Client, options: &FetchOptions) -> Result<GameConfig, Error> {
        let mut params: Vec<(&str, String)> = vec![
            ("authToken", options.auth_token.clone()),
            ("userId", options.user_id.clone()),
        ];

        if let Some(filter) = &options.filter {
            params.push(("filter", join_filter(filter)));
        }

        if let Some(platform) = options.platform {
            params.push(("platform", platform.as_str().to_owned()));
        }

        if let Some(language) = options.language {
            params.push(("language", language.as_str().to_owned()));
        }

        let response = client
            .post(&options.url)
            .query(&params)
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json().await?)
    }

    /// Fetches a raw configuration file and keeps only the requested sections
    /// (all of them when `filter` is `None` or empty).
    pub async fn fetch_raw(options: &FetchRawOptions) -> Result<GameConfig, Error> {
        Self::fetch_raw_with(&Client::new(), options).await
    }

    pub async fn fetch_raw_with(
        client: &Client,
        options: &FetchRawOptions,
    ) -> Result<GameConfig, Error> {
        let full: GameConfig = client
            .get(&options.url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(filter_config(full, options.filter.as_deref()))
    }
}

fn join_filter(filter: &[ConfigFilter]) -> String {
    filter
        .iter()
        .map(ConfigFilter::as_str)
        .collect::<Vec<_>>()
        .join(",")
}

pub(crate) fn filter_config(full: GameConfig, filter: Option<&[ConfigFilter]>) -> GameConfig {
    let filter = match filter {
        Some(filter) if !filter.is_empty() => filter,
        _ => return full,
    };

    let mut sections = full.game_data.config.0;
    let mut filtered = Map::new();

    for key in filter {
        if let Some(value) = sections.shift_remove(key.as_str()) {
            filtered.insert(key.as_str().to_owned(), value);
        }
    }

    GameConfig {
        game_data: GameData {
            config: GameDataConfig(filtered),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample() -> GameConfig {
        serde_json::from_value(json!({
            "game_data": { "config": { "items": [1], "chests": {"a": 1}, "levels": [] } }
        }))
        .unwrap()
    }

    #[test]
    fn filter_keeps_only_requested_present_sections() {
        let filtered = filter_config(
            sample(),
            Some(&[ConfigFilter::Items, ConfigFilter::Skills]),
        );

        let keys: Vec<_> = filtered.config().keys().collect();
        assert_eq!(keys, vec!["items"]);
    }

    #[test]
    fn empty_filter_keeps_everything() {
        assert_eq!(filter_config(sample(), None).config().keys().count(), 3);
        assert_eq!(filter_config(sample(), Some(&[])).config().keys().count(), 3);
    }

    #[test]
    fn typed_access() {
        let items: Vec<u32> = sample().config().get(ConfigFilter::Items).unwrap().unwrap();
        assert_eq!(items, vec![1]);
        assert!(sample().config().get::<Vec<u32>>(ConfigFilter::Skills).unwrap().is_none());
    }

    #[test]
    fn debug_hides_the_auth_token() {
        let options = FetchOptions::new("https://x", "1", "secret-token");
        assert!(!format!("{options:?}").contains("secret-token"));
    }
}
