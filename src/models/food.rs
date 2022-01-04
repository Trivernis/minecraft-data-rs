#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct Food {
    pub id: u32,
    pub display_name: String,
    pub stack_size: u8,
    pub name: String,
    pub food_points: f32,
    pub saturation: f32,
    pub effective_quality: f32,
    pub saturation_ratio: f32,
    pub variations: Option<Vec<Variation>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct Variation {
    pub metadata: u32,
    pub display_name: String,
}
