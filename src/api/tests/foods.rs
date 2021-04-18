use crate::api::tests::{get_api, get_test_versions};

#[test]
pub fn test_foods_array() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        assert_ne!(api.foods.foods_array().unwrap().len(), 0)
    }
}

#[test]
pub fn test_foods_by_name() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let by_name = api.foods.foods_by_name().unwrap();
        assert!(by_name.get("bread").is_some());
        assert!(by_name.get("golden_carrot").is_some());
    }
}
