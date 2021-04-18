use crate::api::tests::{get_api, get_test_versions};

#[test]
pub fn test_biomes_array() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        assert_ne!(api.biomes.biomes_array().unwrap().len(), 0)
    }
}

#[test]
pub fn test_biomes_by_name() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let by_name = api.biomes.biomes_by_name().unwrap();
        assert!(by_name.get("ocean").is_some());
        assert!(by_name.get("river").is_some());
        assert_eq!(
            by_name.get("jungle").unwrap().dimension,
            "overworld".to_string()
        )
    }
}

#[test]
pub fn test_biomes_by_id() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let by_name = api.biomes.biomes().unwrap();
        assert!(by_name.get(&1).is_some());
        assert!(by_name.get(&5).is_some());
    }
}
