//! Shared enums, settings, tools, and calculators for the DC Highs Dragon City crates.
//!
//! This is a complete Rust implementation of `@dchighs/dc-core`, optimized for
//! high performance.
//!
//! Fan-made library, not affiliated with SocialPoint.

pub mod calculators;
pub mod enums;
pub mod settings;
pub mod tools;

pub use calculators::{
    calculate_attack_damage, calculate_breeding_results, calculate_dragon_feed_cost,
    calculate_element_resistances, calculate_element_resistances_by_name,
    calculate_element_strengths, calculate_element_strengths_by_name,
    calculate_element_weaknesses, calculate_element_weaknesses_by_name,
    calculate_elements_strengths, calculate_elements_strengths_by_names,
    calculate_orb_recall_gain, AttackDamageResult, BreedingConfig, BreedingParent,
    BreedingResultItem, BreedingRule, BreedingSource, ConditionWithIds, DragonAttackStats,
    FeedCostError, Incompatibility, SoulmateRule, SpecialBreedingRule,
};
pub use enums::*;
pub use settings::*;
pub use tools::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_roundtrip() {
        assert_eq!("tr".parse::<ConfigLanguage>().unwrap(), ConfigLanguage::Turkish);
        assert_eq!(ConfigFilter::DeityWorldDragons.as_str(), "deity_world_dragons");
        assert!("nope".parse::<ConfigFilter>().is_err());
        assert_eq!(SpriteQuality::Large.as_str(), "@2x");
    }

    #[test]
    fn element_acronyms_are_unique_and_reversible() {
        for element in DragonElement::ALL {
            assert_eq!(DragonElement::from_acronym(element.acronym()), Some(*element));
        }
    }

    #[test]
    fn defaults_match_the_typescript_package() {
        assert_eq!(ConfigLanguage::default().as_str(), "en");
        assert_eq!(ConfigPlatform::default().as_str(), "ios");
        assert_eq!(StaticFileUrlPlatformPrefix::default().as_str(), "dci");
        assert_eq!(TextureCompressionFormat::default().as_str(), "dxt5");
    }

    #[test]
    fn feed_cost_calculator() {
        assert_eq!(calculate_dragon_feed_cost(10, 10).unwrap(), 0);
        assert!(calculate_dragon_feed_cost(15, 10).is_err());
        let cost = calculate_dragon_feed_cost(1, 5).unwrap();
        assert!(cost > 0);
    }

    #[test]
    fn orb_recall_gain() {
        let gain_0_stars = calculate_orb_recall_gain(30, None).unwrap();
        assert!(gain_0_stars > 0);

        let gain_5_stars = calculate_orb_recall_gain(70, Some(5)).unwrap();
        assert!(gain_5_stars > gain_0_stars);
    }

    #[test]
    fn element_calculators() {
        let strengths = calculate_element_strengths(DragonElement::Terra);
        assert!(strengths.contains(&DragonElement::Flame));
        assert!(strengths.contains(&DragonElement::Electric));

        let weaknesses = calculate_element_weaknesses(DragonElement::Terra);
        assert!(weaknesses.contains(&DragonElement::Metal));
        assert!(weaknesses.contains(&DragonElement::War));
    }

    #[test]
    fn attack_damage_calculator() {
        let res = calculate_attack_damage(
            DragonAttackStats {
                category: DragonCategory::Heroic as u8,
                level: 70,
                rank: DragonRank::PlatinumI as u8,
                stars: 5,
            },
            1000.0,
        )
        .unwrap();

        assert!(res.minimum > 0);
        assert!(res.maximum >= res.minimum);
        assert!(res.average >= res.minimum && res.average <= res.maximum);
        assert!(res.random >= res.minimum && res.random <= res.maximum);
    }

    #[test]
    fn url_parser_tests() {
        let sprite_url = "https://dci-static-s1.socialpointgames.com/dragoncity/mobile/ui/dragons/ui_1000_dragon_terra_0@2x.png";
        let parsed = DragonStaticFileUrlParser::parse_from_sprite(sprite_url);
        assert_eq!(parsed.platform_prefix, Some(StaticFileUrlPlatformPrefix::IOS));
        assert_eq!(parsed.id, Some(1000));
        assert_eq!(parsed.phase, Some(DragonPhase::Egg));
        assert_eq!(parsed.image_quality, DragonSpriteQuality::Large);
    }

    #[test]
    fn music_tag_lookup() {
        let music = get_music_name_from_tag("labyrinth_island_2015");
        assert!(music.is_some());
    }
}
