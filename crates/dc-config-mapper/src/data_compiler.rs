use serde::Serialize;
use serde_json::{Map, Value};

use crate::utils::truthy;
use crate::{Error, Result};

const MAX_DEPTH: usize = 128;

/// Named collections (`"dragons"`, `"attacks"`, `"rewards"`, ...) used to resolve ids.
#[derive(Debug, Clone, Default)]
pub struct CompileContext(Map<String, Value>);

impl CompileContext {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a collection, e.g. `.with("dragons", &items.dragons())?`. Each element needs an `id`.
    pub fn with<T: Serialize + ?Sized>(mut self, key: impl Into<String>, collection: &T) -> Result<Self> {
        self.0.insert(key.into(), serde_json::to_value(collection)?);
        Ok(self)
    }

    pub fn insert(&mut self, key: impl Into<String>, collection: Value) {
        self.0.insert(key.into(), collection);
    }
}

impl From<Map<String, Value>> for CompileContext {
    fn from(map: Map<String, Value>) -> Self {
        Self(map)
    }
}

/// Resolves id references in mapped data using a [`CompileContext`]:
///
/// * `"dragon_id": 5` becomes `"dragon": { ...dragon 5... }`
/// * `"reward_ids": [1, 2]` becomes `"rewards": [ {...}, {...} ]`
///
/// Resolved items are compiled recursively. Keys may carry one of the `accepted_prefixes`
/// (`"basic_attack_ids"` with prefix `"basic_"` becomes `"basic_attacks"`).
#[derive(Debug, Clone)]
pub struct DataCompiler {
    accepted_prefixes: Vec<String>,
    ids_suffix: String,
    id_suffix: String,
}

impl Default for DataCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl DataCompiler {
    pub fn new() -> Self {
        Self {
            accepted_prefixes: Vec::new(),
            ids_suffix: "_ids".to_owned(),
            id_suffix: "_id".to_owned(),
        }
    }

    /// Prefixes a key may have before the name of the collection, e.g. `["basic_", "trainable_"]`.
    pub fn accepted_prefixes<I, S>(mut self, prefixes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.accepted_prefixes = prefixes.into_iter().map(Into::into).collect();
        self
    }

    /// Suffix of keys holding a list of ids (default `_ids`).
    pub fn ids_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.ids_suffix = suffix.into();
        self
    }

    /// Suffix of keys holding a single id (default `_id`).
    pub fn id_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = suffix.into();
        self
    }

    /// Returns a compiled copy of `data`.
    pub fn compile(&self, data: &Value, context: &CompileContext) -> Result<Value> {
        self.compile_value(data, &context.0, 0)
    }

    pub fn compile_many(&self, data: &[Value], context: &CompileContext) -> Result<Vec<Value>> {
        data.iter().map(|item| self.compile(item, context)).collect()
    }

    fn compile_value(&self, value: &Value, context: &Map<String, Value>, depth: usize) -> Result<Value> {
        if depth > MAX_DEPTH {
            return Err(Error::MaxDepthExceeded(MAX_DEPTH));
        }

        match value {
            Value::Object(object) => Ok(Value::Object(self.compile_object(object, context, depth)?)),
            Value::Array(items) => Ok(Value::Array(
                items
                    .iter()
                    .map(|item| self.compile_value(item, context, depth + 1))
                    .collect::<Result<_>>()?,
            )),
            other => Ok(other.clone()),
        }
    }

    fn matching_prefix(&self, key: &str) -> &str {
        self.accepted_prefixes
            .iter()
            .find(|prefix| key.starts_with(prefix.as_str()))
            .map(String::as_str)
            .unwrap_or("")
    }

    /// `key` without `prefix` and `suffix`.
    fn base_key<'a>(key: &'a str, prefix: &str, suffix: &str) -> &'a str {
        let rest = &key[prefix.len().min(key.len())..];
        &rest[..rest.len().saturating_sub(suffix.len())]
    }

    fn collection<'a>(context: &'a Map<String, Value>, name: &str) -> Result<Option<&'a Vec<Value>>> {
        match context.get(name).filter(|value| truthy(value)) {
            None => Ok(None),
            Some(Value::Array(items)) => Ok(Some(items)),
            Some(_) => Err(Error::InvalidContext(name.to_owned())),
        }
    }

    fn compile_object(
        &self,
        original: &Map<String, Value>,
        context: &Map<String, Value>,
        depth: usize,
    ) -> Result<Map<String, Value>> {
        let mut data = original.clone();

        for (key, value) in original {
            if value.is_array() || value.is_object() {
                data.insert(key.clone(), self.compile_value(value, context, depth + 1)?);
            }

            let prefix = self.matching_prefix(key);

            if key.ends_with(&self.ids_suffix) && value.is_array() {
                let base = Self::base_key(key, prefix, &self.ids_suffix);
                let plural = pluralizer::pluralize(base, 2, false);

                if let Some(items) = Self::collection(context, &plural)? {
                    let related = value
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter_map(|id| items.iter().find(|item| same_id(item.get("id"), id)))
                        .map(|found| self.compile_value(found, context, depth + 1))
                        .collect::<Result<Vec<_>>>()?;

                    data.insert(format!("{prefix}{plural}"), Value::Array(related));
                    data.shift_remove(key);
                }
            } else if key.ends_with(&self.id_suffix) && truthy(value) {
                let base = Self::base_key(key, prefix, &self.id_suffix);
                let plural = pluralizer::pluralize(base, 2, false);

                if let Some(items) = Self::collection(context, &plural)? {
                    let compiled = match items.iter().find(|item| same_id(item.get("id"), value)) {
                        Some(found) => self.compile_value(found, context, depth + 1)?,
                        None => Value::Null,
                    };

                    data.insert(format!("{prefix}{base}"), compiled);
                    data.shift_remove(key);
                }
            }
        }

        Ok(data)
    }
}

