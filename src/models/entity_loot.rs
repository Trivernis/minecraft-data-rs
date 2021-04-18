#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct EntityLoot {
    pub entity: String,
    pub drops: Vec<ItemDrop>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct ItemDrop {
    pub item: String,
    pub drop_chance: f32,
    pub stack_size_range: [usize; 2],
    pub player_kill: Option<bool>,
}
