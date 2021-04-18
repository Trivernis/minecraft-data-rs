use crate::api::tests::{get_api, get_test_versions};

#[test]
pub fn test_blocks_array() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        assert_ne!(api.entities.entities_array().unwrap().len(), 0)
    }
}

#[test]
pub fn test_entities_by_name() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let by_name = api.entities.entities_by_name().unwrap();
        assert!(by_name.get("cow").is_some());
        assert!(by_name.get("armor_stand").is_some());
    }
}

#[test]
pub fn test_entities_by_id() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let by_name = api.entities.entities().unwrap();
        assert!(by_name.get(&1).is_some());
        assert!(by_name.get(&5).is_some());
    }
}
