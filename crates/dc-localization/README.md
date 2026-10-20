# dc-localization

Download, search and apply the [Dragon City](https://dragoncitygame.com/) translations.
Rust port of [`@dchighs/dc-localization`](https://github.com/DC-Highs/dc-localization), needed by `dc-config-mapper`.
Fan-made, not affiliated with SocialPoint.

```sh
cargo add dc-localization dc-core
```

```rust
use dc_core::ConfigLanguage;
use dc_localization::{Localization, NormalizeOptions};

#[tokio::main]
async fn main() -> Result<(), dc_localization::Error> {
    let localization = Localization::create(ConfigLanguage::BrazilianPortuguese).await?;

    println!("{:?}", localization.get_dragon_name(1000));
    println!("{:?}", localization.search_values("dragao", NormalizeOptions::default())); // ignores case and accents

    // Replace *_key fields by their translation:
    let translated = localization.translate(serde_json::json!({ "id": 1, "name_key": "tid_unit_1000_name" }));
    println!("{translated}");
    Ok(())
}
```

`Localization::compare` reports new, edited and deleted keys between two versions.

## License

MIT
