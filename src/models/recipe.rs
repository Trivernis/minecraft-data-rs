#[derive(Deserialize, Debug, Clone)]
#[serde(
    rename_all(deserialize = "camelCase", serialize = "snake_case"),
    untagged
)]
pub enum Recipe {
    Shaped(ShapedRecipe),
    Shapeless(ShapelessRecipe),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct ShapedRecipe {
    pub result: RecipeItem,
    pub in_shape: Shape,
    pub out_shape: Option<Shape>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct ShapelessRecipe {
    pub result: RecipeItem,
    pub ingredients: Vec<RecipeItem>,
}

pub type Shape = Vec<Vec<RecipeItem>>;

#[derive(Deserialize, Debug, Clone)]
#[serde(
    rename_all(deserialize = "camelCase", serialize = "snake_case"),
    untagged
)]
pub enum RecipeItem {
    ID(u32),
    IDMetadataArray([u32; 2]),
    IDMetadataCountObject(IDMetadataCountObject),
    Null(Option<()>),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct IDMetadataCountObject {
    pub id: i32,
    pub metadata: Option<i32>,
    pub count: Option<u32>,
}
