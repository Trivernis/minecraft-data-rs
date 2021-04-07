use crate::data::{get_version_specific_file, ENCHANTMENTS_FILE};
use crate::models::enchantment::Enchantment;
use crate::models::version::Version;
use crate::{DataError, DataResult};
use itertools::*;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Enchantments {
    version: Arc<Version>,
}

impl Enchantments {
    pub fn new(version: Arc<Version>) -> Self {
        Self { version }
    }

    /// Returns a list of enchantments
    pub fn enchantments_array(&self) -> DataResult<Vec<Enchantment>> {
        let content = get_version_specific_file(&self.version, ENCHANTMENTS_FILE)?;

        serde_json::from_str::<Vec<Enchantment>>(&*content).map_err(DataError::from)
    }

    /// Returns a map of enchantments indexed by ID
    pub fn enchantments(&self) -> DataResult<HashMap<u32, Enchantment>> {
        Ok(HashMap::from_iter(
            self.enchantments_array()?.into_iter().map(|e| (e.id, e)),
        ))
    }

    /// Returns a map of enchantments indexed by Name
    pub fn enchantments_by_name(&self) -> DataResult<HashMap<String, Enchantment>> {
        Ok(HashMap::from_iter(
            self.enchantments_array()?
                .into_iter()
                .map(|e| (e.name.clone(), e)),
        ))
    }

    /// Returns enchantments grouped by category
    pub fn enchantments_by_category(&self) -> DataResult<HashMap<String, Vec<Enchantment>>> {
        Ok(HashMap::from_iter(
            self.enchantments_array()?
                .into_iter()
                .group_by(|e| e.category.clone())
                .into_iter()
                .map(|(key, group)| (key, group.collect())),
        ))
    }
}
