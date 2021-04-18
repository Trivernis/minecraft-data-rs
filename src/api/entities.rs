use crate::data::{get_version_specific_file, ENTITIES_FILE};
use crate::models::entity::Entity;
use crate::models::version::Version;
use crate::DataResult;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::Arc;

pub struct Entities {
    version: Arc<Version>,
}

impl Entities {
    pub fn new(version: Arc<Version>) -> Self {
        Self { version }
    }

    /// Returns an unordered array of entities
    pub fn entities_array(&self) -> DataResult<Vec<Entity>> {
        let content = get_version_specific_file(&self.version, ENTITIES_FILE)?;
        let entities = serde_json::from_str(&content)?;

        Ok(entities)
    }

    /// Returns entities indexed by name
    pub fn entities_by_name(&self) -> DataResult<HashMap<String, Entity>> {
        let entities = self.entities_array()?;
        let entities_map = HashMap::from_iter(entities.into_iter().map(|e| (e.name.clone(), e)));

        Ok(entities_map)
    }

    /// Returns entities indexed by id
    pub fn entities(&self) -> DataResult<HashMap<u32, Entity>> {
        let entities = self.entities_array()?;
        let entities_map = HashMap::from_iter(entities.into_iter().map(|e| (e.id, e)));

        Ok(entities_map)
    }
}
