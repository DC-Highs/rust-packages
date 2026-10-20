use serde::Serialize;
use serde_json::Value;

use super::{mapper, translated};
use crate::schemas::runner::{
    RunnerIslandIn, RunnerMilestoneRewardIn, RunnerMilestoneRewardOut, RunnerMissionIn,
    RunnerMissionPoolIn, RunnerMissionPoolOut, RunnerRewardIn, RunnerRewardOut,
};
use crate::utils::{array, parse, passthrough};
use crate::Result;

mapper!(
    /// Maps `config.runner_island`.
    RunnerIslandsMapper
);

#[derive(Debug, Clone, Serialize)]
pub struct MappedRunnerIslands {
    pub islands: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obstacles: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_chunks: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_rewards: Option<Value>,
    pub missions: Vec<Value>,
    pub rewards: Vec<RunnerRewardOut>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewards_config: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewards_tiers: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestones: Option<Value>,
    pub milestone_rewards: Vec<RunnerMilestoneRewardOut>,
    /// Mapped from the `mission_pool` key of the configuration.
    pub mission_pools: Vec<RunnerMissionPoolOut>,
}

impl RunnerIslandsMapper {
    /// `section` is the `runner_island` section.
    pub fn map(&self, section: &Value) -> Result<MappedRunnerIslands> {
        let loc = self.localization();

        Ok(MappedRunnerIslands {
            islands: array(section, "islands")?
                .iter()
                .map(|island| translated(loc, &parse::<RunnerIslandIn>("runner island", island)?.transform()?))
                .collect::<Result<_>>()?,
            sections: passthrough(section, "sections"),
            parameters: passthrough(section, "parameters"),
            obstacles: passthrough(section, "obstacles"),
            level_chunks: passthrough(section, "level_chunks"),
            level_rewards: passthrough(section, "level_rewards"),
            missions: array(section, "missions")?
                .iter()
                .map(|mission| translated(loc, &parse::<RunnerMissionIn>("runner island mission", mission)?.transform()))
                .collect::<Result<_>>()?,
            rewards: array(section, "rewards")?
                .iter()
                .map(|reward| parse::<RunnerRewardIn>("runner island reward", reward)?.transform())
                .collect::<Result<_>>()?,
            rewards_config: passthrough(section, "rewards_config"),
            rewards_tiers: passthrough(section, "rewards_tiers"),
            milestones: passthrough(section, "milestones"),
            milestone_rewards: array(section, "milestone_rewards")?
                .iter()
                .map(|reward| parse::<RunnerMilestoneRewardIn>("runner island milestone reward", reward)?.transform())
                .collect::<Result<_>>()?,
            mission_pools: array(section, "mission_pool")?
                .iter()
                .map(|pool| Ok(parse::<RunnerMissionPoolIn>("runner island mission pool", pool)?.transform()))
                .collect::<Result<_>>()?,
        })
    }
}
