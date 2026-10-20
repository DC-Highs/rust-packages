//! Dragon City localization: download the translation files, look keys up and
//! translate configuration objects.
//!
//! Fan-made library, not affiliated with SocialPoint.

mod keys;
mod localization;

pub use keys::{
    DESCRIPTION_KEYS_TO_TRANSLATE, NAME_KEYS_TO_TRANSLATE, TYPE_NAME_KEYS_TO_TRANSLATE,
};
pub use localization::{
    ComparisonResult, ComparisonResultDeletedField, ComparisonResultEditedField,
    ComparisonResultEditedFieldValues, ComparisonResultNewField, Error, Localization,
    LocalizationData, NormalizeOptions,
};
