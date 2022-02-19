use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct BlockCollisionShapes {
    pub blocks: HashMap<String, CollisionShapeIds>,
    pub shapes: HashMap<u16, CollisionShape>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(
    rename_all(deserialize = "camelCase", serialize = "snake_case"),
    untagged
)]
pub enum CollisionShapeIds {
    Value(u16),
    Array(Vec<u16>),
}

pub type CollisionShape = Vec<Vec<f32>>;
