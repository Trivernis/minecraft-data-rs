use crate::data::{get_version_specific_file, ENTITY_LOOT_FILE};
use crate::models::entity_loot::EntityLoot;
use crate::models::version::Version;
use crate::DataResult;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::Arc;

/// API to access item information
pub struct Loot {
    version: Arc<Version>,
}

impl Loot {
    pub fn new(version: Arc<Version>) -> Self {
        Self { version }
    }

    /// Returns the entity loot in a list
    pub fn entity_loot_array(&self) -> DataResult<Vec<EntityLoot>> {
        let content = get_version_specific_file(&self.version, ENTITY_LOOT_FILE)?;
        let loot = serde_json::from_str::<Vec<EntityLoot>>(&content)?;

        Ok(loot)
    }

    /// Returns the entity loot indexed by name
    pub fn entity_loot(&self) -> DataResult<HashMap<String, EntityLoot>> {
        let loot = self.entity_loot_array()?;
        let loot_map = HashMap::from_iter(loot.into_iter().map(|l| (l.entity.clone(), l)));

        Ok(loot_map)
    }
}
