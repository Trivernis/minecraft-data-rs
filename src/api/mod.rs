pub use crate::api::biomes::Biomes;
pub use crate::api::blocks::Blocks;
pub use crate::api::enchantments::Enchantments;
pub use crate::api::entities::Entities;
pub use crate::api::foods::Foods;
pub use crate::api::items::Items;
pub use crate::api::loot::Loot;
pub use crate::api::recipes::Recipes;
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
mod protocol;
mod recipes;
mod versions;

use crate::api::protocol::Protocol;
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
    pub protocols: Protocol,
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
            protocols: Protocol::new(Arc::clone(&version)),
        }
    }
}
