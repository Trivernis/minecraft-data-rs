#![doc=include_str!("../README.md")]

#[macro_use]
extern crate serde_derive;

/// Provides data access methods
#[cfg(feature="include_data")]
pub mod api;
#[cfg(feature="include_data")]
pub(crate) mod data;
/// Contains the type definitions for the data
pub mod models;
pub(crate) mod utils;

#[cfg(feature="include_data")]
pub use api::Api;
pub use utils::error::DataError;
pub use utils::error::DataResult;
