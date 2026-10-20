use thiserror::Error;

use crate::settings::feed_costs::FEED_COST_SETTINGS;
use crate::tools::validation::{validate_dragon_level, ValidationError};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum FeedCostError {
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error("Initial dragon level cannot be higher than final dragon level.")]
    InitialLevelHigherThanFinal,
}

pub fn calculate_dragon_feed_cost(
    initial_level: u32,
    final_level: u32,
) -> Result<u64, FeedCostError> {
    validate_dragon_level(initial_level)?;
    validate_dragon_level(final_level)?;

    if initial_level > final_level {
        return Err(FeedCostError::InitialLevelHigherThanFinal);
    }

    if initial_level == final_level {
        return Ok(0);
    }

    let mut total_cost: u64 = 0;
    for i in initial_level..final_level {
        if let Some(entry) = FEED_COST_SETTINGS.get(i as usize) {
            total_cost += entry.total;
        }
    }

    Ok(total_cost)
}
