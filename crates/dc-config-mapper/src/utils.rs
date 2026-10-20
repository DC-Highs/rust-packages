//! Helpers shared by the schemas and mappers (ports of the `utils` of the TypeScript package).

use chrono::{DateTime, SecondsFormat, Utc};
use dc_core::DragonElement;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Number, Value};

use crate::{Error, Result};

/// A JSON number (integer or float), exactly as it came from the configuration.
pub type Num = Number;

pub(crate) fn num_f64(n: &Num) -> f64 {
    n.as_f64().unwrap_or(f64::NAN)
}

/// `numberToBoolean`: only `0` and `1` are accepted.
pub fn number_to_boolean(n: &Num) -> Result<bool> {
    match n.as_f64() {
        Some(1.0) => Ok(true),
        Some(0.0) => Ok(false),
        _ => Err(Error::InvalidBoolean(n.to_string())),
    }
}

/// `new Date(ts * 1000).toISOString()`.
pub fn iso_from_ts(ts: &Num) -> Result<String> {
    let millis = num_f64(ts) * 1000.0;

    if !millis.is_finite() {
        return Err(Error::InvalidTimestamp(ts.to_string()));
    }

    DateTime::<Utc>::from_timestamp_millis(millis as i64)
        .map(|date| date.to_rfc3339_opts(SecondsFormat::Millis, true))
        .ok_or_else(|| Error::InvalidTimestamp(ts.to_string()))
}

/// `path.basename`.
pub fn basename(path: &str) -> String {
    path.trim_end_matches(['/', '\\'])
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or("")
        .to_owned()
}

fn from_f64(f: f64, what: &str) -> Result<Num> {
    if f.fract() == 0.0 && f.abs() < 9.0e15 {
        return Ok(Number::from(f as i64));
    }

    Number::from_f64(f).ok_or_else(|| Error::InvalidNumber(what.to_owned()))
}

/// `a - b` keeping integers as integers.
pub fn num_sub(a: &Num, b: &Num) -> Result<Num> {
    if let (Some(x), Some(y)) = (a.as_i64(), b.as_i64()) {
        if let Some(r) = x.checked_sub(y) {
            return Ok(Number::from(r));
        }
    }

    from_f64(num_f64(a) - num_f64(b), &format!("{a} - {b}"))
}

/// `a * b` keeping integers as integers.
pub fn num_mul(a: &Num, b: &Num) -> Result<Num> {
    if let (Some(x), Some(y)) = (a.as_i64(), b.as_i64()) {
        if let Some(r) = x.checked_mul(y) {
            return Ok(Number::from(r));
        }
    }

    from_f64(num_f64(a) * num_f64(b), &format!("{a} * {b}"))
}

/// `z.coerce.number()` for the values that show up in the configuration (numbers and numeric strings).
pub(crate) fn coerce_number<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Num, D::Error> {
    use serde::de::Error as _;

    match Value::deserialize(deserializer)? {
        Value::Number(n) => Ok(n),
        Value::String(s) => {
            let trimmed = s.trim();

            if trimmed.is_empty() {
                return Ok(Number::from(0));
            }

            if let Ok(i) = trimmed.parse::<i64>() {
                return Ok(Number::from(i));
            }

            trimmed
                .parse::<f64>()
                .ok()
                .and_then(Number::from_f64)
                .ok_or_else(|| D::Error::custom(format!("cannot coerce {s:?} to a number")))
        }
        Value::Bool(b) => Ok(Number::from(b as i64)),
        Value::Null => Ok(Number::from(0)),
        other => Err(D::Error::custom(format!("cannot coerce {other} to a number"))),
    }
}

