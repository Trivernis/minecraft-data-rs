use crate::api::biomes::Biomes;
use crate::api::blocks::Blocks;
use crate::api::enchantments::Enchantments;
use crate::api::entities::Entities;
use crate::api::foods::Foods;
use crate::api::items::Items;
use crate::api::loot::Loot;
use crate::api::recipes::Recipes;
use crate::api::versions::latest_stable;
use crate::models::version::Version;
use crate::DataResult;
use std::sync::Arc;

#[cfg(test)]
mod tests;

mod biomes;
mod blocks;
mod enchantments;
mod entities;
mod foods;
mod items;
mod loot;
mod recipes;
mod versions;

pub use biomes::*;
pub use blocks::*;
pub use enchantments::*;
pub use entities::*;
pub use foods::*;
pub use items::*;
pub use loot::*;
pub use recipes::*;
pub use versions::*;

/// A type wrapping access to all the metadata
/// about the selected minecraft version
#[allow(missing_docs)]
pub struct Api {
    pub version: Arc<Version>,
    pub items: Items,
    pub recipes: Recipes,
    pub enchantments: Enchantments,
    pub loot: Loot,
    pub blocks: Blocks,
    pub foods: Foods,
    pub biomes: Biomes,
    pub entities: Entities,
}

impl Api {
    /// Creates a new API wrapper for the latest version
    pub fn latest() -> DataResult<Self> {
        Ok(Self::new(latest_stable()?))
    }

    /// Creates a new API wrapper for the provided version
    pub fn new(version: Version) -> Self {
        let version = Arc::new(version);
        Self {
            version: Arc::clone(&version),
            items: Items::new(Arc::clone(&version)),
            recipes: Recipes::new(Arc::clone(&version)),
            enchantments: Enchantments::new(Arc::clone(&version)),
            loot: Loot::new(Arc::clone(&version)),
            blocks: Blocks::new(Arc::clone(&version)),
            foods: Foods::new(Arc::clone(&version)),
            biomes: Biomes::new(Arc::clone(&version)),
            entities: Entities::new(Arc::clone(&version)),
        }
    }
}
