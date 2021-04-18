use crate::api::enchantments::Enchantments;
use crate::api::items::Items;
use crate::api::loot::Loot;
use crate::api::recipes::Recipes;
use crate::models::version::Version;
use std::sync::Arc;

#[cfg(test)]
mod tests;

pub mod enchantments;
pub mod items;
mod loot;
mod recipes;
pub mod versions;

pub struct Api {
    pub version: Arc<Version>,
    pub items: Items,
    pub recipes: Recipes,
    pub enchantments: Enchantments,
    pub loot: Loot,
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
        }
    }
}
