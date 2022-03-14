use crate::data::{get_common_file, PROTOCOL_VERSIONS_FILE, VERSIONS_FILE};
use crate::models::version::Version;
use crate::{DataError, DataResult};
use itertools::Itertools;
use std::collections::HashMap;

/// Returns the unsorted list of versions
pub fn versions() -> DataResult<Vec<Version>> {
    let content = get_common_file(PROTOCOL_VERSIONS_FILE)?;
    let versions = serde_json::from_str::<Vec<Version>>(&*content)?;

    Ok(versions)
}

/// Returns the versions indexed by minecraft version
pub fn versions_by_minecraft_version() -> DataResult<HashMap<String, Version>> {
    let indexed_versions = versions()?
        .into_iter()
        .map(|v| (v.minecraft_version.clone(), v))
        .collect();

    Ok(indexed_versions)
}

/// Returns the latest stable version for which data paths exists.
/// Patch versions using the same data path as the major version are ignored.
pub fn latest_stable() -> DataResult<Version> {
    let latest = available_versions()?
        .into_iter()
        .filter_map(|v| {
            let version_string = v.clone();
            let mut parts = version_string.split(".");

            Some((
                v,
                parts.next()?.parse::<u32>().ok()?,
                parts.next()?.parse::<u32>().ok()?,
                parts.next().and_then(|p| p.parse::<u32>().ok()),
            ))
        })
        .sorted_by_key(|(_, maj, min, patch)| {
            format!("{:#05}.{:#05}.{:#05}", maj, min, patch.unwrap_or(0))
        })
        .map(|(v, _, _, _)| v)
        .filter_map(|v| versions_by_minecraft_version().ok()?.remove(&v))
        .rev()
        .next();

    latest.ok_or_else(|| DataError::NotFoundError(String::from("latest version")))
}

/// Returns a list of available version information
pub fn available_versions() -> DataResult<Vec<String>> {
    let content = get_common_file(VERSIONS_FILE)?;
    serde_json::from_str::<Vec<String>>(&*content).map_err(DataError::from)
}
