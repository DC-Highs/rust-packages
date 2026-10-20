use serde::Serialize;
use serde_json::Value;

use super::{mapper, translated};
use crate::schemas::heroic::{
    HeroicEncounterIn, HeroicEncounterOut, HeroicEnemy, HeroicLapIn, HeroicLapOut, HeroicMissionIn,
    HeroicMissionOut, HeroicNodeIn, HeroicNodeOut, HeroicRacesIslandIn, HeroicRewardIn,
    HeroicRewardOut,
};
use crate::utils::{array, parse, passthrough};
use crate::Result;

mapper!(
    /// Maps `config.heroic_races`.
    HeroicRacesMapper
);

#[derive(Debug, Clone, Serialize)]
pub struct MappedHeroicRaces {
    pub islands: Vec<Value>,
    pub rewards: Vec<HeroicRewardOut>,
    pub encounters: Vec<HeroicEncounterOut>,
    pub enemies: Vec<HeroicEnemy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lap_rewards: Option<Value>,
    pub laps: Vec<HeroicLapOut>,
    pub missions: Vec<HeroicMissionOut>,
    pub nodes: Vec<HeroicNodeOut>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes_position: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spin_rewards: Option<Value>,
}

impl HeroicRacesMapper {
    /// `section` is the `heroic_races` section.
    pub fn map(&self, section: &Value) -> Result<MappedHeroicRaces> {
        Ok(MappedHeroicRaces {
            islands: array(section, "islands")?
                .iter()
                .map(|island| {
                    translated(
                        self.localization(),
                        &parse::<HeroicRacesIslandIn>("heroic race island", island)?.transform()?,
                    )
                })
                .collect::<Result<_>>()?,
            rewards: array(section, "rewards")?
                .iter()
                .map(|reward| parse::<HeroicRewardIn>("heroic race reward", reward)?.transform())
                .collect::<Result<_>>()?,
            encounters: array(section, "encounters")?
                .iter()
                .map(|encounter| Ok(parse::<HeroicEncounterIn>("heroic race encounter", encounter)?.transform()))
                .collect::<Result<_>>()?,
            enemies: array(section, "enemies")?
                .iter()
                .map(|enemy| parse::<HeroicEnemy>("heroic race enemy", enemy))
                .collect::<Result<_>>()?,
            lap_rewards: passthrough(section, "lap_rewards"),
            laps: array(section, "laps")?
                .iter()
                .map(|lap| Ok(parse::<HeroicLapIn>("heroic race lap", lap)?.transform()))
                .collect::<Result<_>>()?,
            missions: array(section, "missions")?
                .iter()
                .map(|mission| parse::<HeroicMissionIn>("heroic race mission", mission)?.transform())
                .collect::<Result<_>>()?,
            nodes: array(section, "nodes")?
                .iter()
                .map(|node| Ok(parse::<HeroicNodeIn>("heroic race node", node)?.transform()))
                .collect::<Result<_>>()?,
            nodes_position: passthrough(section, "nodes_position"),
            parameters: passthrough(section, "parameters"),
            spin_rewards: passthrough(section, "spin_rewards"),
        })
    }
}
