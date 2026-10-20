use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DragonCategoriesStatistics {
    pub unique_categories_count: usize,
    pub dragon_counts_per_category: HashMap<u8, usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DragonElementsStatistics {
    pub unique_elements_count: usize,
    pub dragons_with_one_element: usize,
    pub dragons_with_two_elements: usize,
    pub dragons_with_three_elements: usize,
    pub dragons_with_four_elements: usize,
    pub dragons_with_other_elements: usize,
    pub dragon_counts_per_element: HashMap<String, usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DragonsStatistics {
    pub total_count: usize,
    pub common_count: usize,
    pub rare_count: usize,
    pub very_rare_count: usize,
    pub epic_count: usize,
    pub legendary_count: usize,
    pub mythical_count: usize,
    pub heroic_count: usize,
    pub elements: DragonElementsStatistics,
    pub categories: DragonCategoriesStatistics,
    pub breedable_count: usize,
    pub in_store_count: usize,
}

pub fn generate_dragons_statistics(dragons: &[Value]) -> DragonsStatistics {
    let mut common_count = 0;
    let mut rare_count = 0;
    let mut very_rare_count = 0;
    let mut epic_count = 0;
    let mut legendary_count = 0;
    let mut mythical_count = 0;
    let mut heroic_count = 0;

    let mut breedable_count = 0;
    let mut in_store_count = 0;

    for dragon in dragons {
        let rarity = dragon
            .get("dragon_rarity")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        match rarity {
            "C" => common_count += 1,
            "R" => rare_count += 1,
            "V" => very_rare_count += 1,
            "E" => epic_count += 1,
            "L" => legendary_count += 1,
            "M" => mythical_count += 1,
            "H" => heroic_count += 1,
            _ => {}
        }

        let breedable = dragon
            .get("breedable")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        let can_breed = dragon
            .get("can_breed")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);

        if breedable == 1 || can_breed == 1 {
            breedable_count += 1;
        }

        let in_store = dragon
            .get("in_store")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);

        if in_store == 1 {
            in_store_count += 1;
        }
    }

    DragonsStatistics {
        total_count: dragons.len(),
        common_count,
        rare_count,
        very_rare_count,
        epic_count,
        legendary_count,
        mythical_count,
        heroic_count,
        elements: generate_dragon_elements_statistics(dragons),
        categories: generate_dragon_categories_statistics(dragons),
        breedable_count,
        in_store_count,
    }
}

pub fn generate_dragon_categories_statistics(dragons: &[Value]) -> DragonCategoriesStatistics {
    let mut category_counts: HashMap<u8, usize> = HashMap::new();

    for dragon in dragons {
        if let Some(cat_val) = dragon.get("category").and_then(|c| c.as_u64()) {
            *category_counts.entry(cat_val as u8).or_insert(0) += 1;
        }
    }

    DragonCategoriesStatistics {
        unique_categories_count: category_counts.len(),
        dragon_counts_per_category: category_counts,
    }
}

pub fn generate_dragon_elements_statistics(dragons: &[Value]) -> DragonElementsStatistics {
    let mut element_counts: HashMap<String, usize> = HashMap::new();
    let mut dragons_with_one_element = 0;
    let mut dragons_with_two_elements = 0;
    let mut dragons_with_three_elements = 0;
    let mut dragons_with_four_elements = 0;
    let mut dragons_with_other_elements = 0;

    for dragon in dragons {
        let attributes: Vec<String> = dragon
            .get("attributes")
            .and_then(|a| a.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| item.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        match attributes.len() {
            1 => dragons_with_one_element += 1,
            2 => dragons_with_two_elements += 1,
            3 => dragons_with_three_elements += 1,
            4 => dragons_with_four_elements += 1,
            _ => dragons_with_other_elements += 1,
        }

        for attr in attributes {
            *element_counts.entry(attr).or_insert(0) += 1;
        }
    }

    DragonElementsStatistics {
        unique_elements_count: element_counts.len(),
        dragons_with_one_element,
        dragons_with_two_elements,
        dragons_with_three_elements,
        dragons_with_four_elements,
        dragons_with_other_elements,
        dragon_counts_per_element: element_counts,
    }
}
