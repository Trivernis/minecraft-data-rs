use crate::api::tests::{get_api, get_test_versions};

#[test]
pub fn test_entity_loot_array() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        assert_ne!(api.loot.entity_loot_array().unwrap().len(), 0)
    }
}

#[test]
pub fn test_entity_loot_by_name() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let by_name = api.loot.entity_loot().unwrap();
        assert!(by_name.get("sheep").is_some());
        assert!(by_name.get("zombie").is_some());
    }
}

#[test]
pub fn test_block_loot_array() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        assert_ne!(api.loot.block_loot_array().unwrap().len(), 0)
    }
}

#[test]
pub fn test_block_loot_by_name() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let by_name = api.loot.block_loot().unwrap();
        assert!(by_name.get("poppy").is_some());
        assert!(by_name.get("stone").is_some());
    }
}
