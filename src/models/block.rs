use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct Block {
    pub id: u32,
    pub display_name: String,
    pub name: String,
    pub hardness: Option<f32>,
    pub stack_size: u8,
    pub diggable: bool,
    pub bounding_box: BoundingBox,
    pub material: Option<String>,
    pub harvest_tools: Option<HashMap<u32, bool>>,
    pub variations: Option<Vec<Variation>>,
    pub drops: Vec<u32>,
    pub transparent: bool,
    pub emit_light: u8,
    pub filter_light: u8,
    pub min_state_id: Option<u32>,
    pub max_state_id: Option<u32>,
    pub states: Option<Vec<State>>,
    pub default_state: Option<u32>,
    #[serde(alias = "resistance")]
    pub blast_resistance: Option<f32>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub enum BoundingBox {
    Block,
    Empty,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct Variation {
    pub metadata: u32,
    pub display_name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
// The fields in this part of the schema are not camelCase.
#[serde(rename_all(serialize = "snake_case"))]
pub struct State {
    pub name: String,
    #[serde(alias = "type")]
    pub state_type: StateType,
    pub values: Option<Vec<String>>,
    pub num_values: u32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub enum StateType {
    Enum,
    Bool,
    Int,
}
