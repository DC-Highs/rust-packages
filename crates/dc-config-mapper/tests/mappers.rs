use std::collections::HashMap;

use dc_config_mapper::*;
use dc_core::ConfigLanguage;
use serde_json::{json, Value};

fn localization() -> Localization {
    Localization::new(
        ConfigLanguage::English,
        HashMap::from([
            ("tid_chest_name_2".to_owned(), "Wooden Chest".to_owned()),
            ("tid_chest_type_2".to_owned(), "Chest".to_owned()),
            ("tid_island".to_owned(), "Maze".to_owned()),
        ]),
    )
}

fn chests_section() -> Value {
    json!({
        "chests": [{
            "id": 2, "type": "CHEST", "rewards": [3, 4], "img_name": "101_wood_chest",
            "chest_name_key": "tid_chest_name_2", "type_name_key": "tid_chest_type_2",
            "level_tiers": [5, 11], "default_reward": 3, "pool_size": 5
        }],
        "rewards": [
            {"id": 3, "reward": {"resource": {"g": 100}}, "tier_multi": 1, "weight": 85},
            {"id": 4, "reward": {"eggs": [1042], "resource": {"el_token": 3}}, "tier_multi": 1, "weight": 1}
        ]
    })
}

#[test]
fn chests_are_resolved_restructured_and_translated() {
    let chests = ChestsMapper::new(localization()).map(&chests_section()).unwrap();

    assert_eq!(chests.len(), 1);
    let chest = &chests[0];

    assert_eq!(chest["name"], "Wooden Chest");
    assert_eq!(chest["type_name"], "Chest");
    assert_eq!(chest["image_name"], "101_wood_chest");
    assert_eq!(chest["default_reward_id"], 3);
    assert!(chest.get("img_name").is_none() && chest.get("chest_name_key").is_none());

    assert_eq!(chest["rewards"][0]["reward"]["resource"], json!({"type": "gold", "amount": 100}));
    assert_eq!(chest["rewards"][1]["reward"]["eggs_ids"], json!([1042]));
    assert_eq!(
        chest["rewards"][1]["reward"]["resource"],
        json!({"type": "habitat_token", "element": "electric", "amount": 3})
    );
}

#[test]
fn schemas_are_strict() {
    let mut section = chests_section();
    section["chests"][0]["brand_new_field"] = json!(1);

    let error = ChestsMapper::new(localization()).map(&section).unwrap_err();
    assert!(error.to_string().contains("brand_new_field"), "{error}");
}

#[test]
fn missing_reward_is_reported() {
    let mut section = chests_section();
    section["chests"][0]["rewards"] = json!([3, 999]);

    assert!(matches!(
        ChestsMapper::new(localization()).map(&section),
        Err(Error::RewardNotFound(_))
    ));
}

#[test]
fn maze_islands_get_iso_dates_and_ids() {
    let section = json!({
        "actions": [], "clouds": [], "currencies": [], "happy_hours": [], "rewards": [],
        "rewards_config": [], "rewards_tiers": [], "encounters": [], "enemies": [], "parameters": [],
        "paths": [{
            "id": 1, "color": [1, 2, 3], "visibility": 1, "dragon_type": 1042, "new_badge": 1,
            "mobile_dragon_scale": 1, "mobile_dragon_offset": [0, 0], "nodes": [10, 11]
        }],
        "nodes": [{"id": 10, "position": [1, 2], "cost": {"ep": 20}, "reward": [{"g": 5}, {"b": [7, 8]}], "highlighted": true}],
        "islands": [{
            "id": 303, "tid_name": "tid_island", "name": "Maze", "availability": {"from": 0, "to": 1700000000},
            "paths": [1], "happy_hours": [], "currency_id": 303, "pool_size": 2000, "pool_time": 86400,
            "initial_points": 700, "clouds": [], "cloud_radius": 1, "dragon_radius": 1, "min_level": 7,
            "building_id": 1, "mobile_tutorial_id": 1, "tutorial_path_id": 1,
            "tutorial_initial_camera_position": [0, 0], "tutorial_initial_zoom": 1, "tutorial_final_zoom": 1,
            "zip_file": "path/to/mi_spring_b.zip", "mobile_building_position": [0, 0],
            "active_platforms": {}, "help_view_id": 1, "sound_tag": "FANTASY"
        }]
    });

    let maze = MazeIslandsMapper::new(localization()).map(&section).unwrap();

    assert_eq!(maze.islands[0]["name"], "Maze");
    assert_eq!(maze.islands[0]["availability"]["to"], "2023-11-14T22:13:20.000Z");
    assert_eq!(maze.islands[0]["zip_file_name"], "mi_spring_b.zip");
    assert_eq!(maze.islands[0]["pool"], json!({"size": 2000, "time": 86400}));
    assert_eq!(maze.paths[0]["has_new_badge"], true);
    assert_eq!(maze.paths[0]["dragon_id"], 1042);
    assert_eq!(maze.nodes[0]["cost"], 20);
    assert_eq!(maze.nodes[0]["rewards"][1], json!({"type": "building", "building_id": 7}));
    assert_eq!(maze.nodes[0]["encounter_skip_cost"], Value::Null);
}

#[test]
fn heroic_enemies_are_coerced_to_numbers() {
    let section = json!({
        "islands": [], "rewards": [], "encounters": [], "laps": [], "nodes": [],
        "enemies": [{
            "id": 1, "dragon_ids": ["1042", 7], "handicaps": [], "view_type": "x",
            "elements_view_type": [], "cooldown": 0, "enemies": ["3"]
        }],
        "missions": [{
            "id": 1, "type": "BREED", "goal_points": 10, "price": 1, "price_increment": 0,
            "pool_size": 4, "spawn_time": 60, "collect_chance": 1, "collect_chance_modifier": 0
        }]
    });

    let heroic = HeroicRacesMapper::new(localization()).map(&section).unwrap();
    let json = serde_json::to_value(&heroic).unwrap();

    assert_eq!(json["enemies"][0]["dragon_ids"], json!([1042, 7]));
    assert_eq!(json["enemies"][0]["enemies"], json!([3]));
    assert_eq!(json["missions"][0]["spawn_time"], json!({"one": 60, "all": 360}));
}

#[test]
fn items_are_grouped_and_unknown_groups_are_kept() {
    let section = json!([
        {"id": 1, "name": "Some Booster", "group_type": "BOOSTER"},
        {"id": 2, "name": "Social", "group_type": "SOCIAL BUILDING"},
        {"id": 3, "name": "Future", "group_type": "NEW THING"},
    ]);

    let items = ItemsMapper::new(localization()).map(&section).unwrap();

    assert_eq!(items.group("boosters").len(), 1);
    assert_eq!(items.group("social_buildings").len(), 1);
    assert_eq!(items.group("new_things").len(), 1);
    assert!(items.dragons().is_empty());
}

#[test]
fn invalid_item_is_reported_with_its_schema() {
    let section = json!([{"id": 1, "name": "broken", "group_type": "DRAGON"}]);

    let error = ItemsMapper::new(localization()).map(&section).unwrap_err();
    assert!(matches!(error, Error::Schema { schema: "dragon", .. }), "{error}");
}
