use crate::data::{get_version_specific_file, RECIPES_FILE};
use crate::models::recipe::Recipe;
use crate::models::version::Version;
use crate::{DataError, DataResult};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Recipes {
    version: Arc<Version>,
}

impl Recipes {
    pub fn new(version: Arc<Version>) -> Self {
        Self { version }
    }

    /// Returns a list of recipes indexed by item ID
    pub fn recipes(&self) -> DataResult<HashMap<u32, Vec<Recipe>>> {
        let content = get_version_specific_file(&self.version, RECIPES_FILE)?;
        serde_json::from_str::<HashMap<u32, Vec<Recipe>>>(&*content).map_err(DataError::from)
    }
}
