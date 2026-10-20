//! Generate comprehensive statistics from Dragon City configuration data.
//!
//! Port of the `@dchighs/dc-statistics` TypeScript package for Rust.
//! Designed for high performance.
//!
//! Fan-made library, not affiliated with SocialPoint.

use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod dragons;
pub mod islands;
pub mod items;

pub use dragons::*;
pub use islands::*;
pub use items::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStatistics {
    pub items: ItemsStatistics,
    pub dragons: DragonsStatistics,
    pub islands: IslandsStatistics,
    pub max_level: u64,
    pub chests_count: usize,
    pub chest_rewards_count: usize,
    pub dragon_collections_count: usize,
    pub dragon_skins_count: usize,
}

pub fn generate_statistics(config_value: &Value) -> GameStatistics {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    generate_statistics_at(config_value, now)
}

pub fn generate_statistics_at(config_value: &Value, now_seconds: u64) -> GameStatistics {
    // Navigate config: either directly, or under game_data.config
    let config = if let Some(c) = config_value
        .get("game_data")
        .and_then(|gd| gd.get("config"))
    {
        c
    } else if let Some(c) = config_value.get("config") {
        c
    } else {
        config_value
    };

    let empty_vec = Vec::new();

    let items: Vec<Value> = config
        .get("items")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let dragons: Vec<Value> = items
        .iter()
        .filter(|item| {
            item.get("group_type")
                .and_then(|g| g.as_str())
                .map(|g| g == "DRAGON")
                .unwrap_or(false)
        })
        .cloned()
        .collect();

    let items_statistics = generate_items_statistics(&items);
    let dragons_statistics = generate_dragons_statistics(&dragons);

    let get_island_array = |key: &str| -> Vec<Value> {
        config
            .get(key)
            .and_then(|section| section.get("islands"))
            .and_then(|arr| arr.as_array())
            .cloned()
            .unwrap_or_default()
    };

    let maze_islands = get_island_array("maze_island");
    let tower_islands = get_island_array("tower_island");
    let fog_islands = get_island_array("fog_island");
    let grid_islands = get_island_array("grid_island");
    let heroic_islands = get_island_array("heroic_races");
    let runner_islands = get_island_array("runner_island");
    let puzzle_islands = get_island_array("puzzle_island");

    let island_sections = IslandSections {
        maze: &maze_islands,
        tower: &tower_islands,
        fog: &fog_islands,
        grid: &grid_islands,
        heroic_races: &heroic_islands,
        runner: &runner_islands,
        puzzle: &puzzle_islands,
    };

    let islands_statistics = generate_islands_statistics_at(&island_sections, now_seconds);

    let levels = config
        .get("levels")
        .and_then(|l| l.as_array())
        .unwrap_or(&empty_vec);

    let max_level = levels
        .last()
        .and_then(|lvl| lvl.get("level"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let chests_count = config
        .get("chests")
        .and_then(|c| c.get("chests"))
        .and_then(|arr| arr.as_array())
        .map(|arr| arr.len())
        .unwrap_or(0);

    let chest_rewards_count = config
        .get("chests")
        .and_then(|c| c.get("rewards"))
        .and_then(|arr| arr.as_array())
        .map(|arr| arr.len())
        .unwrap_or(0);

    let dragon_collections_count = config
        .get("dragon_collections")
        .and_then(|c| c.get("collections"))
        .and_then(|arr| arr.as_array())
        .map(|arr| arr.len())
        .unwrap_or(0);

    let dragon_skins_count = config
        .get("dragon_skins")
        .and_then(|c| c.get("dragon_skins"))
        .and_then(|arr| arr.as_array())
        .map(|arr| arr.len())
        .unwrap_or(0);

    GameStatistics {
        items: items_statistics,
        dragons: dragons_statistics,
        islands: islands_statistics,
        max_level,
        chests_count,
        chest_rewards_count,
        dragon_collections_count,
        dragon_skins_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn matches_typescript_mock_test() {
        let mock_config = json!({
            "game_data": {
                "config": {
                    "items": [
                        { "group_type": "DRAGON", "dragon_rarity": "C", "attributes": ["f", "e"], "in_store": 1, "breedable": 1, "category": 1 },
                        { "group_type": "DRAGON", "dragon_rarity": "H", "attributes": ["l", "li", "d", "c"], "in_store": 0, "can_breed": 1, "category": 10 },
                        { "group_type": "HABITAT" },
                        { "group_type": "HABITAT" },
                        { "group_type": "BUILDING" },
                    ],
                    "maze_island": { "islands": [{ "start_ts": 1, "end_ts": 9999999999_u64 }] },
                    "tower_island": { "islands": [] },
                    "fog_island": { "islands": [] },
                    "grid_island": { "islands": [] },
                    "heroic_races": { "islands": [] },
                    "runner_island": { "islands": [] },
                    "puzzle_island": { "islands": [] },
                    "levels": [{ "level": 1 }, { "level": 2 }],
                    "chests": { "chests": [{}, {}, {}], "rewards": [] },
                    "dragon_collections": { "collections": [{}] },
                    "dragon_skins": { "dragon_skins": [{}] }
                }
            }
        });

        let stats = generate_statistics(&mock_config);

        // Items
        assert_eq!(stats.items.total_count, 5);
        assert_eq!(stats.items.dragons_count, 2);
        assert_eq!(stats.items.habitats_count, 2);
        assert_eq!(stats.items.buildings_count, 1);
        assert_eq!(stats.items.obstacles_count, 0);

        // Dragons
        assert_eq!(stats.dragons.total_count, 2);
        assert_eq!(stats.dragons.common_count, 1);
        assert_eq!(stats.dragons.heroic_count, 1);
        assert_eq!(stats.dragons.rare_count, 0);
        assert_eq!(stats.dragons.in_store_count, 1);
        assert_eq!(stats.dragons.breedable_count, 2);

        // Elements
        assert_eq!(stats.dragons.elements.unique_elements_count, 6);
        assert_eq!(stats.dragons.elements.dragons_with_two_elements, 1);
        assert_eq!(stats.dragons.elements.dragons_with_four_elements, 1);
        assert_eq!(stats.dragons.elements.dragons_with_one_element, 0);
        assert_eq!(stats.dragons.elements.dragon_counts_per_element["f"], 1);
        assert_eq!(stats.dragons.elements.dragon_counts_per_element["d"], 1);

        // Categories
        assert_eq!(stats.dragons.categories.unique_categories_count, 2);
        assert_eq!(stats.dragons.categories.dragon_counts_per_category[&1], 1);
        assert_eq!(stats.dragons.categories.dragon_counts_per_category[&10], 1);

        // Islands
        assert_eq!(stats.islands.total_count, 1);
        assert_eq!(stats.islands.actives_count, 1);
        assert_eq!(stats.islands.pasts_count, 0);
        assert_eq!(stats.islands.maze.total_count, 1);
        assert_eq!(stats.islands.maze.actives_count, 1);
        assert_eq!(stats.islands.tower.total_count, 0);

        // General
        assert_eq!(stats.max_level, 2);
        assert_eq!(stats.chests_count, 3);
        assert_eq!(stats.dragon_collections_count, 1);
        assert_eq!(stats.dragon_skins_count, 1);
    }
}
