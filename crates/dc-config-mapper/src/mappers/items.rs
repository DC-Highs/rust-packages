use serde::Serialize;
use serde_json::{Map, Value};

use super::{mapper, translated};
use crate::schemas::items::{BuildingIn, DragonIn};
use crate::utils::parse;
use crate::Result;

mapper!(
    /// Maps `config.items`: groups the items by `group_type` (`"DRAGON"` -> `dragons`,
    /// `"SOCIAL BUILDING"` -> `social_buildings`, ...), validates dragons and buildings and translates names.
    ItemsMapper
);

/// `group_type` of the game -> key of the mapped result.
const GROUPS: &[(&str, &str)] = &[
    ("ALLIANCE_PORTAL", "alliance_portals"),
    ("BUILDING", "buildings"),
    ("BOOSTER", "boosters"),
    ("COLLECTIBLE", "collectibles"),
    ("COLLECTISLAND", "collectislands"),
    ("FARM", "farms"),
    ("HABITAT", "habitats"),
    ("ALLIANCE_RACE", "alliance_races"),
    ("CROSSPROMOTION", "crosspromotions"),
    ("ATREZZO", "atrezzos"),
    ("BRAND_DECO", "brand_decos"),
    ("BREEDING_ISLAND", "breeding_islands"),
    ("BURROW", "burrows"),
    ("CHESTCONTAINER", "chestcontainers"),
    ("COLLECTORS_HUNT", "collectors_hunts"),
    ("CUM_CALENDAR_ISLAND", "cum_calendar_islands"),
    ("DECO", "decos"),
    ("DEUSISLAND", "deusislands"),
    ("DRAGON_CINEMA", "dragon_cinemas"),
    ("DRAGON_OBSTACLE", "dragon_obstacles"),
    ("DRAGON_PETS", "dragon_pets"),
    ("DRAGON_TRAILS", "dragon_trails"),
    ("DRAGON_TV", "dragon_tvs"),
    ("DRAGONARIUM", "dragonariums"),
    ("DWPORTAL", "dwportals"),
    ("EVENT_ISLAND", "event_islands"),
    ("FOG_ISLAND", "fog_islands"),
    ("GREENHOUSE", "greenhouses"),
    ("GD_TOWER", "gd_towers"),
    ("GRID_ISLAND", "grid_islands"),
    ("HALLOWEEN_ISLAND", "halloween_islands"),
    ("HEROIC_RACE", "heroic_races"),
    ("KINDERGARTEN", "kindergartens"),
    ("LAND", "lands"),
    ("LOSTITEM", "lostitems"),
    ("MAGE_SHRINE", "mage_shrines"),
    ("MAZE_ISLAND", "maze_islands"),
    ("MEME_GENERATOR", "meme_generators"),
    ("MISTERY_EGG", "mistery_eggs"),
    ("ORB_HABITAT", "orb_habitats"),
    ("OBSTACLE", "obstacles"),
    ("OFFERWALL", "offerwalls"),
    ("POPUP", "popups"),
    ("PROGRESSIVE_ISLAND", "progressive_islands"),
    ("PROMOISLAND", "promoislands"),
    ("PROMO", "promos"),
    ("PUZZLE_ISLAND", "puzzle_islands"),
    ("RUNNER_ISLAND", "runner_islands"),
    ("SOCIAL BUILDING", "social_buildings"),
    ("TEASER_ISLAND", "teaser_islands"),
    ("TOWER_ISLAND", "tower_islands"),
    ("TRADER_ISLAND", "trader_islands"),
    ("TREE_OF_LIFE", "tree_of_lives"),
    ("UNIT", "units"),
    ("WIZARDS_CAVE", "wizards_caves"),
    ("DRAGON", "dragons"),
];

/// Key of a group in the mapped result. Unknown group types fall back to the generic
/// rule (lowercase, pluralize, spaces to underscores) so new game groups still show up.
fn group_key(group_type: &str) -> String {
    GROUPS
        .iter()
        .find(|(known, _)| *known == group_type)
        .map(|(_, key)| (*key).to_owned())
        .unwrap_or_else(|| pluralizer::pluralize(&group_type.to_lowercase(), 2, false).replace(' ', "_"))
}

/// Result of [`ItemsMapper::map`]: item groups by key (`dragons`, `buildings`, `decos`, ...).
///
/// Every known group is present, empty when the configuration has no item of that group.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(transparent)]
pub struct MappedItems(pub Map<String, Value>);

impl MappedItems {
    /// Items of a group, e.g. `items.group("habitats")`.
    pub fn group(&self, key: &str) -> &[Value] {
        self.0.get(key).and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default()
    }

    pub fn dragons(&self) -> &[Value] {
        self.group("dragons")
    }

    pub fn buildings(&self) -> &[Value] {
        self.group("buildings")
    }

    /// Names of the groups.
    pub fn group_keys(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(String::as_str)
    }
}

impl ItemsMapper {
    /// `items` is the `items` section: an array of items with a `group_type`.
    pub fn map(&self, items: &Value) -> Result<MappedItems> {
        let items = items
            .as_array()
            .ok_or_else(|| crate::Error::MissingField("items".to_owned()))?;

        let mut groups: Map<String, Value> = GROUPS
            .iter()
            .map(|(_, key)| ((*key).to_owned(), Value::Array(Vec::new())))
            .collect();

        for item in items {
            let Some(group_type) = item.get("group_type").and_then(Value::as_str) else {
                continue;
            };

            let key = group_key(group_type);

            let mapped = match key.as_str() {
                "dragons" => translated(
                    self.localization(),
                    &parse::<DragonIn>("dragon", item)?.transform()?,
                )?,
                "buildings" => translated(
                    self.localization(),
                    &parse::<BuildingIn>("building", item)?.transform()?,
                )?,
                _ => self.localization().translate(item.clone()),
            };

            if let Value::Array(group) = groups.entry(key).or_insert_with(|| Value::Array(Vec::new())) {
                group.push(mapped);
            }
        }

        Ok(MappedItems(groups))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_rule_agrees_with_the_known_groups() {
        for (group_type, key) in GROUPS {
            let generic = pluralizer::pluralize(&group_type.to_lowercase(), 2, false).replace(' ', "_");
            assert_eq!(&generic, key, "group {group_type}");
        }
    }
}
