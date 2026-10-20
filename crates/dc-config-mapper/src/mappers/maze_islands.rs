use serde::Serialize;
use serde_json::Value;

use super::{mapper, translated};
use crate::schemas::maze::{MazeIslandIn, MazeNodeIn, MazePathIn};
use crate::utils::{array, parse, passthrough};
use crate::Result;

mapper!(
    /// Maps `config.maze_island`.
    MazeIslandsMapper
);

#[derive(Debug, Clone, Serialize)]
pub struct MappedMazeIslands {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clouds: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub happy_hours: Option<Value>,
    pub paths: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewards_config: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewards_tiers: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounters: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enemies: Option<Value>,
    pub islands: Vec<Value>,
    pub nodes: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,
}

impl MazeIslandsMapper {
    /// `section` is the `maze_island` section.
    pub fn map(&self, section: &Value) -> Result<MappedMazeIslands> {
        let loc = self.localization();

        Ok(MappedMazeIslands {
            actions: passthrough(section, "actions"),
            clouds: passthrough(section, "clouds"),
            currencies: passthrough(section, "currencies"),
            happy_hours: passthrough(section, "happy_hours"),
            paths: array(section, "paths")?
                .iter()
                .map(|path| translated(loc, &parse::<MazePathIn>("maze island path", path)?.transform()?))
                .collect::<Result<_>>()?,
            rewards: passthrough(section, "rewards"),
            rewards_config: passthrough(section, "rewards_config"),
            rewards_tiers: passthrough(section, "rewards_tiers"),
            encounters: passthrough(section, "encounters"),
            enemies: passthrough(section, "enemies"),
            islands: array(section, "islands")?
                .iter()
                .map(|island| translated(loc, &parse::<MazeIslandIn>("maze island", island)?.transform()?))
                .collect::<Result<_>>()?,
            nodes: array(section, "nodes")?
                .iter()
                .map(|node| translated(loc, &parse::<MazeNodeIn>("maze island node", node)?.transform()?))
                .collect::<Result<_>>()?,
            parameters: passthrough(section, "parameters"),
        })
    }
}