pub(crate) fn coerce_number_vec<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<Num>, D::Error> {
    #[derive(Deserialize)]
    struct Item(#[serde(deserialize_with = "coerce_number")] Num);

    Ok(Vec::<Item>::deserialize(deserializer)?
        .into_iter()
        .map(|item| item.0)
        .collect())
}

pub(crate) fn coerce_number_vec_opt<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Vec<Num>>, D::Error> {
    #[derive(Deserialize)]
    struct Item(#[serde(deserialize_with = "coerce_number")] Num);

    Ok(Option::<Vec<Item>>::deserialize(deserializer)?
        .map(|items| items.into_iter().map(|item| item.0).collect()))
}

// ---------------------------------------------------------------------------
// Prices
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PriceType {
    Gold,
    Gem,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Price {
    #[serde(rename = "type")]
    pub kind: PriceType,
    pub amount: Num,
}

/// `processPrice`: `{ g: 10 }` -> `{ type: gold, amount: 10 }`, `{}` -> `None`.
pub fn process_price(gem: Option<&Num>, gold: Option<&Num>) -> Result<Option<Price>> {
    match (gold, gem) {
        (None, None) => Ok(None),
        (Some(amount), None) => Ok(Some(Price { kind: PriceType::Gold, amount: amount.clone() })),
        (None, Some(amount)) => Ok(Some(Price { kind: PriceType::Gem, amount: amount.clone() })),
        (Some(_), Some(_)) => Err(Error::UnexpectedPrice(format!(
            "multiple price keys: {{\"g\":{},\"c\":{}}}",
            gold.map(|n| n.to_string()).unwrap_or_default(),
            gem.map(|n| n.to_string()).unwrap_or_default()
        ))),
    }
}

// ---------------------------------------------------------------------------
// Rewards
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RewardType {
    Chest,
    Dragon,
    Building,
    Gold,
    Gem,
    Food,
    HabitatToken,
    IslandCoin,
    EventPoints,
    DragonOrbs,
    RunnerIslandEnergy,
    PetFoodPack,
    AlbumPackAces,
}

/// A reward in the normalized shape produced by [`process_rewards`].
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ProcessedReward {
    #[serde(rename = "type")]
    pub kind: RewardType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element: Option<DragonElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Num>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chest_id: Option<Num>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dragon_id: Option<Num>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dragon_ids: Option<Vec<Num>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building_id: Option<Num>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orb_dragon_id: Option<Num>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

impl ProcessedReward {
    fn of(kind: RewardType) -> Self {
        Self {
            kind,
            element: None,
            amount: None,
            chest_id: None,
            dragon_id: None,
            dragon_ids: None,
            building_id: None,
            orb_dragon_id: None,
            size: None,
        }
    }

    fn with_amount(kind: RewardType, amount: Num) -> Self {
        Self { amount: Some(amount), ..Self::of(kind) }
    }
}

/// JavaScript truthiness of a JSON value.
pub(crate) fn truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().map_or(true, |f| f != 0.0 && !f.is_nan()),
        Value::String(s) => !s.is_empty(),
        Value::Array(_) | Value::Object(_) => true,
    }
}

fn truthy_entry<'a>(reward: &'a Map<String, Value>, key: &str) -> Option<&'a Value> {
    reward.get(key).filter(|value| truthy(value))
}

fn number_of(value: &Value, what: &str) -> Result<Num> {
    value
        .as_number()
        .cloned()
        .ok_or_else(|| Error::UnprocessedRewards(format!("{what} is not a number: {value}")))
}

