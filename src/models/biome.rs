#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct Biome {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub temperature: f32,
    pub precipitation: String,
    pub depth: f32,
    pub dimension: String,
    pub display_name: String,
    pub color: u32,
    pub rainfall: f32,
}
