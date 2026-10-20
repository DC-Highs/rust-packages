//! Runs every mapper against a real, full configuration file.
//!
//! The file is not part of the repository. Point `DC_CONFIG_FIXTURE` at a JSON saved from
//! `Config::fetch` (`{ "game_data": { "config": { ... } } }`) to run it:
//!
//! ```text
//! DC_CONFIG_FIXTURE=/path/to/config.json cargo test -p dc-config-mapper --test real_config -- --nocapture
//! ```

use std::collections::HashMap;

use dc_config::{ConfigFilter, GameConfig};
use dc_config_mapper::*;
use dc_core::ConfigLanguage;

fn load() -> Option<GameConfig> {
    let path = std::env::var("DC_CONFIG_FIXTURE").ok()?;
    Some(GameConfig::from_file(path).expect("fixture must be a valid configuration file"))
}

fn localization() -> Localization {
    Localization::new(
        ConfigLanguage::English,
        HashMap::from([("tid_unit_1000_name".to_owned(), "Nature Dragon".to_owned())]),
    )
}

macro_rules! section {
    ($config:expr, $filter:expr) => {
        $config
            .config()
            .section($filter)
            .unwrap_or_else(|| panic!("fixture has no {} section", $filter))
    };
}

#[test]
fn maps_every_section_of_a_real_config() {
    let Some(config) = load() else {
        eprintln!("DC_CONFIG_FIXTURE not set, skipping");
        return;
    };

    let loc = localization();

    let chests = ChestsMapper::new(loc.clone()).map(section!(config, ConfigFilter::Chests)).unwrap();
    assert!(!chests.is_empty());
    assert!(chests[0].get("image_name").is_some());
    println!("chests: {}", chests.len());

    let items = ItemsMapper::new(loc.clone()).map(section!(config, ConfigFilter::Items)).unwrap();
    assert!(!items.dragons().is_empty());
    assert!(!items.buildings().is_empty());
    println!("dragons: {}, buildings: {}", items.dragons().len(), items.buildings().len());

    let skills = SkillsMapper::new(loc.clone()).map(section!(config, ConfigFilter::Skills)).unwrap();
    assert!(!skills.attacks.is_empty());

    let heroic = HeroicRacesMapper::new(loc.clone()).map(section!(config, ConfigFilter::HeroicRaces)).unwrap();
    println!("heroic races: {}", heroic.islands.len());

    let maze = MazeIslandsMapper::new(loc.clone()).map(section!(config, ConfigFilter::MazeIsland)).unwrap();
    println!("maze islands: {}", maze.islands.len());

    let tower = TowerIslandsMapper::new(loc.clone()).map(section!(config, ConfigFilter::TowerIsland)).unwrap();
    println!("tower islands: {}", tower.islands.len());

    let grid = GridIslandsMapper::new(loc.clone()).map(section!(config, ConfigFilter::GridIsland)).unwrap();
    println!("grid islands: {}", grid.islands.len());

    let fog = FogIslandsMapper::new(loc.clone()).map(section!(config, ConfigFilter::FogIsland)).unwrap();
    println!("fog islands: {}", fog.islands.len());

    let runner = RunnerIslandsMapper::new(loc).map(section!(config, ConfigFilter::RunnerIsland)).unwrap();
    println!("runner islands: {}", runner.islands.len());

    // Everything the mappers produce must be serializable and compilable.
    let context = CompileContext::new()
        .with("laps", &heroic.laps)
        .unwrap()
        .with("nodes", &heroic.nodes)
        .unwrap()
        .with("missions", &heroic.missions)
        .unwrap()
        .with("rewards", &heroic.rewards)
        .unwrap()
        .with("attacks", &skills.attacks)
        .unwrap()
        .with("dragons", items.dragons())
        .unwrap();

    let compiler = DataCompiler::new().accepted_prefixes(["basic_", "trainable_", "top_", "middle_", "bottom_"]);

    if let Some(island) = heroic.islands.first() {
        let compiled = compiler.compile(island, &context).unwrap();
        assert!(compiled.get("laps").is_some());
    }
}
