use serde::Serialize;
use serde_json::Value;

use super::{mapper, translate_all, translated};
use crate::schemas::skills::AttackIn;
use crate::utils::{array, parse};
use crate::Result;

mapper!(
    /// Maps `config.skills`.
    SkillsMapper
);

#[derive(Debug, Clone, Serialize)]
pub struct MappedSkills {
    pub skills: Vec<Value>,
    pub attacks: Vec<Value>,
    pub auras: Vec<Value>,
    pub post: Vec<Value>,
    pub effects: Vec<Value>,
    pub passive: Vec<Value>,
    pub world_skills: Vec<Value>,
    pub world_effects: Vec<Value>,
    pub level_based_parameters: Vec<Value>,
}

impl SkillsMapper {
    /// `section` is the `skills` section.
    pub fn map(&self, section: &Value) -> Result<MappedSkills> {
        let loc = self.localization();

        let attacks = array(section, "attacks")?
            .iter()
            .map(|attack| translated(loc, &parse::<AttackIn>("attack", attack)?.transform()))
            .collect::<Result<Vec<_>>>()?;

        Ok(MappedSkills {
            skills: translate_all(loc, section, "skills")?,
            attacks,
            auras: translate_all(loc, section, "auras")?,
            post: translate_all(loc, section, "post")?,
            effects: translate_all(loc, section, "effects")?,
            passive: translate_all(loc, section, "passive")?,
            world_skills: translate_all(loc, section, "world_skills")?,
            world_effects: translate_all(loc, section, "world_effects")?,
            level_based_parameters: translate_all(loc, section, "level_based_parameters")?,
        })
    }
}
