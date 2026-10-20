use thiserror::Error;

use crate::enums::{DragonCategory, DragonElement, DragonRank, DragonRarity};
use crate::settings::dragons::{
    EMPOWERMENT_STAGES, MAXIMUM_LEVEL, MAXIMUM_LEVEL_WITHOUT_EMPOWERMENT, MAXIMUM_STARS,
    MINIMUM_LEVEL, MINIMUM_LEVEL_TO_EMPOWER, MINIMUM_STARS,
};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ValidationError {
    #[error("The dragon cannot have a level lower than {MINIMUM_LEVEL}!")]
    LevelTooLow,
    #[error("The dragon cannot have a level higher than {MAXIMUM_LEVEL}!")]
    LevelTooHigh,
    #[error("Dragon stars must be at least {MINIMUM_STARS}!")]
    StarsTooLow,
    #[error("Dragon stars cannot exceed {MAXIMUM_STARS}!")]
    StarsTooHigh,
    #[error("Dragon level cannot be higher than {MAXIMUM_LEVEL_WITHOUT_EMPOWERMENT} when stars are 0.")]
    LevelTooHighForZeroStars,
    #[error("Dragon level must be higher than or equal to {MINIMUM_LEVEL_TO_EMPOWER} when stars are not 0.")]
    LevelTooLowForEmpowerment,
    #[error("Dragon stars cannot be higher than the maximum number of empowerment stages.")]
    StarsExceedStages,
    #[error("Dragon level cannot be higher than {max_level} when stars are {stars}.")]
    LevelTooHighForStars { max_level: u32, stars: u32 },
    #[error("Invalid dragon category: {category}.")]
    InvalidCategory { category: u8 },
    #[error("Invalid dragon rank: {rank}.")]
    InvalidRank { rank: u8 },
    #[error("Invalid dragon rarity: {rarity}.")]
    InvalidRarity { rarity: String },
    #[error("Invalid element name: {name}.")]
    InvalidElement { name: String },
}

pub fn validate_dragon_level(level: u32) -> Result<(), ValidationError> {
    if level < MINIMUM_LEVEL {
        return Err(ValidationError::LevelTooLow);
    }
    if level > MAXIMUM_LEVEL {
        return Err(ValidationError::LevelTooHigh);
    }
    Ok(())
}

pub fn validate_dragon_stars(stars: u32) -> Result<(), ValidationError> {
    if stars > MAXIMUM_STARS {
        return Err(ValidationError::StarsTooHigh);
    }
    Ok(())
}

pub fn validate_dragon_level_compatibility_with_stars(
    level: u32,
    stars: u32,
) -> Result<(), ValidationError> {
    validate_dragon_level(level)?;
    validate_dragon_stars(stars)?;

    if stars == 0 {
        if level > MAXIMUM_LEVEL_WITHOUT_EMPOWERMENT {
            return Err(ValidationError::LevelTooHighForZeroStars);
        }
        return Ok(());
    }

    if stars > 0 && level < MINIMUM_LEVEL_TO_EMPOWER {
        return Err(ValidationError::LevelTooLowForEmpowerment);
    }

    if stars > EMPOWERMENT_STAGES.len() as u32 {
        return Err(ValidationError::StarsExceedStages);
    }

    let stage_index = (stars - 1) as usize;
    let max_level = EMPOWERMENT_STAGES[stage_index].dragon_maximum_level;

    if level > max_level {
        return Err(ValidationError::LevelTooHighForStars { max_level, stars });
    }

    Ok(())
}

pub fn validate_dragon_category(category: u8) -> Result<DragonCategory, ValidationError> {
    DragonCategory::from_u8(category).ok_or(ValidationError::InvalidCategory { category })
}

pub fn validate_dragon_rank(rank: u8) -> Result<Option<DragonRank>, ValidationError> {
    if rank == 0 {
        return Ok(None);
    }
    DragonRank::from_u8(rank)
        .map(Some)
        .ok_or(ValidationError::InvalidRank { rank })
}

pub fn validate_dragon_rarity(
    rarity: &str,
    use_lower_case: bool,
) -> Result<DragonRarity, ValidationError> {
    let target = if use_lower_case {
        rarity.to_lowercase()
    } else {
        rarity.to_string()
    };

    for r in DragonRarity::ALL {
        let cmp = if use_lower_case {
            r.as_str().to_lowercase()
        } else {
            r.as_str().to_string()
        };
        if cmp == target {
            return Ok(*r);
        }
    }

    Err(ValidationError::InvalidRarity {
        rarity: rarity.to_string(),
    })
}

pub fn validate_element_name(element_name: &str) -> Result<DragonElement, ValidationError> {
    let lower = element_name.to_lowercase();
    for elem in DragonElement::ALL {
        if elem.as_str() == lower {
            return Ok(*elem);
        }
    }
    Err(ValidationError::InvalidElement {
        name: element_name.to_string(),
    })
}
