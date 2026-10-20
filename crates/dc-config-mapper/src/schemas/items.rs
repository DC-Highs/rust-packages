#![allow(dead_code)] // some fields only exist so that unknown keys are rejected like the strict zod schemas do

use dc_core::DragonElement;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{number_to_boolean, process_price, Num, Price};
use crate::Result;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Costs {
    #[serde(default)]
    pub c: Option<Num>,
    #[serde(default)]
    pub g: Option<Num>,
}

impl Costs {
    fn price(&self) -> Result<Option<Price>> {
        process_price(self.c.as_ref(), self.g.as_ref())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GoldPrice {
    #[serde(default)]
    pub g: Option<Num>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RequiredGoldPrice {
    pub g: Num,
}

// ---------------------------------------------------------------------------
// Dragons
// ---------------------------------------------------------------------------

/// Raw dragon (`items` with `group_type == "DRAGON"`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DragonIn {
    pub id: Num,
    pub name: String,
    pub group_type: String,
    pub speed: Num,
    pub mobile_version: Num,
    pub android_version: Num,
    pub display_order: Num,
    pub hatching_time: Num,
    pub breeding_time: Num,
    pub xp: Num,
    pub lvl: Num,
    pub costs: Costs,
    pub sell_price: RequiredGoldPrice,
    pub starting_coins: Num,
    pub coins_added: Num,
    pub can_breed: Num,
    pub breedable: Num,
    pub deity_breeding: Num,
    pub difficulty: Num,
    pub category: Num,
    pub rarity: Num,
    pub dragon_rarity: String,
    pub attacks: Vec<Num>,
    pub attributes: Vec<String>,
    pub trainable_attacks: Vec<Num>,
    pub upgrade_levels: Vec<Num>,
    pub background_vfx: Vec<Num>,
    pub foreground_vfx: Vec<Num>,
    pub in_store: Num,
    pub in_store_min_level: Num,
    pub flying_level: Num,
    pub swim: Num,
    pub new_item: Num,
    #[serde(default)]
    pub img_name: Option<String>,
    #[serde(default)]
    pub img_name_mobile: Option<String>,
    #[serde(default)]
    pub img_name_android: Option<String>,
    #[serde(default)]
    pub giftable: Option<Num>,
    pub gift_level: Num,
    #[serde(default)]
    pub description: Option<String>,
    pub seeds_to_summon: Num,
    pub hatching_time_10: Num,
    pub breeding_time_40: Num,
    pub breeding_time_10: Num,
    pub hatching_time_40: Num,
    pub base_attack: Num,
    pub base_life: Num,
    #[serde(default)]
    pub in_store_ch: Option<Num>,
    #[serde(default)]
    pub costs_ch_1: Option<Costs>,
    #[serde(default)]
    pub costs_ch_2: Option<Costs>,
    #[serde(default)]
    pub hatching_time_reawaken: Option<Num>,
    #[serde(default)]
    pub breeding_time_reawaken: Option<Num>,
    #[serde(default)]
    pub hatching_time_ngu_aggressive: Option<Num>,
    #[serde(default)]
    pub breeding_time_ngu_aggressive: Option<Num>,
    #[serde(default)]
    pub hatching_time_ngu_soft: Option<Num>,
    #[serde(default)]
    pub breeding_time_ngu_soft: Option<Num>,
    pub dragon_ownership_id: Num,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub speed_override: Option<Num>,
    #[serde(default)]
    pub passive_skills: Option<Vec<Num>>,
    #[serde(default)]
    pub post_skills: Option<Vec<Num>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DragonStats {
    pub attack: Num,
    pub life: Num,
    pub speed: Num,
    pub level: Num,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_override: Option<Num>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HatchingTimers {
    pub default: Num,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatching_reawaken: Option<Num>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatching_ngu_aggressive: Option<Num>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatching_ngu_soft: Option<Num>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BreedingTimers {
    pub default: Num,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breeding_reawaken: Option<Num>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breeding_ngu_aggressive: Option<Num>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breeding_ngu_soft: Option<Num>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DragonTimers {
    pub hatching: HatchingTimers,
    pub breeding: BreedingTimers,
}

#[derive(Debug, Clone, Serialize)]
pub struct DragonProduction {
    pub coins_added: Num,
    pub starting_coins: Num,
}

#[derive(Debug, Clone, Serialize)]
pub struct DragonImageNames {
    pub default: Option<String>,
    pub mobile: Option<String>,
    pub android: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DragonOut {
    pub id: Num,
    pub name: String,
    pub name_key: String,
    pub rarity: String,
    /// `null` for elements whose acronym is unknown.
    pub elements: Vec<Option<DragonElement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub description_key: String,
    pub buy_price: Option<Price>,
    pub sell_price: Option<Price>,
    pub is_breedable: bool,
    pub can_breed: bool,
    pub is_in_store: bool,
    pub min_store_level: Num,
    pub basic_attack_ids: Vec<Num>,
    pub trainable_attack_ids: Vec<Num>,
    pub passive_skill_ids: Option<Vec<Num>>,
    pub post_skill_ids: Option<Vec<Num>>,
    pub category: Num,
    pub stats: DragonStats,
    pub timers: DragonTimers,
    pub production: DragonProduction,
    pub xp_on_hatch: Num,
    pub seeds_to_summon: Num,
    pub upgrade_phase_levels: Vec<Num>,
    pub image_names: DragonImageNames,
    pub tags: Vec<String>,
}

impl DragonIn {
    pub fn transform(self) -> Result<DragonOut> {
        let sell_price = process_price(None, Some(&self.sell_price.g))?;

        Ok(DragonOut {
            name_key: format!("tid_unit_{}_name", self.id),
            description_key: format!("tid_unit_{}_description", self.id),
            buy_price: self.costs.price()?,
            sell_price,
            is_breedable: number_to_boolean(&self.breedable)?,
            can_breed: number_to_boolean(&self.can_breed)?,
            is_in_store: number_to_boolean(&self.in_store)?,
            elements: self
                .attributes
                .iter()
                .map(|acronym| DragonElement::from_acronym(acronym))
                .collect(),
            id: self.id,
            name: self.name,
            rarity: self.dragon_rarity,
            description: self.description,
            min_store_level: self.in_store_min_level,
            basic_attack_ids: self.attacks,
            trainable_attack_ids: self.trainable_attacks,
            passive_skill_ids: self.passive_skills,
            post_skill_ids: self.post_skills,
            category: self.category,
            stats: DragonStats {
                attack: self.base_attack,
                life: self.base_life,
                speed: self.speed,
                level: self.lvl,
                speed_override: self.speed_override,
            },
            timers: DragonTimers {
                hatching: HatchingTimers {
                    default: self.hatching_time,
                    hatching_reawaken: self.hatching_time_reawaken,
                    hatching_ngu_aggressive: self.hatching_time_ngu_aggressive,
                    hatching_ngu_soft: self.hatching_time_ngu_soft,
                },
                breeding: BreedingTimers {
                    default: self.breeding_time,
                    breeding_reawaken: self.breeding_time_reawaken,
                    breeding_ngu_aggressive: self.breeding_time_ngu_aggressive,
                    breeding_ngu_soft: self.breeding_time_ngu_soft,
                },
            },
            production: DragonProduction {
                coins_added: self.coins_added,
                starting_coins: self.starting_coins,
            },
            xp_on_hatch: self.xp,
            seeds_to_summon: self.seeds_to_summon,
            upgrade_phase_levels: self.upgrade_levels,
            image_names: DragonImageNames {
                default: self.img_name,
                mobile: self.img_name_mobile,
                android: self.img_name_android,
            },
            tags: self.tags.unwrap_or_default(),
        })
    }
}

// ---------------------------------------------------------------------------
// Buildings
// ---------------------------------------------------------------------------

/// `properties` of a building. Unknown keys are dropped, like zod does for non-strict objects.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BuildingProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incubator: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bulldozable: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_dragon_level: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_items: Option<Vec<Num>>,
    #[serde(default, rename = "workingBuilding", skip_serializing_if = "Option::is_none")]
    pub working_building: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breeding: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breeding_factor: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_gold: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub friend_assistable: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upgrade_from: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub floating: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_position: Option<Vec<Num>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_fixed: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crosspromotion: Option<Num>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ft_flying: Option<Num>,
}

/// Raw building (`items` with `group_type == "BUILDING"`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildingIn {
    pub id: Num,
    pub name: String,
    pub group_type: String,
    #[serde(default)]
    pub name_key: Option<String>,
    pub animated: Num,
    pub build_time: Num,
    pub mobile_version: Num,
    pub android_version: Num,
    pub show_on_mobile: Num,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub element_type: Option<String>,
    pub xp: Num,
    pub min_level: Num,
    pub costs: Costs,
    pub display_order: Num,
    pub boost_modifier: Num,
    pub gift_level: Num,
    pub giftable: Num,
    #[serde(default)]
    pub cost_unit_cash: Option<Num>,
    pub collect: Num,
    #[serde(default)]
    pub collect_xp: Option<Num>,
    pub category_id: Num,
    pub subcategory_id: Num,
    pub building_limit_same_id: Num,
    pub in_store: Num,
    pub width: Num,
    pub height: Num,
    pub new_item: Num,
    pub img_name: String,
    pub img_name_mobile: String,
    pub img_name_android: String,
    pub activation: Num,
    pub upgrades_to: Num,
    #[serde(default)]
    pub collect_type: Option<Value>,
    pub velocity: Num,
    #[serde(default)]
    pub description: Option<String>,
    pub properties: BuildingProperties,
    pub building_ownership_id: Num,
    #[serde(default)]
    pub sell_price: Option<GoldPrice>,
    #[serde(default)]
    pub inventory_ids: Option<Num>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BuildingImageNames {
    pub default: String,
    pub mobile: String,
    pub android: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BuildingOut {
    pub id: Num,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_key: Option<String>,
    pub animated: bool,
    pub build_time: Num,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type: Option<String>,
    pub xp_on_build: Num,
    pub min_level: Num,
    pub buy_price: Option<Price>,
    pub display_order: Num,
    pub boost_modifier: Num,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_unit_cash: Option<Num>,
    pub collect: Num,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collect_xp: Option<Num>,
    pub category_id: Num,
    pub subcategory_id: Num,
    pub building_limit_same_id: Num,
    pub in_store: bool,
    pub width: Num,
    pub height: Num,
    pub new_item: bool,
    #[serde(rename = "imageNames")]
    pub image_names: BuildingImageNames,
    pub activation: Num,
    pub upgrades_to: Num,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collect_type: Option<Value>,
    pub velocity: Num,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub properties: BuildingProperties,
    pub building_ownership_id: Num,
    pub sell_price: Option<Price>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_ids: Option<Num>,
}

impl BuildingIn {
    pub fn transform(self) -> Result<BuildingOut> {
        let sell_price = match &self.sell_price {
            Some(price) => process_price(None, price.g.as_ref())?,
            None => None,
        };

        Ok(BuildingOut {
            buy_price: self.costs.price()?,
            animated: number_to_boolean(&self.animated)?,
            in_store: number_to_boolean(&self.in_store)?,
            new_item: number_to_boolean(&self.new_item)?,
            sell_price,
            id: self.id,
            name: self.name,
            name_key: self.name_key,
            build_time: self.build_time,
            kind: self.kind,
            element_type: self.element_type,
            xp_on_build: self.xp,
            min_level: self.min_level,
            display_order: self.display_order,
            boost_modifier: self.boost_modifier,
            cost_unit_cash: self.cost_unit_cash,
            collect: self.collect,
            collect_xp: self.collect_xp,
            category_id: self.category_id,
            subcategory_id: self.subcategory_id,
            building_limit_same_id: self.building_limit_same_id,
            width: self.width,
            height: self.height,
            image_names: BuildingImageNames {
                default: self.img_name,
                mobile: self.img_name_mobile,
                android: self.img_name_android,
            },
            activation: self.activation,
            upgrades_to: self.upgrades_to,
            collect_type: self.collect_type,
            velocity: self.velocity,
            description: self.description,
            properties: self.properties,
            building_ownership_id: self.building_ownership_id,
            inventory_ids: self.inventory_ids,
        })
    }
}
