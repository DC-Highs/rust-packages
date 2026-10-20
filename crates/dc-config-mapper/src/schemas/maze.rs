#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{
    basename, iso_from_ts, number_to_boolean, process_rewards, reward_map, Availability, Num,
    ProcessedReward, TsRange,
};
use crate::Result;

/// Raw maze island (`maze_island.islands[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MazeIslandIn {
    pub id: Num,
    pub tid_name: String,
    pub name: String,
    pub availability: TsRange,
    pub paths: Vec<Num>,
    pub happy_hours: Vec<Value>,
    pub currency_id: Num,
    pub pool_size: Num,
    pub pool_time: Num,
    pub initial_points: Num,
    pub clouds: Vec<Value>,
    pub cloud_radius: Num,
    pub dragon_radius: Num,
    pub min_level: Num,
    pub building_id: Num,
    pub mobile_tutorial_id: Num,
    pub tutorial_path_id: Num,
    pub tutorial_initial_camera_position: Vec<Num>,
    pub tutorial_initial_zoom: Num,
    pub tutorial_final_zoom: Num,
    pub zip_file: String,
    pub mobile_building_position: Vec<Num>,
    pub active_platforms: Value,
    pub help_view_id: Num,
    pub sound_tag: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MazePool {
    pub size: Num,
    pub time: Num,
}

#[derive(Debug, Clone, Serialize)]
pub struct MazeIslandOut {
    pub id: Num,
    pub name: String,
    pub name_key: String,
    pub availability: Availability,
    pub path_ids: Vec<Num>,
    pub currency_id: Num,
    pub pool: MazePool,
    pub initial_points: Num,
    pub min_level: Num,
    pub zip_file_name: String,
    pub sound_tag: String,
}

impl MazeIslandIn {
    pub fn transform(self) -> Result<MazeIslandOut> {
        Ok(MazeIslandOut {
            availability: Availability::from_ts(&self.availability.from, &self.availability.to)?,
            zip_file_name: basename(&self.zip_file),
            id: self.id,
            name: self.name,
            name_key: self.tid_name,
            path_ids: self.paths,
            currency_id: self.currency_id,
            pool: MazePool { size: self.pool_size, time: self.pool_time },
            initial_points: self.initial_points,
            min_level: self.min_level,
            sound_tag: self.sound_tag,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ep {
    pub ep: Num,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NumOrVec {
    One(Num),
    Many(Vec<Num>),
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MazeRewardEntry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chest: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub b: Option<NumOrVec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub f: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub g: Option<Num>,
}

/// Raw maze node (`maze_island.nodes[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MazeNodeIn {
    pub id: Num,
    pub position: Vec<Num>,
    #[serde(default)]
    pub cost: Option<Ep>,
    #[serde(default)]
    pub reward_id: Option<Num>,
    #[serde(default)]
    pub reward: Option<Vec<MazeRewardEntry>>,
    #[serde(default)]
    pub highlighted: Option<bool>,
    #[serde(default)]
    pub key: Option<Num>,
    #[serde(default)]
    pub encounter: Option<Num>,
    #[serde(default)]
    pub encounter_skip_cost: Option<Ep>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MazeNodeOut {
    pub id: Num,
    pub cost: Num,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Vec<ProcessedReward>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_highlighted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_path_id: Option<Num>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounter_id: Option<Num>,
    pub encounter_skip_cost: Option<Num>,
}

impl MazeNodeIn {
    pub fn transform(self) -> Result<MazeNodeOut> {
        let rewards = match &self.reward {
            Some(entries) => {
                let maps = entries.iter().map(reward_map).collect::<Result<Vec<_>>>()?;
                Some(process_rewards(&maps)?)
            }
            None => None,
        };

        Ok(MazeNodeOut {
            id: self.id,
            cost: self.cost.map(|c| c.ep).unwrap_or_else(|| Num::from(0)),
            rewards,
            is_highlighted: self.highlighted,
            key_path_id: self.key,
            encounter_id: self.encounter,
            encounter_skip_cost: self.encounter_skip_cost.map(|c| c.ep),
        })
    }
}

/// Raw maze path (`maze_island.paths[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MazePathIn {
    pub id: Num,
    pub color: Vec<Num>,
    pub visibility: Num,
    pub dragon_type: Num,
    pub new_badge: Num,
    pub mobile_dragon_scale: Num,
    pub mobile_dragon_offset: Vec<Num>,
    pub nodes: Vec<Num>,
    #[serde(default)]
    pub availability: Option<TsRange>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MazePathOut {
    pub id: Num,
    pub color: Vec<Num>,
    pub visibility: Num,
    pub dragon_id: Num,
    pub has_new_badge: bool,
    pub node_ids: Vec<Num>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,
}

impl MazePathIn {
    pub fn transform(self) -> Result<MazePathOut> {
        let availability = match &self.availability {
            Some(range) => Some(Availability { from: iso_from_ts(&range.from)?, to: iso_from_ts(&range.to)? }),
            None => None,
        };

        Ok(MazePathOut {
            has_new_badge: number_to_boolean(&self.new_badge)?,
            availability,
            id: self.id,
            color: self.color,
            visibility: self.visibility,
            dragon_id: self.dragon_type,
            node_ids: self.nodes,
        })
    }
}
