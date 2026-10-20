# dc-statistics

A high-performance Rust crate to compute comprehensive statistics from Dragon City configuration documents.

## Features

- **Item Statistics**: Breakdown of all 55+ item categories (dragons, habitats, buildings, decorations, farms, etc.).
- **Dragon Statistics**: Counts grouped by rarity (Common to Heroic), breedability, store availability, category distribution, and element amounts/types.
- **Island Statistics**: Active, upcoming, and past event islands (maze, tower, fog, grid, heroic races, runner, puzzle).
- **Game Totals**: Chest counts, skin counts, collection counts, max player level.

## Usage

```rust
use dc_statistics::generate_statistics;
use serde_json::Value;

fn main() {
    let config_json: Value = serde_json::from_str("...").unwrap();
    let stats = generate_statistics(&config_json);

    println!("Total Dragons: {}", stats.dragons.total_count);
    println!("Heroic Dragons: {}", stats.dragons.heroic_count);
    println!("Active Islands: {}", stats.islands.actives_count);
}
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dc-statistics = { path = "path/to/rust-packages/crates/dc-statistics" }
```
