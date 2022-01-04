use crate::data::{get_version_specific_file, BIOMES_FILE};
use crate::models::biome::Biome;
use crate::models::version::Version;
use crate::DataResult;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::Arc;

pub struct Biomes {
    version: Arc<Version>,
}

impl Biomes {
    pub fn new(version: Arc<Version>) -> Self {
        Self { version }
    }

    /// Returns all biomes as an unordered list
    pub fn biomes_array(&self) -> DataResult<Vec<Biome>> {
        let content = get_version_specific_file(&self.version, BIOMES_FILE)?;
        let biomes = serde_json::from_str(&content)?;

        Ok(biomes)
    }

    /// Returns the biomes indexed by id
    pub fn biomes(&self) -> DataResult<HashMap<u32, Biome>> {
        let biomes = self.biomes_array()?;
        let biomes_map = HashMap::from_iter(biomes.into_iter().map(|b| (b.id, b)));

        Ok(biomes_map)
    }

    /// Returns the biomes indexed by name
    pub fn biomes_by_name(&self) -> DataResult<HashMap<String, Biome>> {
        let biomes = self.biomes_array()?;
        let biomes_map = HashMap::from_iter(biomes.into_iter().map(|b| (b.name.clone(), b)));

        Ok(biomes_map)
    }
}
