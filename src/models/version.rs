#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct Version {
    pub version: i32,
    pub minecraft_version: String,
    pub major_version: String,
}
