use std::collections::HashMap;

use dc_core::ConfigLanguage;
use serde_json::{Map, Value};
use unicode_normalization::UnicodeNormalization;

use crate::keys::{
    DESCRIPTION_KEYS_TO_TRANSLATE, NAME_KEYS_TO_TRANSLATE, TYPE_NAME_KEYS_TO_TRANSLATE,
};

pub type LocalizationData = HashMap<String, String>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("languages do not match: {0} != {1}")]
    LanguageMismatch(ConfigLanguage, ConfigLanguage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparisonResultNewField {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparisonResultEditedFieldValues {
    pub old: String,
    pub new: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparisonResultEditedField {
    pub key: String,
    pub values: ComparisonResultEditedFieldValues,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparisonResultDeletedField {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ComparisonResult {
    pub new_fields: Vec<ComparisonResultNewField>,
    pub edited_fields: Vec<ComparisonResultEditedField>,
    pub deleted_fields: Vec<ComparisonResultDeletedField>,
}

/// Text normalization used by [`Localization::search_keys`] and
/// [`Localization::search_values`]. Everything is enabled by default.
#[derive(Debug, Clone, Copy)]
pub struct NormalizeOptions {
    pub lower_case: bool,
    pub normalize_letters: bool,
    pub trim_spaces: bool,
}

impl Default for NormalizeOptions {
    fn default() -> Self {
        Self {
            lower_case: true,
            normalize_letters: true,
            trim_spaces: true,
        }
    }
}

impl NormalizeOptions {
    fn apply(&self, text: &str) -> String {
        let mut result = text.to_owned();

        if self.lower_case {
            result = result.to_lowercase();
        }

        if self.normalize_letters {
            result = result
                .nfd()
                .filter(|c| !('\u{0300}'..='\u{036f}').contains(c))
                .collect();
        }

        if self.trim_spaces {
            result = result.trim().to_owned();
        }

        result
    }
}

#[derive(Debug, Clone)]
pub struct Localization {
    pub language: ConfigLanguage,
    pub url: String,
    pub data: LocalizationData,
}

impl Localization {
    pub fn new(language: ConfigLanguage, data: LocalizationData) -> Self {
        Self {
            language,
            url: Self::build_url(language),
            data,
        }
    }

    pub fn build_url(language: ConfigLanguage) -> String {
        format!(
            "https://sp-translations.socialpointgames.com/deploy/dc/android/prod/dc_android_{language}_prod_wetd46pWuR8J5CmS.json"
        )
    }

    /// Downloads the raw translation file (an array of small key/value objects).
    pub async fn fetch(language: ConfigLanguage) -> Result<Vec<LocalizationData>, Error> {
        Self::fetch_with(&reqwest::Client::new(), language).await
    }

    pub async fn fetch_with(
        client: &reqwest::Client,
        language: ConfigLanguage,
    ) -> Result<Vec<LocalizationData>, Error> {
        let response = client
            .get(Self::build_url(language))
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json().await?)
    }

    /// Downloads and merges the translation file for `language`.
    pub async fn create(language: ConfigLanguage) -> Result<Self, Error> {
        Self::create_with(&reqwest::Client::new(), language).await
    }

    pub async fn create_with(
        client: &reqwest::Client,
        language: ConfigLanguage,
    ) -> Result<Self, Error> {
        let array_data = Self::fetch_with(client, language).await?;
        let data = array_data.into_iter().flatten().collect();

        Ok(Self::new(language, data))
    }

    pub fn get_value_from_key(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(String::as_str)
    }

    pub fn get_key_from_value(&self, value: &str) -> Option<&str> {
        self.data
            .iter()
            .find(|(_, v)| v.as_str() == value)
            .map(|(k, _)| k.as_str())
    }

    pub fn get_dragon_name(&self, id: u64) -> Option<&str> {
        self.get_value_from_key(&format!("tid_unit_{id}_name"))
    }

    pub fn get_dragon_description(&self, id: u64) -> Option<&str> {
        self.get_value_from_key(&format!("tid_unit_{id}_description"))
    }

    pub fn get_attack_name(&self, id: u64) -> Option<&str> {
        self.get_value_from_key(&format!("tid_attack_name_{id}"))
    }

    pub fn get_skill_name(&self, id: u64) -> Option<&str> {
        self.get_value_from_key(&format!("tid_skill_name_{id}"))
    }

    pub fn get_skill_description(&self, id: u64) -> Option<&str> {
        self.get_value_from_key(&format!("tid_skill_description_{id}"))
    }

    /// Keys that contain `query` (accent/case-insensitive by default).
    pub fn search_keys(&self, query: &str, options: NormalizeOptions) -> Vec<&str> {
        let query = options.apply(query);

        self.data
            .keys()
            .filter(|key| options.apply(key).contains(&query))
            .map(String::as_str)
            .collect()
    }

    /// Values that contain `query` (accent/case-insensitive by default).
    pub fn search_values(&self, query: &str, options: NormalizeOptions) -> Vec<&str> {
        let query = options.apply(query);

        self.data
            .values()
            .filter(|value| options.apply(value).contains(&query))
            .map(String::as_str)
            .collect()
    }

    pub fn to_array(&self) -> Vec<LocalizationData> {
        self.data
            .iter()
            .map(|(k, v)| HashMap::from([(k.clone(), v.clone())]))
            .collect()
    }

    /// Differences between `self` (old) and `other` (new).
    pub fn compare(&self, other: &Localization) -> Result<ComparisonResult, Error> {
        Self::compare_between(self, other)
    }

    /// Differences between `a` (old) and `b` (new).
    pub fn compare_between(a: &Localization, b: &Localization) -> Result<ComparisonResult, Error> {
        if a.language != b.language {
            return Err(Error::LanguageMismatch(a.language, b.language));
        }

        let mut result = ComparisonResult::default();

        for (key, new_value) in &b.data {
            match a.data.get(key) {
                None => result.new_fields.push(ComparisonResultNewField {
                    key: key.clone(),
                    value: new_value.clone(),
                }),
                Some(old_value) if old_value != new_value => {
                    result.edited_fields.push(ComparisonResultEditedField {
                        key: key.clone(),
                        values: ComparisonResultEditedFieldValues {
                            old: old_value.clone(),
                            new: new_value.clone(),
                        },
                    })
                }
                Some(_) => {}
            }
        }

        for (key, old_value) in &a.data {
            if !b.data.contains_key(key) {
                result.deleted_fields.push(ComparisonResultDeletedField {
                    key: key.clone(),
                    value: old_value.clone(),
                });
            }
        }

        Ok(result)
    }

    /// Replaces translation keys of a JSON object (`name_key`, `tid_name`,
    /// `type_name_key`, `description_key`, ...) by the translated `name`,
    /// `type_name` and `description`. Keys without a translation are kept as is.
    /// Only the top level of the object is translated; non-objects are returned unchanged.
    pub fn translate(&self, value: Value) -> Value {
        match value {
            Value::Object(mut object) => {
                self.translate_object(&mut object);
                Value::Object(object)
            }
            other => other,
        }
    }

    pub fn translate_object(&self, object: &mut Map<String, Value>) {
        let keys: Vec<String> = object.keys().cloned().collect();

        for key in keys {
            let translation_of = |object: &Map<String, Value>| {
                object
                    .get(&key)
                    .and_then(Value::as_str)
                    .and_then(|k| self.get_value_from_key(k))
                    .filter(|translated| !translated.is_empty())
                    .map(str::to_owned)
            };

            if NAME_KEYS_TO_TRANSLATE.contains(&key.as_str()) {
                if let Some(name) = translation_of(object) {
                    object.insert("name".to_owned(), Value::String(name));
                    object.shift_remove(&key);
                }
            } else if TYPE_NAME_KEYS_TO_TRANSLATE.contains(&key.as_str()) {
                if let Some(type_name) = translation_of(object) {
                    object.insert("type_name".to_owned(), Value::String(type_name));
                    object.shift_remove(&key);
                }
            }

            if DESCRIPTION_KEYS_TO_TRANSLATE.contains(&key.as_str()) {
                if let Some(description) = translation_of(object) {
                    object.insert("description".to_owned(), Value::String(description));
                    object.shift_remove(&key);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample() -> Localization {
        Localization::new(
            ConfigLanguage::English,
            HashMap::from([
                ("tid_unit_1000_name".to_owned(), "Nature Dragon".to_owned()),
                ("tid_chest".to_owned(), "Chest".to_owned()),
                ("tid_chest_type".to_owned(), "Épico".to_owned()),
            ]),
        )
    }

    #[test]
    fn translates_known_keys_and_keeps_unknown_ones() {
        let l = sample();
        let out = l.translate(json!({
            "id": 1,
            "chest_name_key": "tid_chest",
            "type_name_key": "tid_chest_type",
            "description_key": "tid_missing",
        }));

        assert_eq!(out["name"], "Chest");
        assert_eq!(out["type_name"], "Épico");
        assert!(out.get("chest_name_key").is_none());
        assert_eq!(out["description_key"], "tid_missing");
    }

    #[test]
    fn searches_ignore_case_and_accents() {
        let l = sample();
        assert_eq!(l.search_values("epico", NormalizeOptions::default()), vec!["Épico"]);
        assert_eq!(l.get_dragon_name(1000), Some("Nature Dragon"));
    }

    #[test]
    fn compare_detects_changes() {
        let a = sample();
        let mut b = sample();
        b.data.insert("tid_chest".into(), "Big Chest".into());
        b.data.insert("new".into(), "x".into());
        b.data.remove("tid_unit_1000_name");

        let diff = a.compare(&b).unwrap();
        assert_eq!(diff.new_fields.len(), 1);
        assert_eq!(diff.edited_fields.len(), 1);
        assert_eq!(diff.deleted_fields.len(), 1);
    }
}
