#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::utils::{
    basename, number_to_boolean, process_rewards, reward_map, Availability, Num, ProcessedReward,
};
use crate::Result;

/// Translation key of each runner mission type.
pub fn runner_island_mission_name_key(mission_type: &str) -> Option<&'static str> {
    Some(match mission_type {
        "feed" => "tid_runner_mission_feed",
        "gold" => "tid_runner_mission_collect_gold",
        "food" => "tid_runner_mission_collect_food",
        "breed" => "tid_runner_mission_breed",
        "hatch" => "tid_runner_mission_hatch",
        "pvp" => "tid_runner_mission_pvp",
        "battle" => "tid_runner_mission_battle",
        _ => return None,
    })
}

/// Raw runner island (`runner_island.islands[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunnerIslandIn {
    pub id: Num,
    pub start_ts: Num,
    pub end_ts: Num,
    pub building_id: Num,
    pub building_position: Vec<Num>,
    pub dragon_id: Num,
    pub dragon_offset: Vec<Num>,
    pub dragon_scale: Num,
    #[serde(default)]
    pub runner_dragon_asset: Option<String>,
    pub zip_file: String,
    pub help_id: Num,
    pub run_cost: Num,
    pub sound_tag: String,
    pub mission_pool: Vec<Num>,
    pub sections: Vec<Num>,
    pub building_tooltip_position: Vec<Num>,
    pub building_timer_position: Vec<Num>,
    pub milestone_rewards: Vec<Num>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunnerIslandOut {
    pub id: Num,
    pub availability: Availability,
    pub name_key: String,
    pub sound_tag: String,
    pub zip_file_name: String,
    pub run_cost: Num,
    pub mission_pool_ids: Vec<Num>,
    pub milestone_reward_ids: Vec<Num>,
}

impl RunnerIslandIn {
    pub fn transform(self) -> Result<RunnerIslandOut> {
        Ok(RunnerIslandOut {
            availability: Availability::from_ts(&self.start_ts, &self.end_ts)?,
            name_key: format!("tid_runner_island_{}_title", self.id),
            zip_file_name: basename(&self.zip_file),
            id: self.id,
            sound_tag: self.sound_tag,
            run_cost: self.run_cost,
            mission_pool_ids: self.mission_pool,
            milestone_reward_ids: self.milestone_rewards,
        })
    }
}

/// Raw milestone reward (`runner_island.milestone_rewards[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunnerMilestoneRewardIn {
    pub id: Num,
    pub points: Num,
    pub reward_id: Num,
    pub highlighted: Num,
    pub focused: Num,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunnerMilestoneRewardOut {
    pub id: Num,
    pub points: Num,
    pub reward_id: Num,
    pub is_highlight: bool,
    pub is_focused: bool,
}

impl RunnerMilestoneRewardIn {
    pub fn transform(self) -> Result<RunnerMilestoneRewardOut> {
        Ok(RunnerMilestoneRewardOut {
            is_highlight: number_to_boolean(&self.highlighted)?,
            is_focused: number_to_boolean(&self.focused)?,
            id: self.id,
            points: self.points,
            reward_id: self.reward_id,
        })
    }
}

/// Raw mission (`runner_island.missions[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunnerMissionIn {
    pub id: Num,
    #[serde(rename = "type")]
    pub kind: String,
    pub goal_points: Num,
    pub price: Num,
    pub price_increment: Num,
    pub pool_size: Num,
    pub spawn_time: Num,
    pub collect_chance: Num,
    pub collect_chance_modifier: Num,
    pub energy: Num,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunnerMissionOut {
    pub id: Num,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_key: Option<&'static str>,
    pub goal_points: Num,
    pub price: Num,
    pub price_increment: Num,
    pub pool_size: Num,
    pub spawn_time: Num,
    pub collect_chance: Num,
    pub collect_chance_modifier: Num,
    pub points: Num,
}

impl RunnerMissionIn {
    pub fn transform(self) -> RunnerMissionOut {
        RunnerMissionOut {
            name_key: runner_island_mission_name_key(&self.kind),
            id: self.id,
            kind: self.kind,
            goal_points: self.goal_points,
            price: self.price,
            price_increment: self.price_increment,
            pool_size: self.pool_size,
            spawn_time: self.spawn_time,
            collect_chance: self.collect_chance,
            collect_chance_modifier: self.collect_chance_modifier,
            points: self.energy,
        }
    }
}

/// Raw mission pool (`runner_island.mission_pool[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunnerMissionPoolIn {
    pub id: Num,
    pub buy_all_discounts: Num,
    pub missions: Vec<Num>,
    pub price: Num,
    pub price_increment: Num,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunnerMissionPoolOut {
    pub id: Num,
    pub buy_all_discounts: Num,
    pub mission_ids: Vec<Num>,
    pub price: Num,
    pub price_increment: Num,
}

impl RunnerMissionPoolIn {
    pub fn transform(self) -> RunnerMissionPoolOut {
        RunnerMissionPoolOut {
            id: self.id,
            buy_all_discounts: self.buy_all_discounts,
            mission_ids: self.missions,
            price: self.price,
            price_increment: self.price_increment,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RunnerSeed {
    pub id: Num,
    pub amount: Num,
}

/// One entry of `reward[]`. Field order is the order `process_rewards` expects.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RunnerRewardEntry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chest: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ep: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub egg: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub c: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seeds: Option<Vec<RunnerSeed>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub f: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub g: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_runner: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub f_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pu_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wr_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub el_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wd_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub m_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pr_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub e_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub li_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l_token: Option<Num>,
}

/// Raw runner reward (`runner_island.rewards[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunnerRewardIn {
    pub id: Num,
    pub reward: Vec<RunnerRewardEntry>,
    pub type_id: Num,
    #[serde(default)]
    pub double_with_video_ad: Option<bool>,
    #[serde(default)]
    pub premium_rewards: Option<Vec<Num>>,
    #[serde(default)]
    pub new_badge: Option<Vec<Num>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunnerRewardOut {
    pub id: Num,
    pub rewards: Vec<ProcessedReward>,
    pub type_id: Num,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_with_video_ad: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_rewards: Option<Vec<Num>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_badge: Option<Vec<Num>>,
}

impl RunnerRewardIn {
    pub fn transform(self) -> Result<RunnerRewardOut> {
        let maps = self.reward.iter().map(reward_map).collect::<Result<Vec<_>>>()?;

        Ok(RunnerRewardOut {
            rewards: process_rewards(&maps)?,
            id: self.id,
            type_id: self.type_id,
            double_with_video_ad: self.double_with_video_ad,
            premium_rewards: self.premium_rewards,
            new_badge: self.new_badge,
        })
    }
}
