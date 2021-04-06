use crate::api::tests::{get_api, get_test_versions};

#[test]
pub fn test_items_array() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        assert_ne!(api.items.items_array().unwrap().len(), 0)
    }
}
