//! One mapper per configuration section. Each takes the raw section (`serde_json::Value`, e.g.
//! `config.section(ConfigFilter::Chests)`), validates it against the strict schemas, restructures it
//! and translates names/descriptions with the given [`Localization`](dc_localization::Localization).

mod chests;
mod fog_islands;
mod grid_islands;
mod heroic_races;
mod items;
mod maze_islands;
mod runner_islands;
mod skills;
mod tower_islands;

pub use chests::ChestsMapper;
pub use fog_islands::{FogIslandsMapper, MappedFogIslands};
pub use grid_islands::{GridIslandsMapper, MappedGridIslands};
pub use heroic_races::{HeroicRacesMapper, MappedHeroicRaces};
pub use items::{ItemsMapper, MappedItems};
pub use maze_islands::{MappedMazeIslands, MazeIslandsMapper};
pub use runner_islands::{MappedRunnerIslands, RunnerIslandsMapper};
pub use skills::{MappedSkills, SkillsMapper};
pub use tower_islands::{MappedTowerIslands, TowerIslandsMapper};

use dc_localization::Localization;
use serde::Serialize;
use serde_json::Value;

use crate::Result;

/// Serializes a normalized value and translates its `*_key` fields.
pub(crate) fn translated<T: Serialize>(localization: &Localization, value: &T) -> Result<Value> {
    Ok(localization.translate(serde_json::to_value(value)?))
}

/// Translates every element of `section[key]` as is.
pub(crate) fn translate_all(localization: &Localization, section: &Value, key: &str) -> Result<Vec<Value>> {
    Ok(crate::utils::array(section, key)?
        .iter()
        .map(|item| localization.translate(item.clone()))
        .collect())
}

/// Defines the `new`/`localization` boilerplate shared by every mapper.
macro_rules! mapper {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone)]
        pub struct $name {
            localization: ::std::sync::Arc<::dc_localization::Localization>,
        }

        impl $name {
            /// Accepts a `Localization` or an `Arc<Localization>` (to share it between mappers).
            pub fn new(localization: impl Into<::std::sync::Arc<::dc_localization::Localization>>) -> Self {
                Self { localization: localization.into() }
            }

            pub fn localization(&self) -> &::dc_localization::Localization {
                &self.localization
            }
        }
    };
}

pub(crate) use mapper;
