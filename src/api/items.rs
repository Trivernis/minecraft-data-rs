use crate::data::{get_version_specific_file, ITEMS_FILE};
use crate::models::item::Item;
use crate::models::version::Version;
use crate::{DataError, DataResult};
use std::collections::HashMap;
use std::sync::Arc;

/// API to access item information
pub struct Items {
    version: Arc<Version>,
}

impl Items {
    pub fn new(version: Arc<Version>) -> Self {
        Self { version }
    }

    /// Returns the items
    pub fn items_array(&self) -> DataResult<Vec<Item>> {
        let content = get_version_specific_file(&self.version, ITEMS_FILE)?;

        serde_json::from_str::<Vec<Item>>(&*content).map_err(DataError::from)
    }

    /// Returns the items indexed by name
    pub fn items_by_name(&self) -> DataResult<HashMap<String, Item>> {
        Ok(self
            .items_array()?
            .into_iter()
            .map(|i| (i.name.clone(), i))
            .collect())
    }

    /// Returns the items indexed by ID
    pub fn items(&self) -> DataResult<HashMap<u32, Item>> {
        Ok(self
            .items_array()?
            .into_iter()
            .map(|i| (i.id, i))
            .collect())
    }
}
