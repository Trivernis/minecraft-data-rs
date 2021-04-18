#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct Entity {
    pub id: u32,
    pub internal_id: Option<u32>,
    pub display_name: String,
    pub name: String,
    #[serde(alias = "type")]
    pub entity_type: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub category: Option<String>,
}
