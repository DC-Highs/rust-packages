#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::schemas::fog::Pool;
use crate::utils::{basename, number_to_boolean, Availability, Num, Size};
use crate::Result;

/// Raw grid island (`grid_island.islands[]`).
///
/// `ui_configuration`/`active_platforms` are not part of the mapped output, so they are kept as raw JSON.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GridIslandIn {
    pub id: Num,
    pub analytics_id: String,
    pub tid_name: String,
    pub tutorial_id: Num,
    pub tutorial_claimable_cell_id: Num,
    pub pool_points: Num,
    pub pool_time: Num,
    pub initial_points: Num,
    pub start_ts: Num,
    pub end_ts: Num,
    pub currency_id: Num,
    pub min_level: Num,
    pub building_id: Num,
    pub building_position: Vec<Num>,
    pub zip_file: String,
    pub sound_tag: String,
    pub canvas_assets_url: String,
    pub ui_configuration: Value,
    pub active_platforms: Value,
    pub episodes: Vec<Num>,
    pub board_size: Vec<Num>,
    pub initial_square_id: Num,
    pub backgrounds: Vec<Num>,
    pub background_plist: String,
    pub foregrounds: Vec<Value>,
    pub foregrounds_plists: Vec<Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GridIslandOut {
    pub id: Num,
    pub min_level: Num,
    pub availability: Availability,
    pub name_key: String,
    pub sound_tag: String,
    pub zip_file_name: String,
    pub currency_id: Num,
    pub pool: Pool,
    pub initial_points: Num,
    pub board_size: Size,
    pub initial_square_id: Num,
    pub episode_ids: Vec<Num>,
}

impl GridIslandIn {
    pub fn transform(self) -> Result<GridIslandOut> {
        Ok(GridIslandOut {
            availability: Availability::from_ts(&self.start_ts, &self.end_ts)?,
            board_size: Size::from_pair(&self.board_size),
            zip_file_name: basename(&self.zip_file),
            id: self.id,
            min_level: self.min_level,
            name_key: self.tid_name,
            sound_tag: self.sound_tag,
            currency_id: self.currency_id,
            pool: Pool { points: self.pool_points, time: self.pool_time },
            initial_points: self.initial_points,
            initial_square_id: self.initial_square_id,
            episode_ids: self.episodes,
        })
    }
}

/// Raw grid episode (`grid_island.episodes[]`) plus the injected `decoration_ids`.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GridEpisodeIn {
    pub id: Num,
    pub island_id: Num,
    pub board_size: Vec<Num>,
    pub initial_square_id: Num,
    pub final_square_id: Num,
    pub tid_title: String,
    pub mobile_begin_tutorial_id: Num,
    pub mobile_end_tutorial_id: Num,
    pub canvas_background: String,
    pub backgrounds: Vec<String>,
    pub background_plist: String,
    pub foregrounds: Vec<Value>,
    pub foregrounds_plists: Vec<Value>,
    pub squares: Vec<Num>,
    // injected props
    pub decoration_ids: Vec<Num>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GridEpisodeOut {
    pub id: Num,
    pub board_size: Size,
    pub initial_square_id: Num,
    pub final_square_id: Num,
    pub name_key: String,
    pub square_ids: Vec<Num>,
    pub decoration_ids: Vec<Num>,
}

impl GridEpisodeIn {
    pub fn transform(self) -> GridEpisodeOut {
        GridEpisodeOut {
            board_size: Size::from_pair(&self.board_size),
            id: self.id,
            initial_square_id: self.initial_square_id,
            final_square_id: self.final_square_id,
            name_key: self.tid_title,
            square_ids: self.squares,
            decoration_ids: self.decoration_ids,
        }
    }
}

/// Raw grid square (`grid_island.squares[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GridSquareIn {
    pub id: Num,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub type_id: Option<Num>,
    pub highlight: Num,
    pub x: Num,
    pub y: Num,
    pub island_id: Num,
    pub episode_id: Num,
    pub claim_cost: Num,
    #[serde(default)]
    pub wall: Option<String>,
    #[serde(default)]
    pub wall_suffix: Option<String>,
    #[serde(default)]
    pub resource: Option<Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GridSquareOut {
    pub id: Num,
    #[serde(rename = "type")]
    pub kind: String,
    /// `null` for `NONE` squares; otherwise `{ type, chest_id | dragon_id }`.
    pub reward: Option<Value>,
    pub is_highlight: bool,
    pub x: Num,
    pub y: Num,
    pub cost: Num,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wall: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wall_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Value>,
}

impl GridSquareIn {
    pub fn transform(self) -> Result<GridSquareOut> {
        let reward = if self.kind == "NONE" {
            None
        } else {
            let mut reward = Map::new();
            reward.insert("type".to_owned(), Value::String(self.kind.to_lowercase()));

            if let Some(type_id) = &self.type_id {
                let key = if self.kind == "CHEST" { "chest_id" } else { "dragon_id" };
                reward.insert(key.to_owned(), Value::Number(type_id.clone()));
            }

            Some(Value::Object(reward))
        };

        Ok(GridSquareOut {
            is_highlight: number_to_boolean(&self.highlight)?,
            reward,
            id: self.id,
            kind: self.kind,
            x: self.x,
            y: self.y,
            cost: self.claim_cost,
            wall: self.wall,
            wall_suffix: self.wall_suffix,
            resource: self.resource,
        })
    }
}

/// Raw grid decoration (`grid_island.decorations[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GridDecorationIn {
    pub id: Num,
    pub file: String,
    pub x: Num,
    pub y: Num,
    pub island_id: Num,
    pub episode_id: Num,
}

#[derive(Debug, Clone, Serialize)]
pub struct GridDecorationOut {
    pub id: Num,
    pub file_name: String,
    pub x: Num,
    pub y: Num,
}

impl GridDecorationIn {
    pub fn transform(self) -> GridDecorationOut {
        GridDecorationOut {
            id: self.id,
            file_name: format!("{}.png", self.file),
            x: self.x,
            y: self.y,
        }
    }
}

/// Raw grid encounter (`grid_island.encounters[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GridEncounterIn {
    pub id: Num,
    pub enemies: Vec<Num>,
    pub speed_up_price_per_hour: Num,
    pub fight_background: String,
    pub fight_background_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GridEncounterOut {
    pub id: Num,
    pub enemy_ids: Vec<Num>,
    pub speed_up_price_per_hour: Num,
    pub fight_background: String,
    pub fight_background_id: String,
}

impl GridEncounterIn {
    pub fn transform(self) -> GridEncounterOut {
        GridEncounterOut {
            id: self.id,
            enemy_ids: self.enemies,
            speed_up_price_per_hour: self.speed_up_price_per_hour,
            fight_background: self.fight_background,
            fight_background_id: self.fight_background_id,
        }
    }
}
