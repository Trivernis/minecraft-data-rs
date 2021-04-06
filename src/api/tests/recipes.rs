use crate::api::tests::{get_api, get_test_versions};

#[test]
pub fn test_recipes() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let recipes = api.recipes.recipes().unwrap();
        let bread_id = api.items.items_by_name().unwrap().get("bread").unwrap().id;
        assert_ne!(recipes.len(), 0);
        assert!(recipes.get(&bread_id).is_some());
    }
}
