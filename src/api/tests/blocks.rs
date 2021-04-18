use crate::api::tests::{get_api, get_test_versions};

#[test]
pub fn test_blocks_array() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        assert_ne!(api.blocks.blocks_array().unwrap().len(), 0)
    }
}

#[test]
pub fn test_blocks_by_name() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let by_name = api.blocks.blocks_by_name().unwrap();
        assert!(by_name.get("dirt").is_some());
        assert!(by_name.get("stone").is_some());
        assert_eq!(by_name.get("grass").unwrap().stack_size, 64)
    }
}

#[test]
pub fn test_blocks_by_id() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let by_name = api.blocks.blocks().unwrap();
        assert!(by_name.get(&1).is_some());
        assert!(by_name.get(&5).is_some());
    }
}
