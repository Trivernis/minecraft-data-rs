/// A type wrapping a minecraft version
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct Version {
    /// API Version
    pub version: i32,
    /// The full minecraft version (e.g. (`1.8.1`)
    pub minecraft_version: String,
    /// The major version of this minecraft version (e.g. `1.8`)
    pub major_version: String,
}
