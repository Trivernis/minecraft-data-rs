use std::collections::HashMap;
#[allow(unused)]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub(crate) struct Datapaths {
    pub pc: HashMap<String, HashMap<String, String>>,
    pub bedrock: HashMap<String, HashMap<String, String>>,
}
