use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub(crate) struct Datapaths {
    pub pc: HashMap<String, HashMap<String, String>>,
    pub pe: HashMap<String, HashMap<String, String>>,
}
