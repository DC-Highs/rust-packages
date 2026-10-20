# dc-core

Core types, enums, settings, validation tools and game calculators for [Dragon City](https://dragoncitygame.com/).
Port of the `@dchighs/dc-core` TypeScript package for Rust.

Fan-made, not affiliated with SocialPoint.

## Features

- **Enums**: `ConfigFilter`, `ConfigLanguage`, `ConfigPlatform`, `DragonElement`, `DragonPhase`, `DragonCategory`, `DragonRank`, `DragonRarity`, `IslandType`, `SpriteQuality`, `TextureCompressionFormat`.
- **Calculators**:
  - `calculate_attack_damage`: calculate min, max, average, and random attack damage taking dragon level, rank, stars, and category into account.
  - `calculate_breeding_results`: determine breeding outcomes from parent dragons, elements, soulmates, and special breeding rules.
  - `calculate_dragon_feed_cost`: total food required to level up a dragon between any two levels (1 to 70).
  - `calculate_element_strengths`, `calculate_element_weaknesses`, `calculate_element_resistances`, `calculate_elements_strengths`.
  - `calculate_orb_recall_gain`: calculate recalled joker/dragon orbs based on dragon level and empowerment stars.
- **Tools**:
  - `DragonStaticFileUrlParser`: parse static CDN image and animation URLs (phase, skin, platform prefix, ID, quality).
  - `get_music_name_from_tag`: resolve game music assets from island tags.
  - Validations: level, stars, category, rank, rarity, and compatibility checks.
- **Settings**: Complete static settings data (feed costs, dragon level limits, orb recall tables, sound names, element relationships).

## Usage

```rust
use dc_core::{
    calculate_attack_damage, calculate_dragon_feed_cost, calculate_orb_recall_gain,
    DragonAttackStats, DragonCategory, DragonElement, DragonRank,
};

fn main() {
    // Feed cost from level 1 to level 10
    let food = calculate_dragon_feed_cost(1, 10).unwrap();
    println!("Food required: {food}");

    // Damage calculator
    let damage = calculate_attack_damage(
        DragonAttackStats {
            category: DragonCategory::Heroic as u8,
            level: 70,
            rank: DragonRank::PlatinumI as u8,
            stars: 5,
        },
        1000.0,
    ).unwrap();
    println!("Damage: min={}, max={}, avg={}", damage.minimum, damage.maximum, damage.average);

    // Orbs on recall
    let orbs = calculate_orb_recall_gain(70, Some(5)).unwrap();
    println!("Recall orbs: {orbs}");
}
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dc-core = { path = "path/to/rust-packages/crates/dc-core" }
```

## License

MIT
