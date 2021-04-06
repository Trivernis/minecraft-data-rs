#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct Version {
    pub version: i32,
    pub minecraft_version: String,
    pub major_version: String,
}

impl Version {
    /// Returns the first version of the current major version
    pub(crate) fn major_first(&self) -> String {
        format!("{}.1", self.major_version)
    }

    /// Returns the previous major version
    pub(crate) fn previous_major(&self) -> String {
        let major = self.major_version.split('.').last().unwrap();
        let major_num = major.parse::<i32>().unwrap();

        self.major_version
            .replace(major, format!("{}", major_num - 1).as_str())
    }

    /// Returns the first version of the previous major version
    pub(crate) fn previous_major_first(&self) -> String {
        format!("{}.1", self.previous_major())
    }
}
