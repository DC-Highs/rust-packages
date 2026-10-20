# dc-config

Access the game configuration data of [Dragon City](https://dragoncitygame.com/).
Rust port of [`@dchighs/dc-config`](https://github.com/DC-Highs/dc-config). Fan-made, not affiliated with SocialPoint.

> Don't ask how to get your URL, auth token or user id. If you don't know how, you shouldn't be using this.

## Installation

```sh
cargo add dc-config
cargo add tokio -F macros,rt-multi-thread
```

## Usage

### Fetching the configuration

```rust
use dc_config::{Config, ConfigFilter, ConfigLanguage, ConfigPlatform, FetchOptions};

#[tokio::main]
async fn main() -> Result<(), dc_config::Error> {
    let options = FetchOptions::new(
        std::env::var("URL").unwrap(),
        std::env::var("USER_ID").unwrap(),
        std::env::var("AUTH_TOKEN").unwrap(),
    )
    .language(ConfigLanguage::Turkish) // optional - English is the default
    .platform(ConfigPlatform::Android) // optional - iOS is the default
    .filter([ConfigFilter::Items]);    // optional - everything by default

    let config = Config::create(options).await?;

    let items = config.data.config().section(ConfigFilter::Items);
    println!("{items:?}");
    Ok(())
}
```

`Config::fetch(&options)` returns just the `GameConfig`. `FetchOptions`' `Debug` output hides the auth token.

### Loading a saved configuration

```rust
use dc_config::{Config, ConfigLanguage, ConfigOptions, ConfigPlatform, GameConfig};

let data = GameConfig::from_file("config.json")?;

let config = Config::new(ConfigOptions {
    language: ConfigLanguage::Turkish,
    platform: ConfigPlatform::Android,
    filter: None,
    data,
});
```

### Fetching a raw configuration file

```rust
use dc_config::{Config, ConfigFilter, CreateRawOptions, FetchRawOptions};

let config = Config::create_raw(
    CreateRawOptions::new("https://example.com/config.json").filter([ConfigFilter::Items]),
)
.await?;

// or only the data:
let data = Config::fetch_raw(&FetchRawOptions::new("https://example.com/config.json")).await?;
```

### Typed access to a section

The configuration changes with almost every game update, so sections are kept as raw JSON
(`serde_json::Value`) instead of one giant struct that breaks on the next field. Deserialize
the sections you care about into your own types:

```rust
#[derive(serde::Deserialize)]
struct Level { xp: u64 }

let levels: Option<Vec<Level>> = config.data.config().get(ConfigFilter::Levels)?;
```

For restructured, translated data use [`dc-config-mapper`](../dc-config-mapper).

## Differences from the TypeScript package

* `GameConfigDto` (the generated 4000-line interface) is replaced by `GameDataConfig` with raw sections + `get::<T>()`.
* `Config::language` replaces the misspelled `langauge` field.

## License

MIT
