#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The request failed, the server answered with a non-2xx status or the body was not the expected JSON.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    /// A section could not be deserialized into the requested type, or a file held invalid JSON.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    /// A configuration file could not be read.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
