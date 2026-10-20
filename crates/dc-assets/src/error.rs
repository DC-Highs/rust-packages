#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The request failed or the server answered with a non-2xx status.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    /// The file could not be written to disk.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    /// The downloader was built with inconsistent arguments.
    #[error("invalid arguments: {0}")]
    InvalidArguments(String),
}
