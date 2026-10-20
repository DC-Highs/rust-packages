use dc_core::ConfigFilter;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::Error;

/// Root of the configuration document: `{ "game_data": { "config": { ... } } }`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GameConfig {
    pub game_data: GameData,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GameData {
    pub config: GameDataConfig,
}

/// The configuration sections (`items`, `chests`, `heroic_races`, ...), keyed by name.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GameDataConfig(pub Map<String, Value>);

impl GameDataConfig {
    /// Raw JSON of a section, if present.
    pub fn section(&self, filter: ConfigFilter) -> Option<&Value> {
        self.0.get(filter.as_str())
    }

    /// Raw JSON of a section by its name.
    pub fn section_by_name(&self, name: &str) -> Option<&Value> {
        self.0.get(name)
    }

    /// Deserializes a section into your own type.
    ///
    /// Returns `Ok(None)` when the section is not part of the configuration.
    pub fn get<T: DeserializeOwned>(&self, filter: ConfigFilter) -> Result<Option<T>, Error> {
        self.section(filter)
            .map(|value| T::deserialize(value).map_err(Error::from))
            .transpose()
    }

    /// Names of the sections that are present.
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(String::as_str)
    }
}

impl GameConfig {
    /// Parses a configuration JSON text (for example one saved on disk).
    pub fn from_json_str(json: &str) -> Result<Self, Error> {
        Ok(serde_json::from_str(json)?)
    }

    /// Reads and parses a configuration JSON file.
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self, Error> {
        Self::from_json_str(&std::fs::read_to_string(path)?)
    }

    pub fn config(&self) -> &GameDataConfig {
        &self.game_data.config
    }
}
