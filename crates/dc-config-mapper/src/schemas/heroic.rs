#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{
    basename, coerce_number_vec, coerce_number_vec_opt, iso_from_ts, num_mul, num_sub,
    number_to_boolean, process_rewards, reward_map, Num, ProcessedReward,
};
use crate::Result;

/// Raw heroic race (`heroic_races.islands[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeroicRacesIslandIn {
    pub id: Num,
    pub active_platforms: Value,
    pub min_level: Num,
    pub start_ts: Num,
    pub end_ts: Num,
    pub end_ts_threshold: Num,
    pub secs_before_cache_freeze: Num,
    pub min_qualifying_laps: Num,
    pub laps: Vec<Num>,
    pub building_id: Num,
    pub building_id_96: Num,
    pub building_claim_id: Num,
    pub building_free_spin_id: Num,
    pub building_position: Vec<Num>,
    pub dragon_building_position: Vec<Num>,
    pub dragon_building_scale: Num,
    pub zip_file: String,
    pub canvas_assets_url: String,
    pub island_title_tid: String,
    pub sound_tag: String,
    pub level_tiers: Vec<Num>,
    pub rewards: Vec<Num>,
    pub dragon_race_id: Num,
    pub dragon_race_scale: Num,
    pub dragon_race_offset: Vec<Num>,
    pub dragon_reward_scale: Num,
    pub dragon_reward_offset: Vec<Num>,
    pub help_id: Num,
    pub dragon_is_new: Num,
    pub spinner_enabled: Num,
    pub spin_rewards: Vec<Num>,
    pub spin_cooldown: Num,
    pub buy_spin_price: Num,
    pub notification_anim_remaining_time: Num,
    pub notification_anim_per_day: Num,
    pub notification_anim_duration: Num,
    pub lap_completed_to_hide_notification: Num,
}

#[derive(Debug, Clone, Serialize)]
pub struct HeroicAvailability {
    pub from: String,
    pub to: String,
    pub to_threshold: Num,
}

