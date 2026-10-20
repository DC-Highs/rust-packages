#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::schemas::maze::{Ep, MazePool};
use crate::utils::{
    basename, number_to_boolean, process_rewards, reward_map, Availability, Num, ProcessedReward,
    Size,
};
use crate::Result;

/// Raw tower island (`tower_island.islands[]`) plus the injected `square_ids`/`floor_ids`/`reward_ids`.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TowerIslandIn {
    pub id: Num,
    pub analytics_id: String,
    pub tid_name: String,
    pub start_ts: Num,
    pub end_ts: Num,
    pub initial_square_id: Num,
    pub tutorial_id: Num,
    pub tutorial_die_result: Num,
    pub pool_size: Num,
    pub pool_time: Num,
    pub initial_points: Num,
    pub currency_id: Num,
    pub min_level: Num,
    pub building_id: Num,
    pub building_position: Vec<Num>,
    pub tower_size: Vec<Num>,
    pub zip_file: String,
    pub sound_tag: String,
    pub canvas_assets_url: String,
    pub mobile_first_floor_position: Vec<Num>,
    pub mobile_floor_height: Num,
    pub mobile_interior_wall_margin: Num,
    pub mobile_final_dragon_scale: Vec<Num>,
    pub mobile_final_dragon_position: Vec<Num>,
    pub mobile_final_dragon_info_position: Vec<Num>,
    pub max_die_roll: Num,
    pub help_view_id: Num,
    pub active_platforms: Value,
    // injected properties
    pub square_ids: Vec<Num>,
    pub floor_ids: Vec<Num>,
    pub reward_ids: Vec<Num>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TowerIslandOut {
    pub id: Num,
    pub name_key: String,
    pub availability: Availability,
    pub initial_square_id: Num,
    pub pool: MazePool,
    pub initial_points: Num,
    pub currency_id: Num,
    pub min_level: Num,
    pub zip_file_name: String,
    pub sound_tag: String,
    pub max_die_roll: Num,
    pub help_view_id: Num,
    pub square_ids: Vec<Num>,
    pub floor_ids: Vec<Num>,
    pub reward_ids: Vec<Num>,
    pub board_size: Size,
}

impl TowerIslandIn {
    pub fn transform(self) -> Result<TowerIslandOut> {
        Ok(TowerIslandOut {
            availability: Availability::from_ts(&self.start_ts, &self.end_ts)?,
            board_size: Size::from_pair(&self.tower_size),
            zip_file_name: basename(&self.zip_file),
            id: self.id,
            name_key: self.tid_name,
            initial_square_id: self.initial_square_id,
            pool: MazePool { size: self.pool_size, time: self.pool_time },
            initial_points: self.initial_points,
            currency_id: self.currency_id,
            min_level: self.min_level,
            sound_tag: self.sound_tag,
            max_die_roll: self.max_die_roll,
            help_view_id: self.help_view_id,
            square_ids: self.square_ids,
            floor_ids: self.floor_ids,
            reward_ids: self.reward_ids,
        })
    }
}

/// Raw tower floor (`tower_island.floors[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TowerFloorIn {
    pub id: Num,
    pub y: Num,
    pub x_flip: Num,
    pub floor_image: String,
    pub area: Num,
    pub roll_die_price: Ep,
    pub island_id: Num,
}

#[derive(Debug, Clone, Serialize)]
pub struct TowerFloorOut {
    pub id: Num,
    pub y: Num,
    pub x_flip: bool,
    pub floor_image: String,
    pub area: Num,
    pub roll_die_price: Num,
}

impl TowerFloorIn {
    pub fn transform(self) -> Result<TowerFloorOut> {
        Ok(TowerFloorOut {
            x_flip: number_to_boolean(&self.x_flip)?,
            id: self.id,
            y: self.y,
            floor_image: self.floor_image,
            area: self.area,
            roll_die_price: self.roll_die_price.ep,
        })
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TowerRewardEntry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chest: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub egg: Option<Num>,
}

/// Raw tower square (`tower_island.squares[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TowerSquareIn {
    pub id: Num,
    #[serde(rename = "type")]
    pub kind: String,
    pub highlight: Num,
    pub x: Num,
    pub y: Num,
    pub island_id: Num,
    pub ixy: String,
    #[serde(default)]
    pub rewards_array: Option<Vec<TowerRewardEntry>>,
    #[serde(default)]
    pub wall: Option<String>,
    #[serde(default)]
    pub catapult_destination_square_id: Option<Num>,
    #[serde(default)]
    pub piece_reward_id: Option<Num>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TowerSquareOut {
    pub id: Num,
    #[serde(rename = "type")]
    pub kind: String,
    pub is_highlight: bool,
    pub x: Num,
    pub y: Num,
    pub ixy: String,
    pub rewards: Option<Vec<ProcessedReward>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wall: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catapult_destination_square_id: Option<Num>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub piece_reward_id: Option<Num>,
}

impl TowerSquareIn {
    pub fn transform(self) -> Result<TowerSquareOut> {
        let rewards = match &self.rewards_array {
            Some(entries) => {
                let maps = entries.iter().map(reward_map).collect::<Result<Vec<_>>>()?;
                Some(process_rewards(&maps)?)
            }
            None => None,
        };

        Ok(TowerSquareOut {
            is_highlight: number_to_boolean(&self.highlight)?,
            rewards,
            id: self.id,
            kind: self.kind,
            x: self.x,
            y: self.y,
            ixy: self.ixy,
            wall: self.wall,
            catapult_destination_square_id: self.catapult_destination_square_id,
            piece_reward_id: self.piece_reward_id,
        })
    }
}

/// Raw tower reward (`tower_island.rewards[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TowerRewardIn {
    pub id: Num,
    pub dragon_reward_id: Num,
    pub last_piece_cost: Num,
    pub num_pieces: Num,
    pub show_new_badge: Num,
    pub area: Num,
    pub island_id: Num,
}

#[derive(Debug, Clone, Serialize)]
pub struct TowerRewardOut {
    pub id: Num,
    pub dragon_id: Num,
    pub last_piece_cost: Num,
    pub pieces_count: Num,
    pub show_new_badge: bool,
    pub area: Num,
}

impl TowerRewardIn {
    pub fn transform(self) -> Result<TowerRewardOut> {
        Ok(TowerRewardOut {
            show_new_badge: number_to_boolean(&self.show_new_badge)?,
            id: self.id,
            dragon_id: self.dragon_reward_id,
            last_piece_cost: self.last_piece_cost,
            pieces_count: self.num_pieces,
            area: self.area,
        })
    }
}
