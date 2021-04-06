use crate::api::items::Items;
use crate::models::version::Version;
use std::sync::Arc;

#[cfg(test)]
mod tests;

pub mod items;
pub mod versions;

pub struct Api {
    version: Arc<Version>,
    items: Items,
}

impl Api {
    pub fn new(version: Version) -> Self {
        let version = Arc::new(version);
        Self {
            version: Arc::clone(&version),
            items: Items::new(Arc::clone(&version)),
        }
    }
}
