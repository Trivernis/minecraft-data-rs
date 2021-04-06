use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct Item {
    pub id: u32,
    pub display_name: String,
    pub stack_size: u8,
    pub enchant_categories: Option<Vec<String>>,
    pub fixed_with: Option<Vec<String>>,
    pub max_durability: Option<u32>,
    pub name: String,
    pub variations: Option<Value>,
    pub durability: Option<u32>,
}
