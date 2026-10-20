use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use bytes::Bytes;
use reqwest::Client;

use crate::Error;

/// Anything that points to a downloadable file.
///
/// `urls` lists the candidates in order: the first one that answers with a
/// successful status wins.
pub trait AssetSource {
    fn urls(&self) -> Vec<&str>;
}

/// Shared HTTP client used by `fetch()`/`download()`. Use the `*_with`
/// variants to bring your own client (proxies, timeouts, ...).
pub fn default_client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    CLIENT.get_or_init(Client::new)
}

pub(crate) async fn fetch_first(client: &Client, urls: &[&str]) -> Result<Bytes, Error> {
    let mut last_error = None;

    for url in urls {
        let attempt = async {
            let response = client.get(*url).send().await?.error_for_status()?;
            response.bytes().await
        };

        match attempt.await {
            Ok(bytes) => return Ok(bytes),
            Err(error) => last_error = Some(error),
        }
    }

    match last_error {
        Some(error) => Err(Error::Http(error)),
        None => Err(Error::InvalidArguments("no url to download".to_owned())),
    }
}

pub(crate) async fn download_first(
    client: &Client,
    urls: &[&str],
    path: &Path,
) -> Result<PathBuf, Error> {
    let bytes = fetch_first(client, urls).await?;
    tokio::fs::write(path, &bytes).await?;
    Ok(path.to_path_buf())
}

/// Generates `url`, `fetch`, `fetch_with`, `download` and `download_with` for a downloader type.
macro_rules! impl_downloader {
    ($ty:ty) => {
        impl $ty {
            /// Primary URL of the file.
            pub fn url(&self) -> &str {
                &self.url
            }

            /// Downloads the file into memory.
            pub async fn fetch(&self) -> Result<::bytes::Bytes, $crate::Error> {
                self.fetch_with($crate::default_client()).await
            }

            /// Same as [`fetch`](Self::fetch) with a custom HTTP client.
            pub async fn fetch_with(
                &self,
                client: &::reqwest::Client,
            ) -> Result<::bytes::Bytes, $crate::Error> {
                $crate::client::fetch_first(client, &$crate::AssetSource::urls(self)).await
            }

            /// Downloads the file to `path` and returns that path.
            pub async fn download(
                &self,
                path: impl AsRef<::std::path::Path>,
            ) -> Result<::std::path::PathBuf, $crate::Error> {
                self.download_with($crate::default_client(), path).await
            }

            /// Same as [`download`](Self::download) with a custom HTTP client.
            pub async fn download_with(
                &self,
                client: &::reqwest::Client,
                path: impl AsRef<::std::path::Path>,
            ) -> Result<::std::path::PathBuf, $crate::Error> {
                $crate::client::download_first(
                    client,
                    &$crate::AssetSource::urls(self),
                    path.as_ref(),
                )
                .await
            }
        }

        impl $crate::AssetSource for $ty {
            fn urls(&self) -> Vec<&str> {
                vec![self.url.as_str()]
            }
        }
    };
}

pub(crate) use impl_downloader;
