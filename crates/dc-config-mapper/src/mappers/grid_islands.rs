use serde::Serialize;
use serde_json::Value;

use super::{mapper, translated};
use crate::schemas::grid::{
    GridDecorationIn, GridDecorationOut, GridEncounterIn, GridEncounterOut, GridEpisodeIn,
    GridIslandIn, GridSquareIn, GridSquareOut,
};
use crate::utils::{array, ids_where, parse, passthrough, with_fields};
use crate::Result;

mapper!(
    /// Maps `config.grid_island`.
    GridIslandsMapper
);

#[derive(Debug, Clone, Serialize)]
pub struct MappedGridIslands {
    pub islands: Vec<Value>,
    pub episodes: Vec<Value>,
    pub squares: Vec<GridSquareOut>,
    pub decorations: Vec<GridDecorationOut>,
    pub encounters: Vec<GridEncounterOut>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enemies: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,
}

impl GridIslandsMapper {
    /// `section` is the `grid_island` section.
    pub fn map(&self, section: &Value) -> Result<MappedGridIslands> {
        let loc = self.localization();
        let decorations = array(section, "decorations")?;

        let islands = array(section, "islands")?
            .iter()
            .map(|island| translated(loc, &parse::<GridIslandIn>("grid island", island)?.transform()?))
            .collect::<Result<Vec<_>>>()?;

        let episodes = array(section, "episodes")?
            .iter()
            .map(|episode| {
                let decoration_ids = ids_where(decorations, "episode_id", episode);
                let input = with_fields(episode, [("decoration_ids", Value::Array(decoration_ids))]);

                translated(loc, &parse::<GridEpisodeIn>("grid island episode", &input)?.transform())
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(MappedGridIslands {
            islands,
            episodes,
            squares: array(section, "squares")?
                .iter()
                .map(|square| parse::<GridSquareIn>("grid island square", square)?.transform())
                .collect::<Result<_>>()?,
            decorations: decorations
                .iter()
                .map(|decoration| Ok(parse::<GridDecorationIn>("grid island decoration", decoration)?.transform()))
                .collect::<Result<_>>()?,
            encounters: array(section, "encounters")?
                .iter()
                .map(|encounter| Ok(parse::<GridEncounterIn>("grid island encounter", encounter)?.transform()))
                .collect::<Result<_>>()?,
            enemies: passthrough(section, "enemies"),
            currencies: passthrough(section, "currencies"),
            actions: passthrough(section, "actions"),
            parameters: passthrough(section, "parameters"),
        })
    }
}