/// `processRewards`: turns the compact reward objects of the game
/// (`{ "g": 10 }`, `{ "egg": 1042 }`, `{ "el_token": 3 }`, ...) into [`ProcessedReward`]s.
pub fn process_rewards(rewards: &[Map<String, Value>]) -> Result<Vec<ProcessedReward>> {
    const PET_FOOD_PACK_SIZES: [&str; 4] = ["s", "m", "l", "xl"];

    let filtered: Vec<&Map<String, Value>> = rewards.iter().filter(|r| !r.is_empty()).collect();
    let mut processed: Vec<ProcessedReward> = Vec::new();

    for reward in &filtered {
        if let Some(chest) = truthy_entry(reward, "chest") {
            processed.push(ProcessedReward {
                chest_id: Some(number_of(chest, "chest")?),
                ..ProcessedReward::of(RewardType::Chest)
            });
        }

        if let Some(egg) = truthy_entry(reward, "egg") {
            processed.push(match egg {
                Value::Array(ids) => ProcessedReward {
                    dragon_ids: Some(ids.iter().map(|id| number_of(id, "egg")).collect::<Result<_>>()?),
                    ..ProcessedReward::of(RewardType::Dragon)
                },
                other => ProcessedReward {
                    dragon_id: Some(number_of(other, "egg")?),
                    ..ProcessedReward::of(RewardType::Dragon)
                },
            });
        }

        for (key, kind) in [("g", RewardType::Gold), ("c", RewardType::Gem), ("f", RewardType::Food)] {
            if let Some(amount) = truthy_entry(reward, key) {
                processed.push(ProcessedReward::with_amount(kind, number_of(amount, key)?));
            }
        }

        if let Some(building) = truthy_entry(reward, "b") {
            let building_id = match building {
                Value::Array(ids) => ids.first().map(|id| number_of(id, "b")).transpose()?,
                other => Some(number_of(other, "b")?),
            };

            processed.push(ProcessedReward { building_id, ..ProcessedReward::of(RewardType::Building) });
        }

        if let Some(amount) = truthy_entry(reward, "ep") {
            processed.push(ProcessedReward::with_amount(RewardType::EventPoints, number_of(amount, "ep")?));
        }

        if let Some(seeds) = truthy_entry(reward, "seeds") {
            for seed in seeds.as_array().map(Vec::as_slice).unwrap_or_default() {
                processed.push(ProcessedReward {
                    orb_dragon_id: Some(number_of(&seed["id"], "seeds.id")?),
                    amount: Some(number_of(&seed["amount"], "seeds.amount")?),
                    ..ProcessedReward::of(RewardType::DragonOrbs)
                });
            }
        }

        if let Some(amount) = truthy_entry(reward, "en_runner") {
            processed.push(ProcessedReward::with_amount(
                RewardType::RunnerIslandEnergy,
                number_of(amount, "en_runner")?,
            ));
        }

        if let Some(key) = reward.keys().find(|key| key.ends_with("_token")) {
            let acronym = key.split('_').next().unwrap_or_default();

            if acronym.is_empty() {
                return Err(Error::UnprocessedRewards("invalid reward".to_owned()));
            }

            processed.push(ProcessedReward {
                element: DragonElement::from_acronym(acronym),
                amount: Some(number_of(&reward[key.as_str()], key)?),
                ..ProcessedReward::of(RewardType::HabitatToken)
            });
        }

        for size in PET_FOOD_PACK_SIZES {
            let key = format!("pet_food_pack.{size}");

            if let Some(amount) = reward.get(&key) {
                processed.push(ProcessedReward {
                    size: Some(size.to_owned()),
                    amount: Some(number_of(amount, &key)?),
                    ..ProcessedReward::of(RewardType::PetFoodPack)
                });
            }
        }

        if let Some(key) = reward.keys().find(|key| key.starts_with("album_pack_aces.")) {
            let size = key.rsplit('.').next().unwrap_or_default();

            if size.is_empty() {
                return Err(Error::UnprocessedRewards("invalid reward".to_owned()));
            }

            processed.push(ProcessedReward {
                size: Some(size.to_owned()),
                amount: Some(number_of(&reward[key.as_str()], key)?),
                ..ProcessedReward::of(RewardType::AlbumPackAces)
            });
        }
    }

    if processed.len() < filtered.len() {
        return Err(Error::UnprocessedRewards(format!(
            "{} -> {}",
            serde_json::to_string(rewards)?,
            serde_json::to_string(&processed)?
        )));
    }

    Ok(processed)
}

