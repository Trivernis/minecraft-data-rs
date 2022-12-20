use crate::api::tests::{get_api, get_test_versions};
use crate::models::block_collision_shapes::CollisionShapeIds;

#[test]
pub fn test_blocks_array() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        assert_ne!(api.blocks.blocks_array().unwrap().len(), 0)
    }
}

#[test]
pub fn test_blocks_by_state_id() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let by_state = api.blocks.blocks_by_state_id();
        assert!(by_state.is_ok());
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
        let by_id = api.blocks.blocks().unwrap();
        assert!(by_id.get(&1).is_some());
        assert!(by_id.get(&5).is_some());
    }
}

#[test]
pub fn test_block_states() {
    let versions = get_test_versions();

    for version in versions {
        let api = get_api(version);
        let by_name = api.blocks.blocks_by_name().unwrap();

        let air = by_name.get("air").unwrap();
        if let Some(states) = &air.states {
            // Air has no states.
            assert_eq!(states.len(), 0);
        }

        let water = by_name.get("water").unwrap();
        if let Some(states) = &water.states {
            // Water has states.
            assert_ne!(states.len(), 0);
        }
    }
}

#[test]
pub fn test_block_collision_states() {
    for version in get_test_versions() {
        let api = get_api(version);
        let shapes = api.blocks.block_collision_shapes().unwrap();

        for (_block, ids) in shapes.blocks {
            match ids {
                CollisionShapeIds::Value(id) => {
                    assert!(shapes.shapes.get(&id).is_some());
                }
                CollisionShapeIds::Array(ids) => {
                    for id in ids {
                        assert!(shapes.shapes.get(&id).is_some());
                    }
                }
            }
        }
    }
}
