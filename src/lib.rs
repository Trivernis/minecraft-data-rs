//! This crate is a wrapper for accessing information from [minecraft-data)(https://github.com/PrismarineJS/minecraft-data).
//!
//! Usage:
//! ```
//! use std::collections::HashMap;
//! use minecraft_data_rs::Api;
//! use minecraft_data_rs::models::food::Food;
//! use minecraft_data_rs::models::version::Version;
//!
//! // create an api wrapper for the latest stable version
//! let api = Api::latest().expect("failed to retrieve latest version");
//! let food: Vec<Food> = api.foods.foods_array().unwrap();
//!
//! for food in food {
//!     println!("When eating {} you gain {} food points", food.name, food.food_points);
//! }
//! ```
//!

#[macro_use]
extern crate serde_derive;

/// Provides data access methods
pub mod api;
pub(crate) mod data;
/// Contains the type definitions for the data
pub mod models;
pub(crate) mod utils;

pub use api::Api;
pub use utils::error::DataError;
pub use utils::error::DataResult;
