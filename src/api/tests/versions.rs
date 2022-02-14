use crate::api::versions::{latest_stable, versions, versions_by_minecraft_version};
use crate::Api;

#[test]
fn test_versions() {
    let unordered_versions = versions().unwrap();
    assert_ne!(unordered_versions.len(), 0)
}

#[test]
fn test_versions_by_minecraft_version() {
    let versions = versions_by_minecraft_version().unwrap();
    assert!(versions.get("1.16").is_some());
    assert!(versions.get("1.14.12").is_none());
    assert_eq!(versions.get("1.16.3").unwrap().major_version, "1.16")
}

#[test]
fn test_latest_stable_version() {
    assert!(latest_stable().is_ok());
    assert!(Api::latest().is_ok());
}