/// `item.id === id` (`1` and `1.0` are the same number in JavaScript).
fn same_id(item_id: Option<&Value>, id: &Value) -> bool {
    match (item_id, id) {
        (Some(Value::Number(a)), Value::Number(b)) => a.as_f64() == b.as_f64(),
        (Some(a), b) => a == b,
        (None, _) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn context() -> CompileContext {
        CompileContext::new()
            .with("dragons", &json!([{"id": 1, "name": "A", "attack_ids": [10]}, {"id": 2, "name": "B"}]))
            .unwrap()
            .with("attacks", &json!([{"id": 10, "name": "Bite"}]))
            .unwrap()
            .with("rewards", &json!([{"id": 7, "dragon_id": 2}, {"id": 8, "dragon_id": 1}]))
            .unwrap()
    }

    #[test]
    fn resolves_single_ids_recursively() {
        let out = DataCompiler::new()
            .compile(&json!({"id": 5, "reward_id": 7}), &context())
            .unwrap();

        assert_eq!(out, json!({"id": 5, "reward": {"id": 7, "dragon": {"id": 2, "name": "B"}}}));
    }

    #[test]
    fn resolves_id_lists_and_drops_unknown_ids() {
        let out = DataCompiler::new()
            .compile(&json!({"reward_ids": [8, 99]}), &context())
            .unwrap();

        assert_eq!(
            out,
            json!({"rewards": [{"id": 8, "dragon": {"id": 1, "name": "A", "attacks": [{"id": 10, "name": "Bite"}]}}]})
        );
    }

    #[test]
    fn unknown_id_becomes_null_and_unknown_collections_are_left_alone() {
        let compiler = DataCompiler::new();
        let ctx = context();

        assert_eq!(compiler.compile(&json!({"dragon_id": 404}), &ctx).unwrap(), json!({"dragon": null}));
        assert_eq!(compiler.compile(&json!({"island_id": 3}), &ctx).unwrap(), json!({"island_id": 3}));
        assert_eq!(compiler.compile(&json!({"dragon_id": 0}), &ctx).unwrap(), json!({"dragon_id": 0}));
    }

    #[test]
    fn prefixes_are_kept() {
        let out = DataCompiler::new()
            .accepted_prefixes(["basic_", "trainable_"])
            .compile(&json!({"basic_attack_ids": [10], "trainable_attack_ids": []}), &context())
            .unwrap();

        assert_eq!(out, json!({"basic_attacks": [{"id": 10, "name": "Bite"}], "trainable_attacks": []}));
    }

    #[test]
    fn walks_nested_objects_and_arrays() {
        let out = DataCompiler::new()
            .compile(&json!({"list": [{"dragon_id": 1}, 3, "x"], "inner": {"attack_id": 10}}), &context())
            .unwrap();

        assert_eq!(out["list"][0]["dragon"]["name"], "A");
        assert_eq!(out["list"][1], 3);
        assert_eq!(out["inner"]["attack"]["name"], "Bite");
    }

    #[test]
    fn circular_references_error_instead_of_overflowing() {
        let ctx = CompileContext::new()
            .with("dragons", &json!([{"id": 1, "dragon_id": 1}]))
            .unwrap();

        assert!(matches!(
            DataCompiler::new().compile(&json!({"dragon_id": 1}), &ctx),
            Err(Error::MaxDepthExceeded(_))
        ));
    }
}
