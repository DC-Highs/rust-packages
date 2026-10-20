# dc-core

Shared enums for the DC Highs [Dragon City](https://dragoncitygame.com/) crates: `ConfigFilter`, `ConfigLanguage`,
`ConfigPlatform`, `StaticFileUrlPlatformPrefix`, `TextureCompressionFormat`, `SpriteQuality`, `IslandType`,
`DragonElement` (with config acronyms) and `DragonPhase`.

This is a **subset** of the TypeScript `@dchighs/dc-core` (no calculators or tools): only what
`dc-assets`, `dc-config`, `dc-config-mapper` and `dc-localization` need.
Fan-made, not affiliated with SocialPoint.

```rust
use dc_core::{ConfigLanguage, DragonElement};

assert_eq!("tr".parse::<ConfigLanguage>().unwrap(), ConfigLanguage::Turkish);
assert_eq!(DragonElement::from_acronym("el"), Some(DragonElement::Electric));
assert_eq!(ConfigLanguage::default().as_str(), "en");
```

Every enum has `as_str()`, `ALL`, `Display`, `FromStr` and serde support with the game's wire values.

## License

MIT
