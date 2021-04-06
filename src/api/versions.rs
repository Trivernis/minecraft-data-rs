use crate::data::{get_common_file, PROTOCOL_VERSIONS_FILE, VERSIONS_FILE};
use crate::models::version::Version;
use crate::{DataError, DataResult};
use std::collections::HashMap;
use std::iter::FromIterator;

/// Returns the unsorted list of versions
pub fn versions() -> DataResult<Vec<Version>> {
    let content = get_common_file(PROTOCOL_VERSIONS_FILE)?;
    let versions = serde_json::from_str::<Vec<Version>>(&*content)?;

    Ok(versions)
}

/// Returns the versions indexed by minecraft version
pub fn versions_by_minecraft_version() -> DataResult<HashMap<String, Version>> {
    let indexed_versions = HashMap::from_iter(
        versions()?
            .into_iter()
            .map(|v| (v.minecraft_version.clone(), v)),
    );

    Ok(indexed_versions)
}

/// Returns the latest stable version (hardcoded at the moment)
pub fn latest_stable() -> DataResult<Version> {
    versions_by_minecraft_version()?
        .get("1.16.5")
        .cloned()
        .ok_or(DataError::NotFoundError("1.16.5".to_string()))
}

/// Returns a list of available version information
pub(crate) fn available_versions() -> DataResult<Vec<String>> {
    let content = get_common_file(VERSIONS_FILE)?;
    serde_json::from_str::<Vec<String>>(&*content).map_err(DataError::from)
}
