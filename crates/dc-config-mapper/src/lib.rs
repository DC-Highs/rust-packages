//! Restructure and populate the Dragon City game configuration.
//!
//! * [`mappers`] validate a configuration section against strict schemas, restructure it
//!   (`img_name` -> `image_name`, `0/1` -> `bool`, timestamps -> ISO dates, compact rewards ->
//!   typed rewards, ...) and translate it with a [`Localization`].
//! * [`DataCompiler`] resolves id references (`dragon_id`, `reward_ids`) into the referenced objects.
//!
//! ```no_run
//! use dc_config::{Config, ConfigFilter, ConfigLanguage};
//! use dc_config_mapper::ChestsMapper;
//! use dc_localization::Localization;
//!
//! # async fn run(url: String, user_id: String, auth_token: String) -> Result<(), Box<dyn std::error::Error>> {
//! let config = Config::create(
//!     dc_config::FetchOptions::new(url, user_id, auth_token).filter([ConfigFilter::Chests]),
//! )
//! .await?;
//! let localization = Localization::create(ConfigLanguage::English).await?;
//!
//! let chests = ChestsMapper::new(localization)
//!     .map(config.data.config().section(ConfigFilter::Chests).unwrap())?;
//! # let _ = chests;
//! # Ok(())
//! # }
//! ```
//!
//! Fan-made library, not affiliated with SocialPoint.

mod data_compiler;
mod error;
pub mod mappers;
pub mod schemas;
mod utils;

pub use data_compiler::{CompileContext, DataCompiler};
pub use error::{Error, Result};
pub use mappers::*;
pub use utils::{
    basename, iso_from_ts, number_to_boolean, process_price, process_rewards,
    resolve_grid_decoration_positions, Availability, Num, Price, PriceType, ProcessedReward,
    RewardType, Size, TsRange,
};

pub use dc_localization::Localization;
