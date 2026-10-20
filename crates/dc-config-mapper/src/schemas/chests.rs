use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{process_rewards, reward_map, Num, ProcessedReward};
use crate::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Seed {
    pub id: Num,
    pub amount: Num,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RaritySeed {
    pub rarity: String,
    pub amount: Num,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BattlegroundKey {
    pub battleground_id: Num,
    pub key_id: Num,
    pub amount: Num,
}

/// `reward.resource` of a chest reward. Field order matters: it is the order
/// in which [`process_rewards`] looks for the `*_token` key.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChestResourceIn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub f: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub g: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub c: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pr_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pu_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wr_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub e_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub f_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub el_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub m_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub li_token: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ep: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<Num>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChestRewardBodyIn {
    #[serde(default)]
    pub resource: Option<ChestResourceIn>,
    #[serde(default)]
    pub buildings: Option<Vec<Num>>,
    #[serde(default)]
    pub eggs: Option<Vec<Num>>,
    #[serde(default)]
    pub seeds: Option<Vec<Seed>>,
    #[serde(default)]
    pub moves: Option<Num>,
    #[serde(default)]
    pub rarity_seeds: Option<Vec<RaritySeed>>,
    #[serde(default)]
    pub skin: Option<Vec<Num>>,
    #[serde(default)]
    pub battleground_keys: Option<Vec<BattlegroundKey>>,
}

/// Raw chest reward (`chests.rewards[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChestRewardIn {
    pub id: Num,
    pub reward: ChestRewardBodyIn,
    pub tier_multi: Num,
    pub weight: Num,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChestRewardBodyOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<ProcessedReward>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building_ids: Option<Vec<Num>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eggs_ids: Option<Vec<Num>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seeds: Option<Vec<Seed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moves: Option<Num>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rarity_seeds: Option<Vec<RaritySeed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin_ids: Option<Vec<Num>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battleground_keys: Option<Vec<BattlegroundKey>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChestRewardOut {
    pub id: Num,
    pub reward: ChestRewardBodyOut,
    pub tier_multi: Num,
    pub weight: Num,
}

impl ChestRewardIn {
    pub fn transform(self) -> Result<ChestRewardOut> {
        let resource = match &self.reward.resource {
            Some(resource) => process_rewards(&[reward_map(resource)?])?.into_iter().next(),
            None => None,
        };

        Ok(ChestRewardOut {
            id: self.id,
            reward: ChestRewardBodyOut {
                resource,
                building_ids: self.reward.buildings,
                eggs_ids: self.reward.eggs,
                seeds: self.reward.seeds,
                moves: self.reward.moves,
                rarity_seeds: self.reward.rarity_seeds,
                skin_ids: self.reward.skin,
                battleground_keys: self.reward.battleground_keys,
            },
            tier_multi: self.tier_multi,
            weight: self.weight,
        })
    }
}

/// Raw chest (`chests.chests[]`) whose `rewards` are still reward ids.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChestIn {
    pub id: Num,
    #[serde(rename = "type")]
    pub kind: String,
    pub rewards: Vec<Num>,
    pub img_name: String,
    pub chest_name_key: String,
    pub type_name_key: String,
    pub level_tiers: Vec<Num>,
    #[serde(default)]
    pub description_key: Option<String>,
    pub default_reward: Num,
    pub pool_size: Num,
    #[serde(default)]
    pub gatcha_ids: Option<Vec<Num>>,
    #[serde(default)]
    pub instant: Value,
    #[serde(default)]
    pub foreground_asset: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChestOut {
    pub id: Num,
    #[serde(rename = "type")]
    pub kind: String,
    pub rewards: Vec<ChestRewardOut>,
    pub image_name: String,
    pub name_key: String,
    pub type_name_key: String,
    pub level_tier_ids: Vec<Num>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_key: Option<String>,
    pub default_reward_id: Num,
    pub pool_size: Num,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gatcha_ids: Option<Vec<Num>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground_asset: Option<String>,
}

impl ChestIn {
    pub fn transform(self, rewards: Vec<ChestRewardOut>) -> ChestOut {
        ChestOut {
            id: self.id,
            kind: self.kind,
            rewards,
            image_name: self.img_name,
            name_key: self.chest_name_key,
            type_name_key: self.type_name_key,
            level_tier_ids: self.level_tiers,
            description_key: self.description_key,
            default_reward_id: self.default_reward,
            pool_size: self.pool_size,
            gatcha_ids: self.gatcha_ids,
            foreground_asset: self.foreground_asset,
        }
    }
}
