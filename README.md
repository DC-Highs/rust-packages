# DC Highs for Rust

Rust ports of the [DC Highs](https://github.com/DC-Highs) Dragon City libraries, for high-performance Discord/Telegram bots.
Fan-made, not affiliated with SocialPoint.

| crate | port of | purpose |
| --- | --- | --- |
| [`dc-assets`](crates/dc-assets) | `@dchighs/dc-assets` | download sprites, thumbnails, animations, music |
| [`dc-config`](crates/dc-config) | `@dchighs/dc-config` | fetch/load the game configuration |
| [`dc-config-mapper`](crates/dc-config-mapper) | `@dchighs/dc-config-mapper` | restructure, translate and populate the configuration |
| [`dc-localization`](crates/dc-localization) | `@dchighs/dc-localization` | translations (required by the mapper) |
| [`dc-core`](crates/dc-core) | `@dchighs/dc-core` (subset) | shared enums |

## Using it before publishing

```toml
[dependencies]
dc-assets = { path = "D:/v2Projects/@dchighs/rust-packages/crates/dc-assets" }
dc-config = { path = "D:/v2Projects/@dchighs/rust-packages/crates/dc-config" }
dc-config-mapper = { path = "D:/v2Projects/@dchighs/rust-packages/crates/dc-config-mapper" }
```

Or from a git repository: `cargo add --git https://github.com/<you>/<repo> dc-assets`.

## Publishing to crates.io

Dependencies must exist on the registry first, so publish in this order (`cargo login` once):

```sh
cargo publish -p dc-core
cargo publish -p dc-localization
cargo publish -p dc-assets
cargo publish -p dc-config
cargo publish -p dc-config-mapper
```

After that: `cargo add dc-assets dc-config dc-config-mapper`.

Before publishing, check the `repository` URL in the root `Cargo.toml` (it currently points to a placeholder
`DC-Highs/dchighs-rs`) and bump `version` there (all crates share it).

## Development

```sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
DC_CONFIG_FIXTURE=/path/to/config.json cargo test -p dc-config-mapper --test real_config -- --nocapture
```
