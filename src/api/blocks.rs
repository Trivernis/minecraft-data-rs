use crate::data::{get_version_specific_file, BLOCKS_FILE, BLOCK_COLLISION_SHAPES_FILE};
use crate::models::block::Block;
use crate::models::block_collision_shapes::BlockCollisionShapes;
use crate::models::version::Version;
use crate::DataResult;
use std::collections::HashMap;
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

    // Returns the references of blocks indexed by state ID
    pub fn blocks_by_state_id<'a>(&self, blocks: &'a Vec<Block>) -> DataResult<HashMap<u32, &'a Block>> {
        let mut blocks_map = HashMap::new();
        blocks.iter().for_each(|b| {
            let min_state_id = b.min_state_id.unwrap_or(b.id << 4);
            let max_state_id = b.max_state_id.unwrap_or(min_state_id + 15);
            (min_state_id..max_state_id).for_each(|s| {
                blocks_map.insert(s, b);
            });
        });
        
        Ok(blocks_map)
    }

    /// Returns the blocks indexed by name
    pub fn blocks_by_name(&self) -> DataResult<HashMap<String, Block>> {
        let blocks = self.blocks_array()?;
        let blocks_map = blocks.into_iter().map(|b| (b.name.clone(), b)).collect();

        Ok(blocks_map)
    }

    /// Returns the blocks indexed by ID
    pub fn blocks(&self) -> DataResult<HashMap<u32, Block>> {
        let blocks = self.blocks_array()?;
        let blocks_map = blocks.into_iter().map(|b| (b.id, b)).collect();

        Ok(blocks_map)
    }

    /// Returns the block collision shapes object
    pub fn block_collision_shapes(&self) -> DataResult<BlockCollisionShapes> {
        let content = get_version_specific_file(&self.version, BLOCK_COLLISION_SHAPES_FILE)?;
        let collision_shapes = serde_json::from_str(&content)?;

        Ok(collision_shapes)
    }
}
