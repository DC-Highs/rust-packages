use serde::Serialize;
use serde_json::Value;

use super::{mapper, translate_all, translated};
use crate::schemas::tower::{TowerFloorIn, TowerIslandIn, TowerRewardIn, TowerSquareIn};
use crate::utils::{array, ids_where, parse, with_fields};
use crate::Result;

mapper!(
    /// Maps `config.tower_island`.
    TowerIslandsMapper
);

#[derive(Debug, Clone, Serialize)]
pub struct MappedTowerIslands {
    pub islands: Vec<Value>,
    pub floors: Vec<Value>,
    pub squares: Vec<Value>,
    pub rewards: Vec<Value>,
    pub happy_hours: Vec<Value>,
    pub parameters: Vec<Value>,
    pub currencies: Vec<Value>,
    pub actions: Vec<Value>,
}

impl TowerIslandsMapper {
    /// `section` is the `tower_island` section.
    pub fn map(&self, section: &Value) -> Result<MappedTowerIslands> {
        let loc = self.localization();
        let squares = array(section, "squares")?;
        let floors = array(section, "floors")?;
        let rewards = array(section, "rewards")?;

        let islands = array(section, "islands")?
            .iter()
            .map(|island| {
                let input = with_fields(
                    island,
                    [
                        ("square_ids", Value::Array(ids_where(squares, "island_id", island))),
                        ("floor_ids", Value::Array(ids_where(floors, "island_id", island))),
                        ("reward_ids", Value::Array(ids_where(rewards, "island_id", island))),
                    ],
                );

                translated(loc, &parse::<TowerIslandIn>("tower island", &input)?.transform()?)
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(MappedTowerIslands {
            islands,
            floors: floors
                .iter()
                .map(|floor| translated(loc, &parse::<TowerFloorIn>("tower island floor", floor)?.transform()?))
                .collect::<Result<_>>()?,
            squares: squares
                .iter()
                .map(|square| translated(loc, &parse::<TowerSquareIn>("tower island square", square)?.transform()?))
                .collect::<Result<_>>()?,
            rewards: rewards
                .iter()
                .map(|reward| translated(loc, &parse::<TowerRewardIn>("tower island reward", reward)?.transform()?))
                .collect::<Result<_>>()?,
            happy_hours: translate_all(loc, section, "happy_hours")?,
            parameters: translate_all(loc, section, "parameters")?,
            currencies: translate_all(loc, section, "currencies")?,
            actions: translate_all(loc, section, "actions")?,
        })
    }
}
