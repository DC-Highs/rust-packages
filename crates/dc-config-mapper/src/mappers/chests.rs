use std::collections::HashMap;

use serde_json::Value;

use super::{mapper, translated};
use crate::schemas::chests::{ChestIn, ChestRewardIn};
use crate::utils::{array, parse};
use crate::{Error, Result};

mapper!(
    /// Maps `config.chests`: resolves the reward ids of every chest and translates names.
    ChestsMapper
);

impl ChestsMapper {
    /// `section` is the `chests` section (`{ chests: [...], rewards: [...] }`).
    pub fn map(&self, section: &Value) -> Result<Vec<Value>> {
        let chests = array(section, "chests")?;
        let rewards = array(section, "rewards")?;

        let mut rewards_by_id: HashMap<String, &Value> = HashMap::with_capacity(rewards.len());

        for reward in rewards {
            if let Some(id) = reward.get("id").and_then(Value::as_number) {
                rewards_by_id.entry(id.to_string()).or_insert(reward);
            }
        }

        chests
            .iter()
            .map(|raw| {
                let chest: ChestIn = parse("chest", raw)?;

                let resolved = chest
                    .rewards
                    .iter()
                    .map(|id| {
                        let reward = rewards_by_id
                            .get(&id.to_string())
                            .ok_or_else(|| Error::RewardNotFound(Value::Number(id.clone())))?;

                        parse::<ChestRewardIn>("chest reward", reward)?.transform()
                    })
                    .collect::<Result<Vec<_>>>()?;

                translated(self.localization(), &chest.transform(resolved))
            })
            .collect()
    }
}
