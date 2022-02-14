use crate::data::{get_common_file, PROTOCOL_VERSIONS_FILE, VERSIONS_FILE};
use crate::models::version::Version;
use crate::{DataError, DataResult};
use itertools::Itertools;
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
    let latest = versions()?
        .into_iter()
        .filter_map(|v| {
            let version_string = v.minecraft_version.clone();
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
        .rev()
        .next();

    latest.ok_or_else(|| DataError::NotFoundError(String::from("latest version")))
}

/// Returns a list of available version information
pub fn available_versions() -> DataResult<Vec<String>> {
    let content = get_common_file(VERSIONS_FILE)?;
    serde_json::from_str::<Vec<String>>(&*content).map_err(DataError::from)
}
