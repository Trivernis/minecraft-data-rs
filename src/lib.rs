#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate include_dir;

pub mod api;
pub(crate) mod data;
pub mod models;
pub(crate) mod utils;

pub use utils::error::DataError;
pub use utils::error::DataResult;
