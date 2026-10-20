#![allow(dead_code)]

use dc_core::DragonElement;
use serde::{Deserialize, Serialize};

use crate::utils::Num;

/// Raw attack (`skills.attacks[]`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AttackIn {
    pub id: Num,
    pub name: String,
    pub name_key: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub element: String,
    pub damage: Num,
    pub ui_damage: Num,
    #[serde(default)]
    pub effect_atk: Option<String>,
    pub effect_def: String,
    pub effect_sfx: Num,
    pub training_time: Num,
    pub button_style: Num,
    pub special_icon: Num,
    #[serde(default)]
    pub skill_id: Option<Num>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AttackOut {
    pub id: Num,
    pub name: String,
    pub name_key: String,
    /// Omitted when the element acronym is unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element: Option<DragonElement>,
    pub damage: Num,
    pub ui_damage: Num,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_atk: Option<String>,
    pub effect_def: String,
    pub effect_sfx: Num,
    pub training_time: Num,
    pub button_style: Num,
    pub special_icon: Num,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_id: Option<Num>,
}

impl AttackIn {
    pub fn transform(self) -> AttackOut {
        AttackOut {
            element: DragonElement::from_acronym(&self.element),
            id: self.id,
            name: self.name,
            name_key: self.name_key,
            damage: self.damage,
            ui_damage: self.ui_damage,
            effect_atk: self.effect_atk,
            effect_def: self.effect_def,
            effect_sfx: self.effect_sfx,
            training_time: self.training_time,
            button_style: self.button_style,
            special_icon: self.special_icon,
            skill_id: self.skill_id,
        }
    }
}