/// Serializes a struct into the `Map` form expected by [`process_rewards`].
pub(crate) fn reward_map<T: Serialize>(reward: &T) -> Result<Map<String, Value>> {
    match serde_json::to_value(reward)? {
        Value::Object(map) => Ok(map),
        other => Err(Error::UnprocessedRewards(format!("reward is not an object: {other}"))),
    }
}

// ---------------------------------------------------------------------------
// Grid decorations
// ---------------------------------------------------------------------------

fn coordinate(value: &Value, key: &str) -> Result<i64> {
    value
        .get(key)
        .and_then(|v| v.as_f64())
        .map(|f| f as i64)
        .ok_or_else(|| Error::MissingField(key.to_owned()))
}

/// `resolveGridDecorationPositions`: grid decorations come without coordinates; they occupy,
/// in order, the board cells that are not squares.
///
/// `squares` and `decorations` are the raw `grid_island.squares`/`grid_island.decorations`
/// objects; the returned decorations get their `x`/`y` filled in.
pub fn resolve_grid_decoration_positions(
    squares: &[Value],
    decorations: &[Value],
    board_width: i64,
    board_height: i64,
) -> Result<Vec<Value>> {
    let mut taken = std::collections::HashSet::with_capacity(squares.len());

    for square in squares {
        taken.insert((coordinate(square, "x")?, coordinate(square, "y")?));
    }

    let mut positions = Vec::new();

    for x in 1..=board_width {
        for y in 1..=board_height {
            if !taken.contains(&(x, y)) {
                positions.push((x, y));
            }
        }
    }

    if positions.len() != decorations.len() {
        return Err(Error::DecorationPositionMismatch);
    }

    Ok(decorations
        .iter()
        .zip(positions)
        .map(|(decoration, (x, y))| {
            let mut decoration = decoration.clone();

            if let Value::Object(map) = &mut decoration {
                map.insert("x".to_owned(), Value::from(x));
                map.insert("y".to_owned(), Value::from(y));
            }

            decoration
        })
        .collect())
}

// ---------------------------------------------------------------------------
// Small JSON helpers used by the mappers
// ---------------------------------------------------------------------------

/// Deserializes `value` into a strict schema, tagging the error with the schema name.
pub(crate) fn parse<'de, T: Deserialize<'de>>(schema: &'static str, value: &'de Value) -> Result<T> {
    T::deserialize(value).map_err(|source| Error::Schema { schema, source })
}

/// Clone of an object with extra fields injected (what the TS mappers do with `{ ...item, extra }`).
pub(crate) fn with_fields(value: &Value, fields: impl IntoIterator<Item = (&'static str, Value)>) -> Value {
    let mut value = value.clone();

    if let Value::Object(map) = &mut value {
        for (key, field) in fields {
            map.insert(key.to_owned(), field);
        }
    }

    value
}

/// Required array of a section.
pub(crate) fn array<'a>(section: &'a Value, key: &str) -> Result<&'a Vec<Value>> {
    section
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| Error::MissingField(key.to_owned()))
}

/// Ids (`id` field) of the elements of `items` whose `field` equals the `id` of `owner`.
pub(crate) fn ids_where(items: &[Value], field: &str, owner: &Value) -> Vec<Value> {
    let owner_id = owner.get("id");

    items
        .iter()
        .filter(|item| item.get(field).is_some() && item.get(field) == owner_id)
        .filter_map(|item| item.get("id").cloned())
        .collect()
}

/// Section value to copy as-is (`None` when absent, so it is left out of the JSON).
pub(crate) fn passthrough(section: &Value, key: &str) -> Option<Value> {
    section.get(key).cloned()
}

