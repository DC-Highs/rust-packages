#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::utils::{basename, number_to_boolean, Availability, Num, Size};
use crate::Result;

/// Raw fog island (`fog_island.islands[]`) plus the injected `square_ids`.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FogIslandIn {
    pub id: Num,
    pub analytics_id: String,
    pub tid_name: String,
    pub tutorial_id: Num,
    pub tutorial_claimable_cell_id: Num,
    pub board_size: Vec<Num>,
    pub cell_size: Vec<Num>,
    pub origin_offset: Vec<Num>,
    pub initial_square_id: Num,
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
    pub background_plist: String,
    pub foregrounds_plists: Vec<Value>,
    pub particles_position: Vec<Num>,
    pub tutorial_claim_cells: Vec<Num>,
    // injected props
    pub square_ids: Vec<Num>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Pool {
    pub points: Num,
    pub time: Num,
}

#[derive(Debug, Clone, Serialize)]
pub struct FogIslandOut {
    pub id: Num,
    pub availability: Availability,
    pub name_key: String,
    pub board_size: Size,
    pub cell_size: Size,
    pub initial_square_id: Num,
    pub pool: Pool,
    pub initial_points: Num,
    pub currency_id: Num,
    pub min_level: Num,
    pub zip_file_name: String,
    pub sound_tag: String,
    pub square_ids: Vec<Num>,
}

impl FogIslandIn {
    pub fn transform(self) -> Result<FogIslandOut> {
        Ok(FogIslandOut {
            availability: Availability::from_ts(&self.start_ts, &self.end_ts)?,
            board_size: Size::from_pair(&self.board_size),
            cell_size: Size::from_pair(&self.cell_size),
            zip_file_name: basename(&self.zip_file),
            id: self.id,
            name_key: self.tid_name,
            initial_square_id: self.initial_square_id,
            pool: Pool { points: self.pool_points, time: self.pool_time },
            initial_points: self.initial_points,
            currency_id: self.currency_id,
            min_level: self.min_level,
            sound_tag: self.sound_tag,
            square_ids: self.square_ids,
        })
    }
}

/// Raw fog island reward (`fog_island.rewards[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FogRewardIn {
    pub id: Num,
    #[serde(rename = "type")]
    pub kind: String,
    pub island_id: Num,
    pub reward_id: Num,
    pub last_piece_cost: Num,
    pub num_pieces: Num,
    pub show_new_badge: Num,
}

#[derive(Debug, Clone, Serialize)]
pub struct FogRewardOut {
    pub id: Num,
    #[serde(rename = "type")]
    pub kind: String,
    pub dragon_id: Num,
    pub last_piece_cost: Num,
    pub num_pieces: Num,
    pub show_new_badge: bool,
}

impl FogRewardIn {
    pub fn transform(self) -> Result<FogRewardOut> {
        Ok(FogRewardOut {
            show_new_badge: number_to_boolean(&self.show_new_badge)?,
            id: self.id,
            kind: self.kind,
            dragon_id: self.reward_id,
            last_piece_cost: self.last_piece_cost,
            num_pieces: self.num_pieces,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FogSquareResourceIn {
    #[serde(default)]
    pub skin: Option<Num>,
}

/// Raw fog island square (`fog_island.squares[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FogSquareIn {
    pub id: Num,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub type_id: Option<Num>,
    pub highlight: Num,
    pub x: Num,
    pub y: Num,
    pub island_id: Num,
    pub claim_cost: Num,
    pub come_back_cost: Num,
    #[serde(default)]
    pub reward_id: Option<Num>,
    #[serde(default)]
    pub resource: Option<FogSquareResourceIn>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FogSquareOut {
    pub id: Num,
    #[serde(rename = "type")]
    pub kind: String,
    /// `null` for `NONE`/`STEP` squares; otherwise `{ type, chest_id | skin_id | piece_dragon_id }`.
    pub reward: Option<Value>,
    pub is_highlight: bool,
    pub x: Num,
    pub y: Num,
    pub cost: Num,
    pub come_back_cost: Num,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_id: Option<Num>,
}

impl FogSquareIn {
    pub fn transform(self) -> Result<FogSquareOut> {
        let reward = if self.kind == "NONE" || self.kind == "STEP" {
            None
        } else {
            let mut reward = Map::new();
            reward.insert("type".to_owned(), Value::String(self.kind.clone()));

            match self.kind.as_str() {
                "CHEST" => {
                    if let Some(type_id) = &self.type_id {
                        reward.insert("chest_id".to_owned(), Value::Number(type_id.clone()));
                    }
                }
                "RESOURCE" => {
                    if let Some(skin) = self.resource.as_ref().and_then(|r| r.skin.clone()) {
                        reward.insert("skin_id".to_owned(), Value::Number(skin));
                    }
                }
                _ => {
                    if let Some(type_id) = &self.type_id {
                        reward.insert("piece_dragon_id".to_owned(), Value::Number(type_id.clone()));
                    }
                }
            }

            Some(Value::Object(reward))
        };

        Ok(FogSquareOut {
            is_highlight: number_to_boolean(&self.highlight)?,
            reward,
            id: self.id,
            kind: self.kind,
            x: self.x,
            y: self.y,
            cost: self.claim_cost,
            come_back_cost: self.come_back_cost,
            reward_id: self.reward_id,
        })
    }
}
