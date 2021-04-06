use crate::api::tests::{get_api, get_test_versions};

#[test]
pub fn test_items_array() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        assert_ne!(api.items.items_array().unwrap().len(), 0)
    }
}

#[test]
pub fn test_items_by_name() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let by_name = api.items.items_by_name().unwrap();
        assert!(by_name.get("bread").is_some());
        assert!(by_name.get("stone").is_some());
        assert_eq!(by_name.get("ender_pearl").unwrap().stack_size, 16)
    }
}
