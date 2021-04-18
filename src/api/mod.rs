use crate::api::biomes::Biomes;
use crate::api::blocks::Blocks;
use crate::api::enchantments::Enchantments;
use crate::api::entities::Entities;
use crate::api::foods::Foods;
use crate::api::items::Items;
use crate::api::loot::Loot;
use crate::api::recipes::Recipes;
use crate::models::version::Version;
use std::sync::Arc;

#[cfg(test)]
mod tests;

pub mod biomes;
pub mod blocks;
pub mod enchantments;
pub mod entities;
pub mod foods;
pub mod items;
pub mod loot;
pub mod recipes;
pub mod versions;

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
