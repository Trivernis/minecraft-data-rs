use crate::data::{get_version_specific_file, FOODS_FILE};
use crate::models::food::Food;
use crate::models::version::Version;
use crate::DataResult;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Foods {
    version: Arc<Version>,
}

impl Foods {
    pub fn new(version: Arc<Version>) -> Self {
        Self { version }
    }

    /// Returns the unindexed list of food
    pub fn foods_array(&self) -> DataResult<Vec<Food>> {
        let content = get_version_specific_file(&self.version, FOODS_FILE)?;
        let foods = serde_json::from_str(&content)?;

        Ok(foods)
    }

    /// Returns food indexed by id
    pub fn foods(&self) -> DataResult<HashMap<u32, Food>> {
        let foods = self.foods_array()?;
        let food_map = foods.into_iter().map(|f| (f.id, f)).collect();

        Ok(food_map)
    }

    /// Returns food indexed by name
    pub fn foods_by_name(&self) -> DataResult<HashMap<String, Food>> {
        let foods = self.foods_array()?;
        let food_map = foods.into_iter().map(|f| (f.name.clone(), f)).collect();

        Ok(food_map)
    }
}
