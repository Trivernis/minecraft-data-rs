#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct Enchantment {
    pub id: u32,
    pub name: String,
    pub display_name: String,
    pub max_level: u8,
    pub min_cost: Cost,
    pub max_cost: Cost,
    pub treasure_only: bool,
    pub exclude: Vec<String>,
    pub category: String,
    pub weight: u8,
    pub tradeable: bool,
    pub discoverable: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct Cost {
    pub a: i32,
    pub b: i32,
}
