use crate::data::{get_version_specific_file, BLOCKS_FILE};
use crate::models::block::Block;
use crate::models::version::Version;
use crate::DataResult;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::Arc;

pub struct Blocks {
    version: Arc<Version>,
}

impl Blocks {
    pub fn new(version: Arc<Version>) -> Self {
        Self { version }
    }

    /// Returns the list of blocks
    pub fn blocks_array(&self) -> DataResult<Vec<Block>> {
        let content = get_version_specific_file(&self.version, BLOCKS_FILE)?;
        let blocks = serde_json::from_str::<Vec<Block>>(&content)?;

        Ok(blocks)
    }

    /// Returns the blocks indexed by ID
    pub fn blocks(&self) -> DataResult<HashMap<u32, Block>> {
        let blocks = self.blocks_array()?;
        let blocks_map = HashMap::from_iter(blocks.into_iter().map(|b| (b.id, b)));

        Ok(blocks_map)
    }

    /// Returns the blocks indexed by name
    pub fn blocks_by_name(&self) -> DataResult<HashMap<String, Block>> {
        let blocks = self.blocks_array()?;
        let blocks_map = HashMap::from_iter(blocks.into_iter().map(|b| (b.name.clone(), b)));

        Ok(blocks_map)
    }
}
