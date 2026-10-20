use serde::Serialize;
use serde_json::Value;

use super::{mapper, translated};
use crate::schemas::fog::{FogIslandIn, FogRewardIn, FogRewardOut, FogSquareIn, FogSquareOut};
use crate::utils::{array, parse, passthrough, with_fields, ids_where};
use crate::Result;

mapper!(
    /// Maps `config.fog_island`.
    FogIslandsMapper
);

#[derive(Debug, Clone, Serialize)]
pub struct MappedFogIslands {
    pub islands: Vec<Value>,
    pub squares: Vec<FogSquareOut>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Value>,
    pub rewards: Vec<FogRewardOut>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,
}

impl FogIslandsMapper {
    /// `section` is the `fog_island` section.
    pub fn map(&self, section: &Value) -> Result<MappedFogIslands> {
        let squares = array(section, "squares")?;

        let islands = array(section, "islands")?
            .iter()
            .map(|island| {
                let square_ids = ids_where(squares, "island_id", island);
                let input = with_fields(island, [("square_ids", Value::Array(square_ids))]);

                translated(
                    self.localization(),
                    &parse::<FogIslandIn>("fog island", &input)?.transform()?,
                )
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(MappedFogIslands {
            islands,
            squares: squares
                .iter()
                .map(|square| parse::<FogSquareIn>("fog island square", square)?.transform())
                .collect::<Result<_>>()?,
            currencies: passthrough(section, "currencies"),
            rewards: array(section, "rewards")?
                .iter()
                .map(|reward| parse::<FogRewardIn>("fog island reward", reward)?.transform())
                .collect::<Result<_>>()?,
            actions: passthrough(section, "actions"),
            hints: passthrough(section, "hints"),
            parameters: passthrough(section, "parameters"),
        })
    }
}
