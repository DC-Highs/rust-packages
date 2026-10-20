use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A configuration object did not match the (strict) schema of its mapper.
    #[error("invalid {schema}: {source}")]
    Schema {
        schema: &'static str,
        #[source]
        source: serde_json::Error,
    },
    /// A section is missing a key (or it has the wrong JSON type).
    #[error("section is missing the array/object `{0}`")]
    MissingField(String),
    /// A chest references a reward id that does not exist.
    #[error("reward {0} not found")]
    RewardNotFound(Value),
    /// A flag that should be `0` or `1` had another value.
    #[error("invalid number {0}, expected 0 or 1")]
    InvalidBoolean(String),
    /// A `costs` object had more than one currency, or an unknown one.
    #[error("unexpected price: {0}")]
    UnexpectedPrice(String),
    /// A reward object contained something the mapper does not understand.
    #[error("rewards not processed correctly: {0}")]
    UnprocessedRewards(String),
    /// A timestamp does not fit in a date.
    #[error("invalid timestamp: {0}")]
    InvalidTimestamp(String),
    /// A numeric operation overflowed or produced a non-finite number.
    #[error("invalid numeric operation on {0}")]
    InvalidNumber(String),
    /// Number of free board positions differs from the number of decorations.
    #[error("number of decoration positions does not match number of decorations")]
    DecorationPositionMismatch,
    /// The compile context entry used to resolve ids is not an array.
    #[error("compile context entry `{0}` is not an array")]
    InvalidContext(String),
    /// The compiled data references itself (or is nested too deep).
    #[error("data compilation exceeded the maximum depth of {0} (circular reference?)")]
    MaxDepthExceeded(usize),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
