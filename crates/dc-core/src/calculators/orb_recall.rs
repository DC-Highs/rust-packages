use crate::settings::orb_recall_return::{ORB_RECALL_PER_LEVELS, ORB_RECALL_PER_STARS};
use crate::tools::validation::{
    validate_dragon_level_compatibility_with_stars, ValidationError,
};

pub fn calculate_orb_recall_gain(
    level: u32,
    stars: Option<u32>,
) -> Result<u32, ValidationError> {
    let adjusted_stars = stars.unwrap_or(0);
    validate_dragon_level_compatibility_with_stars(level, adjusted_stars)?;

    let adjusted_level = level.min(30);
    let mut total_orbs: u32 = 0;

    for level_index in 0..adjusted_level as usize {
        if let Some(&orbs) = ORB_RECALL_PER_LEVELS.get(level_index) {
            total_orbs += orbs;
        }
    }

    for star_index in 0..adjusted_stars as usize {
        if let Some(&orbs) = ORB_RECALL_PER_STARS.get(star_index) {
            total_orbs += orbs;
        }
    }

    Ok(total_orbs)
}
