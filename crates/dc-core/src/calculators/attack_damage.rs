use rand::Rng;


use crate::settings::dragons::{
    category_power, rank_bonus_from_u8, DAMAGE_VARIANT, ONE_STAR_POWER,
};
use crate::tools::validation::{
    validate_dragon_category, validate_dragon_level_compatibility_with_stars, validate_dragon_rank,
    ValidationError,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DragonAttackStats {
    pub category: u8,
    pub level: u32,
    pub rank: u8,
    pub stars: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttackDamageResult {
    pub minimum: i64,
    pub maximum: i64,
    pub average: i64,
    pub random: i64,
}

pub fn calculate_attack_damage(
    dragon: DragonAttackStats,
    attack_power: f64,
) -> Result<AttackDamageResult, ValidationError> {
    validate_dragon_level_compatibility_with_stars(dragon.level, dragon.stars)?;
    let category = validate_dragon_category(dragon.category)?;
    validate_dragon_rank(dragon.rank)?;

    let initial_damage = category_power(category);
    let level_bonus = (dragon.level as f64).powf(1.5) + 10.0;
    let rank_bonus = rank_bonus_from_u8(dragon.rank);
    let stars_bonus = if dragon.stars > 0 {
        ONE_STAR_POWER * dragon.stars as f64
    } else {
        0.0
    };

    let damage = 1.5 * (1.0 + rank_bonus + stars_bonus) * (initial_damage * level_bonus / 250.0)
        + attack_power;

    let minimum = (DAMAGE_VARIANT.0 * damage).round() as i64;
    let maximum = (DAMAGE_VARIANT.1 * damage).round() as i64;
    let average = ((minimum + maximum) as f64 / 2.0).round() as i64;

    let random_factor = rand::thread_rng().gen_range(DAMAGE_VARIANT.0..=DAMAGE_VARIANT.1);
    let random_damage = (random_factor * damage).round() as i64;

    Ok(AttackDamageResult {
        minimum,
        maximum,
        average,
        random: random_damage,
    })
}
