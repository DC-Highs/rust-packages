use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreedingParent {
    pub id: u64,
    pub elements: Vec<String>,
    pub level: u32,
    #[serde(default)]
    pub empower_level: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreedingRule {
    pub element_one: String,
    pub element_two: String,
    pub dragon_id_1: u64,
    pub empower_1: u32,
    pub dragon_id_2: u64,
    pub empower_2: u32,
    pub dragon_id_3: u64,
    pub empower_3: u32,
    #[serde(default)]
    pub dragon_id_4: Option<u64>,
    #[serde(default)]
    pub empower_4: Option<u32>,
    #[serde(default)]
    pub dragon_id_5: Option<u64>,
    #[serde(default)]
    pub empower_5: Option<u32>,
    #[serde(default)]
    pub dragon_id_6: Option<u64>,
    #[serde(default)]
    pub empower_6: Option<u32>,
    #[serde(default)]
    pub dragon_id_7: Option<u64>,
    #[serde(default)]
    pub empower_7: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Incompatibility {
    pub element_one: String,
    pub element_two: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoulmateRule {
    pub parent_1_id: u64,
    pub parent_2_id: u64,
    pub dragon_id: u64,
    pub chance: f64,
    pub level_parents: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConditionWithIds {
    pub id: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecialBreedingRule {
    pub condition1: ConditionWithIds,
    pub condition2: ConditionWithIds,
    pub result: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BreedingConfig {
    pub breeding: Vec<BreedingRule>,
    #[serde(default)]
    pub incompatibilities: Vec<Incompatibility>,
    #[serde(default)]
    pub soulmates: Vec<SoulmateRule>,
    #[serde(default)]
    pub special_breeding: Vec<SpecialBreedingRule>,
    #[serde(default)]
    pub sanctuary_unlocked_dragons: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BreedingSource {
    Soulmate,
    Special,
    Sanctuary,
    Regular,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreedingResultItem {
    pub dragon_id: u64,
    pub sources: Vec<BreedingSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_empower: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chance: Option<f64>,
}

pub fn calculate_breeding_results(
    parent1: &BreedingParent,
    parent2: &BreedingParent,
    config: &BreedingConfig,
) -> Vec<BreedingResultItem> {
    let mut results_map: HashMap<u64, BreedingResultItem> = HashMap::new();

    let mut add_result = |dragon_id: u64, source: BreedingSource, empower: Option<u32>, chance: Option<f64>| {
        if dragon_id == 0 {
            return;
        }

        results_map
            .entry(dragon_id)
            .and_modify(|existing| {
                if !existing.sources.contains(&source) {
                    existing.sources.push(source);
                }
                if let Some(emp) = empower {
                    existing.required_empower = Some(
                        existing
                            .required_empower
                            .map_or(emp, |prev| prev.min(emp)),
                    );
                }
                if let Some(ch) = chance {
                    existing.chance = Some(existing.chance.map_or(ch, |prev| prev.max(ch)));
                }
            })
            .or_insert_with(|| BreedingResultItem {
                dragon_id,
                sources: vec![source],
                required_empower: empower,
                chance,
            });
    };

    // Special breeding
    for special in &config.special_breeding {
        let matches_p1_c1 = special.condition1.id.contains(&parent1.id);
        let matches_p2_c2 = special.condition2.id.contains(&parent2.id);

        let matches_p1_c2 = special.condition2.id.contains(&parent1.id);
        let matches_p2_c1 = special.condition1.id.contains(&parent2.id);

        if (matches_p1_c1 && matches_p2_c2) || (matches_p1_c2 && matches_p2_c1) {
            add_result(special.result, BreedingSource::Special, None, None);
        }
    }

    // Soulmates
    for soulmate in &config.soulmates {
        let matches = (soulmate.parent_1_id == parent1.id && soulmate.parent_2_id == parent2.id)
            || (soulmate.parent_1_id == parent2.id && soulmate.parent_2_id == parent1.id);
        let levels_match =
            parent1.level >= soulmate.level_parents && parent2.level >= soulmate.level_parents;

        if matches && levels_match {
            add_result(
                soulmate.dragon_id,
                BreedingSource::Soulmate,
                None,
                Some(soulmate.chance),
            );
        }
    }

    let is_incompatible = |e1: &str, e2: &str| {
        config.incompatibilities.iter().any(|inc| {
            (inc.element_one == e1 && inc.element_two == e2)
                || (inc.element_one == e2 && inc.element_two == e1)
        })
    };

    for e1 in &parent1.elements {
        for e2 in &parent2.elements {
            if is_incompatible(e1, e2) {
                continue;
            }

            let breeding_rule = config.breeding.iter().find(|b| {
                (b.element_one == *e1 && b.element_two == *e2)
                    || (b.element_one == *e2 && b.element_two == *e1)
            });

            if let Some(rule) = breeding_rule {
                let pairs: [(u64, u32); 7] = [
                    (rule.dragon_id_1, rule.empower_1),
                    (rule.dragon_id_2, rule.empower_2),
                    (rule.dragon_id_3, rule.empower_3),
                    (rule.dragon_id_4.unwrap_or(0), rule.empower_4.unwrap_or(0)),
                    (rule.dragon_id_5.unwrap_or(0), rule.empower_5.unwrap_or(0)),
                    (rule.dragon_id_6.unwrap_or(0), rule.empower_6.unwrap_or(0)),
                    (rule.dragon_id_7.unwrap_or(0), rule.empower_7.unwrap_or(0)),
                ];

                for (id, emp) in pairs {
                    if id > 0 {
                        let is_sanctuary = config.sanctuary_unlocked_dragons.contains(&id);
                        let source = if is_sanctuary {
                            BreedingSource::Sanctuary
                        } else {
                            BreedingSource::Regular
                        };
                        add_result(id, source, Some(emp), None);
                    }
                }
            }
        }
    }

    results_map.into_values().collect()
}