#[derive(Debug, Clone, Serialize)]
pub struct HeroicRace {
    pub dragon_id: Num,
    pub dragon_is_new: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct HeroicRacesIslandOut {
    pub id: Num,
    pub min_level: Num,
    pub availability: HeroicAvailability,
    pub name_key: String,
    pub sound_tag: String,
    pub zip_file_name: String,
    pub min_qualifying_laps: Num,
    pub lap_ids: Vec<Num>,
    pub level_tier_ids: Vec<Num>,
    pub reward_ids: Vec<Num>,
    pub race: HeroicRace,
    pub spinner_enabled: Num,
    pub spin_cooldown: Num,
    pub spin_rewards: Vec<Num>,
    pub buy_spin_price: Num,
}

impl HeroicRacesIslandIn {
    pub fn transform(self) -> Result<HeroicRacesIslandOut> {
        Ok(HeroicRacesIslandOut {
            availability: HeroicAvailability {
                from: iso_from_ts(&self.start_ts)?,
                to: iso_from_ts(&self.end_ts)?,
                to_threshold: self.end_ts_threshold,
            },
            zip_file_name: basename(&self.zip_file),
            race: HeroicRace {
                dragon_id: self.dragon_race_id,
                dragon_is_new: number_to_boolean(&self.dragon_is_new)?,
            },
            id: self.id,
            min_level: self.min_level,
            name_key: self.island_title_tid,
            sound_tag: self.sound_tag,
            min_qualifying_laps: self.min_qualifying_laps,
            lap_ids: self.laps,
            level_tier_ids: self.level_tiers,
            reward_ids: self.rewards,
            spinner_enabled: self.spinner_enabled,
            spin_cooldown: self.spin_cooldown,
            spin_rewards: self.spin_rewards,
            buy_spin_price: self.buy_spin_price,
        })
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HeroicRewardEntry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub egg: Option<Vec<Num>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub c: Option<Num>,
}

/// Raw heroic race reward (`heroic_races.rewards[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeroicRewardIn {
    pub id: Num,
    pub positions: Vec<Num>,
    pub rewards: Vec<HeroicRewardEntry>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HeroicRewardOut {
    pub id: Num,
    pub positions: Vec<Num>,
    pub reward: Vec<ProcessedReward>,
}

impl HeroicRewardIn {
    pub fn transform(self) -> Result<HeroicRewardOut> {
        let rewards = self.rewards.iter().map(reward_map).collect::<Result<Vec<_>>>()?;

        Ok(HeroicRewardOut {
            id: self.id,
            positions: self.positions,
            reward: process_rewards(&rewards)?,
        })
    }
}

/// Raw encounter (`heroic_races.encounters[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeroicEncounterIn {
    pub id: Num,
    pub enemies: Vec<Num>,
    pub speed_up_price_per_hour: Num,
    pub fight_background_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HeroicEncounterOut {
    pub id: Num,
    pub enemy_ids: Vec<Num>,
    pub speed_up_price_per_hour: Num,
    pub fight_background_id: String,
}

impl HeroicEncounterIn {
    pub fn transform(self) -> HeroicEncounterOut {
        HeroicEncounterOut {
            id: self.id,
            enemy_ids: self.enemies,
            speed_up_price_per_hour: self.speed_up_price_per_hour,
            fight_background_id: self.fight_background_id,
        }
    }
}

/// Enemy (`heroic_races.enemies[]`): validated and returned unchanged, except that
/// `dragon_ids`/`enemies` are coerced to numbers (they come as strings sometimes).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HeroicEnemy {
    pub id: Num,
    #[serde(deserialize_with = "coerce_number_vec")]
    pub dragon_ids: Vec<Num>,
    pub handicaps: Vec<Num>,
    pub view_type: String,
    pub elements_view_type: Vec<Value>,
    pub cooldown: Num,
    #[serde(default, deserialize_with = "coerce_number_vec_opt", skip_serializing_if = "Option::is_none")]
    pub enemies: Option<Vec<Num>>,
}

/// Raw lap (`heroic_races.laps[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeroicLapIn {
    pub id: Num,
    pub nodes: Vec<Num>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HeroicLapOut {
    pub id: Num,
    pub node_ids: Vec<Num>,
}

impl HeroicLapIn {
    pub fn transform(self) -> HeroicLapOut {
        HeroicLapOut { id: self.id, node_ids: self.nodes }
    }
}

/// Raw mission (`heroic_races.missions[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeroicMissionIn {
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
    #[serde(default)]
    pub encounter: Option<Num>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SpawnTime {
    pub one: Num,
    pub all: Num,
}

#[derive(Debug, Clone, Serialize)]
pub struct HeroicMissionOut {
    pub id: Num,
    #[serde(rename = "type")]
    pub kind: String,
    pub goal_points: Num,
    pub price: Num,
    pub price_increment: Num,
    pub pool_size: Num,
    pub spawn_time: SpawnTime,
    pub collect_chance: Num,
    pub collect_chance_modifier: Num,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounter_id: Option<Num>,
}

impl HeroicMissionIn {
    pub fn transform(self) -> Result<HeroicMissionOut> {
        let all = num_mul(&self.spawn_time, &num_sub(&self.goal_points, &self.pool_size)?)?;

        Ok(HeroicMissionOut {
            spawn_time: SpawnTime { one: self.spawn_time, all },
            id: self.id,
            kind: self.kind,
            goal_points: self.goal_points,
            price: self.price,
            price_increment: self.price_increment,
            pool_size: self.pool_size,
            collect_chance: self.collect_chance,
            collect_chance_modifier: self.collect_chance_modifier,
            encounter_id: self.encounter,
        })
    }
}

/// Raw node (`heroic_races.nodes[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeroicNodeIn {
    pub id: Num,
    pub buy_all_discount: Num,
    pub missions: Vec<Num>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HeroicNodeOut {
    pub id: Num,
    pub buy_all_discount: Num,
    pub mission_ids: Vec<Num>,
}

impl HeroicNodeIn {
    pub fn transform(self) -> HeroicNodeOut {
        HeroicNodeOut {
            id: self.id,
            buy_all_discount: self.buy_all_discount,
            mission_ids: self.missions,
        }
    }
}
