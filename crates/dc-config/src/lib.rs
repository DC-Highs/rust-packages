//! Access the game configuration data of Dragon City.
//!
//! The configuration is a very large JSON document whose shape changes with
//! almost every game update, so [`GameDataConfig`] keeps each section as raw
//! JSON ([`serde_json::Value`]) and offers typed access on demand
//! ([`GameDataConfig::get`]). Pair it with `dc-config-mapper` to get the
//! restructured/translated data.
//!
//! Fan-made library, not affiliated with SocialPoint.

mod config;
mod dto;
mod error;

pub use config::{
    Config, ConfigOptions, CreateOptions, CreateRawOptions, FetchOptions, FetchRawOptions,
};
pub use dto::{GameConfig, GameData, GameDataConfig};
pub use error::Error;

pub use dc_core::{ConfigFilter, ConfigLanguage, ConfigPlatform};
