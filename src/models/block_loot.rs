#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct BlockLoot {
    pub block: String,
    pub drops: Vec<ItemDrop>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct ItemDrop {
    pub item: String,
    pub drop_chance: f32,
    pub stack_size_range: [Option<isize>; 2],
    pub block_age: Option<usize>,
    pub silk_touch: Option<bool>,
    pub no_silk_touch: Option<bool>,
}