/// Typed element of a `Vec` (index access that tolerates short arrays).
pub(crate) fn nth(values: &[Num], index: usize) -> Option<Num> {
    values.get(index).cloned()
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Size {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Num>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Num>,
}

impl Size {
    pub(crate) fn from_pair(values: &[Num]) -> Self {
        Self { width: nth(values, 0), height: nth(values, 1) }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Availability {
    pub from: String,
    pub to: String,
}

impl Availability {
    pub(crate) fn from_ts(from: &Num, to: &Num) -> Result<Self> {
        Ok(Self { from: iso_from_ts(from)?, to: iso_from_ts(to)? })
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TsRange {
    pub from: Num,
    pub to: Num,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn rewards(value: Value) -> Vec<Map<String, Value>> {
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn booleans() {
        assert!(number_to_boolean(&Number::from(1)).unwrap());
        assert!(!number_to_boolean(&Number::from(0)).unwrap());
        assert!(number_to_boolean(&Number::from(2)).is_err());
    }

    #[test]
    fn iso_dates() {
        assert_eq!(iso_from_ts(&Number::from(0)).unwrap(), "1970-01-01T00:00:00.000Z");
        assert_eq!(iso_from_ts(&Number::from(1_700_000_000)).unwrap(), "2023-11-14T22:13:20.000Z");
    }

    #[test]
    fn basenames() {
        assert_eq!(basename("a/b/c.zip"), "c.zip");
        assert_eq!(basename("c.zip"), "c.zip");
    }

    #[test]
    fn arithmetic_keeps_integers() {
        let a = Number::from(5);
        let b = Number::from(2);
        assert_eq!(num_mul(&a, &num_sub(&a, &b).unwrap()).unwrap().to_string(), "15");
        assert_eq!(num_mul(&Number::from_f64(1.5).unwrap(), &b).unwrap().to_string(), "3");
    }

    #[test]
    fn prices() {
        let ten = Number::from(10);
        assert_eq!(process_price(None, None).unwrap(), None);
        assert_eq!(process_price(None, Some(&ten)).unwrap().unwrap().kind, PriceType::Gold);
        assert_eq!(process_price(Some(&ten), None).unwrap().unwrap().kind, PriceType::Gem);
        assert!(process_price(Some(&ten), Some(&ten)).is_err());
    }

    #[test]
    fn processes_every_reward_kind() {
        let out = process_rewards(&rewards(json!([
            {"chest": 5},
            {"egg": [1, 2]},
            {"egg": 7},
            {"g": 10},
            {"c": 3},
            {"f": 4},
            {"b": [9, 10]},
            {"ep": 6},
            {"seeds": [{"id": 1, "amount": 2}, {"id": 3, "amount": 4}]},
            {"en_runner": 2},
            {"el_token": 8},
            {"pet_food_pack.xl": 1},
            {"album_pack_aces.big": 2},
            {}
        ])))
        .unwrap();

        assert_eq!(out.len(), 14); // 13 non-empty entries, one of them with two seeds
        assert_eq!(serde_json::to_value(&out[0]).unwrap()["type"], "chest");
        assert_eq!(out[1].dragon_ids.as_ref().unwrap().len(), 2);
        assert_eq!(out[6].building_id.as_ref().unwrap().to_string(), "9");
        assert_eq!(out[11].element, Some(DragonElement::Electric));
        assert_eq!(out[12].size.as_deref(), Some("xl"));
        assert_eq!(out[13].size.as_deref(), Some("big"));
    }

    #[test]
    fn unknown_reward_is_an_error() {
        assert!(process_rewards(&rewards(json!([{"x": 1}]))).is_err());
    }

    #[test]
    fn grid_decorations_fill_free_cells() {
        let squares = vec![json!({"x": 1, "y": 1}), json!({"x": 2, "y": 2})];
        let decorations = vec![json!({"id": 1}), json!({"id": 2})];
        let out = resolve_grid_decoration_positions(&squares, &decorations, 2, 2).unwrap();

        assert_eq!((out[0]["x"].as_i64(), out[0]["y"].as_i64()), (Some(1), Some(2)));
        assert_eq!((out[1]["x"].as_i64(), out[1]["y"].as_i64()), (Some(2), Some(1)));
        assert!(resolve_grid_decoration_positions(&squares, &decorations[..1], 2, 2).is_err());
    }
}
