use crate::api::tests::{get_api, get_test_versions};

#[test]
pub fn test_enchantments_array() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let enchantments_array = api.enchantments.enchantments_array().unwrap();
        assert_ne!(enchantments_array.len(), 0);
    }
}

#[test]
pub fn test_enchantments() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let enchantments = api.enchantments.enchantments().unwrap();
        assert_ne!(enchantments.len(), 0);
    }
}

#[test]
pub fn test_enchantments_by_name() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let by_name = api.enchantments.enchantments_by_name().unwrap();
        assert!(by_name.get("unbreaking").is_some());
        assert!(by_name.get("protection").is_some());
    }
}

#[test]
pub fn test_enchantments_by_category() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let by_category = api.enchantments.enchantments_by_category().unwrap();
        assert!(by_category.get("breakable").is_some());
        assert_ne!(by_category.get("breakable").unwrap().len(), 0);
    }
}
