use crate::data::{get_version_specific_file, BLOCK_LOOT_FILE, ENTITY_LOOT_FILE};
use crate::models::block_loot::BlockLoot;
use crate::models::entity_loot::EntityLoot;
use crate::models::version::Version;
use crate::DataResult;
use std::collections::HashMap;
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

    /// Returns the entity loot indexed by entity name
    pub fn entity_loot(&self) -> DataResult<HashMap<String, EntityLoot>> {
        let loot = self.entity_loot_array()?;
        let loot_map = loot.into_iter().map(|l| (l.entity.clone(), l)).collect();

        Ok(loot_map)
    }

    /// Returns the block loot in a list
    pub fn block_loot_array(&self) -> DataResult<Vec<BlockLoot>> {
        let content = get_version_specific_file(&self.version, BLOCK_LOOT_FILE)?;
        let loot = serde_json::from_str::<Vec<BlockLoot>>(&content)?;

        Ok(loot)
    }

    /// Returns the block loot indexed by block name
    pub fn block_loot(&self) -> DataResult<HashMap<String, BlockLoot>> {
        let loot = self.block_loot_array()?;
        let loot_map = loot.into_iter().map(|l| (l.block.clone(), l)).collect();

        Ok(loot_map)
    }
}
