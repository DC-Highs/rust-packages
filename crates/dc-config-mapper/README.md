# dc-config-mapper

Restructure and populate the [Dragon City](https://dragoncitygame.com/) game configuration.
Rust port of [`@dchighs/dc-config-mapper`](https://github.com/DC-Highs/dc-config-mapper). Fan-made, not affiliated with SocialPoint.

## Installation

```sh
cargo add dc-config-mapper dc-config dc-localization dc-core
cargo add tokio -F macros,rt-multi-thread
```

## Usage

Mappers take a configuration **section** (`serde_json::Value`), validate it against strict schemas
(unknown fields are rejected, so a game update never silently drops data), restructure it
(`img_name` -> `image_name`, `0/1` -> `bool`, timestamps -> ISO dates, compact rewards -> typed rewards...)
and translate names/descriptions. Share one `Localization` between mappers with `Arc`.

```rust
use std::sync::Arc;

use dc_config::{Config, ConfigFilter, ConfigLanguage, FetchOptions};
use dc_config_mapper::{ChestsMapper, ItemsMapper, Localization};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::create(FetchOptions::new(
        std::env::var("URL")?,
        std::env::var("USER_ID")?,
        std::env::var("AUTH_TOKEN")?,
    ))
    .await?;

    let localization = Arc::new(Localization::create(ConfigLanguage::Turkish).await?);
    let sections = config.data.config();

    let chests = ChestsMapper::new(localization.clone())
        .map(sections.section(ConfigFilter::Chests).unwrap())?;
    let items = ItemsMapper::new(localization)
        .map(sections.section(ConfigFilter::Items).unwrap())?;

    println!("{} chests, {} dragons", chests.len(), items.dragons().len());
    Ok(())
}
```

| TypeScript | Rust | section |
| --- | --- | --- |
| `ChestsMapper` | `ChestsMapper` -> `Vec<Value>` | `chests` |
| `ItemsMapper` | `ItemsMapper` -> `MappedItems` (`dragons()`, `buildings()`, `group("decos")`...) | `items` |
| `SkillsMapper` | `SkillsMapper` -> `MappedSkills` | `skills` |
| `HeroicRacesMapper` | `HeroicRacesMapper` -> `MappedHeroicRaces` | `heroic_races` |
| `MazeIslandsMapper` | `MazeIslandsMapper` -> `MappedMazeIslands` | `maze_island` |
| `TowerIslandsMapper` | `TowerIslandsMapper` -> `MappedTowerIslands` | `tower_island` |
| `GridIslandsMapper` | `GridIslandsMapper` -> `MappedGridIslands` | `grid_island` |
| `FogIslandsMapper` | `FogIslandsMapper` -> `MappedFogIslands` | `fog_island` |
| `RunnerIslandsMapper` | `RunnerIslandsMapper` -> `MappedRunnerIslands` | `runner_island` |

Everything the mappers return implements `Serialize`; entities that go through translation are `serde_json::Value`s.
The typed schemas live in `dc_config_mapper::schemas` (`*In` = strict input, `*Out` = normalized output).

### Compiling data

`DataCompiler` replaces id references by the referenced objects:

```rust
use dc_config_mapper::{CompileContext, DataCompiler};

let compiler = DataCompiler::new().accepted_prefixes(["basic_", "trainable_", "top_", "middle_", "bottom_", "1_", "2_", "3_", "4_"]);

let context = CompileContext::new()
    .with("laps", &heroic_races.laps)?
    .with("nodes", &heroic_races.nodes)?
    .with("missions", &heroic_races.missions)?
    .with("rewards", &heroic_races.rewards)?
    .with("attacks", &skills.attacks)?
    .with("dragons", items.dragons())?;

// `lap_ids: [1, 2]` becomes `laps: [{...}, {...}]`, `dragon_id: 5` becomes `dragon: {...}`, ...
let compiled = compiler.compile(&heroic_races.islands[0], &context)?;
```

Circular references return `Error::MaxDepthExceeded` instead of overflowing the stack.

### Other helpers

* `resolve_grid_decoration_positions(squares, decorations, width, height)`
* `process_rewards`, `process_price`, `number_to_boolean`, `iso_from_ts`

## Differences from the TypeScript package

* Results are not mutated in place: `DataCompiler::compile` returns a new value.
* Unknown item groups are kept (under the pluralized, lower-cased `group_type`) instead of dropped, and known groups missing from the configuration are empty instead of an error.
* `GridIslandsMapper` keeps `squares[].resource` as is (zod's `z.object()` there drops its content).
* `DecoSchema` of the TS package is not ported: `ItemsMapper` never used it.

## Tests against a real configuration

```sh
DC_CONFIG_FIXTURE=/path/to/config.json cargo test -p dc-config-mapper --test real_config -- --nocapture
```

## License

MIT
